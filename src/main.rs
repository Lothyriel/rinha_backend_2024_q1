use axum::Router;

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    db::grant_database_tables()?;
    db::seed_data()?;

    let app = Router::new().nest("/clientes/:id", routes::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
