pub struct Users {
    // pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

pub struct UserProfiles {
    // pub user_profile_id: i32,
    pub user_id: i32,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

pub struct ChatRooms {
    // pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
    pub chat_room_pubkey: Vec<u8>,
}

pub struct UpdatableChatRooms {
    // pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
}

pub struct ChatRoomParticipants {
    // pub participant_id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

pub struct SolanaWallet {
    pub user_id: i32,
    pub wallet_addr: Vec<u8>,
    pub wallet_backup: Vec<u8>,
}
// --  models with queryable primary keys -- //

pub struct QUsers {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct QChatRooms {
    pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
    pub chat_room_pubkey: Vec<u8>,
}

pub struct QChatRoomParticipants {
    pub participant_id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}
