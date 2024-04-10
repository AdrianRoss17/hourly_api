use diesel::prelude::Queryable;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}