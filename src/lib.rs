// this module will perform the db interactions and the db level sensitive data en-decryption.
#![recursion_limit = "256"]
pub mod db_models;
pub mod schema;

use crate::schema::{chat_rooms, messages, wallet};
use db_models::{ChatRooms, Messages, QChatRooms, QWallet, Wallet};
pub use diesel;
// pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use schema::{chat_rooms::dsl::*, messages::dsl::*, wallet::dsl::*};

use std::collections::hash_map::DefaultHasher;
pub use std::env;
use std::hash::{Hash, Hasher};

pub fn establish_connection() -> SqliteConnection {
    // loading the env vars into the current scope
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_solana_wallet(
    reset: bool,
    new_wallet_info: &Wallet,
) -> Result<QWallet, Box<dyn std::error::Error>> {
    // reset is for kp leakage conditions
    let mut conn = establish_connection();

    let is_wallet_initialized: bool =
        get_wallet_with_userid(&mut conn, new_wallet_info.user_id).is_err();
    if reset {
        match diesel::insert_into(wallet::table)
            .values(new_wallet_info)
            // .returning(QWallet::as_returning())
            .execute(&mut conn)
        {
            Ok(res) => match get_wallet_with_userid(&mut conn, new_wallet_info.user_id) {
                Ok(q_wallet) => return Ok(q_wallet),
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{:?}", e),
                    )))
                }
            },
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("couldn't add the wallet due to \n {:?}", e),
                )))
            }
        };
    } else {
        if is_wallet_initialized {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
            "user already has an wallet , if you want to reset the wallet call the function with reset = true",
            )));
        } else {
            match diesel::insert_into(wallet::table)
                .values(new_wallet_info)
                // .returning(QWallet::as_returning())
                .execute(&mut conn)
            {
                Ok(res) => match get_wallet_with_userid(&mut conn, new_wallet_info.user_id) {
                    Ok(q_wallet) => return Ok(q_wallet),
                    Err(e) => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{:?}", e),
                        )))
                    }
                },
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("couldn't add the wallet due to \n {:?}", e),
                    )))
                }
            };
        }
    }
}

fn get_wallet_with_userid(
    _conn: &mut SqliteConnection,
    _user_id: i32,
) -> Result<QWallet, Box<dyn std::error::Error>> {
    let wallet_row: Vec<QWallet> = wallet
        .filter(user_id.eq(&user_id))
        .select(QWallet::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if wallet_row.len() == 1 {
        Ok(QWallet {
            id: wallet_row[0].id,
            user_id: wallet_row[0].user_id,
            keypair: wallet_row[0].keypair.to_owned(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id associated wallet not found !",
        )))
    }
}

pub fn add_chat_room(
    reset: bool,
    new_chat_room_info: &ChatRooms,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    // reset is for kp leakage, or recrating a chat room after its deleted
    let mut conn = establish_connection();

    let is_wallet_initialized: bool =
        get_chat_room_with_chat_room_id(&mut conn, new_chat_room_info.chat_room_id).is_err();
    if reset {
        match diesel::insert_into(chat_rooms::table)
            .values(new_chat_room_info)
            // .returning(QWallet::as_returning())
            .execute(&mut conn)
        {
            Ok(res) => {
                match get_chat_room_with_chat_room_id(&mut conn, new_chat_room_info.chat_room_id) {
                    Ok(q_chat_rooms) => return Ok(q_chat_rooms),
                    Err(e) => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("{:?}", e),
                        )))
                    }
                }
            }
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("couldn't add the chat room kp due to \n {:?}", e),
                )))
            }
        };
    } else {
        if is_wallet_initialized {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
            "the chat room already has an kp, if your want to reset the kp of the chat room, call the function with reset = true",
            )));
        } else {
            match diesel::insert_into(chat_rooms::table)
                .values(new_chat_room_info)
                // .returning(QWallet::as_returning())
                .execute(&mut conn)
            {
                Ok(res) => {
                    match get_chat_room_with_chat_room_id(
                        &mut conn,
                        new_chat_room_info.chat_room_id,
                    ) {
                        Ok(q_chat_rooms) => return Ok(q_chat_rooms),
                        Err(e) => {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{:?}", e),
                            )))
                        }
                    }
                }
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("couldn't add the chat room kp due to \n {:?}", e),
                    )))
                }
            };
        }
    }
}

fn get_chat_room_with_chat_room_id(
    _conn: &mut SqliteConnection,
    _user_id: i32,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    let wallet_row: Vec<QChatRooms> = chat_rooms
        .filter(chat_room_id.eq(&chat_room_id))
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if wallet_row.len() == 1 {
        Ok(QChatRooms {
            id: wallet_row[0].id,
            chat_room_id: wallet_row[0].chat_room_id,
            keypair: wallet_row[0].keypair.to_owned(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id associated wallet not found !",
        )))
    }
}

fn delete_wallet() {
    todo!()
}
fn delete_chatroom() {
    todo!()
}
fn encrypt_msg() {
    todo!()
}

fn decrypt_msg() {
    todo!()
}

fn sign_tx() {
    todo!()
}

fn add_msg() {
    todo!()
}

fn edit_msg() {
    todo!()
}

fn delete_msg() {
    todo!()
}
