use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TestMessage {
    pub message: String,
}

#[get("/")]
pub fn index() -> Json<TestMessage> {
    return Json(TestMessage {
        message: String::from("Welcome to the Auto Commits API"),
    });
}
