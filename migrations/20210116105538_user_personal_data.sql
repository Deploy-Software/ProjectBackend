CREATE TABLE user_personal_data (
     id serial PRIMARY KEY,
     user_id integer NOT NULL REFERENCES users (id),
     name varchar(255) NOT NULL,
     job_title varchar(255),
     date timestamp with time zone NOT NULL DEFAULT now()
);
