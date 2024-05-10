extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;

use self::models::Password;
use self::schema::password::dsl::*;

use rocket::http::Method;
use rocket_cors::AllowedOrigins;
use rocket_cors::CorsOptions;

use std::error::Error;
use rocket::get;

use rocket::routes;

mod database;
mod models;
mod schema;

#[get("/auth")]
fn is_authenticated() -> Json<Vec<Password>> {
    let connection = &mut database::establish_connection();
    let result = password.load::<Password>(connection).map(Json).expect("Error loading posts");

    return result

}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors2 = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true).to_cors()?;

    rocket::build()
        .mount("/auth", routes![is_authenticated])
        .attach(cors2)
        .launch()
        .await?;

    Ok(())
}