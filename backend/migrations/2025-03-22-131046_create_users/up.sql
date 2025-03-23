CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  tg_id INT NOT NULL UNIQUE,
  username VARCHAR NOT NULL UNIQUE,
  first_name VARCHAR,
  second_name VARCHAR
);