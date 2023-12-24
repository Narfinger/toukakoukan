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
    /// the expense groups they have
    pub(crate) groups: Vec<ExpenseGroup>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct Group {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) users: Vec<String>
}

pub(crate) const GROUP_QUERY_STRING: &str = "SELECT id,expense_group_id, user_id, groups.name, users.name, password_hash FROM groups INNER JOIN expense_group_people INNER JOIN users WHERE users.id = ?";
/// this is the wrong one because it doesn't give us all the users and just on user that matchs.
#[derive(Debug, FromRow)]
pub(crate) struct GroupQueryResult {
    id: u32,
    expense_group_id: u32,
    user_id: u32,
    group_name: String,
    user_name: String,
    password_hash: String,
}

impl From<GroupQueryResult> for Group {
    fn from(value: GroupQueryResult) -> Self {
        Group {
            id: value.id,
            name: value.group_name,
        }
    }
}
