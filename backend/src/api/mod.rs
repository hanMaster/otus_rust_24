use crate::model::device::DeviceForAdd;
use crate::model::house::HouseForAdd;
use crate::model::room::RoomForAdd;
use crate::model::ModelManager;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use axum::{Json, Router};

pub fn routes_init(mm: ModelManager) -> Router {
    Router::new()
        .route("/house", get(get_house_list).post(add_house))
        .route(
            "/house/{id}",
            get(get_house_report).post(add_room).delete(remove_house),
        )
        .route("/house-room-list/{house_id}", get(get_room_list))
        .route(
            "/room/{id}",
            get(get_device_list_by_room)
                .post(add_device)
                .delete(remove_room),
        )
        .route("/device/{id}", delete(remove_device))
        .with_state(mm)
}

async fn get_house_report(
    State(mm): State<ModelManager>,
    Path(house_id): Path<i64>,
) -> impl IntoResponse {
    mm.house_report(house_id).await.map(Json)
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
    Path(house_id): Path<i64>,
    Json(data): Json<RoomForAdd>,
) -> impl IntoResponse {
    mm.create_room(house_id, data.room_name).await.map(Json)
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

async fn add_device(
    State(mm): State<ModelManager>,
    Path(room_id): Path<i64>,
    Json(data): Json<DeviceForAdd>,
) -> impl IntoResponse {
    mm.create_device(room_id, data).await.map(Json)
}

async fn get_device_list_by_room(
    State(mm): State<ModelManager>,
    Path(room_id): Path<i64>,
) -> impl IntoResponse {
    mm.devices_list(room_id).await.map(Json)
}

async fn remove_device(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    mm.delete_device(id).await.map(Json)
}
