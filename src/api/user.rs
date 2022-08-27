use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer,
};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use crate::models::user_model::User;

#[post("/register")]

async fn register(client: web::Data<Client>, user: Json<User>) -> HttpResponse {
    let collection = client.database("rustcms").collection("users");

    let data = User {
        fullname: user.fullname.to_owned(),
        email: user.email.to_owned(),
        password: user.password.to_owned(),
        bio: user.bio.to_owned(),
        phone: user.phone.to_owned(),
    };

    let result = collection.insert_one(data, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("register successfully"),
        Err(err) => {
            println!("Error while getting, {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
