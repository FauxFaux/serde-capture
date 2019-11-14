use std::collections::HashMap;

use failure::Error;
use serde_derive::Deserialize;
use serde_json::{json, Value};
use serde_remainder::Capture;

#[derive(Deserialize)]
struct Doc {
    foo: HashMap<String, Capture<JustValue>>,
}

#[derive(Deserialize)]
struct JustValue {
    value: u64,
}

#[test]
fn foo() -> Result<(), Error> {
    let src = serde_json::to_string_pretty(&json!(
    {
        "foo": {
            "bar": {"complex": true, "five": 5, "value": 2},
            "baz": {"complex": "hello", "value": 7},
        }
    }))
    .unwrap();

    let d: Doc = serde_json::from_str(&src)?;
    assert_eq!(2, d.foo["bar"].extracted.value);
    assert_eq!(
        json!({"complex": true, "five": 5, "value": 2}),
        serde_json::from_slice::<Value>(&d.foo["bar"].bytes)?
    );
    Ok(())
}
