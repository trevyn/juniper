// cargo install afl
// cargo afl build
// cargo afl fuzz -i in -o out ../target/debug/juniper_fuzz
// cargo afl tmin -i out/crashes/<infile> -o <outfile> ../target/debug/juniper_fuzz

#[macro_use]
extern crate afl;

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

fn main() {
    fuzz!(|data: &[u8]| {
        exec(data);
    });
}

#[cfg(test)]
mod tests {
    static CRASHES_DIR: include_dir::Dir = include_dir::include_dir!("crashes");

    #[test]
    fn test_crashes() {
        for entry in CRASHES_DIR.find("crash-*.min").unwrap() {
            eprintln!("Testing {}", entry.path().display());
            let data = CRASHES_DIR.get_file(entry.path()).unwrap().contents();
            super::exec(data);
        }
    }
}

pub fn exec(data: &[u8]) {
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
                    InputValue::object(vec![("field", InputValue::null())].into_iter().collect()),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            &ctx,
        )
        .ok();
    }
}
