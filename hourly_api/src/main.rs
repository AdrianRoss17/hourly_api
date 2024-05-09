extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;

use self::models::*;
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

#[get("/password/<id>")]
fn is_authenticated(id: i16) -> Json<Vec<Password>> {
    let connection = &mut database::establish_connection();
    let pass = connection.(&id);
    match pass {
        Ok(Password) => Ok(Json(Password)),
    }
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
        .mount("/password", routes![is_authenticated])
        .attach(cors2)
        .launch()
        .await?;

    Ok(())
}