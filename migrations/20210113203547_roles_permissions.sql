CREATE TABLE role_permissions (
    id serial PRIMARY KEY,
    role_id integer NOT NULL REFERENCES roles (id),
    permission_id integer NOT NULL REFERENCES permissions (id),
    date timestamp with time zone NOT NULL DEFAULT now()
);
