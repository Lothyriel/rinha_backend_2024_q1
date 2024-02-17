use chrono::{DateTime, Utc};

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
    value: u64,
    #[serde(alias = "tipo")]
    transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    description: String,
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
