# 📊 Rust Parquet Query API with DataFusion & Apache Arrow

This project is a **Rust-based microservice** that exposes an **HTTP API to query a Parquet file** using [Apache Arrow](https://arrow.apache.org/) and [DataFusion](https://docs.rs/datafusion).

---

## 🚀 Features

- Load and query a local Parquet file with SQL via HTTP
- Powered by `DataFusion` for efficient in-memory analytics
- JSON-based REST API built on `Axum`

---

## 🛠 Technologies Used

- [Rust](https://www.rust-lang.org/) (2024 edition)
- [Apache Arrow](https://arrow.apache.org/) and [DataFusion](https://docs.rs/datafusion)
- [Tokio](https://tokio.rs/)
- [Axum](https://docs.rs/axum)

---

## 📂 Project Structure

```bash
.
├── data/
│   └── ejemplo.parquet         # Your Parquet file
├── src/
│   ├── api.rs                  # Axum router and endpoint handler
│   ├── config.rs               # Loads configuration from .env
│   ├── query.rs                # DataFusion SQL execution logic
│   └── main.rs                 # Application entrypoint
├── .env                        # Environment variables
├── Cargo.toml                 # Rust dependencies and metadata
```

---

## ⚙️ .env Configuration

Example `.env`:

```env
APP_PORT=3000
PARQUET_PATH=data/ejemplo.parquet
RUST_LOG=info
```

---

## ▶️ Running the Project

1. **Place your Parquet file** at `data/ejemplo.parquet` or update the path in `.env`.
2. Install Rust (if you haven't): https://rustup.rs
3. Run:

```bash
cargo run
```

It will start a server on:  
`http://localhost:3000`

---

## 🔎 Example Query

Endpoint: `POST /query`

### Request Body

```json
{
  "sql": "SELECT * FROM mi_tabla LIMIT 5"
}
```

### Example with `curl`

```bash
curl -X POST http://localhost:3000/query \
  -H "Content-Type: application/json" \
  -d '{ "sql": "SELECT * FROM mi_tabla LIMIT 5" }'
```

## 📈 Tracing and Logging

Enable more logs via `.env`:

```env
RUST_LOG=debug
```

---

## 🧾 Example Parquet Data

The sample file `data/ejemplo.parquet` contains the following data:

```json
[
  { "id": 1, "nombre": "Alice" },
  { "id": 2, "nombre": "Bob" },
  { "id": 3, "nombre": "Charlie" },
  { "id": 4, "nombre": "Diana" }
]
```

You can query it like this:

```json
{
  "sql": "SELECT * FROM mi_tabla WHERE id > 2"
}
```

Which would return:

```json
{
  "result": [
    { "id": 3, "nombre": "Charlie" },
    { "id": 4, "nombre": "Diana" }
  ]
}
```

## 📬 Future Ideas

- Support for multiple tables / dynamic uploads
- Support for joins or aggregations
- Web frontend for query input
- WASM version

---
