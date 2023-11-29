CREATE TABLE expense_group (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE expense_group_people (
    id INTEGER PRIMARY KEY,
    expense_group_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (expense_group_id) REFERENCES expense_group (expense_group_id)
);

CREATE TABLE expense (
    id INTEGER PRIMARY KEY,
    payed_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    expense_group_id INTEGER NOT NULL,
    time TEXT NOT NULL,
    FOREIGN KEY (expense_group_id) REFERENCES expense_group (expense_group_id)
);

-- example values for testing
insert into expense_group (name) VALUES ('test1');
insert into expense_group (name) VALUES ('test2');

insert into expense_group_people (expense_group_id, name) VALUES (1, 'Person1'), (1, 'Person2'), (2, 'Person3'), (2, 'Person4');

insert into expense (payed_type, amount, expense_group_id, time) VALUES ('EVEN 0', 300, 1, '2023-11-29 12:19');
insert into expense (payed_type, amount, expense_group_id, time) VALUES ('EVEN 1', 200, 1, '2023-11-29 12:20');