use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasArguments, encode::IsNull, prelude::Type, sqlite::SqliteTypeInfo, Database,
    Decode, Encode,
};
use sqlx::{Pool, Sqlite};
use time::serde::iso8601;
use time::OffsetDateTime;

pub(crate) type DBPool = Pool<Sqlite>;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) pool: DBPool,
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
#[serde(tag = "t", content = "c")]
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
    #[serde(with = "iso8601")]
    pub(crate) time: OffsetDateTime,
    /// the expense group id
    pub(crate) expense_group_id: i64,
}

/// An Expense Group, a collection of people and expenses
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct ExpenseGroup {
    /// the id of the expense group
    pub(crate) id: i64,
    /// the name of the expense group
    pub(crate) name: String,
}

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
