use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use askama_actix::{Template, TemplateToResponse};
use std::env;
mod drivers;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    color: &'a str,
}
#[get("/hello/{name}/{color}/")]
async fn greet(args: web::Path<(String, String)>) -> Result<HttpResponse> {
    let args = args.into_inner();
    let s = HelloTemplate {
        name: &args.0,
        color: &args.1,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
