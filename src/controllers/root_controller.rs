use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub message: String,
}

#[get("/")]
pub fn index() -> Json<Message> {
    return Json(Message {
        message: String::from("Welcome to the Auto Commits API"),
    });
}
