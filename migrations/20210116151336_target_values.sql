CREATE TABLE target_values (
     id serial PRIMARY KEY,
     target_id integer NOT NULL REFERENCES targets (id),
     value varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
