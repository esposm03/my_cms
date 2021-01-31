use crate::collection::Entry;
use actix_web::{HttpRequest, Responder, web};
use juniper::{DefaultScalarValue, EmptyMutation, EmptySubscription, FieldResult, GraphQLValue, RootNode, graphql_object};

/// Handler for the `/graphql` route
pub async fn graphql_route(req: HttpRequest, payload: web::Payload, schema: web::Data<Schema>) -> impl Responder {
    juniper_actix::graphql_handler(&schema, &Context(vec![]), req, payload).await
}

/// Handler for the `/graphiql` route
pub async fn graphiql_route() -> impl Responder {
    juniper_actix::graphiql_handler("/graphql", None).await
}

/// Build a `Schema`
pub fn build_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}


pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Context(Vec<Entry>);

pub struct Query;
#[graphql_object(context = Context)]
impl Query {
    fn entries(context: &Context, collection: String) -> FieldResult<Vec<Entry>> {
        Ok(
            context.0
                .iter()
                .filter(|e| e.type_name(&"Hello".into()).unwrap() == collection)
                .cloned()
                .collect()
        )
    }
}