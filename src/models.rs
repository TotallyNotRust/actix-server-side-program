use diesel::{
    deserialize, insert_into,
    internal::derives,
    sql_types::{Integer, Nullable},
    Associations, Identifiable, Insertable, Queryable,
};
use serde::Deserialize;

use crate::schema::*;

#[derive(Queryable, Debug, Identifiable, PartialEq)]
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
