mod controllers;
mod models;
mod database;

use actix_web::{App, HttpServer};
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::basic_controller::status;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(), components())]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
        HttpServer::new(|| {
        App::new()
            .service(status)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )

            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
