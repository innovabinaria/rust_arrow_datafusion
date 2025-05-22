use std::env;

pub struct AppConfig {
    pub port: u16,
    pub parquet_path: String,
    pub rust_log: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .expect("APP_PORT must be a number");

        let parquet_path = env::var("PARQUET_PATH")
            .unwrap_or_else(|_| "data/ejemplo.parquet".into());

        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".into());

        Self { port, parquet_path, rust_log }
    }
}
