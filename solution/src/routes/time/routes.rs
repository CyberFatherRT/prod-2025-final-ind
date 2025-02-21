use axum::{extract::State, Json};
use redis::AsyncCommands;

use crate::{
    db::Rclient, errors::ProdError, forms::time::TimeForm, models::time::TimeModel, AppState,
};

/// Set the current date
#[utoipa::path(
    post,
    tag = "Time",
    path = "/time/advance",
    responses(
        (status = 200, body = TimeModel)
    )
)]
pub async fn set_date(
    State(state): State<AppState>,
    Json(date): Json<TimeForm>,
) -> Result<Json<TimeModel>, ProdError> {
    let mut rclient = state.rclient.conn().await?;
    rclient
        .set::<_, _, ()>("date", date.current_date.unwrap_or(0))
        .await
        .map_err(ProdError::RedisError)?;

    let current_date = rclient.get("date").await.map_err(ProdError::RedisError)?;

    Ok(Json(TimeModel { current_date }))
}
