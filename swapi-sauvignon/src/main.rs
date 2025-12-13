use std::sync::Arc;

use sauvignon_axum::{
    axum::{
        self,
        routing::{get, post},
        Extension, Router,
    },
    graphiql, graphql,
};
use tokio::net::TcpListener;

use shared::get_db_pool;
use swapi_sauvignon::get_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let schema = get_schema();
    let db_pool = get_db_pool().await?;

    let app = Router::new()
        .route("/graphql", post(graphql))
        .route("/graphiql", get(graphiql("/graphql")))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(db_pool));

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
