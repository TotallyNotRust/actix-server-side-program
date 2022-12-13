extern crate askama;

use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{Course, NewCourse};

use crate::schema::course::dsl::course;

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

pub async fn new_course(new_course: web::Form<NewCourse>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(course::table())
        .values(&new_course.0)
        .execute(connection)
    {
        Ok(_) => println!("Inserted couse titled {}", &new_course.0.title),
        Err(n) => println!("Could not insert course, got error:\n{:?}", n),
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

    println!("{:?}", courses);

    let page = CoursePage { courses: courses };

    HttpResponse::Ok().body(page.render().unwrap())
}
