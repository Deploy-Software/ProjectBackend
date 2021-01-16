CREATE TABLE targets (
     id serial PRIMARY KEY,
     project_id integer NOT NULL REFERENCES projects (id),
     name varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
