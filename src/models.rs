use chrono::{DateTime, Utc};
use rusqlite::{
    types::{FromSql, FromSqlError},
    ToSql,
};

use crate::routes::ErrorResponse;

pub struct ClientData {
    pub id: ClientId,
    pub limit: u64,
    pub balance: i64,
}

pub type ClientId = u32;

#[derive(serde::Serialize)]
pub struct ExtractResponse {
    #[serde(alias = "saldo")]
    balance: ExtractData,
    #[serde(alias = "ultimas_transacoes")]
    transactions: Vec<TransactionData>,
}

#[derive(serde::Deserialize, Debug)]
pub struct TransactionRequest {
    #[serde(alias = "valor")]
    pub value: u64,
    #[serde(alias = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct TransactionResponse {
    #[serde(alias = "limite")]
    pub limit: u64,
    #[serde(alias = "saldo")]
    pub balance: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum TransactionType {
    #[serde(alias = "d")]
    Debit,
    #[serde(alias = "c")]
    Credit,
}

#[derive(serde::Serialize)]
pub struct ExtractData {
    #[serde(alias = "total")]
    total: u64,
    #[serde(alias = "data_extrato")]
    date: DateTime<Utc>,
    #[serde(alias = "limite")]
    limit: u64,
}

#[derive(serde::Serialize)]
pub struct TransactionData {
    #[serde(alias = "valor")]
    value: i64,
    #[serde(alias = "tipo")]
    transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    description: String,
    #[serde(alias = "realizada_em")]
    date: DateTime<Utc>,
}

#[derive(thiserror::Error, Debug)]
pub enum TransientError {
    #[error("{0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

impl From<rusqlite::Error> for ErrorResponse {
    fn from(value: rusqlite::Error) -> Self {
        value.into()
    }
}

impl FromSql for TransactionType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Integer(_)
            | rusqlite::types::ValueRef::Null => Err(FromSqlError::InvalidType),
            rusqlite::types::ValueRef::Blob(e) | rusqlite::types::ValueRef::Text(e) => {
                match e.first().ok_or_else(|| FromSqlError::InvalidType)? {
                    b'd' => Ok(TransactionType::Debit),
                    b'c' => Ok(TransactionType::Credit),
                    _ => Err(FromSqlError::OutOfRange(e[0] as i64)),
                }
            }
        }
    }
}

impl ToSql for TransactionType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let value = match self {
            TransactionType::Debit => "d",
            TransactionType::Credit => "c",
        };

        Ok(value.into())
    }
}
