mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), models::TransientError> {
    db::grant_database_tables()?;
    db::seed_data()?;

    let app = axum::Router::new().nest("/clientes/:id", routes::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
