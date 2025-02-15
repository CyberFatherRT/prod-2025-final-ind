use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    controllers::clients::ClientController, db::Db, errors::ProdError, forms::clients::ClientForm,
    models::clients::ClientModel, AppState,
};

/// Bulk insert/update of clients
#[utoipa::path(
    post,
    tag = "Clients",
    path = "/clients/bulk",
    responses(
        (status = 201, body = Vec<ClientModel>),
        (status = 400,  description = "Invalid request")
    ),
)]
pub async fn bulk(
    State(state): State<AppState>,
    Json(clients): Json<Vec<ClientForm>>,
) -> Result<(StatusCode, Json<Vec<ClientModel>>), ProdError> {
    clients.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;
    let clients = ClientModel::bulk(&mut conn, clients).await?;

    Ok((StatusCode::CREATED, Json(clients)))
}

/// Get client by ID
#[utoipa::path(
    get,
    tag = "Clients",
    path = "/clients/{client_id}",
    responses(
        (status = 201, body = ClientModel),
        (status = 400,  description = "Invalid request")
    ),
    params(
        ("client_id" = uuid::Uuid, Path, description = "Client ID"),
    )
)]
pub async fn get_client_by_id(
    State(state): State<AppState>,
    Path(client_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<ClientModel>), ProdError> {
    let mut conn = state.pool.conn().await?;
    let client = ClientModel::get_client_by_id(&mut conn, client_id).await?;

    Ok((StatusCode::OK, Json(client)))
}
