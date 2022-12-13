use diesel::{
    deserialize, insert_into,
    internal::derives,
    sql_types::{Integer, Nullable},
    Associations, Identifiable, Insertable, Queryable,
};
use serde::Deserialize;

use crate::schema::*;

#[derive(Queryable, Debug, Identifiable, PartialEq, Clone)]
#[diesel(table_name = instructor)]
pub struct Instructor {
    pub id: i32,
    pub last_name: Option<String>,
    pub first_name: String,
    pub hire_date: String,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = instructor)]
pub struct NewInstructor {
    pub last_name: Option<String>,
    pub first_name: String,
    pub hire_date: String,
}

#[derive(Queryable, Debug, Identifiable, PartialEq, Clone)]
#[diesel(table_name = student)]
pub struct Student {
    pub id: i32,
    pub last_name: Option<String>,
    pub first_name: String,
    pub enrollment_date: String,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = student)]
pub struct NewStudent {
    pub last_name: Option<String>,
    pub first_name: String,
    pub enrollment_date: String,
}

#[derive(Queryable, Debug, Identifiable, PartialEq, Clone)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub credits: i32,
    pub department_id: Option<i32>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = course)]
pub struct NewCourse {
    pub title: String,
    pub credits: i32,
    pub department_id: Option<i32>,
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone)]
#[diesel(table_name = enrollment)]
pub struct Enrollment {
    pub id: i32,
    pub course_id: i32,
    pub student_id: i32,
    pub grade: Option<String>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = enrollment)]
pub struct NewEnrollment {
    pub course_id: i32,
    pub student_id: i32,
    pub grade: Option<String>,
}
