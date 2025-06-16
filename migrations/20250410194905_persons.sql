CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS UserSettings (
	id SERIAL PRIMARY KEY,

	per_page SMALLINT NULL
);

CREATE TABLE IF NOT EXISTS Users (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(100) NOT NULL,
	settings_id INT NOT NULL REFERENCES UserSettings(id) ON DELETE CASCADE,

        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS users_name_idx ON Users(name);


CREATE TABLE IF NOT EXISTS KnownFromSources (
    source_id SERIAL PRIMARY KEY,             -- Auto-incrementing integer PK
    user_id UUID NOT NULL,
    source_name VARCHAR(100) NOT NULL, -- Name of the source (e.g., 'Work')
    description TEXT NULL,                     -- Optional longer description
    location_search TEXT NULL,				-- Optional location that gets appended when selecting this KnownFromSource

    UNIQUE (user_id, source_name),

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES Users(id)
        ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS Persons (
    id SERIAL PRIMARY KEY,             -- Auto-incrementing integer PK

    user_id UUID NOT NULL,

    -- === Name ===
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,

    known_from_source_id INT NULL,             -- Foreign Key to KnownFromSources

    -- === Coordinates ===
    coordinate_with_search JSONB NULL,
	-- Coordinates of place plus the place-search itself since the user might want to see the place he search exactly the way he searched it for

    -- =========================================

    -- === Job Information ===
    job_title VARCHAR(150) NOT NULL,
    company VARCHAR(150) NOT NULL,
    website VARCHAR(150) NOT NULL,

    birthday DATE,

    -- === Extra Notes ===
    notes TEXT NOT NULL,

    -- === Timestamp ===
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- Records creation time

    -- === Foreign Key Constraints ===
    CONSTRAINT fk_known_from
        FOREIGN KEY (known_from_source_id)
        REFERENCES KnownFromSources(source_id)
        ON DELETE SET NULL  -- If the source is deleted, set this field to NULL
        ON UPDATE CASCADE,   -- If source_id changes (rare), update it here

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES Users(id)
        ON UPDATE CASCADE
);

ALTER TABLE Persons
    ADD COLUMN IF NOT EXISTS searchable tsvector
        GENERATED ALWAYS AS (to_tsvector('english', first_name || ' ' || last_name || ' ' || job_title || ' ' || company || ' ' || website || ' ' || notes || ' ' || coordinate_with_search::jsonb)) STORED;

-- === Indexes for Performance ===
CREATE INDEX IF NOT EXISTS idx_persons_lastname ON Persons(last_name);
CREATE INDEX IF NOT EXISTS idx_persons_coordinate ON Persons(coordinate_with_search);
CREATE INDEX IF NOT EXISTS idx_persons_job ON Persons(job_title);
CREATE INDEX IF NOT EXISTS idx_persons_company ON Persons(company);
CREATE INDEX IF NOT EXISTS idx_persons_known_from ON Persons(known_from_source_id); -- Good for FK lookups
