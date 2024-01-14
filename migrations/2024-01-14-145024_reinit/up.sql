CREATE TABLE IF NOT EXISTS wallet (
    wallet_id INTEGER NOT NULL PRIMARY KEY,
    wallet_owner_id INTEGER NOT NULL,
    keypair BLOB UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS chat_rooms (
    pch_id INTEGER NOT NULL PRIMARY KEY,
    chat_room_id INTEGER NOT NULL,
    keypair BLOB UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS messages (
    message_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    sender_id INTEGER NOT NULL,
    recipient_id INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    content TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    delivery_status VARCHAR(255) NOT NULL DEFAULT 'sent',
    parent_message_id INTEGER,
    replied_on INTEGER
);
CREATE TABLE user (
    pu_id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    bio VARCHAR(255),
    pp VARCHAR(255)
);