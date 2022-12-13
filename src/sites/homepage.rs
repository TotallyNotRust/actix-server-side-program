use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomePage {}

pub async fn homepage() -> HttpResponse {
    let page = HomePage {};

    HttpResponse::Ok().body(page.render().unwrap())
}
