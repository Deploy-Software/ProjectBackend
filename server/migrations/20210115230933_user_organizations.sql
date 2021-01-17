CREATE TABLE user_organizations (
    id serial PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users (id),
    organization_id integer NOT NULL REFERENCES organizations (id),
    date timestamp with time zone NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX user_organizations_user_id_organization_id_idx ON user_organizations (user_id, organization_id);
