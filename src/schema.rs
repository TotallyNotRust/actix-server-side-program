// @generated automatically by Diesel CLI.

diesel::table! {
    course (course_id) {
        course_id -> Integer,
        title -> Text,
        credits -> Integer,
        department_id -> Nullable<Integer>,
    }
}

diesel::table! {
    course_assignment (id) {
        id -> Integer,
        instructor_id -> Integer,
        course_id -> Integer,
    }
}

diesel::table! {
    department (department_id) {
        department_id -> Integer,
        department_name -> Text,
        budget -> Integer,
        start_date -> Nullable<Text>,
        instructor_id -> Nullable<Integer>,
    }
}

diesel::table! {
    enrollment (enrollment_id) {
        enrollment_id -> Integer,
        course_id -> Integer,
        student_id -> Integer,
        grade -> Nullable<Text>,
    }
}

diesel::table! {
    instructor (instructor_id) {
        instructor_id -> Integer,
        last_name -> Nullable<Text>,
        first_name -> Text,
        hire_date -> Text,
    }
}

diesel::table! {
    office_assignment (id) {
        id -> Integer,
        instructor_id -> Integer,
        office_location -> Nullable<Text>,
    }
}

diesel::table! {
    student (student_id) {
        student_id -> Integer,
        last_name -> Nullable<Text>,
        first_name -> Text,
        enrollment_date -> Text,
    }
}

diesel::joinable!(course -> department (department_id));
diesel::joinable!(course_assignment -> course (course_id));
diesel::joinable!(course_assignment -> instructor (instructor_id));
diesel::joinable!(department -> instructor (instructor_id));
diesel::joinable!(enrollment -> course (course_id));
diesel::joinable!(enrollment -> student (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    course,
    course_assignment,
    department,
    enrollment,
    instructor,
    office_assignment,
    student,
);
