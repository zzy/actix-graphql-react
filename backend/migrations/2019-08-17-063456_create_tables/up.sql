CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR(100) UNIQUE NOT NULL,
	username VARCHAR(32) NOT NULL,
  -- title VARCHAR(100) NOT NULL DEFAULT '-',
	password VARCHAR(32) NOT NULL,
	-- bio TEXT,
	-- avatar VARCHAR(200),
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  banned BOOLEAN NOT NULL DEFAULT 'f'
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  -- country VARCHAR(40),
  -- industry VARCHAR(60) NOT NULL,
  subject VARCHAR(200) NOT NULL,
  -- tags VARCHAR(200) NOT NULL DEFAULT '-',
  website VARCHAR(200) NOT NULL DEFAULT '-',
  -- body TEXT NOT NULL,
	source_code VARCHAR(200) NOT NULL DEFAULT '-',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  published BOOLEAN NOT NULL DEFAULT 'f',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

SELECT diesel_manage_updated_at('projects');
