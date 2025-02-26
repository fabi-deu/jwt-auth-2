CREATE TABLE IF NOT EXISTS "users" (
    uuid TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,

    permission TEXT NOT NULL,
    tokenversion INTEGER DEFAULT 0,

    timestamp INTEGER DEFAULT (strftime('%s', 'now'))
);
