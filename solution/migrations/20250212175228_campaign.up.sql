-- Add up migration script here

DO
$$
    BEGIN
        CREATE TYPE CAMPAIGN_GENDER AS ENUM ('MALE', 'FEMALE', 'ALL');
    EXCEPTION
        WHEN DUPLICATE_OBJECT THEN NULL;
    END;
$$;

CREATE TABLE IF NOT EXISTS campaigns
(
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    advertiser_id       UUID  NOT NULL,
    impressions_limit   INT   NOT NULL,
    clicks_limit        INT   NOT NULL,
    cost_per_impression FLOAT NOT NULL,
    cost_per_click      FLOAT NOT NULL,
    ad_title            TEXT  NOT NULL,
    ad_text             TEXT  NOT NULL,
    start_date          INT   NOT NULL,
    end_date            INT   NOT NULL,
    gender              CAMPAIGN_GENDER,
    age_from            INT,
    age_to              INT,
    location            TEXT,
    CONSTRAINT advertiser_id_fk FOREIGN KEY (advertiser_id) REFERENCES advertisers (id)
);

CREATE INDEX campaign_advertiser_id_idx ON campaigns (advertiser_id);
