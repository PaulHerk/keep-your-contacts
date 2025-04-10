-- Add migration script here
CREATE TABLE IF NOT EXISTS persons (
  id serial PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT,
  city TEXT NOT NULL,
  job TEXT,
  note TEXT
);