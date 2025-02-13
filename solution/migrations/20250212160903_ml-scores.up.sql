-- Add up migration script here

CREATE TABLE IF NOT EXISTS ml_scores
(
    client_id     UUID,
    advertiser_id UUID,
    score         INT,
    FOREIGN KEY (client_id) REFERENCES clients (id),
    FOREIGN KEY (advertiser_id) REFERENCES advertisers (id),
    PRIMARY KEY (client_id, advertiser_id)
);

CREATE INDEX IF NOT EXISTS ml_scores_client_id_idx ON ml_scores (client_id);
CREATE INDEX IF NOT EXISTS ml_scores_advertiser_id_idx ON ml_scores (advertiser_id);
