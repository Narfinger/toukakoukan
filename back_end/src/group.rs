use crate::types::DBPool;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Result for JSON for returning a group
pub(crate) struct Group {
    /// the group id
    pub(crate) id: i64,
    /// the name of the group
    pub(crate) name: String,
    /// a vector of names of the users in the group
    pub(crate) users: Vec<String>,
}

impl Group {
    pub(crate) async fn get_total(&self, pool: &DBPool) -> Result<i64> {
        let even0: (i64,) = sqlx::query_as(
            "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = ?",
        )
        .bind(self.id)
        .bind("EVEN 0")
        .fetch_one(pool)
        .await
        .context("even0")?;

        let owed0: (i64,) = sqlx::query_as(
            "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = ?",
        )
        .bind(self.id)
        .bind("OWED 0")
        .fetch_one(pool)
        .await
        .context("OWED0")?;
        let even1: (i64,) = sqlx::query_as(
            "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = ?",
        )
        .bind(self.id)
        .bind("EVEN 0")
        .fetch_one(pool)
        .await
        .context("EVEN 0")?;

        let owed1: (i64,) = sqlx::query_as(
            "SELECT SUM(amount) FROM expense WHERE expense_group_id = ? AND payed_type = ?",
        )
        .bind(self.id)
        .bind("EVEN 1")
        .fetch_one(pool)
        .await
        .context("EVEN 1")?;
        Ok(even0.0 / 2 + owed0.0 - even1.0 / 2 - owed1.0)
    }
}
