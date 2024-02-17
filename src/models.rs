use chrono::{DateTime, Utc};

use async_sqlite::rusqlite::{
    self,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Error, ToSql,
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
    pub balance: ExtractData,
    #[serde(alias = "ultimas_transacoes")]
    pub transactions: Vec<TransactionData>,
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
    pub total: i64,
    #[serde(alias = "data_extrato")]
    pub date: DateTime<Utc>,
    #[serde(alias = "limite")]
    pub limit: u64,
}

#[derive(serde::Serialize)]
pub struct TransactionData {
    #[serde(alias = "valor")]
    pub value: i64,
    #[serde(alias = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    pub description: String,
    #[serde(alias = "realizada_em")]
    pub date: DateTime<Utc>,
}

#[derive(thiserror::Error, Debug)]
pub enum TransientError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Sql(#[from] SqlError),
}

#[derive(thiserror::Error, Debug)]
enum SqlError {
    #[error("{0}")]
    Sqlite(#[from] Error),
    #[error("{0}")]
    AsyncSqlite(#[from] async_sqlite::Error),
}

impl From<async_sqlite::Error> for TransientError {
    fn from(value: async_sqlite::Error) -> Self {
        value.into()
    }
}

impl From<async_sqlite::Error> for ErrorResponse {
    fn from(value: async_sqlite::Error) -> Self {
        value.into()
    }
}

impl FromSql for TransactionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Real(_) | ValueRef::Integer(_) | ValueRef::Null => {
                Err(FromSqlError::InvalidType)
            }
            ValueRef::Blob(e) | ValueRef::Text(e) => {
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
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            TransactionType::Debit => "d",
            TransactionType::Credit => "c",
        };

        Ok(value.into())
    }
}
