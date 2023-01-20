mod controllers;
mod database;
mod enumeration;
mod errors;
mod models;
mod tests;
mod utils;
mod validator;

use actix_web::{App, HttpServer, web};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::env;
use utoipa_swagger_ui::SwaggerUi;
extern crate argon2;

use crate::controllers::basic_controller::status;
use crate::controllers::user::register;
use utoipa::OpenApi;
use diesel::prelude::*;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(OpenApi)]
#[openapi(paths(), components())]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new( move || {
        App::new().app_data(web::Data::new(pool.clone()))
        .service(status).service(register).service(
            SwaggerUi::new("/documentation/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()),
        )

        //.service(echo)
        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
