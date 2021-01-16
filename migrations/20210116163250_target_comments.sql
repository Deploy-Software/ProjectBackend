CREATE TABLE target_comments (
     id serial PRIMARY KEY,
     target_id integer NOT NULL REFERENCES targets (id),
     text text NOT NULL,
     created_by integer NOT NULL REFERENCES users (id),
     date timestamp with time zone NOT NULL DEFAULT now()
);
