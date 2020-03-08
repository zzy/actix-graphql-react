CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR(70) UNIQUE NOT NULL,
	first_name VARCHAR(50) NOT NULL,
	last_name VARCHAR(50) NOT NULL,
	password VARCHAR NOT NULL,
	bio TEXT,
	avatar VARCHAR,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE sites (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

SELECT diesel_manage_updated_at('sites');
