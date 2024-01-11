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
    pub id: Option<i32>,
    pub user_id: i32,
    pub keypair: Vec<u8>,
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
