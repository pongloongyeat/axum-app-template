CREATE TABLE forgot_password_transactions (
    id INTEGER NOT NULL PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    updated_at DATETIME NOT NULL DEFAULT (DATETIME('subsec')),
    user_id INTEGER NOT NULL REFERENCES users(id),
    token TEXT NOT NULL,
    reset_password_token TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    verified_at DATETIME,
    used_at DATETIME
);

CREATE INDEX forgot_password_transactions_user_id_idx ON forgot_password_transactions(user_id);
CREATE UNIQUE INDEX forgot_password_transactions_reset_password_token_uc ON forgot_password_transactions(reset_password_token);
CREATE UNIQUE INDEX forgot_password_transactions_user_id_token_uc ON forgot_password_transactions(user_id, token);
CREATE UNIQUE INDEX forgot_password_transactions_user_id_reset_password_token_uc ON forgot_password_transactions(user_id, reset_password_token);

CREATE TRIGGER forgot_password_transactions_timestamp AFTER UPDATE ON forgot_password_transactions
BEGIN
    UPDATE forgot_password_transactions SET updated_at = (DATETIME('subsec')) WHERE id = NEW.id;
END;
