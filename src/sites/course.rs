extern crate askama;

use std::future::{ready, IntoFuture, Ready};
use std::vec;

use actix_web::cookie::Display;
use futures_util::Future;
use lazy_static::lazy_static;

use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{
    Course, Enrollment, Instructor, NewCourse, NewEnrollment, NewInstructor, Student,
};

use crate::schema::course::dsl::{course, course_id};
use crate::schema::course::{credits, title};

#[derive(Template)]
#[template(path = "courses.html")]
struct CoursePage {
    courses: Vec<Course>,
}

impl IntoIterator for CoursePage {
    type Item = Course;
    type IntoIter = std::vec::IntoIter<Course>;

    fn into_iter(self) -> Self::IntoIter {
        self.courses.into_iter()
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteCourseRequest {
    id: i32,
}

pub async fn update_course(updated_course: web::Json<Course>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    diesel::update(course::table())
        .filter(course_id.eq(updated_course.id))
        .set((
            title.eq(updated_course.title.to_owned()),
            credits.eq(updated_course.credits),
        ))
        .execute(connection)
        .unwrap();

    HttpResponse::Ok().finish()

    //instructor_home().await
}

pub async fn delete_course(_course: Json<DeleteCourseRequest>) -> HttpResponse {
    let connection: &mut SqliteConnection = &mut match SqliteConnection::establish("./database.db")
    {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::delete(course::table())
        .filter(course_id.eq(_course.id))
        .execute(connection)
    {
        Ok(n) => {}
        Err(n) => panic!("{:?}", n),
    }

    HttpResponse::Ok().finish()
}

pub async fn new_course(new_course: web::Form<NewCourse>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(course::table())
        .values(&new_course.0)
        .execute(connection)
    {
        Ok(_) => {}
        Err(n) => panic!("{:?}", n),
    }

    course_home().await
}

pub async fn course_home() -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    let courses = course
        .load::<Course>(connection)
        .expect("Could not load instructors");

    let page = CoursePage { courses: courses };

    HttpResponse::Ok().body(page.render().unwrap())
}
