use std::collections::{HashMap, HashSet};

use anyhow::Context;
use app::{curriculum, curriculum_module, discipline, person, student, study_group, subdivision};
use axum::{debug_handler, extract::Path, response::IntoResponse, routing::post, Json, Router};
use http::StatusCode;
use serde_json::json;
use utils::{di::Module, entity::Id};

use crate::utils::{extractors::ReqScopeModule, ApiResult, CommonState};

pub fn router<S: CommonState>() -> Router<S> {
    Router::new()
        .route("/", axum::routing::get(get_infos))
        .route("/:id", axum::routing::get(get_info))
}

#[debug_handler]
async fn get_infos(ReqScopeModule(module): ReqScopeModule) -> ApiResult {
    let repo = module.adapters.resolve::<subdivision::BoxedRepo>();

    let entities = match repo.list().await {
        Ok(entities) => entities
            .into_iter()
            .map(|e| json!({ "id": e.id.value.to_string(), "name": e.name.to_string() }))
            .collect::<Vec<_>>(),
        Err(_) => {
            let msg = Json(json!({
                "message": "subdivisions not found",
            }));
            return ApiResult::new((StatusCode::BAD_REQUEST, msg).into_response());
        }
    };

    ApiResult::new((StatusCode::OK, Json(entities)))
}

#[debug_handler]
async fn get_info(module: ReqScopeModule, Path(id): Path<i32>) -> ApiResult {
    let val = match load_info(module, id).await {
        Ok(val) => Json(val),
        Err(err) => {
            dbg!(err);
            let msg = Json(json!({
                "message": "subdivision not found",
            }));
            return ApiResult::new((StatusCode::BAD_REQUEST, msg).into_response());
        }
    };

    ApiResult::new((StatusCode::OK, val))
}

async fn load_info(
    ReqScopeModule(module): ReqScopeModule,
    id: i32,
) -> Result<serde_json::Value, anyhow::Error> {
    let repo = module.adapters.resolve::<subdivision::BoxedRepo>();
    let person_repo = module.adapters.resolve::<person::BoxedRepo>();

    let subdivision = repo.find(Id::new(id)).await?.context("")?;

    let mut members = Vec::new();
    for member in subdivision.members {
        let person = person_repo.find(member.person_id).await?.context("")?;
        members.push(json!({
            "personId": person.id.value,
            "fullName": person.full_name,
            "role": member.role,
        }));
    }

    Ok(json!({
        "name": subdivision.name,
        "members": members,
    }))
}
