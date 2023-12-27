use anyhow::{Context, Result};
use password_auth::verify_password;
use sqlx::FromRow;
use std::collections::HashMap;
use tower_sessions::Session;

use crate::types::{DBPool, Group};

/// Users
#[derive(Debug, sqlx::FromRow)]
pub(crate) struct User {
    /// the id in the database
    pub(crate) id: i64,
    /// name of the user and login
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
    pub(crate) async fn get_user_from_session(pool: &DBPool, session: &Session) -> Result<User> {
        let user_id_val = session.get_value("user_id").await.unwrap();
        let user_id: i64 = serde_json::from_value(user_id_val.unwrap())?;
        let user = User::from_id(&pool, user_id.into()).await?;
        Ok(user)
    }

    pub(crate) async fn in_group(&self, pool: &DBPool, group_id: u32) -> bool {
        let q = sqlx::query(
            "SELECT user_id FROM expense_group_people WHERE user_id=? AND expense_group_id = ?",
        )
        .bind(self.id)
        .bind(group_id)
        .fetch_optional(pool)
        .await;
        q.is_ok()
    }

    pub(crate) async fn get_user_from_username(pool: &DBPool, name: &str) -> Result<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM user where name=?")
            .bind(name)
            .fetch_one(pool)
            .await
            .context("Could not find user")
    }

    pub(crate) async fn from_id(pool: &DBPool, id: i64) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user where id=?")
            .bind(id)
            .fetch_one(pool)
            .await
            .context("Could not find user in db")
    }

    pub(crate) async fn groups(&self, pool: &DBPool) -> Result<Vec<Group>> {
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

    pub(crate) fn check_password(&self, given_password: &str) -> bool {
        verify_password(given_password, &self.password_hash).is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::{types::DBPool, users::User};

    #[sqlx::test(migrations = "./migrations/", fixtures("../fixtures/all.sql"))]
    async fn get_user(pool: DBPool) {
        //let pool = setup_db_connection().await.expect("NO DB");
        let user = User::from_id(&pool, 1).await.expect("NO USER");

        assert_eq!(user.name, "test1");
    }

    #[sqlx::test(migrations = "./migrations/", fixtures("../fixtures/all.sql"))]
    async fn get_users_groups(pool: DBPool) {
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
