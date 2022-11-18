use std::{iter::Map, sync::Arc};

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{
    get, post,
    web::{Json, Query},
    FromRequest, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hi {
    name: String,
}

pub async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Hi!")
}

pub async fn hi_named(query: Query<Hi>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hi {}", query.name).to_owned())
    // HttpResponse::Ok().body("Hello {}")
}
