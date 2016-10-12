CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  age INTEGER NULL
);

INSERT INTO users (name, age) VALUES ('Jayson Reis', 28);
INSERT INTO users (name, age) VALUES ('Vô', 78);
INSERT INTO users (name, age) VALUES ('Karthik', 15);
INSERT INTO users (name, age) VALUES ('Oussama', 5);