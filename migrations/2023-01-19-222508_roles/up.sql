-- Your SQL goes here
CREATE TABLE api_roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE users_roles (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL
);

INSERT INTO api_roles (name) VALUES ('YD');
INSERT INTO  api_roles (name) VALUES ('ADMINISTRATOR');
INSERT INTO  api_roles (name) VALUES ('API_GET_ANIME');
INSERT INTO  api_roles (name) VALUES ('API_GET_EPISODE');
INSERT INTO  api_roles (name) VALUES ('API_GET_SAISON');
INSERT INTO  api_roles (name) VALUES ('API_GET_OST');
INSERT INTO  api_roles (name) VALUES ('API_MODIFY_ANIME');
INSERT INTO  api_roles (name) VALUES ('API_MODIFY_EPISODE');
INSERT INTO  api_roles (name) VALUES ('API_MODIFY_SAISON');
INSERT INTO  api_roles (name) VALUES ('API_MODIFY_OST');
INSERT INTO  api_roles (name) VALUES ('API_CREATE_ANIME');
INSERT INTO  api_roles (name) VALUES ('API_CREATE_EPISODE');
INSERT INTO  api_roles (name) VALUES ('API_CREATE_SAISON');
INSERT INTO  api_roles (name) VALUES ('API_CREATE_OST');
INSERT INTO  api_roles (name) VALUES ('SEE_OWN_PROFILE');
INSERT INTO  api_roles (name) VALUES ('MODIFY_OWN_PROFILE');
INSERT INTO  api_roles (name) VALUES ('SEE_PERMISSIONS');
INSERT INTO  api_roles (name) VALUES ('MODIFY_PERMISSIONS');