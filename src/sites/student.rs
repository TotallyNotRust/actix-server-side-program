extern crate askama;

use actix_web::web::Json;
use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{NewStudent, Student};

use crate::schema::student::dsl::{first_name, last_name, student, student_id};
use crate::schema::student::enrollment_date;

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

pub async fn update_student(updated_student: web::Json<Student>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    diesel::update(student::table())
        .filter(student_id.eq(updated_student.id))
        .set((
            first_name.eq(updated_student.first_name.to_owned()),
            last_name.eq(updated_student.last_name.to_owned()),
            enrollment_date.eq(updated_student.enrollment_date.to_owned()),
        ))
        .execute(connection)
        .unwrap();

    HttpResponse::Ok().finish()

    //instructor_home().await
}

#[derive(Debug, Deserialize)]
pub struct DeleteStudentRequest {
    id: i32,
}

pub async fn delete_student(_course: Json<DeleteStudentRequest>) -> HttpResponse {
    let connection: &mut SqliteConnection = &mut match SqliteConnection::establish("./database.db")
    {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::delete(student::table())
        .filter(student_id.eq(_course.id))
        .execute(connection)
    {
        Ok(n) => {}
        Err(n) => panic!("{:?}", n),
    }

    HttpResponse::Ok().finish()
}

pub async fn new_student(new_student: web::Form<NewStudent>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::insert_into(student::table())
        .values(&new_student.0)
        .execute(connection)
    {
        Ok(_) => {}
        Err(n) => panic!("{:?}", n),
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

    let page = StudentPage { students: students };

    HttpResponse::Ok().body(page.render().unwrap())
}
