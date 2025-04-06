-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
  id serial PRIMARY KEY,
  note TEXT NOT NULL
);