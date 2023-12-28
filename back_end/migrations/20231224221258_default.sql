-- Add migration script here
DROP TABLE IF EXISTS gexpense;
DROP TABLE IF EXISTS expense_group_people;
DROP TABLE IF EXISTS expense_group;
DROP TABLE IF EXISTS user;

CREATE TABLE expense_group (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE expense_group_people (
    id INTEGER PRIMARY KEY,
    expense_group_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (expense_group_id) REFERENCES expense_group (id),
    FOREIGN KEY (user_id) REFERENCES user (id)
);

CREATE TABLE expense (
    id INTEGER PRIMARY KEY,
    payed_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    name TEXT NOT NULL,
    expense_group_id INTEGER NOT NULL,
    time TEXT NOT NULL,
    FOREIGN KEY (expense_group_id) REFERENCES expense_group (id)
);

CREATE TABLE user (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL
);
