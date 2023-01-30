use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationParams<'r> {
    pub email: &'r str,
    pub password: &'r str,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegistrationPayload {
    pub email: String,
    pub message: String,
}

#[post("/sign-up", data = "<registration_body>")]
pub async fn sign_up(registration_body: Json<RegistrationParams<'_>>) -> Json<RegistrationPayload> {
    // Check if users with provided email exists
    // Check if passwords match
    // Hash password
    // Create user
    return Json(RegistrationPayload {
        email: String::from(registration_body.email),
        message: String::from("Your credentials have been received"),
    });
}
