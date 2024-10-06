CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    hashcode BIGINT
);

CREATE TABLE IF NOT EXISTS responses (
    id SERIAL PRIMARY KEY,
    version_name TEXT,
    protocol BOOLEAN,
    hover_text TEXT[],
    max_players INTEGER,
    players_online INTEGER,
    favicon BYTEA,
    secure_chat BOOLEAN,
    motd JSONB,
    disconnect_msg JSONB
);

CREATE TABLE IF NOT EXISTS address_names (
    address_name TEXT PRIMARY KEY,
    user_id INTEGER,
    response_id INTEGER,
    CONSTRAINT users_fk 
        FOREIGN KEY(user_id)
            REFERENCES users(id),
    CONSTRAINT response_fk 
        FOREIGN KEY(response_id)
            REFERENCES responses(id)
);