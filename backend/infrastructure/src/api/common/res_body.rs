use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct ResBody<Message, Data> {
    pub message: Message,
    pub data: Option<Data>,
}

impl<Message: Into<String>, Data: serde::Serialize> IntoResponse for ResBody<Message, Data> {
    fn into_response(self) -> Response {
        let body = if let Some(data) = self.data {
            json!({
                "message": self.message.into(),
                "data": data,
            })
        } else {
            json!({
                "message": self.message.into(),
            })
        };

        Json(body).into_response()
    }
}
