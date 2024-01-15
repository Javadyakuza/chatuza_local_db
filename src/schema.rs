// @generated automatically by Diesel CLI.

diesel::table! {
    chat_rooms (pch_id) {
        pch_id -> Integer,
        chat_room_id -> Integer,
        keypair -> Text,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Integer,
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
    user (pu_id) {
        pu_id -> Nullable<Integer>,
        user_id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        phone_number -> Text,
        bio -> Nullable<Text>,
        pp -> Nullable<Text>,
    }
}

diesel::table! {
    wallet (wallet_id) {
        wallet_id -> Integer,
        wallet_owner_id -> Integer,
        keypair -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chat_rooms,
    messages,
    user,
    wallet,
);
