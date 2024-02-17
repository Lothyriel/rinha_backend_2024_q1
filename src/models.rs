use axum::{http::StatusCode, response::IntoResponse};

use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct ExtractResponse {
    #[serde(alias = "saldo")]
    balance: ExtractData,
    #[serde(alias = "ultimas_transacoes")]
    transactions: Vec<TransactionData>,
}

pub type ClientId = u32;

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
pub enum ErrorResponse {
    #[error("{0}")]
    IO(#[from] IoError),
    #[error("Client with id {{0}} not found")]
    ClientNotFound(ClientId),
    #[error("Not enough balance to complete this transaction")]
    NoBalance,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            ErrorResponse::ClientNotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::NoBalance => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorResponse::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Sqlite(#[from] rusqlite::Error),
}
