use chatuza_local_db::{
    db_models::{NewUserIN, QUser},
    init_user,
};

fn main() {
    let new_user_info = NewUserIN {
        username_in: "hojat",
        email_in: "hojat@gmail.com",
        password_in: "somehojpass",
        phone_number_in: "12341234",
        bio_in: "hojat cool bio",
        profile_picture_in: "some random url",
    };
    match init_user(&new_user_info) {
        Ok(res) => println!("main response \n {:?}", res),
        Err(e) => println!("main Error \n {:?}", e),
    }
}
