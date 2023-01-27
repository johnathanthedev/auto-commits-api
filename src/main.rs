#[macro_use]
extern crate rocket;

mod config;
mod controllers;

use config::db::set_up;
use controllers::root_controller::index;

#[launch]
async fn rocket() -> _ {
    let db = match set_up().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).mount("/", routes![index])
}
