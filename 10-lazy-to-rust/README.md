# You're too lazy to learn Rust

> Learn Rust with minimal effort, no deep drive, aim to build app/ use lib.

- [ ] hello, println!, macro
- [ ] str, String
- [ ] if let
- [ ] match
- [ ] json!
- [ ] request
- [ ] async/await
- [ ] Future

  > - Once you have a `Future`, you may:
  > - Call `spawn` to add the task to your executor’s task queue and not wait around for an answer.
  > - `await` a result from the task, yielding control to other tasks until a result is available.
  > - Use `join` macros to execute several tasks at once and wait for all of them to complete.
  > - Call `select` to execute several tasks and continue when one of them returns a result.

- [ ] Result, Some, None, Ok, Err

```rust
use std::convert::TryInto;
let z: u32 = (5000_u64).try_into().expect("Conversion error");
```

> The `try_into` function returns a Result type. You can access the contents as you do with other Result types. For example, you can:
>
> - `unwrap` the contents and crash if the conversion failed.
> - `unwrap_or` to substitute a default value.
> - `match` on the Result to handle the error explicitly.
> - `expect`.

> In larger programs, it’s inevitable that you will have to convert between types. You can use as \_ when Rust can be certain of the desired type—but it isn’t recommended as your first port of call. Generally, try to favor type conversions in the following order of preference:
>
> 1. Using `into()` is precise and optimizes very well.
> 2. `try_into()` lets you handle failed conversions.
> 3. Use `as type` when you are certain that conversions are safe.
> 4. Use `as _` when you are really stuck.

- [ ] Use `Vec` instead of arrays.
- [ ] unwrap, ?
- [ ] impl, struct, enum, trait
- [ ] `#[derive(Debug)]`
- [ ] `#[test]`
- [ ] `#[cfg(feature = "french")]`
- [ ] mod
- [ ] pipe closure.
- [ ] reference cycle using `Rc`.// `Rc` and `RefCell` create a dynamic garbage collection structure
- [ ] `Arc`, which is an atomic reference-counted smart pointer + thread-safe access.
- [ ] Generic Type/ Method overload

## Rust Anatomy

- cargo: single, workspace
- dependency: dev, release
- app, lib
- use, mod

## Rust Why?

- Google:
  - https://www.youtube.com/watch?v=Gnp4XP1b82E
- Microsoft:
  - https://www.youtube.com/watch?v=1uAsA1hm52I
  - https://twitter.com/markrussinovich/status/1571995117233504257
- Apple:
  - https://jobs.apple.com/en-us/search?search=rust&sort=relevance
- Facebook:
  - https://engineering.fb.com/2021/04/29/developer-tools/rust/
- AWS:
  - https://aws.amazon.com/blogs/opensource/sustainability-with-rust/
- Android:
  - https://source.android.com/docs/setup/build/rust/building-rust-modules/overview

## Rust Journey

- Programming Rust, 2nd Edition by Jim Blandy, Jason Orendorff, Leonora F. S. Tindall: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/
  > Learn by errors, In deep take a look in memory, compare to C/C++, Python, used by [standford](https://web.stanford.edu/class/cs110l/handouts/course-information/)
- The Rust Programming Language (Covers Rust 2018) by Steve Klabnik and Carol Nichols: https://nostarch.com/Rust2018
  > Learn by why we need this fn, good for newbies.
- Rust for Rustaceans by Jon Gjengset: https://rust-for-rustaceans.com/
  > A sequel from `The Rust Programming Language`.
-
