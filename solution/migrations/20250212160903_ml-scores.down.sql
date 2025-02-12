-- Add down migration script here

DROP INDEX IF EXISTS ml_score_client_id_idx;
DROP INDEX IF EXISTS ml_score_advertiser_id_idx;

DROP TABLE IF EXISTS ml_scores;
