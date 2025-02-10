use crate::model::ModelManager;
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes_init(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/house",
            get(get_house_info).post(add_house).delete(remove_house),
        )
        .route("/room", post(add_room).delete(remove_room))
        .route("/device", post(add_device).delete(remove_device))
        .route("/device/{id}", get(get_device_info))
        .with_state(mm)
}

async fn get_house_info() -> Result<Json<Value>> {
    let body = Json(json!({"house_info": "my house"}));
    Ok(body)
}

#[derive(Deserialize)]
pub struct HouseForAdd {
    pub title: String,
}

#[debug_handler]
async fn add_house(
    State(mm): State<ModelManager>,
    Json(data): Json<HouseForAdd>,
) -> Result<Json<Value>> {
    mm.create_house(data.title).await?;
    let body = Json(json!({"add_house": "add_house"}));
    Ok(body)
}
async fn remove_house() -> Result<Json<Value>> {
    let body = Json(json!({"remove_house": "remove_house"}));
    Ok(body)
}
async fn add_room() -> Result<Json<Value>> {
    let body = Json(json!({"add_room": "add_room"}));
    Ok(body)
}
async fn remove_room() -> Result<Json<Value>> {
    let body = Json(json!({"remove_room": "remove_room"}));
    Ok(body)
}
async fn add_device() -> Result<Json<Value>> {
    let body = Json(json!({"add_device": "add_device"}));
    Ok(body)
}
async fn remove_device() -> Result<Json<Value>> {
    let body = Json(json!({"remove_device": "remove_device"}));
    Ok(body)
}
async fn get_device_info(Path(device_id): Path<i32>) -> Result<Json<Value>> {
    let body = Json(json!({"get_device_info": "get_device_info", "device_id": device_id}));
    Ok(body)
}
