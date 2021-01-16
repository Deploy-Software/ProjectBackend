CREATE TABLE user_password (
     id serial PRIMARY KEY,
     user_id integer NOT NULL REFERENCES users (id),
     password varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);
