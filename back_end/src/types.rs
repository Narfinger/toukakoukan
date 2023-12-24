use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasArguments, encode::IsNull, prelude::Type, sqlite::SqliteTypeInfo, Database,
    Decode, Encode,
};
use sqlx::{FromRow, Pool, Sqlite};
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) pool: Pool<Sqlite>,
}

impl<'q> Decode<'q, Sqlite> for PayedType {
    fn decode(
        value: <Sqlite as sqlx::database::HasValueRef<'q>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<Sqlite>>::decode(value)?;

        let mut it = value.split_whitespace();
        let first = it.next().ok_or("not enough whitespace")?;
        let snd = it.next().ok_or("Not enough whitespace")?.parse()?;
        match first {
            "EVEN" => Ok(PayedType::EvenSplit(snd)),
            "OWED" => Ok(PayedType::OwedTotal(snd)),
            &_ => Err("NOT".into()),
        }
    }
}

/// What type of payment we have
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PayedType {
    /// the split is even and `usize` payed
    EvenSplit(usize),
    /// `usize' owes the total
    OwedTotal(usize),
}

impl<'q> Encode<'q, Sqlite> for PayedType {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let bytes = match self {
            PayedType::EvenSplit(index) => format!("EVEN {}", index),
            PayedType::OwedTotal(index) => format!("OWED {}", index),
        };
        <&String as Encode<Sqlite>>::encode(&bytes, buf)
    }
}

impl Type<Sqlite> for PayedType {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <&String as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&String as Type<Sqlite>>::compatible(ty)
    }
}

/// An expense
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct Expense {
    /// the id of the expense
    pub(crate) id: i64,
    /// payed type
    pub(crate) payed_type: PayedType,
    /// amount
    pub(crate) amount: i64,
    /// name
    pub(crate) name: String,
    /// time the expense was created
    pub(crate) time: OffsetDateTime,
}

/// An Expense Group, a collection of people and expenses
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct ExpenseGroup {
    /// the id of the expense group
    pub(crate) id: i64,
    /// the people that are in the expense group
    pub(crate) people: Vec<String>,
    /// the list of expenses
    pub(crate) expenses: Vec<Expense>,
    /// the name of the expense group
    pub(crate) name: String,
}

/// Users
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    /// name of the user
    pub(crate) name: String,
    /// password hash of the user
    pub(crate) password_hash: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct Group {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) users: Vec<String>,
}

pub(crate) const GROUP_QUERY_STRING: &str = "SELECT group.id, user.id, user.name, group.name FROM (expense_group_people JOIN expense_group JOIN user) WHERE expense_group_id in (
    SELECT expense_group_id from expense_group_people WHERE user_id = ?) GROUP BY group.id";
#[derive(Debug, FromRow)]
pub(crate) struct GroupQueryResult {
    group_id: i64,
    user_id: i64,
    user_name: String,
    group_name: String,
}

pub(crate) fn group_query_result_to_group_query_return(
    groups: Vec<GroupQueryResult>,
) -> Vec<GroupQueryReturn> {
    let mut map: HashMap<(i64, String), Vec<String>> = HashMap::new();
    for i in groups {
        map.entry((i.group_id, i.group_name))
            .or_default()
            .push(i.user_name);
    }
    map.into_iter()
        .map(|((group_id, group_name), users)| GroupQueryReturn {
            group_id,
            group_name,
            users,
        })
        .collect::<Vec<GroupQueryReturn>>()
}

#[derive(Debug, Serialize)]
pub(crate) struct GroupQueryReturn {
    pub(crate) group_id: i64,
    pub(crate) group_name: String,
    pub(crate) users: Vec<String>,
}
