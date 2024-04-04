-- Your SQL goes here
create TABLE to_do (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  status VARCHAR(255) NOT NULL,
  date TIMESTAMP NOT NULL DEFAULT now()
);