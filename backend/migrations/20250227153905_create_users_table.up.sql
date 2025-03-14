CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    updated_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    login_attempts INTEGER NOT NULL DEFAULT 0,
    last_failed_login_attempt DATETIME,
    last_logged_in_at DATETIME
);

CREATE TRIGGER users_timestamp AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = (DATETIME('subsec')) WHERE id = NEW.id;
END;
