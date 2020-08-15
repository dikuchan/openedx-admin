CREATE TABLE courses
(
    id          SERIAL PRIMARY KEY,
    course      VARCHAR(256) UNIQUE NOT NULL,
    description VARCHAR,
    course_link VARCHAR UNIQUE
);

CREATE TABLE students
(
    id          SERIAL PRIMARY KEY,
    student     VARCHAR(256) UNIQUE,
    email       VARCHAR UNIQUE      NOT NULL,
    password    VARCHAR(256)        NOT NULL,
    first_name  VARCHAR(128) UNIQUE NOT NULL,
    second_name VARCHAR(128) UNIQUE NOT NULL
);

CREATE TABLE courses_students
(
    courses_id  INTEGER REFERENCES courses (id) ON UPDATE CASCADE ON DELETE CASCADE,
    students_id INTEGER REFERENCES students (id) ON UPDATE CASCADE,
    CONSTRAINT courses_students_key PRIMARY KEY (courses_id, students_id)
);

CREATE TABLE tests
(
    id        SERIAL PRIMARY KEY,
    course_id INTEGER REFERENCES courses (id),
    test      VARCHAR(256) UNIQUE
);

CREATE TABLE attempts
(
    id           SERIAL PRIMARY KEY,
    student_id   INTEGER REFERENCES students (id),
    course_id    INTEGER REFERENCES courses (id),
    test_id      INTEGER REFERENCES tests (id),
    grade        SMALLINT CHECK (grade > 0),
    attempt_link VARCHAR UNIQUE
);
