INSERT INTO user (id, name, password_hash) VALUES
--- password is test for all
    (1, 'test1', '|$argon2id$v=19$m=19456,t=2,p=1$cgAWHsJF4oIOOZAq+2AOhg$DGnKU5GvNqS+G4U1N0ttzVxDdd4l/Edu3JcNUNa9i1Q'),
    (2, 'test2', '|$argon2id$v=19$m=19456,t=2,p=1$cgAWHsJF4oIOOZAq+2AOhg$DGnKU5GvNqS+G4U1N0ttzVxDdd4l/Edu3JcNUNa9i1Q'),
    (3, 'test3', '|$argon2id$v=19$m=19456,t=2,p=1$cgAWHsJF4oIOOZAq+2AOhg$DGnKU5GvNqS+G4U1N0ttzVxDdd4l/Edu3JcNUNa9i1Q'),
    (4, 'test4', '|$argon2id$v=19$m=19456,t=2,p=1$cgAWHsJF4oIOOZAq+2AOhg$DGnKU5GvNqS+G4U1N0ttzVxDdd4l/Edu3JcNUNa9i1Q'),
    (5, 'test5', '|$argon2id$v=19$m=19456,t=2,p=1$cgAWHsJF4oIOOZAq+2AOhg$DGnKU5GvNqS+G4U1N0ttzVxDdd4l/Edu3JcNUNa9i1Q');

INSERT INTO expense_group (id, name) VALUES
    (1, 'group1'),
    (2, 'group2'),
    (3, 'group3'),
    (4, 'group4');

INSERT INTO expense_group_people (id, expense_group_id, user_id) VALUES
--- first group
    (1,1,1),
    (2,1,2),
-- second group
    (3,2,1),
    (4,2,2),
--- third group
    (5,3,3),
    (6,3,5),
--- fourth group
    (7,4,2);

INSERT INTO expense (payed_type, amount, name, expense_group_id, time) VALUES
--- group 1
    ('EVEN 0', 500, 'Name1', 1, '2023-12-01 13:00:00.000'),
    ('EVEN 1', 500, 'Name2', 1, '2023-12-01 13:05:00.000'),
    ('OWED 0', 300, 'Name3', 1, '2023-12-01 13:10:00.000'),
    ('OWED 1', 300, 'Name4', 1, '2023-12-01 13:15:00.000'),
--- group 2
    ('EVEN 1', 300, 'Name5', 2, '2023-12-01 13:15:00.000'),
--- group 3
    ('OWED 1', 300, 'Name6', 3, '2023-12-01 13:15:00.000');