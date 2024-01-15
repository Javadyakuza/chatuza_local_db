// this module will perform the db interactions and the db level sensitive data en-decryption.
#![recursion_limit = "256"]
pub mod db_models;
pub mod schema;
pub mod server_models;
extern crate tokio;
use crate::{
    db_models::NewUserOut,
    schema::{chat_rooms, messages, wallet},
};
use db_models::{ChatRooms, Messages, NewUserIN, QUser, QWallet, User, Wallet};
pub use diesel;
pub use server_models::QUsers;
// pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use schema::{chat_rooms::dsl::*, messages::dsl::*, user::dsl::*, wallet::dsl::*};

use reqwest::{Client, Request};
use serde_json::Value;
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
pub fn init_user(_new_user_info: &NewUserIN) -> Result<QUser, Box<dyn std::error::Error>> {
    // checking if user is already added to the server db or not.
    // if added before => err , if not added => add to the server and get the user id and add it to the local db as well
    // sending request to the server
    let mut conn = establish_connection();

    match request_create_user(&_new_user_info) {
        Ok(q_user) => {
            // adding the user into the  local db
            let _new_user_id: i32 = q_user.user_id;
            match diesel::insert_into(user).values(q_user).execute(&mut conn) {
                Ok(res) => match get_user_with_userid(&mut conn, _new_user_id) {
                    Ok(q_user) => return Ok(q_user),
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
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't add the user due to \n {:?}", e),
            )))
        }
    }
}
// server sync not implemented
pub fn add_solana_wallet(
    reset: bool,
    new_wallet_info: &Wallet,
) -> Result<QWallet, Box<dyn std::error::Error>> {
    // reset is for kp leakage conditions
    let mut conn = establish_connection();

    let is_wallet_initialized: bool =
        get_wallet_with_userid(&mut conn, new_wallet_info.wallet_owner_id).is_err();
    if reset {
        match diesel::insert_into(wallet::table)
            .values(new_wallet_info)
            .execute(&mut conn)
        {
            Ok(res) => match get_wallet_with_userid(&mut conn, new_wallet_info.wallet_owner_id) {
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
                .execute(&mut conn)
            {
                Ok(res) => match get_wallet_with_userid(&mut conn, new_wallet_info.wallet_owner_id)
                {
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
// server sync not implemented
fn get_wallet_with_userid(
    _conn: &mut SqliteConnection,
    _user_id: i32,
) -> Result<QWallet, Box<dyn std::error::Error>> {
    let wallet_row: Vec<QWallet> = wallet
        .filter(wallet::wallet_owner_id.eq(&_user_id))
        .select(QWallet::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);
    if wallet_row.len() == 1 {
        Ok(QWallet {
            wallet_id: wallet_row[0].wallet_id,
            wallet_owner_id: wallet_row[0].wallet_owner_id,
            keypair: wallet_row[0].keypair.to_owned(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id associated wallet not found !",
        )))
    }
}

// server sync not implemented
fn get_user_with_userid(
    _conn: &mut SqliteConnection,
    _user_id: i32,
) -> Result<QUser, Box<dyn std::error::Error>> {
    let user_row: Vec<QUser> = user
        .filter(user::all_columns().1.eq(&_user_id))
        .select(QUser::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Ok(QUser {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
            bio: user_row[0].bio.clone(),
            pp: user_row[0].pp.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id associated wallet not found !",
        )))
    }
}

// server sync not implemented
pub fn add_chat_room(
    reset: bool,
    new_chat_room_info: &ChatRooms,
) -> Result<ChatRooms, Box<dyn std::error::Error>> {
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
// server sync not implemented
fn get_chat_room_with_chat_room_id(
    _conn: &mut SqliteConnection,
    _user_id: i32,
) -> Result<ChatRooms, Box<dyn std::error::Error>> {
    let wallet_row: Vec<ChatRooms> = chat_rooms
        .filter(chat_room_id.eq(&chat_room_id))
        .select(ChatRooms::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if wallet_row.len() == 1 {
        Ok(ChatRooms {
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
// server sync not implemented
// fn delete_chatroom(_chat_room_id: i32) -> Result<bool, <dyn std::error::Error>> {
//     // diesel::delete(source)
// }

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

// EH
#[tokio::main]
pub async fn request_create_user(_new_user_info: &NewUserIN) -> Result<QUser, String> {
    let client = Client::new();

    let form_data = [
        ("username_in", _new_user_info.username_in),
        ("email_in", _new_user_info.email_in),
        ("password_in", _new_user_info.password_in),
        ("phone_number_in", _new_user_info.phone_number_in),
        ("bio_in", _new_user_info.bio_in),
        ("profile_picture_in", _new_user_info.profile_picture_in),
    ];

    let encoded_form_data = form_data
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

    let request = client
        .post("http://localhost:8000/api/create-user")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(encoded_form_data);

    let built_request: Request;
    match request.build() {
        Ok(req) => built_request = req,
        Err(e) => {
            return Err(format!(
                "couldn't build the create account request due to {}",
                e
            ))
        }
    }

    let result: reqwest::Response;
    match client.execute(built_request).await {
        Ok(res) => result = res,
        Err(e) => return Err(format!("sending request failed due to \n {}", e)),
    }

    if result.status() != 200 {
        return Err(format!("server returned {}", result.status()));
    }

    let response_text: String;
    match result.text().await {
        Ok(text) => response_text = text,
        Err(e) => return Err(format!("couldn't fetch the response text due to \n {}", e)),
    }
    let response_val: Value;
    match serde_json::from_str(response_text.as_str()) {
        Ok(val) => response_val = val,
        Err(e) => return Err(format!("failed to parse the response due to {}", e)),
    }

    println!("{}", response_val);

    match response_val["Ok"].as_array() {
        Some(_new_user) => {
            let obj = _new_user[0].as_object().unwrap();
            return Ok(QUser {
                user_id: obj["user_id"].as_str().unwrap().parse::<i32>().unwrap(),
                username: obj["username"].as_str().unwrap().to_string(),
                email: obj["email"].as_str().unwrap().to_string(),
                password: obj["password"].as_str().unwrap().to_string(),
                bio: Some(obj["bio"].as_str().unwrap().to_string()),
                pp: Some(obj["profile_picture"].as_str().unwrap().to_string()),
            });
        }
        None => {
            return Err(format!(
                "failed to read the signatures ! \n server returned {}",
                response_val
            ))
        }
    }
}
