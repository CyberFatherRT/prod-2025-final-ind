-- Add down migration script here

DROP FUNCTION IF EXISTS get_ad_stats(p_campaign_id UUID);
DROP FUNCTION IF EXISTS get_advertiser_stats(p_advertiser_id UUID);
DROP FUNCTION IF EXISTS get_daily_stats_campaign(p_campaign_id UUID);
DROP FUNCTION IF EXISTS get_daily_stats_advertiser(p_advertiser_id UUID);
