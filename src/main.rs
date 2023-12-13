

use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ConfigStruct {
    version: u32,
    message: String
}

#[get("/")]
async fn index() -> Result<impl Responder>  {

    let obj = ConfigStruct { version: 1,message: "SUCCESS".to_string() };

    Ok(web::Json(obj))
}

#[get("/account/{hash_id}")]
async fn account(path: web::Path<String>) -> Result<String>  {
    let hash_id = path.into_inner();
    Ok(format!("Hi, {}", hash_id))
}

async fn app() -> impl Responder {
    HttpResponse::Ok().body("https://www.rust-lang.org")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(account)
            .route("/app", web::get().to(app))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}