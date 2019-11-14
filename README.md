## serde-capture

Capture some part of a document during deserialisation, and store
it serialised, instead of as a structure. This might use less memory.

This provides a:
```rust
pub struct CaptureJson<T> {
    pub inner: T,
    pub bytes: Box<[u8]>,
}
```

... which you can embed in your structure:

```rust
struct Doc {
    foo: Vec<CaptureJson<Nothing>>,
}
```

... which should be smaller than `Vec<Value>`, but with significantly
slower access.
