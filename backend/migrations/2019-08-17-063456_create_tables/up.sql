CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR UNIQUE NOT NULL,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	-- bio TEXT,
	-- avatar VARCHAR,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  -- country VARCHAR,
  -- industry VARCHAR NOT NULL,
  subject VARCHAR NOT NULL,
  -- tags VARCHAR NOT NULL,
  website VARCHAR,
  -- body TEXT NOT NULL,
	source_code VARCHAR,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  published BOOLEAN NOT NULL DEFAULT 'f',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

SELECT diesel_manage_updated_at('projects');
