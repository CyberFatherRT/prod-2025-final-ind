-- Add down migration script here

DROP INDEX IF EXISTS campaign_advertiser_id_idx;
DROP TABLE IF EXISTS campaigns;
DROP TYPE IF EXISTS CAMPAIGN_GENDER;
