use crate::Result;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::extract::Path;
use serde_json::{json, Value};

pub fn routes_init() -> Router {
    Router::new()
        .route("/house", get(get_house_info).post(add_room).delete(remove_room))
        .route("/device", post(add_device).delete(remove_device))
        .route("/device/{id}", get(get_device_info))
}

async fn get_house_info() -> Result<Json<Value>> {
    let body = Json(json!({"house_info": "my house"}));
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