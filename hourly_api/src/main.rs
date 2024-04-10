#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::users::dsl::*;

mod database;
mod models;
mod schema;

#[get("/")]
fn index() -> Json<Vec<User>> {
    let connection = &mut database::establish_connection();
    users.load::<User>(connection).map(Json).expect("Error loading birds")
}

#[launch]
fn rocket() -> Rocket<Build> {
    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);
    rocket::build().mount("/", routes![index])
}