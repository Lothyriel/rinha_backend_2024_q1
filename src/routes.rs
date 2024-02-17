use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use rusqlite::{Connection, OptionalExtension};

use crate::{db::get_connection, models::*};

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
) -> Result<Json<TransactionResponse>, ErrorResponse> {
    let conn = get_connection()?;

    let client = get_client(&conn, client_id)?;

    let new_balance = match request.transaction_type {
        TransactionType::Debit => {
            let new_balance = client.balance + client.limit as i64 - request.value as i64;

            if new_balance < 0 {
                return Err(ErrorResponse::NotEnoughLimit);
            } else {
                new_balance
            }
        }
        TransactionType::Credit => client.balance + request.value as i64,
    };

    conn.execute(
        "INSERT INTO transactions (
            client_id,
            value,
            type,
            description,
            date
         ) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            client_id,
            request.value,
            request.transaction_type,
            request.description,
            Utc::now(),
        ),
    )?;

    conn.execute("UPDATE clients SET balance = (?1);", [new_balance])?;

    Ok(Json(TransactionResponse {
        limit: client.limit,
        balance: new_balance,
    }))
}

async fn get_extract(
    Path(client_id): Path<ClientId>,
) -> Result<Json<ExtractResponse>, ErrorResponse> {
    let conn = get_connection()?;

    let client = get_client(&conn, client_id)?;

    todo!()
}

fn get_client(conn: &Connection, client_id: u32) -> Result<ClientData, ErrorResponse> {
    let client = conn
        .query_row(
            "SELECT
               balance,
               limit
             FROM account
             WHERE id = (?1);",
            [client_id],
            |row| {
                Ok(ClientData {
                    id: client_id,
                    balance: row.get(0)?,
                    limit: row.get(1)?,
                })
            },
        )
        .optional()?
        .ok_or_else(|| ErrorResponse::ClientNotFound(client_id))?;

    Ok(client)
}
