table! {
    attempts (id) {
        id -> Int4,
        student_id -> Nullable<Int4>,
        course_id -> Nullable<Int4>,
        test_id -> Nullable<Int4>,
        grade -> Nullable<Int2>,
        attempt_link -> Nullable<Varchar>,
    }
}

table! {
    courses (id) {
        id -> Int4,
        course -> Varchar,
        description -> Nullable<Varchar>,
        course_link -> Nullable<Varchar>,
    }
}

table! {
    courses_students (courses_id, students_id) {
        courses_id -> Int4,
        students_id -> Int4,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    students (id) {
        id -> Int4,
        student -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        second_name -> Varchar,
    }
}

table! {
    tests (id) {
        id -> Int4,
        course_id -> Nullable<Int4>,
        test -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

joinable!(attempts -> courses (course_id));
joinable!(attempts -> students (student_id));
joinable!(attempts -> tests (test_id));
joinable!(courses_students -> courses (courses_id));
joinable!(courses_students -> students (students_id));
joinable!(login_history -> users (user_id));
joinable!(tests -> courses (course_id));

allow_tables_to_appear_in_same_query!(
    attempts,
    courses,
    courses_students,
    login_history,
    students,
    tests,
    users,
);
