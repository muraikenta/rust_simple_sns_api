#[macro_use]
extern crate diesel;
use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use modules::account::account_service::{self};
use serde::Serialize;

use crate::modules::user::user_dto::NewUser;
mod db;
mod env;
mod error;
mod modules;
mod schema;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let obj = MyObj { name: req_body };
    HttpResponse::Ok().json(obj)
}

#[post("/account")]
async fn create_account(request: Json<NewUser>) -> impl Responder {
    let result = account_service::signup(&request.into_inner());
    match result {
        Ok((user, token)) => return HttpResponse::Ok().json(user),
        Err(err) => {
            print!("{:?}", err);
            return HttpResponse::UnprocessableEntity().finish();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::load();
    establish_connection();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(create_account)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
