// cargo afl build
// cargo afl fuzz -i in -o out ../target/debug/juniper_fuzz

#[macro_use]
extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let ctx = Context;

            juniper::execute_sync(
                &s,
                None,
                &Schema::new(
                    Query,
                    EmptyMutation::<Context>::new(),
                    EmptySubscription::<Context>::new(),
                ),
                &[
                    ("emptyObj".to_string(), InputValue::Object(vec![])),
                    (
                        "literalNullObj".to_string(),
                        InputValue::object(
                            vec![("field", InputValue::null())].into_iter().collect(),
                        ),
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                &ctx,
            )
            .ok();
        }
    });
}

use juniper::*;

pub struct Context;

impl juniper::Context for Context {}

pub struct Query;

#[derive(juniper::GraphQLInputObject)]
struct ObjectInput {
    field: Nullable<i32>,
}

#[graphql_object(Context=Context)]
impl Query {
    fn is_explicit_null(arg: Nullable<i32>) -> bool {
        arg.is_explicit_null()
    }

    fn object_field_is_explicit_null(obj: ObjectInput) -> bool {
        obj.field.is_explicit_null()
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
