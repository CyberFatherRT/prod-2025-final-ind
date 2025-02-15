-- Add up migration script here

CREATE OR REPLACE FUNCTION get_best_ad(
    p_client_id UUID,
    p_current_date INT
)
RETURNS TABLE (
    campaign_id UUID,
    advertiser_id UUID,
    ad_title TEXT,
    ad_text TEXT
)
AS $$
BEGIN
    RETURN QUERY
    WITH campaign_stats AS (
        SELECT
            c.id AS campaign_id,
            c.advertiser_id,
            c.ad_title,
            c.ad_text,
            c.impressions_limit,
            c.clicks_limit,
            c.cost_per_impression,
            c.cost_per_click,

            -- Подсчёт уникальных показов и кликов для кампании:
            COALESCE(ai.impression_count, 0) AS impression_count,
            COALESCE(ac.click_count, 0) AS click_count,

            -- ML скор для пары (клиент, рекламодатель)
            ms.score AS ml_score,

            -- Вычисляем количество ошибок таргетирования:
            (
              (CASE WHEN c.gender IS NOT NULL
                         AND c.gender <> 'ALL'
                         AND c.gender::text <> cl.gender::text
                    THEN 1 ELSE 0 END) +
              (CASE WHEN c.age_from IS NOT NULL
                         AND cl.age < c.age_from
                    THEN 1 ELSE 0 END) +
              (CASE WHEN c.age_to IS NOT NULL
                         AND cl.age > c.age_to
                    THEN 1 ELSE 0 END) +
              (CASE WHEN c.location IS NOT NULL
                         AND c.location <> cl.location
                    THEN 1 ELSE 0 END)
            ) AS targeting_errors
        FROM campaigns c
        JOIN clients cl ON cl.id = p_client_id
        LEFT JOIN ml_scores ms
            ON ms.advertiser_id = c.advertiser_id AND ms.client_id = p_client_id
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
        WHERE p_current_date BETWEEN c.start_date AND c.end_date AND c.is_deleted = false
    ),
    computed AS (
        SELECT
            cs.*,

            -- Вычисляем базовый скор:
            0.5 * ((cs.impression_count * cs.cost_per_impression) + (cs.click_count * cs.cost_per_click))
            + 0.25 * COALESCE(cs.ml_score, 0)
            + 0.15 * (cs.impression_count / cs.impressions_limit)
            + 0.15 * (cs.click_count / cs.clicks_limit) AS base_score,

            -- Штраф за превышение лимита показов (если число показов больше лимита):
            CASE
                WHEN cs.impression_count > cs.impressions_limit THEN
                    0.05 * floor(((cs.impression_count / cs.impressions_limit) - 1) / 0.05)
                ELSE 0
            END AS penalty_impression,
            0.10 * cs.targeting_errors AS penalty_targeting
        FROM campaign_stats cs
    ),
    final_scores AS (
        SELECT
            campaign_id,
            advertiser_id,
            ad_title,
            ad_text,
            base_score,
            penalty_impression,
            penalty_targeting,
            base_score * (1 - (penalty_impression + penalty_targeting)) AS final_score
        FROM computed
    )
    SELECT
        campaign_id,
        advertiser_id,
        ad_title,
        ad_text
    FROM final_scores
    ORDER BY final_score DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

