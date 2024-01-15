use diesel::prelude::*;
// use merge_derivable;

use serde::{self, Deserialize, Serialize};

#[derive(
    Queryable,
    Deserialize,
    Serialize,
    diesel::expression::ValidGrouping,
    Selectable,
    Debug,
    Insertable,
    PartialEq,
)]
#[diesel(table_name = crate::schema::wallet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wallet {
    pub wallet_owner_id: i32,
    pub keypair: String,
}
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::wallet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct QWallet {
    pub wallet_id: i32,
    pub wallet_owner_id: i32,
    pub keypair: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct QUser {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub pp: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChatRooms {
    pub chat_room_id: i32,
    pub keypair: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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

// server endpoints input Interfaces

#[derive(Debug, Serialize)]
pub struct NewUserIN<'a> {
    pub username_in: &'a str,
    pub email_in: &'a str,
    pub password_in: &'a str,
    pub phone_number_in: &'a str,
    pub bio_in: &'a str,
    pub profile_picture_in: &'a str,
}

// server endpoints out Interfaces

#[derive(Debug, Serialize)]
pub struct NewUserOut {
    pub email: &'static str,
    pub password: &'static str,
    pub user_id: i32,
    pub username: &'static str,
}
