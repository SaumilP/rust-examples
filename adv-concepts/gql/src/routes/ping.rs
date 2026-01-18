use actix_web::{get};

#[get("/")]
pub async fn rping() -> String {
    format!("Pong!")
}
