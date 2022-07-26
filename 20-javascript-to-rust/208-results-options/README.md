```rust
    let text: String = "Hello, world!".to_string();
    println!("String: {text:?}");

    let text = text.as_ref();
    println!("String: {text:?}");

    let text: Option<String> = Some("Hello, world!".to_string());
    println!("Option<String>: {text:?}");
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("text_length: {text_length:?}");
```
