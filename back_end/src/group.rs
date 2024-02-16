use crate::types::{CreateGroupJson, DBPool, PayedType, SafeUser};
use anyhow::{Context, Result};
use futures::{future::join_all, stream::iter, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tokio::stream;
use tracing::info;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Result for JSON for returning a group
pub(crate) struct Group {
    /// the group id
    pub(crate) id: i64,
    /// the name of the group
    pub(crate) name: String,
    /// a vector of names of the users in the group
    pub(crate) users: Vec<SafeUser>,
}

/// TODO currently ignores the owed full amount
async fn get_payed(pool: &DBPool, group_id: i64, user: &SafeUser) -> (i64, i64) {
    let t = PayedType::EvenSplit(user.id as usize);
    let payed_half: (i64,) = sqlx::query_as(
        "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = ?",
    )
    .bind(group_id)
    .bind(t)
    .fetch_one(pool)
    .await
    .unwrap();
    /*let owed_full: (i64,) = sqlx::query_as(
            "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = OWED ?",
        )
        .bind(group_id)
        .bind(user.id)
        .fetch_one(pool)
        .await
        .unwrap();
    */
    (user.id, (payed_half.0 / 2))
}

pub(crate) struct GetTotalResult {
    total: i64,
    users: Vec<(i64, i64)>,
}

impl GetTotalResult {
    pub(crate) fn find_id(&self, user_id: i64) -> i64 {
        let user_payed = self
            .users
            .iter()
            .find(|(id, total)| id == &user_id)
            .unwrap_or(&(0, 0));

        self.total - user_payed.0
    }
}

impl Group {
    pub(crate) async fn get_total(&self, pool: &DBPool) -> Result<GetTotalResult> {
        let payed: Vec<(i64, i64)> =
            join_all(self.users.iter().map(|u| get_payed(pool, self.id, u))).await;
        let total: i64 = payed.iter().map(|(_, total)| total).sum();

        Ok(GetTotalResult {
            total,
            users: payed,
        })
    }

    pub(crate) async fn create_group(group: CreateGroupJson, pool: &DBPool) -> Result<()> {
        info!("Trying to insert group {:?}", group);
        let group_id: (i64,) = query_as("INSERT INTO expense_group (name) VALUES (?) RETURNING id")
            .bind(group.name)
            .fetch_one(pool)
            .await?;
        for i in group.users {
            query("INSERT INTO expense_group_people (expense_group_id, user_id) VALUES (?,?)")
                .bind(group_id.0)
                .bind(i)
                .execute(pool)
                .await?;
        }
        Ok(())
    }
}
