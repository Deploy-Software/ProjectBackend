CREATE TABLE team_permissions (
    id serial PRIMARY KEY,
    team_id integer NOT NULL REFERENCES teams (id),
    permission_id integer NOT NULL REFERENCES permissions (id),
    date timestamp with time zone NOT NULL DEFAULT now()
);
