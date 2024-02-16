use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let clients_router = Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract));

    let app = Router::new().nest("/clientes/:id", clients_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

type ClientId = u32;

#[derive(serde::Deserialize)]
struct TransactionRequest {
    #[serde(alias = "valor")]
    value: u64,
    #[serde(alias = "tipo")]
    transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    description: String,
}

#[derive(serde::Serialize)]
struct TransactionResponse {
    #[serde(alias = "limite")]
    limit: u64,
    #[serde(alias = "saldo")]
    balance: i64,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum TransactionType {
    #[serde(alias = "d")]
    Debit,
    #[serde(alias = "c")]
    Credit,
}

async fn add_transaction(
    Path(client_id): Path<ClientId>,
    Json(data): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, ErrorResponse> {
    todo!()
}

#[derive(serde::Serialize)]
struct ExtractResponse {
    #[serde(alias = "saldo")]
    balance: ExtractData,
    #[serde(alias = "ultimas_transacoes")]
    transactions: Vec<TransactionData>,
}

#[derive(serde::Serialize)]
struct ExtractData {
    #[serde(alias = "total")]
    total: u64,
    #[serde(alias = "realizada_em")]
    date: DateTime<Utc>,
    #[serde(alias = "limite")]
    limit: u64,
}

#[derive(serde::Serialize)]
struct TransactionData {
    #[serde(alias = "valor")]
    value: i64,
    #[serde(alias = "tipo")]
    transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    description: String,
    #[serde(alias = "realizada_em")]
    date: DateTime<Utc>,
}

async fn get_extract(
    Path(client_id): Path<ClientId>,
) -> Result<Json<ExtractResponse>, ErrorResponse> {
    todo!()
}

enum ErrorResponse {
    ClientNotFound(ClientId),
    NoBalance(TransactionRequest),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            ErrorResponse::ClientNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Client with id {{{}}} not found", id),
            ),
            ErrorResponse::NoBalance(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Not enough balance to complete this transaction".to_owned(),
            ),
        };

        error.into_response()
    }
}
