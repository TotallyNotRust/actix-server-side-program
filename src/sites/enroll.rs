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

use crate::models::{Course, Enrollment, Instructor, NewEnrollment, NewInstructor, Student};

use crate::schema::enrollment::grade;
use crate::schema::{
    course::dsl::{course, course_id as cid},
    enrollment::dsl::{course_id, enrollment, enrollment_id, student_id},
    student::dsl::{student, student_id as sid},
};

lazy_static! {
    static ref GRADES: Vec<String> = vec![
        String::from("12"),
        String::from("10"),
        String::from("7"),
        String::from("4"),
        String::from("02"),
        String::from("00"),
        String::from("-3"),
    ];
}

#[derive(Template)]
#[template(path = "enroll.html")]
struct EnrollmentPage {
    enrollments: Vec<EnrollWithStudentCourse>,
    students: Vec<Student>,
    courses: Vec<Course>,
    grades: Vec<String>,
}
#[derive(Debug)]
struct EnrollWithStudentCourse {
    enrollment: Enrollment,
    student: Student,
    course: Course,
}

#[derive(Debug, Deserialize)]
pub struct DeleteEnrollmentRequest {
    id: i32,
}

pub async fn delete_enrollment(enroll: Json<DeleteEnrollmentRequest>) -> HttpResponse {
    let connection: &mut SqliteConnection = &mut match SqliteConnection::establish("./database.db")
    {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    match diesel::delete(enrollment::table())
        .filter(enrollment_id.eq(enroll.id))
        .execute(connection)
    {
        Ok(_) => {}
        Err(n) => panic!("{:?}", n),
    }

    HttpResponse::Ok().finish()
}

pub async fn update_enrollment(updated_enroll: web::Json<Enrollment>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    diesel::update(enrollment::table())
        .filter(enrollment_id.eq(updated_enroll.id))
        .set((
            course_id.eq(updated_enroll.course_id),
            student_id.eq(updated_enroll.student_id),
            grade.eq(updated_enroll.grade.to_owned()),
        ))
        .execute(connection)
        .unwrap();

    enrollment_home().await
}

pub async fn new_enrollment(new_enroll: web::Form<NewEnrollment>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    let empty_input = "None".to_owned();

    // Ensure student is only enrolled in course once, if it is enrolled multiple times it should update instead.
    let enrollments = enrollment
        .filter(course_id.eq(new_enroll.course_id))
        .filter(student_id.eq(new_enroll.student_id))
        .load::<Enrollment>(connection)
        .unwrap();
    match enrollments.len() {
        0 => {}
        _ => {
            return update_enrollment(web::Json(Enrollment {
                id: enrollments.first().unwrap().id,
                course_id: new_enroll.course_id,
                student_id: new_enroll.student_id,
                grade: new_enroll.grade.to_owned(),
            }))
            .await
        }
    }

    match diesel::insert_into(enrollment::table())
        .values(NewEnrollment {
            course_id: new_enroll.course_id,
            student_id: new_enroll.student_id,
            grade: match new_enroll.0.grade {
                Some(n) if n == empty_input => None,
                None => None,
                Some(n) => Some(n),
            },
        })
        .execute(connection)
    {
        Ok(_) => {}
        Err(n) => panic!("Could not insert instructor, got error:\n{:?}", n),
    }

    enrollment_home().await
}

pub async fn enrollment_home() -> HttpResponse {
    HttpResponse::Ok().body(generate_body())
}

fn generate_body() -> String {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };
    let students = student
        .load::<Student>(connection)
        .expect("Could not load instructors");

    let courses = course
        .load::<Course>(connection)
        .expect("Could not load instructors");

    let raw_enrollments: Vec<(Enrollment, Option<Course>, Option<Student>)> = enrollment
        .left_join(course)
        .left_join(student)
        .load::<(Enrollment, Option<Course>, Option<Student>)>(connection)
        .expect("Could not load instructors");

    let mut enrollments: Vec<EnrollWithStudentCourse> = vec![];

    for e in &raw_enrollments {
        // Check if a course and student was found
        match e {
            // If there is an enrollment a course and a student
            (_enrollment, Some(_course), Some(_student)) => {
                enrollments.push(EnrollWithStudentCourse {
                    enrollment: _enrollment.clone(),
                    student: _student.clone(),
                    course: _course.clone(),
                })
            }

            // Captare all invalid values
            _ => continue,
        }
    }

    let page = EnrollmentPage {
        enrollments: enrollments,
        students: students,
        courses: courses,
        grades: GRADES.to_owned(),
    };
    page.render().unwrap()
}
