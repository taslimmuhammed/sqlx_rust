CREATE TABLE books(
    id SERIAL PRIMARY KEY,
    title varchar not null,
    author varchar not null
);

-- sqlx migrate run