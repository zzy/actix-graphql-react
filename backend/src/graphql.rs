use super::context::GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

use super::data::Users;
use super::models::{User, CreateUserInput};

// The root GraphQL query
pub struct QueryRoot;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a PgConnection (for brevity's sake)
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::all_todos(conn)
    }

    #[graphql(name = "doneTodos")]
    pub fn done_todos(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::done_todos(conn)
    }

    #[graphql(name = "notDoneTodos")]
    pub fn done_todos(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::not_done_todos(conn)
    }

    #[graphql(name = "getTodoById")]
    pub fn get_todo_by_id(
        context: &GraphQLContext,
        id: i32,
    ) -> FieldResult<Option<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::get_todo_by_id(conn, id)
    }
}

// The root GraphQL mutation
pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    #[graphql(name = "createTodo")]
    pub fn create_todo(
        context: &GraphQLContext,
        input: CreateUserInput,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::create_todo(conn, input)
    }

    #[graphql(name = "markTodoAsDone")]
    pub fn mark_todo_as_done(context: &GraphQLContext, id: i32) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_done(conn, id)
    }

    #[graphql(name = "markTodoAsNotDone")]
    pub fn mark_todo_as_not_done(
        context: &GraphQLContext,
        id: i32,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_not_done(conn, id)
    }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
