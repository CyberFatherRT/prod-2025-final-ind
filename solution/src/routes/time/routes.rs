use axum::{extract::State, Json};
use redis::AsyncCommands;

use crate::{
    db::Rclient, errors::ProdError, forms::time::TimeForm, models::time::TimeModel, AppState,
};

pub async fn set_date(
    State(state): State<AppState>,
    Json(date): Json<TimeForm>,
) -> Result<Json<TimeModel>, ProdError> {
    let mut rclient = state.rclient.conn().await?;
    let current_date: i32 = rclient
        .set("date", date.current_date.unwrap_or(0))
        .await
        .map_err(ProdError::RedisError)?;

    Ok(Json(TimeModel { current_date }))
}
