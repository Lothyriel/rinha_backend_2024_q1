use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use rusqlite::OptionalExtension;

use crate::{db::get_connection, models::*};

pub fn router() -> Router {
    Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract))
}

async fn add_transaction(
    Path(client_id): Path<ClientId>,
    Json(data): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, ErrorResponse> {
    let conn = get_connection()?;

    let data = conn
        .query_row(
            "SELECT
               balance,
               limit
             FROM account
             WHERE id = :client_id;",
            [client_id],
            |row| {
                Ok(TransactionResponse {
                    balance: row.get(0)?,
                    limit: row.get(1)?,
                })
            },
        )
        .optional();

    todo!()
}

async fn get_extract(
    Path(client_id): Path<ClientId>,
) -> Result<Json<ExtractResponse>, ErrorResponse> {
    let conn = get_connection()?;

    let extract = conn.prepare("");

    todo!()
}
