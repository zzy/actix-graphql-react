use super::context::GraphQLContext;

// The root GraphQL mutation
pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {}
