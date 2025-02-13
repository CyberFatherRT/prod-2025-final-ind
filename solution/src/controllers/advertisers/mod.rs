use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    errors::ProdError,
    forms::advertisers::{AdvertiserForm, MlScoreForm},
    map_vec,
    models::advertisers::AdvertiserModel,
};

pub trait AdvertiserContoller {
    async fn bulk(
        conn: &mut PgConnection,
        advertisers: Vec<AdvertiserForm>,
    ) -> Result<Vec<AdvertiserModel>, ProdError>;

    async fn get_advertiser_by_id(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
    ) -> Result<AdvertiserModel, ProdError>;

    async fn ml_scores(conn: &mut PgConnection, ml_score: MlScoreForm) -> Result<(), ProdError>;
}

impl AdvertiserContoller for AdvertiserModel {
    async fn bulk(
        conn: &mut PgConnection,
        advertisers: Vec<AdvertiserForm>,
    ) -> Result<Vec<AdvertiserModel>, ProdError> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO advertisers(id, name)
            SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[])
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name
            "#,
            &map_vec!(advertisers, id),
            &map_vec!(advertisers, name),
        )
        .execute(&mut *conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        let advertisers = advertisers.iter().map(|x| x.into()).collect();
        Ok(advertisers)
    }

    async fn get_advertiser_by_id(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
    ) -> Result<AdvertiserModel, ProdError> {
        let advertiser = sqlx::query_as!(
            AdvertiserModel,
            r#"
            SELECT id, name FROM advertisers
            WHERE id = $1
            "#,
            advertiser_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ProdError::NotFound("No advertiser was found with that id.".to_string())
            }
            err => ProdError::DatabaseError(err),
        })?;

        Ok(advertiser)
    }

    async fn ml_scores(conn: &mut PgConnection, ml_score: MlScoreForm) -> Result<(), ProdError> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO ml_scores(client_id, advertiser_id, score)
            VALUES ($1, $2, $3)
            ON CONFLICT (client_id, advertiser_id) DO UPDATE SET score = excluded.score
            "#,
            ml_score.client_id,
            ml_score.advertiser_id,
            ml_score.score
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
                ProdError::NotFound("No client or advertiser was found with these ids".to_string())
            }
            _ => ProdError::DatabaseError(err),
        })?;

        Ok(())
    }
}
