CREATE TABLE IF NOT EXISTS advertisers
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);
