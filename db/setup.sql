-- setup.sql

CREATE TABLE page_view (
  id SERIAL PRIMARY KEY,
  page_name TEXT,
  user_id INT
);

CREATE TABLE sales_call (
  id SERIAL PRIMARY KEY,
  caller_id INT,
  user_id INT
);