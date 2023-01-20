use crate::database::users::create_user;
use crate::models::users::{self, NewUser};
use crate::{utils, DbPool};
use actix_swagger::StatusCode;
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use utoipa;

#[utoipa::path(
    post,
    path = "/register",
    responses(
        (status = 201, description = "User is well registered"),
    ),
    params()
)]
#[post("/v1/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    creation_form: web::Form<users::UserCreation>,
) -> HttpResponse {
    let errors = creation_form.validate();
    if errors.len() > 0 {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .body(serde_json::to_string(&errors).unwrap());
    }
    return HttpResponse::build(StatusCode::OK).body("All rigth all migth");
}
