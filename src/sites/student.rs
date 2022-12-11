extern crate askama;

use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{NewStudent, Student};

use crate::schema::student::dsl::{first_name, student};

#[derive(Template)]
#[template(path = "students.html")]
struct StudentPage {
    students: Vec<Student>,
}

impl IntoIterator for StudentPage {
    type Item = Student;
    type IntoIter = std::vec::IntoIter<Student>;

    fn into_iter(self) -> Self::IntoIter {
        self.students.into_iter()
    }
}

pub async fn new_student(new_instructor: web::Form<NewStudent>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(student::table())
        .values(&new_instructor.0)
        .execute(connection)
    {
        Ok(_) => println!("Inserted instructor named {}", &new_instructor.0.first_name),
        Err(n) => println!("Could not insert instructor, got error:\n{:?}", n),
    }

    student_home().await
}

pub async fn student_home() -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    let students = student
        .load::<Student>(connection)
        .expect("Could not load instructors");

    println!("{:?}", students);

    let page = StudentPage { students: students };

    HttpResponse::Ok().body(page.render().unwrap())
}
