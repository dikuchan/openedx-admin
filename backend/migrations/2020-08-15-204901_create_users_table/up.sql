CREATE TABLE users
(
    id            SERIAL PRIMARY KEY NOT NULL,
    email         VARCHAR UNIQUE     NOT NULL,
    password      VARCHAR            NOT NULL,
    login_session VARCHAR            NOT NULL DEFAULT ''
);
