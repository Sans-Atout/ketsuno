use actix_web::{get, web, Responder, Result};
use crate::models::app::App;
use utoipa;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Get app naùe version and status"),
    ),
    params()
)]
#[get("/")]
pub async fn status()  -> Result<impl Responder>{
    let app_status = App::new( "ketsuno", "1.0.0", "running");
    return Ok(web::Json(app_status))
}