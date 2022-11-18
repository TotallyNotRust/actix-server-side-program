-- Your SQL goes here
CREATE TABLE instructor (
    instructor_id INTEGER NOT NULL,
    last_name TEXT,
    first_name TEXT NOT NULL,
    hire_date TEXT NOT NULL,
    PRIMARY KEY (instructor_id AUTOINCREMENT)
);

CREATE TABLE office_assignment (
    id INTEGER NOT NULL,
    instructor_id INTEGER NOT NULL,
    office_location TEXT,
    PRIMARY KEY (ID AUTOINCREMENT),
    FOREIGN KEY (instructor_id) REFERENCES instructor(id) ON DELETE CASCADE
);

CREATE TABLE department (
    department_id INTEGER NOT NULL,
    department_name TEXT NOT NULL,
    budget INT NOT NULL,
    start_date TEXT,
    instructor_id INT,
    PRIMARY KEY (department_id AUTOINCREMENT),
    FOREIGN KEY (instructor_id) REFERENCES instructor(instructor_id) ON DELETE CASCADE
);

CREATE TABLE course (
    course_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    credits INTEGER NOT NULL,
    department_id INTEGER,
    PRIMARY KEY (course_id AUTOINCREMENT),
    FOREIGN KEY (department_id) REFERENCES department(department_id) ON DELETE SET NULL
);

CREATE TABLE course_assignment (
    id INTEGER NOT NULL,
    instructor_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    PRIMARY KEY (id AUTOINCREMENT),
    FOREIGN KEY (instructor_id) REFERENCES instructor(instructor_id) ON DELETE CASCADE,
    FOREIGN KEY (course_id) REFERENCES course(course_id) ON DELETE CASCADE
);

CREATE TABLE student (
    student_id INTEGER NOT NULL,
    last_name TEXT,
    first_name TEXT NOT NULL,
    enrollment_date TEXT NOT NULL,
    PRIMARY KEY (student_id AUTOINCREMENT)
);

CREATE TABLE enrollment (
    enrollment_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    student_id INTEGER NOT NULL,
    grade TEXT,
    PRIMARY KEY (enrollment_id AUTOINCREMENT),
    FOREIGN KEY (course_id) REFERENCES course(course_id),
    FOREIGN KEY (student_id) REFERENCES student(student_id) 
);

