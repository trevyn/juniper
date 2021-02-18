// to fuzz:
// rustup default nightly
// cargo install cargo-fuzz
// cargo fuzz run fuzz_target_1

#![no_main]
use libfuzzer_sys::fuzz_target;

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

use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, PartialEq, Eq)]
pub struct ArbString(String);

fuzz_target!(|query: ArbString| {
    let ctx = Context;

    juniper::execute_sync(
        &query.0,
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
                InputValue::object(vec![("field", InputValue::null())].into_iter().collect()),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
        &ctx,
    )
    .ok();
});
