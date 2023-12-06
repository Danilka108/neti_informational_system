use axum::routing::post;
use axum::{extract::State, Router};
use axum_macros::debug_handler;
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::AppState;

use crate::entities::universities;
use crate::utils::{json_with_rejection::JsonWithRejection, res};

pub fn handlers() -> Router<AppState> {
    Router::new().route("/", post(create_univeristy))
}

#[derive(Debug, Clone, Deserialize)]
struct CreatePayload {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
struct CreateResult {
    id: i32,
    name: String,
}

#[debug_handler]
async fn create_univeristy(
    State(state): State<AppState>,
    JsonWithRejection(payload): JsonWithRejection<CreatePayload>,
) ->  {
    let university = universities::ActiveModel {
        name: Set(payload.name.clone()),
        ..Default::default()
    };

    let res = universities::Entity::insert(university)
        .exec(&state.conn)
        .await;

    match res {
        Ok(res) => res::success(res),
        _ => todo!(),
    }

    // let a = match res {
    //     Ok(res) => Json(json!({
    //         "state": "ok",
    //         "data": {
    //             "id": res.last_insert_id,
    //             "name": payload.name,
    //         }
    //     })),
    //     Err(err) => Json(json!({})),
    // };
}
