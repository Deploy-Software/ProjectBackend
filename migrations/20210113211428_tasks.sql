CREATE TABLE tasks (
     id serial PRIMARY KEY,
     target_id integer NOT NULL REFERENCES targets (id),
     name varchar(255) NOT NULL,
     about text,
     created_by integer NOT NULL REFERENCES users (id),
     date timestamp with time zone NOT NULL DEFAULT now()
);
