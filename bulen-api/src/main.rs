use db::FoodClubEntry;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Object;
use std::{io::ErrorKind, sync::Arc};

mod db;
use crate::db::DB;
use cors::CORS;

mod cors;
mod error;
mod prelude;
mod utils;

#[macro_use] extern crate rocket;
use rocket::{serde::json::Json, State};
use rocket::form::Form;

type DB1 = DB<Client>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tasks")]
async fn get_all_tasks(db: &State<DB1>) -> Result<Json<Vec<Object>>, std::io::Error> {
    let tasks = db
        .get_all()
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch all tasks."))?;

    Ok(Json(tasks))
}

#[post("/task", data = "<entry>")]
async fn create_task(db: &State<DB1>, entry: Form<FoodClubEntry>) -> Result<Json<FoodClubEntry>, std::io::Error> {
    let task = db
        .add_entry(entry.into_inner())
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create task."))?;

    Ok(Json(task))
}

// Creates a new static instance of the client
static DB_static: Surreal<Client> = Surreal::init();

#[launch]
async fn rocket() -> _ {
// #[tokio::main]
// async fn main() -> surrealdb::Result<()> {
    // Connect to the database
    DB_static.connect::<Ws>("localhost:8000").await.unwrap();

    // Log into the database
    DB_static.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    DB_static.use_ns("bulen").use_db("madklub").await.unwrap();

    let db = DB::new(&DB_static);


    rocket::build()
        .mount(
        "/",
        routes![index, get_all_tasks, create_task]
        )
        .attach(CORS)
        .manage(db)
}
