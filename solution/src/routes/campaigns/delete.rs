use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{db::Db, errors::ProdError, AppState};

pub async fn route(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM campaigns
        WHERE advertiser_id = $1 AND id = $2
        "#,
        advertiser_id,
        campaign_id
    )
    .execute(&mut *conn)
    .await
    .map_err(ProdError::DatabaseError)?;

    if rows_affected.rows_affected() == 0 {
        return Err(ProdError::NotFound(format!(
            "No campaign was found with id - `{:?}` for advertiser - `{:?}`",
            campaign_id, advertiser_id
        ))
        .into());
    }

    Ok(StatusCode::NO_CONTENT)
}
