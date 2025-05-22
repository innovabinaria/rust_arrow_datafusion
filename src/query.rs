use arrow::array::*;
use arrow::datatypes::DataType;
use datafusion::prelude::*;
use serde_json::{json, Map, Number, Value};
use tracing::error;

/// Executes a SQL query on a Parquet file and returns JSON.
pub async fn ejecutar_consulta(sql: &str, parquet_path: &str) -> Value {
    let ctx = SessionContext::new();

    if let Err(e) = ctx
        .register_parquet("mi_tabla", parquet_path, ParquetReadOptions::default())
        .await
    {
        error!("Error loading Parquet: {}", e);
        return json!({ "error": format!("Failed registration: {}", e) });
    }

    let df = match ctx.sql(sql).await {
        Ok(df) => df,
        Err(e) => {
            error!("Invalid query: {}", e);
            return json!({ "error": format!("Invalid SQL: {}", e) });
        }
    };

    let batches = match df.collect().await {
        Ok(batches) => batches,
        Err(e) => {
            error!("Error when executing: {}", e);
            return json!({ "error": format!("Execution error: {}", e) });
        }
    };

    let results: Vec<Value> = batches
        .into_iter()
        .flat_map(batch_to_json_rows)
        .collect();

    json!({ "result": results })
}

/// Converts an Arrow RecordBatch to JSON rows.
fn batch_to_json_rows(batch: RecordBatch) -> Vec<Value> {
    let schema = batch.schema();
    let mut rows = Vec::with_capacity(batch.num_rows());

    for row_index in 0..batch.num_rows() {
        let mut row = Map::new();
        for (i, field) in schema.fields().iter().enumerate() {
            let value = column_value_to_json(&batch.columns()[i], row_index);
            row.insert(field.name().clone(), value);
        }
        rows.push(Value::Object(row));
    }

    rows
}

/// Convert a column cell to JSON.
fn column_value_to_json(array: &dyn Array, row_index: usize) -> Value {
    match array.data_type() {
        DataType::Utf8 => as_json::<StringArray, _>(array, row_index, |arr, i| {
            Value::String(arr.value(i).to_string())
        }),
        DataType::Int32 => as_json::<Int32Array, _>(array, row_index, |arr, i| {
            Value::Number(arr.value(i).into())
        }),
        DataType::Float64 => as_json::<Float64Array, _>(array, row_index, |arr, i| {
            Number::from_f64(arr.value(i)).map_or(Value::Null, Value::Number)
        }),
        DataType::Boolean => as_json::<BooleanArray, _>(array, row_index, |arr, i| {
            Value::Bool(arr.value(i))
        }),
        _ => Value::Null,
    }
}

/// Helper to extract JSON value from an Arrow array.
fn as_json<T, F>(array: &dyn Array, row_index: usize, f: F) -> Value
where
    T: Array + 'static,
    F: Fn(&T, usize) -> Value,
{
    let typed_array = array.as_any().downcast_ref::<T>().unwrap();
    if typed_array.is_null(row_index) {
        Value::Null
    } else {
        f(typed_array, row_index)
    }
}


