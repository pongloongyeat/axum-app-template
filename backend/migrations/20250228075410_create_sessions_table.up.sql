CREATE TABLE sessions (
    id INTEGER NOT NULL PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    updated_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    token TEXT NOT NULL UNIQUE,
    refresh_token TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token_expiry DATETIME NOT NULL,
    refresh_token_expiry DATETIME NOT NULL,
    revoked_at DATETIME,
    revocation_reason TEXT
);

CREATE INDEX sessions_user_id_idx ON sessions(user_id);

CREATE TRIGGER sessions_timestamp AFTER UPDATE ON sessions
BEGIN
    UPDATE sessions SET updated_at = (DATETIME('subsec')) WHERE id = NEW.id;
END;
