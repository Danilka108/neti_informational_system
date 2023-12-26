use app::university;
use axum::{debug_handler, extract::Path, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde_json::json;
use utils::di::Module;

use crate::utils::{extractors::ReqScopeModule, ApiResult, CommonState};

pub fn router<S: CommonState>() -> Router<S> {
    Router::new()
        .route("/", axum::routing::get(get_infos))
        .route("/:id", axum::routing::get(get_info))
}

#[debug_handler]
async fn get_infos(ReqScopeModule(module): ReqScopeModule) -> ApiResult {
    let repo = module.adapters.resolve::<university::BoxedRepo>();

    let entities = match repo.list().await {
        Ok(entities) => entities
            .into_iter()
            .map(|e| json!({ "universityId": e.id.value.to_string(), "universityName": e.name.to_string() }))
            .collect::<Vec<_>>(),
        Err(_) => {
            let msg = Json(json!({
                "message": "universities not found",
            }));
            return ApiResult::new((StatusCode::BAD_REQUEST, msg).into_response());
        }
    };

    ApiResult::new((StatusCode::OK, Json(entities)))
}

#[debug_handler]
async fn get_info(ReqScopeModule(module): ReqScopeModule, Path(id): Path<i32>) -> ApiResult {
    let repo = module.adapters.resolve::<university::BoxedRepo>();

    let val = match repo.find(university::EntityId::new(id)).await {
        Ok(Some(entity)) => json!({"name": entity.name.to_string() }),
        Ok(None) => {
            let msg = Json(json!({
                "message": "university not found",
            }));
            return ApiResult::new((StatusCode::BAD_REQUEST, msg).into_response());
        }
        Err(_) => {
            let msg = Json(json!({
                "message": "university not found",
            }));
            return ApiResult::new((StatusCode::BAD_REQUEST, msg).into_response());
        }
    };

    ApiResult::new((StatusCode::OK, Json(val)))
}
