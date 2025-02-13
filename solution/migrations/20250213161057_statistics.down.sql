-- Add down migration script here

DROP INDEX IF EXISTS ad_impressions_campaign_id_idx;
DROP INDEX IF EXISTS ad_impressions_advertiser_id_idx;
DROP TABLE IF EXISTS ad_impressions;

DROP INDEX IF EXISTS ad_clicks_campaign_id_idx;
DROP INDEX IF EXISTS ad_clicks_advertiser_id_idx;
DROP TABLE IF EXISTS ad_clicks;

