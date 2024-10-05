CREATE TABLE IF NOT EXISTS Users (
    id SERIAL,
    hashcode BIGINT
);

CREATE TABLE IF NOT EXISTS Responses (
    id SERIAL,
    version_name TEXT,
    protocol TEXT,
    hover_text TEXT[],
    max_players INTEGER,
    players_online INTEGER,
    favicon BYTEA,
    secure_chat BOOLEAN,
    motd JSON,
    disconnect_msg JSON
);

CREATE TABLE IF NOT EXISTS AdressNames (
    address_name TEXT PRIMARY KEY,
    user_id INTEGER,
    response_id INTEGER,
    CONSTRAINT users_fk 
        FOREIGN KEY(user_id)
            REFERENCES Users(id),
    CONSTRAINT response_fk 
        FOREIGN KEY(response_id)
            REFERENCES Responses(id)
);