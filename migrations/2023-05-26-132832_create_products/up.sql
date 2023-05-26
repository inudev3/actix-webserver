CREATE TABLE products (
                          id SERIAL PRIMARY KEY,
                          name VARCHAR NOT NULL,
                          stock FLOAT NOT NULL,
                          price INTEGER --representing cents
)-- Your SQL goes here
CREATE TABLE users (
                       email VARCHAR(100) NOT NULL PRIMARY KEY,
                       password VARCHAR(64) NOT NULL,
                       created_at TIMESTAMP NOT NULL
);
CREATE INDEX users_email_company_idx ON users (email, company);
