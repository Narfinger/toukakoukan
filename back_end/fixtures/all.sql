INSERT INTO user (id, name, password_hash) VALUES (1, 'test1', 'xx'), (2, 'test2', 'xx'), (3, 'test3', 'xx');
INSERT INTO expense_group (id, name) VALUES (1, 'group1'), (2, 'group2'), (3, 'group3'), (4, 'group4');
INSERT INTO expense_group_people (id, expense_group_id, user_id) VALUES (1,1,1), (2,1,2), (3,2,1), (4,2,2), (5,2,3);
