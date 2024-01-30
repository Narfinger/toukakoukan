use anyhow::{anyhow, Context, Result};
use password_auth::verify_password;
use serde::{ser::SerializeStruct, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use tower_sessions::Session;

use crate::{
    group::Group,
    types::{AppState, DBPool},
};

/// Users
#[derive(Debug, sqlx::FromRow, Clone, PartialEq, Eq)]
pub(crate) struct User {
    /// the id in the database
    pub(crate) id: i64,
    /// name of the user and login
    name: String,
    /// password hash of the user
    password_hash: String,
}

/// when we serialize the user we do NOT want to include the password
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("User", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

#[derive(Debug, FromRow)]
/// The result of the group queries. This needs to now have the complete user in because we don't want to leak passwords. Only used as an intermediary FromRow Type
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
    /// Gets a user from the current session
    pub(crate) async fn get_user_from_session(state: &AppState, session: &Session) -> Result<User> {
        if state.args.release {
            let user_id_val = session.get_value("user_id").await.unwrap();
            let user_id: i64 = serde_json::from_value(user_id_val.unwrap())?;
            let user = User::from_id(&state.pool, user_id).await?;
            Ok(user)
        } else {
            Ok(User::from_id(&state.pool, 1).await?)
        }
    }

    /// Checks if a user is in a group, i.e., is allowed to modify it
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

    /// gets a user from a username (to login)
    pub(crate) async fn get_user_from_username(pool: &DBPool, name: &str) -> Result<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM user where name=?")
            .bind(name)
            .fetch_one(pool)
            .await
            .context("Could not find user")
    }

    /// creates the user from a given id (used for session)
    async fn from_id(pool: &DBPool, id: i64) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM user where id=?")
            .bind(id)
            .fetch_one(pool)
            .await
            .context("Could not find user in db")
    }

    /// Gets the groups a user is in and returns the vector
    pub(crate) async fn groups(&self, pool: &DBPool) -> Result<Vec<Group>> {
        let user_id = self.id;

        let groups: Vec<GroupQueryResult> =
            sqlx::query_as::<_, GroupQueryResult>("SELECT user.id AS uid, user.name AS uname, expense_group.name AS egname, expense_group_id AS egid FROM ((user join expense_group_people ON expense_group_people.user_id = user.id) join expense_group on expense_group_people.expense_group_id = expense_group.id) WHERE expense_group_people.expense_group_id in (SELECT expense_group_id FROM expense_group_people WHERE user_id=?) ORDER BY egid")
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
        let mut res = map
            .into_iter()
            .map(|((group_id, group_name), users)| Group {
                id: group_id,
                name: group_name,
                users,
            })
            .collect::<Vec<Group>>();
        res.sort_by_cached_key(|g| g.id);
        Ok(res)
    }

    /// gets a specific group
    pub(crate) async fn get_specific_group(
        &self,
        pool: &DBPool,
        expense_group_id: i64,
    ) -> Result<Group> {
        let people =
        sqlx::query_as::<_, User>("SELECT user.id, user.name, user.password_hash FROM expense_group_people INNER JOIN user on expense_group_people.user_id = user.id WHERE expense_group_id = ?")
        .bind(expense_group_id)
        .fetch_all(pool)
        .await?;

        let group: (String,) =
            sqlx::query_as("SELECT name FROM expense_group WHERE id=? ORDER BY id")
                .bind(expense_group_id)
                .fetch_one(pool)
                .await?;

        Ok(Group {
            name: group.0,
            id: expense_group_id,
            users: people.iter().map(|u| u.name.clone()).collect(),
        })
    }

    /// check if a password matches for the user
    pub(crate) fn check_password(&self, given_password: &str) -> bool {
        verify_password(given_password, &self.password_hash).is_ok()
    }

    /// gets all users
    pub(crate) async fn get_all_users(pool: &DBPool) -> Result<Vec<User>> {
        sqlx::query_as("SELECT * FROM user")
            .fetch_all(pool)
            .await
            .map_err(|_| anyhow!("Error in Sql"))
    }
}

#[cfg(test)]
mod test {
    use crate::{group, types::DBPool, users::User};

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

    #[sqlx::test(migrations = "./migrations", fixtures("../fixtures/all.sql"))]
    async fn get_spcific_group(pool: DBPool) {
        let user = User::from_id(&pool, 1).await.expect("NO USER");
        let group_one = user.get_specific_group(&pool, 1).await.expect("NO GROUP 1");
        assert_eq!(group_one.name, "group1");
        let group_two = user.get_specific_group(&pool, 2).await.expect("NO GROUP 1");
        assert_eq!(group_two.name, "group2");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("../fixtures/all.sql"))]
    async fn get_total(pool: DBPool) {
        let user = User::from_id(&pool, 1).await.expect("NO USER");
        let group = user.get_specific_group(&pool, 2).await.expect("NO GROUP");
        let total = group.get_total(&pool).await.expect("NO TOTAL");
        assert_eq!(total, 0);

        // implemnt the other things
        assert!(false)
    }

    #[sqlx::test(migrations = "./migrations", fixtures("../fixtures/all.sql"))]
    async fn get_users(pool: DBPool) {
        let users = User::get_all_users(&pool).await.expect("NO USER");
        let check_users = vec![
            User {
                id: 1,
                name: String::from("test1"),
                password_hash: String::from("xx"),
            },
            User {
                id: 2,
                name: String::from("test2"),
                password_hash: String::from("xx"),
            },
            User {
                id: 3,
                name: String::from("test3"),
                password_hash: String::from("xx"),
            },
            User {
                id: 4,
                name: String::from("test4"),
                password_hash: String::from("xx"),
            },
        ];

        assert_eq!(users, check_users);
    }
}
