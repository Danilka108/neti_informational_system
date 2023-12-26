mod auth;
mod curriculums;
mod persons;
mod study_groups;
mod subdivisions;
mod universities;
mod user;

use crate::{api_state::ApiState, utils::provide_req_scope_module};
use axum::{middleware, Router};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .nest("/universities", universities::router())
        .nest("/curriculums", curriculums::router())
        .nest("/persons", persons::router())
        .nest("/study_groups", study_groups::router())
        .nest("/subdivisions", subdivisions::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            provide_req_scope_module,
        ))
        .with_state(state)
}
