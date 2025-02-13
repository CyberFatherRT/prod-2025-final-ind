-- Add up migration script here

CREATE OR REPLACE FUNCTION get_best_ad(p_client_id UUID, p_current_date INT)
RETURNS TABLE (
    campaign_id UUID,
    advertiser_id UUID ,
    ad_title TEXT,
    ad_text TEXT
)
AS $$
BEGIN
    RETURN QUERY
    WITH campaigns_data AS (
        SELECT
            c.id AS campaign_id,
            c.advertiser_id,
            c.ad_title,
            c.ad_text,
            (c.impressions_limit * c.cost_per_impression + c.clicks_limit * c.cost_per_click) AS revenue,
            CASE
                WHEN (
                    (c.gender IS NULL OR c.gender = 'ALL' OR c.gender::text = cl.gender::text)
                    AND (c.age_from IS NULL OR cl.age >= c.age_from)
                    AND (c.age_to IS NULL OR cl.age <= c.age_to)
                    AND (c.location IS NULL OR c.location = cl.location)
                )
                THEN TRUE
                ELSE FALSE
            END AS is_targeted
        FROM campaigns c
        JOIN clients cl ON cl.id = p_client_id
        WHERE p_current_date BETWEEN c.start_date AND c.end_date
    ),
    best_targeted AS (
        SELECT MAX(revenue) AS best_targeted_revenue
        FROM campaigns_data
        WHERE is_targeted = TRUE
    )
    SELECT
        cd.campaign_id,
        cd.advertiser_id,
        cd.ad_title,
        cd.ad_text
    FROM campaigns_data cd
    CROSS JOIN best_targeted bt
    WHERE
         cd.is_targeted = TRUE
         OR (cd.is_targeted = FALSE AND (bt.best_targeted_revenue IS NULL OR cd.revenue > bt.best_targeted_revenue))
    ORDER BY cd.revenue DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

