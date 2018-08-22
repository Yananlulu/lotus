use super::context::Context;

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {});
