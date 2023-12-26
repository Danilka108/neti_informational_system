use std::collections::{HashMap, HashSet};

use anyhow::Context;
use app::{
    curriculum, curriculum_module, discipline, person, student, study_group, subdivision, teacher,
};
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
    let repo = module.adapters.resolve::<person::BoxedRepo>();

    let entities = match repo.list().await {
        Ok(entities) => entities
            .into_iter()
            .map(|e| json!({ "id": e.id.value.to_string(), "name": e.full_name.to_string() }))
            .collect::<Vec<_>>(),
        Err(err) => {
            dbg!(err);
            let msg = Json(json!({
                "message": "persons not found",
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
                "message": "person not found",
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
    let repo = module.adapters.resolve::<person::BoxedRepo>();
    let student_repo = module.adapters.resolve::<student::BoxedRepo>();
    let teacher_reop = module.adapters.resolve::<teacher::BoxedRepo>();
    let study_group_repo = module.adapters.resolve::<study_group::BoxedRepo>();
    let subdivision_repo = module.adapters.resolve::<subdivision::BoxedRepo>();

    let person = repo.find(Id::new(id)).await?.context("")?;

    let mut roles = Vec::new();

    let students = student_repo.list_by_person(person.id).await?;
    for student in students {
        let study_group = study_group_repo
            .find(student.study_group_id)
            .await?
            .context("")?;

        roles.push(json!({
            "role": "student",
            "studyGroupId": study_group.id.value,
            "studyGroupName": study_group.name,
        }));
    }

    if let Some(teacher) = dbg!(teacher_reop.find_by_person_id(person.id).await)? {
        let department = subdivision_repo
            .find(teacher.department_id)
            .await?
            .context("")?;
        roles.push(json!({
            "role": "teacher",
            "departmentId": department.id.value,
            "departmentName": department.name,
        }));
    }

    let subdivisions = subdivision_repo.list_by_members([person.id].into()).await?;
    for subdivision in subdivisions {
        let member = subdivision
            .members
            .into_iter()
            .find(|v| v.person_id == person.id)
            .unwrap();

        roles.push(json!({
            "role": "subdivisionMember",
            "subdivisionId": subdivision.id.value,
            "subdivisionName": subdivision.name,
            "subdivisionRole": member.role,
        }));
    }

    Ok(json!({
        "fullName": person.full_name,
        "roles": roles,
    }))
}
