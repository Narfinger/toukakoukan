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
            sqlx::query_as::<_, GroupQueryResult>(GROUP_QUERY_STRING)
                .bind(user_id)
                .fetch_all(pool)
                .await
                .context("Query did not get group results")?;

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

pub(crate) const GROUP_QUERY_STRING: &str = "SELECT expense_group.id AS egid, user.id AS uid, user.name AS uname, expense_group.name AS egname FROM (expense_group_people JOIN expense_group JOIN user) WHERE expense_group_id IN (
    SELECT expense_group_id from expense_group_people WHERE user_id = ?) GROUP BY expense_group.id";
#[derive(Debug, FromRow)]
struct GroupQueryResult {
    #[sqlx(rename = "egid")]
    group_id: i64,
    #[sqlx(rename = "uid")]
    user_id: i64,
    #[sqlx(rename = "uname")]
    user_name: String,
    #[sqlx(rename = "egname")]
    group_name: String,
}

#[derive(Debug, Serialize)]
struct GroupQueryReturn {
    pub(crate) group_id: i64,
    pub(crate) group_name: String,
    pub(crate) users: Vec<String>,
}

#[cfg(test)]
mod test {

    use anyhow::{Context, Result};
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
        let groups = user.groups(&pool).await.expect("NO GROUPS");
        println!("groups {:?}", groups);
        assert_eq!(groups[0].name, "group1");
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].users[0], "test1");
        assert_eq!(groups[0].users[1], "test2");
        assert_eq!(groups[1].users[0], "test1");
    }
}
