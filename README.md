# actix-graphql-react

**NOT** complete - Clean boilerplate for GraphQL server & react-app built with Rust & TypeScript.

* Backend Demo (WIP): https://api.budshome.com/gql
* Frontend Demo (WIP): https://cms.budshome.com

## Features

- [x] DB migration with Diesel
- [x] User: query & mutation
- [x] Project: query & mutation
- [x] User register
- [ ] Sign up & Sign in
- [ ] Encrypt password & Change password
- [ ] Profile Update
- [ ] JSON web token authentication

## Stacks

### Backend

- [Rust](https://www.rust-lang.org/zh-CN)，[Rust 实例手册](https://books.budshome.com/rust-cookbook)，[Rust 参考手册](https://books.budshome.com/rust-reference)，[通过例子学 Rust](https://books.budshome.com/rust-by-example)，[Rust 程序设计语言（2018）](https://books.budshome.com/rust-lang)
- [actix-web](https://crates.io/crates/actix-web) - Web server
- [juniper](https://crates.io/crates/juniper) - GraphQL server，[juniper 中文文档](https://books.budshome.com/juniper)
- [diesel](https://crates.io/crates/diesel) - ORM
- [PostgreSQL](https://postgresql.org) / [MySql](https://dev.mysql.com) - Database
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken) - JSON Web Token
- [GraphQL Playground](https://github.com/prisma-labs/graphql-playground) - GraphQL UI

### Frontend

- [TypeScript](https://www.typescriptlang.org)
- [react](https://zh-hans.reactjs.org) - User Interfaces
- [apollo-client](https://www.apollographql.com/docs/react) - GraphQL client

## How to run?

### Backend

``` Bash
git clone https://github.com/zzy/actix-graphql-react.git
cd actix-graphql-react/backend/
```

#### Put the `DATABASE_URL` & `port` in a `.env` file.

Configure the database backend in Cargo.toml:

``` Toml
[dependencies]
diesel = { version = "<version>", features = ["<postgres|mysql>"] }
```

Then, run at the bash:

``` Bash
cargo install diesel_cli --no-default-features --features <postgres|mysql>
echo DATABASE_URL=<postgres|mysql>://username:password@localhost/actix_graphql > .env
echo GRAPHQL_PORT=5000 >> .env
```

#### Build & Release

- Build

``` Bash
diesel setup
diesel migration run
cargo build
cargo run
```

- Release

``` Bash
cargo build --release
cd target/release
./actix-graphql-react
```

GraphiQL: connect to http://localhost:5000/gql with browser.

### Frontend

``` Bash
git clone https://github.com/zzy/actix-graphql-react.git
cd actix-graphql-react/frontend/
# npm or yarn
<npm|yarn> install
<npm|yarn> start
```

Then go to http://localhost:5001


