use axum::Json;
use serde::Serialize;
use serde_json::json;

pub fn success<D: Serialize>(data: D) -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "data": data,
    }))
}

pub fn default_success() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "data": "null",
    }))
}

pub fn fail<D: Serialize>(data: D) -> Json<serde_json::Value> {
    Json(json!({
        "status": "fail",
        "data": data,
    }))
}

pub fn error<M: Into<String>>(msg: M) -> Json<serde_json::Value> {
    Json(json!({
        "status": "error",
        "message": msg.into(),
    }))
}

pub fn error_with_data<M: Into<String>, D: Serialize>(msg: M, data: D) -> Json<serde_json::Value> {
    Json(json!({
        "status": "error",
        "message": msg.into(),
        "data": data,
    }))
}
