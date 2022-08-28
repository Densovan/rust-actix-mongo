use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};

//import dependency
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use mongodb::bson::{self, doc};
use mongodb::Client;

//import user model
use crate::models::user_model::User;

#[post("/register")]

async fn register(client: web::Data<Client>, user: Json<User>) -> HttpResponse {
    let collection = client.database("rustcms").collection("users");

    let mut sha = Sha256::new();
    sha.input_str(user.password.as_str());
    let hash_password = sha.result_str();

    let data = doc! {
        "fullname": user.fullname.to_owned(),
        "bio": user.bio.to_owned(),
        "phone": user.phone.to_owned(),
        "email": user.email.to_owned(),
        "password": hash_password.to_owned(),
        "created_at": bson::DateTime::now(),
        "updated_at": bson::DateTime::now(),
    };
    // let data = User {
    //     _id: None,
    //     fullname: user.fullname.to_owned(),
    //     bio: user.bio.to_owned(),
    //     phone: user.phone.to_owned(),
    //     email: user.email.to_owned(),
    //     password: user.password.to_owned(),
    //     created_at: bson::DateTime::now(),
    //     updated_at: bson::DateTime::now(),
    // };

    let result = collection.insert_one(data, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("register successfully"),
        Err(err) => {
            println!("Error while getting, {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
