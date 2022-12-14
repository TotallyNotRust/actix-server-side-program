extern crate askama;

use actix_web::web::Json;
use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{Instructor, NewInstructor};

use crate::schema::instructor::dsl::{first_name, hire_date, instructor, instructor_id, last_name};

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

pub async fn update_instructor(updated_instructor: web::Json<Instructor>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    diesel::update(instructor::table())
        .filter(instructor_id.eq(updated_instructor.id))
        .set((
            first_name.eq(updated_instructor.first_name.to_owned()),
            last_name.eq(updated_instructor.last_name.to_owned()),
            hire_date.eq(updated_instructor.hire_date.to_owned()),
        ))
        .execute(connection)
        .unwrap();

    HttpResponse::Ok().finish()

    //instructor_home().await
}

#[derive(Debug, Deserialize)]
pub struct DeleteInstructorRequest {
    id: i32,
}

pub async fn delete_instructor(_instructor: Json<DeleteInstructorRequest>) -> HttpResponse {
    let connection: &mut SqliteConnection = &mut match SqliteConnection::establish("./database.db")
    {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::delete(instructor::table())
        .filter(instructor_id.eq(_instructor.id))
        .execute(connection)
    {
        Ok(n) => {}
        Err(n) => panic!("{:?}", n),
    }

    HttpResponse::Ok().finish()
}

pub async fn new_instructor(new_instructor: web::Form<NewInstructor>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(instructor::table())
        .values(&new_instructor.0)
        .execute(connection)
    {
        Ok(_) => {},
        Err(n) => panic!("Could not insert instructor, got error:\n{:?}", n),
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

    let page = InstructorPage {
        instructors: instructors,
    };

    HttpResponse::Ok().body(page.render().unwrap())
}
