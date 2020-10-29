use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/mutation.graphql",
    schema_path = "schema.graphql"
)]
pub struct TestCircularMutation;

fn main() {
    println!("Hello, world!");
}
