CREATE TABLE target_activity (
      id serial PRIMARY KEY,
      target_id integer NOT NULL REFERENCES targets (id),
      text varchar(255) NOT NULL,
      created_by integer NOT NULL REFERENCES users (id),
      date timestamp with time zone NOT NULL DEFAULT now()
);
