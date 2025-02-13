-- Add up migration script here

CREATE TABLE IF NOT EXISTS ad_impressions
(
    client_id       UUID,
    campaign_id     UUID,
    advertiser_id   UUID,
    impression_date INT,
    PRIMARY KEY (client_id, campaign_id),
    FOREIGN KEY (client_id) REFERENCES clients (id),
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id),
    FOREIGN KEY (advertiser_id) REFERENCES advertisers (id)
);

CREATE INDEX IF NOT EXISTS ad_impressions_campaign_id_idx ON ad_impressions (campaign_id);
CREATE INDEX IF NOT EXISTS ad_impressions_advertiser_id_idx ON ad_impressions (advertiser_id);


CREATE TABLE IF NOT EXISTS ad_clicks
(
    client_id       UUID,
    campaign_id     UUID,
    advertiser_id   UUID,
    click_date INT,
    PRIMARY KEY (client_id, campaign_id),
    FOREIGN KEY (client_id) REFERENCES clients (id),
    FOREIGN KEY (campaign_id) REFERENCES campaigns (id),
    FOREIGN KEY (advertiser_id) REFERENCES advertisers (id)
);

CREATE INDEX IF NOT EXISTS ad_clicks_campaign_id_idx ON ad_clicks (campaign_id);
CREATE INDEX IF NOT EXISTS ad_clicks_advertiser_id_idx ON ad_clicks (advertiser_id);

