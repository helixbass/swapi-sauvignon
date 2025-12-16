use std::sync::Arc;

use sauvignon_axum::{axum, simple_app};
use tokio::net::TcpListener;

use shared::get_db_pool;
use swapi_sauvignon::get_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = get_schema();
    let database = get_database().await?;

    axum::serve(
        TcpListener::bind("0.0.0.0:3001").await?,
        simple_app(Arc::new(schema), Arc::new(database)),
    )
    .await?;

    Ok(())
}
