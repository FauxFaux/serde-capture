use std::collections::HashMap;

use failure::Error;
use serde_capture::CaptureJson;
use serde_capture::Nothing;
use serde_derive::Deserialize;
use serde_json::json;
use serde_json::Value;

#[derive(Deserialize)]
struct Doc {
    foo: HashMap<String, CaptureJson<JustValue>>,
}

#[derive(Deserialize)]
struct DocWithNothing {
    foo: HashMap<String, CaptureJson<Nothing>>,
}

#[derive(Deserialize)]
struct JustValue {
    value: u64,
}

#[test]
fn smoke() -> Result<(), Error> {
    let src = serde_json::to_string_pretty(&json!(
    {
        "foo": {
            "bar": {"complex": true, "five": 5, "value": 2},
            "baz": {"complex": "hello", "value": 7},
        }
    }))
    .unwrap();

    let d: Doc = serde_json::from_str(&src)?;
    assert_eq!(2, d.foo["bar"].inner.value);
    assert_eq!(
        json!({"complex": true, "five": 5, "value": 2}),
        serde_json::from_slice::<Value>(&d.foo["bar"].bytes)?
    );

    let d: DocWithNothing = serde_json::from_str(&src)?;
    assert_eq!(
        json!({"complex": true, "five": 5, "value": 2}),
        serde_json::from_slice::<Value>(&d.foo["bar"].bytes)?
    );

    Ok(())
}
