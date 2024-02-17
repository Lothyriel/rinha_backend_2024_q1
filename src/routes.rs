use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{handlers, models::*};

type Result<T> = core::result::Result<T, ErrorResponse>;

#[derive(thiserror::Error, Debug)]
pub enum ErrorResponse {
    #[error("{0}")]
    Transient(#[from] TransientError),
    #[error("Client with id {{0}} not found")]
    ClientNotFound(ClientId),
    #[error("Not enough limit to complete this transaction")]
    NotEnoughLimit,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            ErrorResponse::ClientNotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::NotEnoughLimit => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorResponse::Transient(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self).into_response()
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract))
}

async fn add_transaction(
    Path(client_id): Path<ClientId>,
    Json(request): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    handlers::add_transaction(client_id, request)
        .await
        .map(Json)
}

async fn get_extract(Path(client_id): Path<ClientId>) -> Result<Json<ExtractResponse>> {
    handlers::get_extract(client_id).await.map(Json)
}
