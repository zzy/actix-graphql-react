use super::context::GraphQLContext;

// The root GraphQL query
pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {}
