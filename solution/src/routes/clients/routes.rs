use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use validator::Validate;

use crate::{
    controllers::clients::ClientController,
    db::Db,
    errors::ProdError,
    models::clients::{ClientModel, GenderModel},
    AppState,
};

pub async fn bulk(
    State(state): State<AppState>,
    Json(clients): Json<Vec<ClientModel>>,
) -> Result<(StatusCode, Json<Vec<ClientModel>>), Response<String>> {
    clients.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;
    let clients = ClientModel::bulk(&mut *conn, clients).await?;

    Ok((StatusCode::CREATED, Json(clients)))
}

pub async fn get_client_by_id(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<ClientModel>), Response<String>> {
    let mut conn = state.pool.conn().await?;
    let client = ClientModel::get_client_by_id(&mut *conn, client_id).await?;

    Ok((StatusCode::OK, Json(client)))
}
