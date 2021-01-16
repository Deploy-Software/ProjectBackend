CREATE TABLE user_sessions (
     id serial PRIMARY KEY,
     user_id integer NOT NULL REFERENCES users (id),
     token varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
