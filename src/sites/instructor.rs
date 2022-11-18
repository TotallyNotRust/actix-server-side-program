extern crate askama;

use diesel::associations::HasTable;
use diesel::prelude::*;

use timer::Timer;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{Instructor, NewInstructor};

use crate::schema::instructor::dsl::{first_name, instructor};

#[derive(Template)]
#[template(path = "instructors.html")]
struct InstructorPage {
    instructors: Vec<Instructor>,
}

impl IntoIterator for InstructorPage {
    type Item = Instructor;
    type IntoIter = std::vec::IntoIter<Instructor>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructors.into_iter()
    }
}

pub async fn new_instructor(new_instructor: web::Form<NewInstructor>) -> HttpResponse {
    Timer::
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(instructor::table())
        .values(&new_instructor.0)
        .execute(connection)
    {
        Ok(_) => println!("Inserted instructor named {}", &new_instructor.0.first_name),
        Err(n) => println!("Could not insert instructor, got error:\n{:?}", n),
    }

    instructor_home().await
}

pub async fn instructor_home() -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    let instructors = instructor
        .load::<Instructor>(connection)
        .expect("Could not load instructors");

    println!("{:?}", instructors);

    let page = InstructorPage {
        instructors: instructors,
    };

    HttpResponse::Ok().body(page.render().unwrap())
}
