use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension};

#[tokio::main]
async fn main() -> Result<(), IoError> {
    grant_database_tables()?;
    seed_data()?;

    let clients_router = Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract));

    let app = Router::new().nest("/clientes/:id", clients_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

fn seed_data() -> Result<Connection, IoError> {
    let conn = get_connection()?;

    let clients_limits = [
        (1, 100000),
        (2, 80000),
        (3, 1000000),
        (4, 10000000),
        (5, 500000),
    ];

    for (id, limit) in clients_limits {
        conn.execute(
            "INSERT INTO clients (id, limit, balance) VALUES (?, ?, ?)",
            (id, limit, 0),
        )?;
    }

    Ok(())
}

fn grant_database_tables() -> Result<Connection, IoError> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE account (
             id INTEGER PRIMARY KEY,
             limit INTEGER NOT NULL,
             balance INTEGER NOT NULL
        );",
        (),
    )?;

    conn.execute(
        "CREATE TABLE transactions (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             client_id INTEGER NOT NULL,
             value REAL NOT NULL,
             type TEXT NOT NULL,
             description TEXT NOT NULL,
             date DATETIME NOT NULL
        );",
        (),
    )?;

    Ok(())
}

fn get_connection() -> Result<Connection, IoError> {
    const DATABASE_FILE: &str = "rinha.db";
    Ok(Connection::open(DATABASE_FILE)?)
}

type ClientId = u32;

#[derive(serde::Deserialize, Debug)]
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

#[derive(thiserror::Error, Debug)]
enum ErrorResponse {
    #[error("{0}")]
    IO(#[from] IoError),
    #[error("Client with id {{0}} not found")]
    ClientNotFound(ClientId),
    #[error("Not enough balance to complete this transaction")]
    NoBalance,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            ErrorResponse::ClientNotFound(id) => StatusCode::NOT_FOUND,
            ErrorResponse::NoBalance => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorResponse::IO(e) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
enum IoError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Sqlite(#[from] rusqlite::Error),
}
