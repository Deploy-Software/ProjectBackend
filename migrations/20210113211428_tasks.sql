CREATE TABLE tasks (
     id serial PRIMARY KEY,
     target_id integer REFERENCES targets (id),
     name varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
