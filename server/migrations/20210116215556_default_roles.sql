INSERT INTO roles (id, name) VALUES (1, 'ADMINISTRATOR');
INSERT INTO role_permissions (role_id, permission_id) VALUES (1, 1);
INSERT INTO role_permissions (role_id, permission_id) VALUES (1, 2);
INSERT INTO role_permissions (role_id, permission_id) VALUES (1, 3);
INSERT INTO role_permissions (role_id, permission_id) VALUES (1, 4);
INSERT INTO roles (id, name) VALUES (2, 'MANAGER');
INSERT INTO role_permissions (role_id, permission_id) VALUES (2, 1);
INSERT INTO role_permissions (role_id, permission_id) VALUES (2, 2);
INSERT INTO role_permissions (role_id, permission_id) VALUES (2, 3);
INSERT INTO role_permissions (role_id, permission_id) VALUES (2, 4);
INSERT INTO roles (id, name) VALUES (3, 'EMPLOYEE');
INSERT INTO role_permissions (role_id, permission_id) VALUES (3, 1);
INSERT INTO role_permissions (role_id, permission_id) VALUES (3, 2);
INSERT INTO roles (id, name) VALUES (4, 'READER');
INSERT INTO role_permissions (role_id, permission_id) VALUES (4, 1);