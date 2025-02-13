-- Add up migration script here

CREATE OR REPLACE FUNCTION get_ad_stats(p_campaign_id UUID)
RETURNS TABLE (
  impressions_count INT,
  clicks_count INT,
  conversion FLOAT,
  spent_impressions FLOAT,
  spent_clicks FLOAT,
  spent_total FLOAT
)
AS $$
DECLARE
  v_imp_count INT;
  v_cl_count INT;
BEGIN
  SELECT COUNT(DISTINCT client_id)
  INTO v_imp_count
  FROM ad_impressions
  WHERE campaign_id = p_campaign_id;

  SELECT COUNT(DISTINCT client_id)
  INTO v_cl_count
  FROM ad_clicks
  WHERE campaign_id = p_campaign_id;

  RETURN QUERY
  SELECT
    v_imp_count AS impressions_count,
    v_cl_count AS clicks_count,
    CASE
      WHEN v_imp_count > 0 THEN (v_cl_count / v_imp_count * 100)
      ELSE 0
    END AS conversion,
    v_imp_count * c.cost_per_impression AS spent_impressions,
    v_cl_count * c.cost_per_click AS spent_clicks,
    (v_imp_count * c.cost_per_impression) + (v_cl_count * c.cost_per_click) AS spent_total
  FROM campaigns c
  WHERE c.id = p_campaign_id;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION get_advertiser_stats(p_advertiser_id UUID)
RETURNS TABLE (
  impressions_count INT,
  clicks_count INT,
  conversion FLOAT,
  spent_impressions FLOAT,
  spent_clicks FLOAT,
  spent_total FLOAT
)
AS $$
BEGIN
  RETURN QUERY
  WITH campaign_data AS (
    SELECT
      c.id,
      c.cost_per_impression,
      c.cost_per_click,
      COALESCE(ai.impression_count, 0) AS imp_count,
      COALESCE(ac.click_count, 0) AS click_count
    FROM campaigns c
    LEFT JOIN (
      SELECT campaign_id, COUNT(DISTINCT client_id) AS impression_count
      FROM ad_impressions
      GROUP BY campaign_id
    ) ai ON ai.campaign_id = c.id
    LEFT JOIN (
      SELECT campaign_id, COUNT(DISTINCT client_id) AS click_count
      FROM ad_clicks
      GROUP BY campaign_id
    ) ac ON ac.campaign_id = c.id
    WHERE c.advertiser_id = p_advertiser_id
  )
  SELECT
    SUM(imp_count) AS impressions_count,
    SUM(click_count) AS clicks_count,
    CASE WHEN SUM(imp_count) > 0
         THEN SUM(click_count) / SUM(imp_count) * 100
         ELSE 0
    END AS conversion,
    SUM(imp_count * cost_per_impression) AS spent_impressions,
    SUM(click_count * cost_per_click) AS spent_clicks,
    SUM(imp_count * cost_per_impression + click_count * cost_per_click) AS spent_total
  FROM campaign_data;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION get_daily_stats_campaign(p_campaign_id UUID)
RETURNS TABLE (
  date INT,
  impressions_count INT,
  clicks_count INT,
  conversion FLOAT,
  spent_impressions FLOAT,
  spent_clicks FLOAT,
  spent_total FLOAT
)
AS $$
BEGIN
  RETURN QUERY
  WITH dates AS (
    SELECT impression_date AS date
    FROM ad_impressions
    WHERE campaign_id = p_campaign_id
    UNION
    SELECT click_date AS date
    FROM ad_clicks
    WHERE campaign_id = p_campaign_id
  ),
  imp AS (
    SELECT impression_date AS date, COUNT(DISTINCT client_id) AS impressions_count
    FROM ad_impressions
    WHERE campaign_id = p_campaign_id
    GROUP BY impression_date
  ),
  clk AS (
    SELECT click_date AS date, COUNT(DISTINCT client_id) AS clicks_count
    FROM ad_clicks
    WHERE campaign_id = p_campaign_id
    GROUP BY click_date
  )
  SELECT
    d.date,
    COALESCE(i.impressions_count, 0) AS impressions_count,
    COALESCE(c.clicks_count, 0) AS clicks_count,
    CASE WHEN COALESCE(i.impressions_count, 0) > 0
         THEN COALESCE(c.clicks_count, 0) / i.impressions_count * 100
         ELSE 0
    END AS conversion,
    COALESCE(i.impressions_count, 0) * cpn.cost_per_impression AS spent_impressions,
    COALESCE(c.clicks_count, 0) * cpn.cost_per_click AS spent_clicks,
    (COALESCE(i.impressions_count, 0) * cpn.cost_per_impression
     + COALESCE(c.clicks_count, 0) * cpn.cost_per_click) AS spent_total
  FROM dates d
  LEFT JOIN imp i ON i.date = d.date
  LEFT JOIN clk c ON c.date = d.date
  JOIN campaigns cpn ON cpn.id = p_campaign_id
  ORDER BY d.date;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION get_daily_stats_advertiser(p_advertiser_id UUID)
RETURNS TABLE (
  date INT,
  impressions_count INT,
  clicks_count INT,
  conversion FLOAT,
  spent_impressions FLOAT,
  spent_clicks FLOAT,
  spent_total FLOAT
)
AS $$
BEGIN
  RETURN QUERY
  WITH advertiser_campaigns AS (
    SELECT id, cost_per_impression, cost_per_click
    FROM campaigns
    WHERE advertiser_id = p_advertiser_id
  ),
  daily_imps AS (
    SELECT ai.campaign_id, ai.impression_date AS date, COUNT(DISTINCT ai.client_id) AS impressions_count
    FROM ad_impressions ai
    WHERE ai.campaign_id IN (SELECT id FROM advertiser_campaigns)
    GROUP BY ai.campaign_id, ai.impression_date
  ),
  daily_clicks AS (
    SELECT ac.campaign_id, ac.click_date AS date, COUNT(DISTINCT ac.client_id) AS clicks_count
    FROM ad_clicks ac
    WHERE ac.campaign_id IN (SELECT id FROM advertiser_campaigns)
    GROUP BY ac.campaign_id, ac.click_date
  ),
  combined AS (
    SELECT
      COALESCE(di.campaign_id, dc.campaign_id) AS campaign_id,
      COALESCE(di.date, dc.date) AS date,
      COALESCE(di.impressions_count, 0) AS impressions_count,
      COALESCE(dc.clicks_count, 0) AS clicks_count
    FROM daily_imps di
    FULL OUTER JOIN daily_clicks dc
      ON di.campaign_id = dc.campaign_id AND di.date = dc.date
  )
  SELECT
    date,
    SUM(impressions_count) AS impressions_count,
    SUM(clicks_count) AS clicks_count,
    CASE WHEN SUM(impressions_count) > 0
         THEN SUM(clicks_count) / SUM(impressions_count) * 100
         ELSE 0
    END AS conversion,
    SUM(impressions_count * ac.cost_per_impression) AS spent_impressions,
    SUM(clicks_count * ac.cost_per_click) AS spent_clicks,
    SUM(impressions_count * ac.cost_per_impression + clicks_count * ac.cost_per_click) AS spent_total
  FROM combined
  JOIN advertiser_campaigns ac ON ac.id = combined.campaign_id
  GROUP BY date
  ORDER BY date;
END;
$$ LANGUAGE plpgsql;

