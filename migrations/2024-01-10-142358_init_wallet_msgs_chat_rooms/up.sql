CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    keypair BLOB UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS chatrooms (
    id INTEGER PRIMARY KEY,
    chat_room_id INTEGER NOT NULL,
    keypair BLOB UNIQUE NOT NULL
);
CREATE TABLE IF NOT EXISTS messages (
    message_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sender_id INTEGER NOT NULL,
    recipient_id INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    content TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    delivery_status VARCHAR(255) NOT NULL DEFAULT 'sent',
    parent_message_id INTEGER,
    replied_on INTEGER
);