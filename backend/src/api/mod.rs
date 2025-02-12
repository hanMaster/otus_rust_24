use crate::model::house::HouseForAdd;
use crate::model::room::RoomForAdd;
use crate::model::ModelManager;
use crate::Result;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde_json::{json, Value};

pub fn routes_init(mm: ModelManager) -> Router {
    Router::new()
        .route("/house", get(get_house_list).post(add_house))
        .route("/house/{id}", get(get_house_report).delete(remove_house))
        .route("/house-room-list/{house_id}", get(get_room_list))
        .route("/room", post(add_room))
        .route("/room/{id}", delete(remove_room))
        .route(
            "/room-device-list/{room_id}",
            post(add_room).delete(remove_room),
        )
        .route("/device", post(add_device).delete(remove_device))
        .route("/device/{id}", get(get_device_info))
        .with_state(mm)
}

// FIXME
async fn get_house_report(Path(house_id): Path<i64>) -> Result<Json<Value>> {
    let body = Json(json!({"house_info": house_id}));
    Ok(body)
}

async fn add_house(
    State(mm): State<ModelManager>,
    Json(data): Json<HouseForAdd>,
) -> impl IntoResponse {
    mm.create_house(data.name).await.map(Json)
}

async fn get_house_list(State(mm): State<ModelManager>) -> impl IntoResponse {
    mm.houses_list().await.map(Json)
}

async fn remove_house(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    mm.delete_house(id).await.map(Json)
}

async fn add_room(
    State(mm): State<ModelManager>,
    Json(data): Json<RoomForAdd>,
) -> impl IntoResponse {
    mm.create_room(data.house_id, data.room_name)
        .await
        .map(Json)
}

async fn get_room_list(
    State(mm): State<ModelManager>,
    Path(house_id): Path<i64>,
) -> impl IntoResponse {
    mm.rooms_list(house_id).await.map(Json)
}

async fn remove_room(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    mm.delete_room(id).await.map(Json)
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
