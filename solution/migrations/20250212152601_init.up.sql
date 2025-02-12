CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO
$$
    BEGIN
        CREATE TYPE GENDER AS ENUM ('MALE', 'FEMALE');
    EXCEPTION
        WHEN DUPLICATE_OBJECT THEN NULL;
    END
$$;

CREATE TABLE IF NOT EXISTS clients
(
    id       UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login    VARCHAR NOT NULL,
    age      INT     NOT NULL,
    location VARCHAR NOT NULL,
    gender   GENDER  NOT NULL
);

