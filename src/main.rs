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

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
// #[get("/goodbye")]
// async fn goodbye() -> impl Responder {
//     HttpResponse::Ok().body("Goodbye world!")
// }

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
        // hello,
        // goodbye,
        web::resource("/").route(web::get().to(sites::homepage::homepage)),
        web::resource("/hi")
            .route(web::get().to(sites::hi::hi))
            .route(web::post().to(sites::hi::hi)),
        web::resource("/instructors")
            .route(web::get().to(sites::instructor::instructor_home))
            .route(web::post().to(sites::instructor::new_instructor))
            .route(web::delete().to(sites::instructor::delete_instructor))
            .route(web::put().to(sites::instructor::update_instructor)),
        web::resource("/students")
            .route(web::get().to(sites::student::student_home))
            .route(web::post().to(sites::student::new_student))
            .route(web::delete().to(sites::student::delete_student))
            .route(web::put().to(sites::student::update_student)),
        web::resource("/courses")
            .route(web::get().to(sites::course::course_home))
            .route(web::post().to(sites::course::new_course))
            .route(web::delete().to(sites::course::delete_course))
            .route(web::put().to(sites::course::update_course)),
        web::resource("/enrollments")
            .route(web::get().to(sites::enroll::enrollment_home))
            .route(web::post().to(sites::enroll::new_enrollment))
            .route(web::delete().to(sites::enroll::delete_enrollment))
            .route(web::put().to(sites::enroll::update_enrollment))
    );

    println!("Started server on http://0.0.0.0:8080");

    server.bind(("0.0.0.0", 8080))?.run().await?;

    return Ok(());
}
