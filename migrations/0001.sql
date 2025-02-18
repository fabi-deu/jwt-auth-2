CREATE TABLE IF NOT EXISTS "user"(
    uuid TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,

    permission TEXT NOT NULL,
    tokenversion INTEGER DEFAULT 0,

    issued_at INTEGER DEFAULT unixepoch()
);
