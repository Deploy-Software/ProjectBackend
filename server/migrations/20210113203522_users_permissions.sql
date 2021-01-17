CREATE TABLE user_permissions (
    id serial PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users (id),
    permission_id integer NOT NULL REFERENCES permissions (id),
    date timestamp with time zone NOT NULL DEFAULT now()
);
