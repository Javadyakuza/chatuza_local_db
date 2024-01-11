// @generated automatically by Diesel CLI.

diesel::table! {
    chat_rooms (id) {
        id -> Nullable<Integer>,
        chat_room_id -> Integer,
        keypair -> Binary,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Nullable<Integer>,
        sender_id -> Integer,
        recipient_id -> Integer,
        timestamp -> Timestamp,
        content -> Text,
        is_read -> Bool,
        delivery_status -> Text,
        parent_message_id -> Nullable<Integer>,
        replied_on -> Nullable<Integer>,
    }
}

diesel::table! {
    wallet (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        keypair -> Binary,
    }
}

diesel::allow_tables_to_appear_in_same_query!(chat_rooms, messages, wallet,);
