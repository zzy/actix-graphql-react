# actix-web-juniper-react-apollo

Clean boilerplate for GraphQL app built with Rust & TypeScript.

- **Backend**: actix-web + juniper(GraphQL server) + diesel(PostgreSQL);
- **Frontend**: react + apollo(GraphQL client).

## How to run?

### Backend

``` Bash
 $ git clone https://github.com/zzy/actix-web-juniper-react-apollo.git
 $ cd actix-web-juniper-react-apollo/backend/
```

#### Put the `DATABASE_URL` in a `.env` file.

``` Shell
$ echo DATABASE_URL=postgres://username:password@localhost/actix-web-juniper-react-apollo > .env
```

### Frontend

``` Bash
 $ git clone https://github.com/zzy/actix-web-juniper-react-apollo.git
 $ cd actix-web-juniper-react-apollo/frontend/
```

#### yarn

``` Bash
 $ yarn install # yarn upgrade; yarn install
 $ yarn start
```

#### npm

``` Bash
 $ npm install # npm up; npm install
 $ npm start
```

## References
