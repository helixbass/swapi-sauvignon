use std::sync::Arc;

use sauvignon_axum::{
    axum::{self, routing::post, Extension, Router},
    graphql,
};
use tokio::net::TcpListener;

use swapi_sauvignon::get_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = get_schema();

    let app = Router::new()
        .route("/graphql", post(graphql))
        .layer(Extension(Arc::new(schema)));

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
