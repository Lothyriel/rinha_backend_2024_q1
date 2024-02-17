use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{self, Result},
    models::*,
    AppState,
};

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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract))
}

async fn add_transaction(
    State(state): State<AppState>,
    Path(client_id): Path<ClientId>,
    Json(request): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    state
        .db_pool
        .conn(move |conn| Ok(handlers::add_transaction(conn, client_id, request).map(Json)))
        .await?
}

async fn get_extract(
    State(state): State<AppState>,
    Path(client_id): Path<ClientId>,
) -> Result<Json<ExtractResponse>> {
    state
        .db_pool
        .conn(move |conn| Ok(handlers::get_extract(conn, client_id).map(Json)))
        .await?
}
