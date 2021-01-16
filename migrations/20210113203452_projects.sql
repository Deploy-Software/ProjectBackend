CREATE TABLE projects (
     id serial PRIMARY KEY,
     organization_id integer NOT NULL REFERENCES organizations (id),
     name varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
