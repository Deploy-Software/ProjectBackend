CREATE TABLE roles (
     id serial PRIMARY KEY,
     name varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
