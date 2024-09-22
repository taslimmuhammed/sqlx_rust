CREATE TABLE book(
    title varchar not null,
    author varchar not null,
    isbn varchar not null,
);

CREATE unique index book_isbn_idx on book(isbn);
