use actix_web::{post, web, App, HttpResponse, HttpServer};
use std::env;

pub mod calc;
pub mod model;
pub mod transformation;

#[post("/api")]
async fn api(data: web::Json<model::Request>) -> Result<HttpResponse, actix_web::Error> {
    // println!("Received the following data:");
    // println!("{:?}", data);
    match &*data.r#type {
        "mahjong" => {
            if let Some(req) = &data.mahjong_request {
                if let Some(res) = calc::calc(req) {
                    let res: model::Response = model::Response::new(
                        "Scores are calculated.".to_string(),
                        Some(res),
                        data.into_inner(),
                    );
                    return Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .json(res));
                } else {
                    let res: model::Response = model::Response::new(
                        "The request is invalid.".to_string(),
                        None,
                        data.into_inner(),
                    );
                    return Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .json(res));
                }
            }
        }
        _ => {}
    }
    let res: model::Response =
        model::Response::new("unknown request type".to_string(), None, data.into_inner());
    return Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(res));
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || App::new().service(api))
        .bind(("0.0.0.0", port))?
        .run()
        .await?;
    Ok(())
}
