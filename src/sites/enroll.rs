extern crate askama;

use std::vec;

use actix_web::cookie::Display;
use lazy_static::lazy_static;

use diesel::associations::HasTable;
use diesel::prelude::*;

use actix_form_data::{Error, Field, Form, Value};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use askama::Template;

use crate::models::{Course, Enrollment, Instructor, NewEnrollment, NewInstructor, Student};

use crate::schema::{
    course::dsl::{course, course_id as cid},
    enrollment::dsl::{course_id, enrollment, student_id},
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

impl IntoIterator for EnrollmentPage {
    type Item = EnrollWithStudentCourse;
    type IntoIter = std::vec::IntoIter<EnrollWithStudentCourse>;

    fn into_iter(self) -> Self::IntoIter {
        return self.enrollments.into_iter();
    }
}

pub async fn new_enrollment(new_enroll: web::Form<NewEnrollment>) -> HttpResponse {
    let connection = &mut match SqliteConnection::establish("./database.db") {
        Ok(n) => n,
        Err(n) => panic!("{:?}", n),
    };

    let empty_input = "None".to_owned();

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
        Ok(_) => println!(
            "Created a new enrollment for student with id {} in course with id {}",
            &new_enroll.0.student_id, &new_enroll.0.course_id
        ),
        Err(n) => println!("Could not insert instructor, got error:\n{:?}", n),
    }

    enrollment_home().await
}

pub async fn enrollment_home() -> HttpResponse {
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

    println!("{:#?}", raw_enrollments);

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

    println!("{:?}", enrollments);

    let page = EnrollmentPage {
        enrollments: enrollments,
        students: students,
        courses: courses,
        grades: GRADES.to_owned(),
    };

    HttpResponse::Ok().body(page.render().unwrap())
}
