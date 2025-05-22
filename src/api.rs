use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;
use std::sync::Arc;

use crate::{config::AppConfig, query::ejecutar_consulta};

#[derive(Deserialize)]
pub struct QueryRequest {
    pub sql: String,
}

pub fn build_router(config: Arc<AppConfig>) -> Router {
    Router::new().route(
        "/query",
        post(move |Json(payload): Json<QueryRequest>| {
            let config = config.clone();
            async move {
                let result = ejecutar_consulta(&payload.sql, &config.parquet_path).await;
                Json(result)
            }
        }),
    )
}
