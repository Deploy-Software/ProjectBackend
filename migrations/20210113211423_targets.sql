CREATE TABLE targets (
     id serial PRIMARY KEY,
     project_id integer NOT NULL REFERENCES projects (id),
     name varchar(255) NOT NULL,
     about text,
     created_by integer NOT NULL REFERENCES users (id),
     date timestamp with time zone NOT NULL DEFAULT now()
);
