CREATE TABLE target_data (
     id serial PRIMARY KEY,
     target_id integer NOT NULL REFERENCES targets (id),
     data varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
