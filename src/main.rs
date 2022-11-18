#![allow(dead_code, unused_imports)]

use actix_web::{
    get, post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use serde_derive::{Deserialize, Serialize};

mod sites;

mod models;
mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/goodbye")]
async fn goodbye() -> impl Responder {
    HttpResponse::Ok().body("Goodbye world!")
}

macro_rules! createServer {
    ($($x:expr), *) => {
        HttpServer::new( || {
            let mut app = App::new();
            $(
                app = app.service($x);
            )*
            return app;
        })
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let get_urls: Vec<Box<dyn Fn() -> dyn Responder<Body = String>>> =
    //     vec![Box::new(goodbye), Box::new(hello)];

    let server = createServer!(
        hello,
        goodbye,
        web::resource("/hi")
            .route(web::get().to(sites::hi::hi))
            .route(web::post().to(sites::hi::hi)),
        web::resource("/instructor")
            .route(web::get().to(sites::instructor::instructor_home))
            .route(web::post().to(sites::instructor::new_instructor))
    );

    server.bind(("127.0.0.1", 8080))?.run().await?;

    return Ok(());
}
