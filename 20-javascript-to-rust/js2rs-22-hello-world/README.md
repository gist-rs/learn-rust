# Hello World

## New project

### 1. New project

```
cargo new my-app
```

### 2. Explore project

```
my-app/
├─ 🔩 .git
├─ 🔩 .gitignore
├─ 📄 Cargo.toml
└─ 📂 src
   └─ 🦀 main.rs
```

### 3. Run

```
cargo run
```

```
Compiling my-app v0.1.0 (./my-app)
Finished dev [unoptimized + debuginfo] target(s) in 0.82s Running `target/debug/my-app`
  Hello, world!
```

### 4. Build

```
cargo build
```

### 5. Test

```rs
fn main() {
  println!("Hello, world!");
}

#[test]
fn test() {
  main();
}
```
