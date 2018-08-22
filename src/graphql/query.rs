use super::super::env;
use super::context::Context;

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> String {
        env::version()
    }
});
