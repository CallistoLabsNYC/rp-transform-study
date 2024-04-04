-- setup.sql

CREATE TABLE page_view (
  id SERIAL PRIMARY KEY,
  page_name TEXT,
  user_id INT,
  created_at DATETIME
);

CREATE TABLE page_event (
  id SERIAL PRIMARY KEY,
  event_name TEXT,
  user_id INT
  created_at DATETIME
);