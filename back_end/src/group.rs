use crate::{
    types::{CreateGroupJson, DBPool, Expense, PayedType, SafeUser},
    users::User,
};
use anyhow::Result;
use futures::{
    future::{join_all, ready},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
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

/// collection users and how much they owe
#[derive(Debug)]
pub(crate) struct OwedAmount<'a> {
    /// the user
    user: &'a SafeUser,
    /// how much they have payed
    owed: i64,
}

/// the basic owed amount result
#[derive(Debug)]
pub(crate) struct GetTotalResult<'a> {
    /// the total amount that is in the pool
    total: i64,
    /// the users and the value they owe
    owed: Vec<OwedAmount<'a>>,
}

impl<'a> GetTotalResult<'a> {
    /// returns how much the user with the `user_id` owes
    pub(crate) fn user_owes(&self, user_id: i64) -> i64 {
        self.owed
            .iter()
            .find(|payed_amount| payed_amount.user.id == user_id)
            .map(|payed_amount| payed_amount.owed)
            .unwrap_or(0)
    }
}

impl Group {
    /// returns the result who owes how much in the group
    pub(crate) async fn get_oweds(&self, pool: &DBPool) -> Result<GetTotalResult> {
        let expenses: Vec<Expense> = query_as("SELECT * FROM expense WHERE expense_group_id = ?")
            .bind(self.id)
            .fetch_all(pool)
            .await
            .unwrap();
        panic!("build way more testcases");
        let number_of_members: i64 = self.users.len() as i64;
        let mut how_much_owed = vec![0 as i64; self.users.len()];
        let mut total = 0;
        for exp in expenses {
            println!("how_much_owed {:?}", how_much_owed);
            println!("exp type {:?}", exp);
            match exp.payed_type {
                PayedType::EvenSplit(i) => {
                    total += exp.amount;
                    how_much_owed[i] -= exp.amount / number_of_members;
                    for j in 0..number_of_members {
                        if j != i as i64 {
                            how_much_owed[j as usize] += exp.amount / number_of_members;
                        }
                    }
                }
                PayedType::OwedTotal(i) => {
                    total += exp.amount;
                    how_much_owed[i] += exp.amount;
                }
            }
        }

        println!("owedtable {:?}", how_much_owed);
        let owed = how_much_owed
            .into_iter()
            .enumerate()
            .map(|(index, owed)| OwedAmount {
                user: &self.users[index],
                owed: owed,
            })
            .collect();
        println!("owed {:?}", owed);
        Ok(GetTotalResult { total, owed })
    }

    pub(crate) async fn create_group(group: CreateGroupJson, pool: &DBPool) -> Result<()> {
        info!("Trying to insert group {:?}", group);
        let group_id: (i64,) = query_as("INSERT INTO expense_group (name) VALUES (?) RETURNING id")
            .bind(group.name)
            .fetch_one(pool)
            .await?;
        for i in group.users {
            query!(
                "INSERT INTO expense_group_people (expense_group_id, user_id) VALUES (?,?)",
                group_id.0,
                i
            )
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub(crate) fn has_user_id(&self, other_id: i64) -> bool {
        self.users.iter().any(|u| u.id == other_id)
    }
}
