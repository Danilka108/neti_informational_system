use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct Reply<Message, Data> {
    pub message: Message,
    pub data: Data,
}

#[derive(Debug)]
pub struct EmptyData;

impl<E: std::error::Error> From<E> for Reply<String, EmptyData> {
    fn from(value: E) -> Self {
        Self {
            message: value.to_string(),
            data: EmptyData,
        }
    }
}

impl<Message: serde::Serialize> IntoResponse for Reply<Message, EmptyData> {
    fn into_response(self) -> Response {
        Json(json!({
            "message": self.message,
        }))
        .into_response()
    }
}

impl<Message: serde::Serialize, Data: serde::Serialize> IntoResponse for Reply<Message, Data> {
    fn into_response(self) -> Response {
        Json(json!({
            "message": self.message,
            "data": self.data,
        }))
        .into_response()
    }
}
