use actix_web::{post, web, App, HttpResponse, HttpServer};
use std::env;

pub mod model;
pub mod transformation;

#[post("/api")]
async fn api(data: web::Json<model::Request>) -> Result<HttpResponse, actix_web::Error> {
    println!("Received the following data:");
    println!("{:?}", data);
    match &*data.r#type {
        "mahjong" => {
            return Ok(HttpResponse::Ok().body(""));
        }
        _ => {
            let res: model::Response =
                model::Response::new("unknown request type".to_string(), None, data.into_inner());
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(res));
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || App::new().service(api))
        .bind(("127.0.0.1", port))?
        .run()
        .await?;
    Ok(())
}
