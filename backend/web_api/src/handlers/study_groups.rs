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
    let repo = module.adapters.resolve::<study_group::BoxedRepo>();

    let entities = match repo.list().await {
        Ok(entities) => entities
            .into_iter()
            .map(|e| json!({ "id": e.id.value.to_string(), "name": e.name.to_string() }))
            .collect::<Vec<_>>(),
        Err(_) => {
            let msg = Json(json!({
                "message": "curriculums not found",
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
                "message": "study group not found",
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
    let curriculum_repo = module.adapters.resolve::<curriculum::BoxedRepo>();
    let student_repo = module.adapters.resolve::<student::BoxedRepo>();
    let person_repo = module.adapters.resolve::<person::BoxedRepo>();
    let repo = module.adapters.resolve::<study_group::BoxedRepo>();

    let study_group = repo.find(Id::new(id)).await?.context("")?;

    let mut curriculums = Vec::new();
    for curriculum in study_group.curriculums {
        let val = curriculum_repo.find(curriculum).await?.context("")?;
        curriculums.push(json!({
            "id": val.id.value,
            "name": val.name,
        }));
    }

    let mut students = Vec::new();
    for student in student_repo.list_by_study_group(study_group.id).await? {
        let person = person_repo.find(student.person_id).await?.context("")?;
        students.push(json!({
            "personId": student.person_id.value,
            "fullName": person.full_name,
        }));
    }

    Ok(json!({
        "name": study_group.name,
        "curriculums": curriculums,
        "students": students,
    }))
    // let curriculum_module_repo = module.adapters.resolve::<curriculum_module::BoxedRepo>();
    // let discipline_repo = module.adapters.resolve::<discipline::BoxedRepo>();
    // let subdivison_repo = module.adapters.resolve::<subdivision::BoxedRepo>();

    // let curriculum = repo.find(Id::new(id)).await?.context("")?;
    // let study_groups = study_group_repo
    //     .list_by_curriculums([Id::new(id)].into())
    //     .await?
    //     .into_iter()
    //     .map(|v| {
    //         json!({
    //             "id": v.id.value,
    //             "name": v.name,
    //         })
    //     })
    //     .collect::<Vec<_>>();
    // let modules = curriculum_module_repo
    //     .list_by_curriculum_id(Id::new(id))
    //     .await?;
    // let mut semesters = HashMap::new();

    // for module in modules {
    //     let discipline = discipline_repo
    //         .find(module.discipline_id)
    //         .await?
    //         .context("")?;
    //     let department = subdivison_repo
    //         .find(discipline.department_id)
    //         .await?
    //         .context("")?;

    //     semesters
    //         .entry(module.semester)
    //         .or_insert(vec![])
    //         .push(json!({
    //             "disciplineName": discipline.name.to_string(),
    //             "departmentName": department.name.to_string(),
    //             "departmentId": department.id.value.to_string(),
    //         }));
    // }

    // let semesters: Vec<_> = semesters
    //     .into_iter()
    //     .map(|(k, v)| {
    //         json!({
    //             "value": k,
    //             "modules": v,
    //         })
    //     })
    //     .collect();

    // Ok(json!({
    //     "name": curriculum.name,
    //     "studyGroups": study_groups,
    //     "semesters": semesters,
    // }))
}
