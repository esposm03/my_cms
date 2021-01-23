CREATE TABLE posts (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL
);
