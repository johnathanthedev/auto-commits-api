#[macro_use]
extern crate rocket;

mod config;
mod controllers;

use config::db::set_up;
use controllers::root_controller::index;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = match set_up().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
