mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), models::TransientError> {
    let pool = async_sqlite::PoolBuilder::new()
        .path("rinha.db")
        .journal_mode(async_sqlite::JournalMode::Wal)
        .open()
        .await?;

    pool.conn(|conn| {
        db::grant_database_tables(conn)?;
        db::seed_data(conn)?;

        Ok(())
    })
    .await?;

    let app = axum::Router::new()
        .with_state(AppState::new(pool))
        .nest("/clientes/:id", routes::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    db_pool: async_sqlite::Pool,
}

impl AppState {
    fn new(db_pool: async_sqlite::Pool) -> Self {
        Self { db_pool }
    }
}
