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
