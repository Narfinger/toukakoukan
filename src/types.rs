use std::collections::HashMap;

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use axum_macros::FromRef;
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasArguments, encode::IsNull, prelude::Type, sqlite::SqliteTypeInfo, Database,
    Decode, Encode, FromRow, Pool, Sqlite, SqlitePool,
};
use time::{OffsetDateTime, PrimitiveDateTime};

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

/// An expense
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub(crate) struct Expense {
    /// the id of the expense
    pub(crate) id: i64,
    /// payed type
    pub(crate) payed_type: PayedType,
    /// amount
    pub(crate) amount: i64,
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

#[derive(Clone, FromRef)]
pub(crate) struct AppState {
    pub(crate) pool: Pool<Sqlite>,
}
