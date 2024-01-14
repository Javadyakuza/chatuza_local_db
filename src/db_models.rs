use diesel::prelude::*;
// use merge_derivable;

use serde::{self, Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::wallet)]
pub struct Wallet {
    pub user_id: i32,
    pub keypair: Vec<u8>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::wallet)]
pub struct QWallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub keypair: Vec<u8>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::user)]
pub struct QUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub pp: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::chat_rooms)]
pub struct ChatRooms {
    pub chat_room_id: i32,
    pub keypair: Vec<u8>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::messages)]
pub struct Messages {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub timestamp: String,
    pub content: String,
    pub is_read: bool,
    pub delivery_status: String,
    pub parent_message_id: Option<i32>,
    pub replied_on: Option<i32>,
}

// server endpoints Interfaces

#[derive(Debug, Serialize)]
pub struct NewUserIN<'a> {
    pub username_in: &'a str,
    pub email_in: &'a str,
    pub password_in: &'a str,
    pub phone_number_in: &'a str,
    pub bio_in: &'a str,
    pub profile_picture_in: &'a str,
}
