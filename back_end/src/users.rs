use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use std::collections::HashMap;

use crate::types::Group;

/// Users
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    /// the id in the database
    id: i64,
    /// name of the user
    name: String,
    /// password hash of the user
    password_hash: String,
}

#[derive(Debug, FromRow)]
struct GroupQueryResult {
    #[sqlx(rename = "uid")]
    user_id: i64,
    #[sqlx(rename = "uname")]
    user_name: String,
    #[sqlx(rename = "egname")]
    group_name: String,
    #[sqlx(rename = "egid")]
    group_id: i64,
}

impl User {
    pub(crate) async fn from_id(pool: &Pool<Sqlite>, id: i64) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user where id=?")
            .bind(id)
            .fetch_one(pool)
            .await
            .context("Could not find user in db")
    }

    pub(crate) async fn groups(&self, pool: &Pool<Sqlite>) -> Result<Vec<Group>> {
        let user_id = self.id;

        let groups: Vec<GroupQueryResult> =
            sqlx::query_as::<_, GroupQueryResult>("SELECT user.id AS uid, user.name AS uname, expense_group.name AS egname, expense_group_id AS egid FROM ((user join expense_group_people ON expense_group_people.user_id = user.id) join expense_group on expense_group_people.expense_group_id = expense_group.id) WHERE expense_group_people.expense_group_id in (SELECT expense_group_id FROM expense_group_people WHERE user_id=?)")
                .bind(user_id)
                .fetch_all(pool)
                .await
                .context("Could not find groups")?;

        let mut map: HashMap<(i64, String), Vec<String>> = HashMap::new();
        for i in groups {
            map.entry((i.group_id, i.group_name))
                .or_default()
                .push(i.user_name);
        }
        Ok(map
            .into_iter()
            .map(|((group_id, group_name), users)| Group {
                id: group_id,
                name: group_name,
                users,
            })
            .collect::<Vec<Group>>())
    }
}

#[cfg(test)]
mod test {
    use sqlx::{Pool, Sqlite};

    use crate::users::User;

    #[sqlx::test(migrations = "./migrations/", fixtures("../fixtures/all.sql"))]
    async fn get_user(pool: Pool<Sqlite>) {
        //let pool = setup_db_connection().await.expect("NO DB");
        let user = User::from_id(&pool, 1).await.expect("NO USER");

        assert_eq!(user.name, "test1");
    }

    #[sqlx::test(migrations = "./migrations/", fixtures("../fixtures/all.sql"))]
    async fn get_users_groups(pool: Pool<Sqlite>) {
        let user = User::from_id(&pool, 1).await.expect("NO USER");
        let mut groups = user.groups(&pool).await.expect("NO GROUPS");
        groups.sort();
        println!("groups {:?}", groups);
        assert_eq!(groups[0].name, "group1");
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].users[0], "test1");
        assert_eq!(groups[0].users[1], "test2");
        assert_eq!(groups[0].users.len(), 2);
        assert_eq!(groups[1].name, "group2");
        assert_eq!(groups[1].users[0], "test1");
        assert_eq!(groups[1].users[1], "test2");
        assert_eq!(groups[1].users.len(), 2);
    }
}
