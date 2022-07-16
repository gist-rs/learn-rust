# String

> https://blog.thoughtram.io/string-vs-str-in-rust/

```
                  buffer
                 /   capacity
                /   /  length
               /   /   /
            +–––+–––+–––+
stack frame │ • │ 8 │ 6 │ <- my_name: String
            +–│–+–––+–––+
              │
            |–│– – – –  capacity - – – – – –|
              v
            +–-–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+

            |– – – – length  – – – –|
```

```
            my_name: String   last_name: &str
            |- - - - - - -|   |- - - -|

            +–––+––––+––––+–––+–––+–––+
stack frame │ • │ 16 │ 13 │   │ • │ 6 │
            +–│–+––––+––––+–––+–│–+–––+
              │                 │
              │                 +–––––––––+
              │                           │
              │                           │
              │                         |–│– – – –  str – – – – |
              v                           v
            +–V–+–––+–––+–––+–––+–––+–––+–-–+–––+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+

```

## String and Error

```rs
// Main function
fn main() {
    // Call greet function
    greet("World");
}

// Greet function
fn greet(target: String) {
    // console.log(`Hello, ${target}`);
    println!("Hello, {}", target);
}

// Test function
#[test]
fn test() {
    // Call main function
    main();
}
```

## Output

```
> Executing task: cargo test --package js2rs-23-string-error --bin js2rs-23-string-error --all-features -- test --exact --nocapture <

   Compiling js2rs-23-string-error v0.1.0 (/Users/katopz/git/stackover/learn/20-javascript-to-rust/js2rs-23-string-error)
error[E0308]: mismatched types
 --> src/main.rs:4:11
  |
4 |     greet("World");
  |     ----- ^^^^^^^- help: try using a conversion method: `.to_string()`
  |     |     |
  |     |     expected struct `String`, found `&str`
  |     arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:8:4
  |
8 | fn greet(target: String) {
  |    ^^^^^ --------------
```
