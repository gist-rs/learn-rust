# learn-rs

## 🦀 Let's learn Rust together!

1. Frontend `Alice` want to build `web app`. // `main`, todomvc, `wasm`, yew, github page
1. Backend `Bob` want to build `RESTful API` for Alice. // `lib`, actix
1. React Frontend `Cat` want to build `wasm` to use with `ReactJS`. // `wasm`, `wasm-bindgen`
1. SmartContract Developer `Dog` want to build smart contract. // `solana`, `anchor`
1. Artist `Elephant` want to create `NFT`. // `metaplex`, `sugar`
1. Trainer `Fox` want to create ticket `NFT`. // `cardinal`
1. Attendee `Giraffe` want to use rented `NFT` as ticket. // `cardinal`
1. Platform Owner `Hippo` want to make a subscription model. // `cadinal`
1. Bot Developer `Iguana` want to build chatbot. // `discord`, `wasm`, `cloudflare`
1. Data Analyst `Jaguar` want to read price data from oracle. // `Dune`
1. Data Engineer `Kiwi` want to extract data from a contract. // `Pyth`, `explorer`, `metaplex`
1. Data Scientist `Lion` want to build and deploy model. // `tensorflow`, `wasm`, `PostgresML`
1. Game Developer // `bevy`, `unreal`
1. Wasm Developer // `wasmer`, `wasmtime`, `solana-playground`

## Base

1. Setup: `vscode`, `rust-analyzer`, `cargo`, main/lib/workspace.
1. Hello World: crate, run, debug.

## R5

> Basic main, focus on simple and straight forward. // Able to hello, condition, loop

1. variable: str, slice,`string`, `vec`, `usize`, ref, `mut`.
1. Structs
1. #[derive(Copy, Clone, Debug, PartialEq)]
1. method: `fn`, `println`, `json`, `serde`, closures.
1. condition: `if`, `match`, `if let`, match guards.
1. Flow of Control: `while`, `loop`, `for`, `range`, `iter`, `map`, `filter`, `collect`, `rev`, `chain`, `inspect`, `clone` , `copied`.
1. find, rfind, find_map, collect.
1. Vec: first, last, get, first_mut, last_mut, to_vec, len, is_empty, with_capacity, capacity, push, pop, insert, remove, resize, resize_with, truncate, clear, extend, split_off, append, drain, retain, dedup, dedup_by, dedup_by_key, concat, join, .
1. conversions: `as`, `as_str`, `to_string`, `From`, `into`, `try_into`, `unwrap`/`?`.
1. error: concept, `panic`, `expect`, `handling`, `anyhow`(main).
1. test, assert!.
1. Option, Result, `ok_or_else`.

## R4

> Advance main, focus on complex, combine, compose // Able to create basic app + lib.

1. variable: `OsString`, `OsStr`, `tuples`, `bigint`, decimals, `lazy_static`.
1. Generic Structs.
1. #[repr(C)],`trait`, `imp`.
1. method: derive, cast with `enum`, `serde`, `borsh`,
1. condition: match, `enum`, `strum`.
1. Flow of Control: `enumerate`, `into_iter`, `filter_map`, `flat_map`, `flatten`. // Happy case
1. Iterator: `fold`,`rfold`,`try_fold`, `try_rfold`, `nth`, `nth_back`, `last`
1. Simple Accumulation: count, sum, product,max, min, max_by, min_by, max_by_key, min_by_key
1. conversions: `to_owned`, `as_ref`, down casting.
1. error: handling loop error, `thiserror`(lib).
1. doc test, cfg(test), allow, feature, package, version.
1. read_line, write, write_all, flush, seek, stdin, stdout.

## R3

> Basic lib, Serve cli, focus on app, use existing lib. // Able to create async app + advance lib

1. HashSet, BTreeSet, LinkedList, HashMap, BTreeMap.
1. `CStr`, `CString`, lifetime, `box`, `dyn`, `const`, `regex`.
1. `take`, `take_while`, `skip`, `skip_while`, `peekable`, `fuse`, `zip`, `by_ref`.
1. Iterator: eq, ne, lt, le, gt, any, all, position, rposition, and ExactSizeIterator.
1. entry, or_insert, or_insert_with, and_modify.
1. extends, partition, for_each and try_for_each.
1. custom struct, module, `Value`, `JsValue`, `JsError`
1. Binding with @ Patterns,
1. sync_std::task::block_on.
1. async: fetch via `reqwest`, `tokio` test.
1. error: custom error, `map_err`. // Failed case
1. cli `clap`, `StructOpt`.
1. mvp `TODO-MVC`, `yew`, `trunk`.
1. wasm worker,`cloudflare`, `vanilla-rpc`, `agora-rpc`.
1. use-web-wasm: `wasm-bindgen`, via `firebase`.
1. connect db.

## R2

> Advance lib, Serve API, Consume as web // Able to create lib, wasm

1. rc, arc, refcall, heap, stack.
1. thread::spawn, join, future, `spawn_local`, `serde_wasm_bindgen` getter/setter, `IntoWasmAbi`, `FromWasmAbi`.
1. Subtraits, Type-Associated Functions, Traits That Define Relationships Between Types,
1. Associated types, Generic Traits , impl Trait, Associated Consts.
1. Structs with Lifetime Parameters.
1. advance cargo, optimize, scope, target, workspace,
1. build lib and use in mvp `API` actix.
1. use same lib as wasm in app.
1. polymorphic: trait objects and generics, orphan rule.
1. rust design pattern.
1. `solana`: read account, `pyth`
1. release, profile.
1. `serde_wasm_bindgen`, `gloo`, `#[wasm_bindgen(getter_with_clone)]`

## R1

> Integrate with other, Security, Speed, Prod, CI/CD // Able to use advance lib, deploy.

1. `Mutex`, `Ref`, `RefCell`, `MutexGuard`, `OnceCell`.
1. `VecDeque`, `BinaryHeap`.
1. Cow.
1. macro_rules.
1. `Fn`, `FnMut`, `FnOnce`.
1. type state: https://yoric.github.io/post/rust-typestate/
1. macro, expand.
1. bench, optimize, `watt`.
1. extern.
1. `solana`: read/write `counter`, cpi.
1. `anchor`: read `tulip`, `friktion`.
1. `tensorflow`.
1. deploy: docker, `cloud run`, multi-stage, build caching.
1. deploy: github page, github action.
1. `wasm-rpc`, rpc blocking/non-blocking, vanilla, lite, playground.
1. OS-specific functionality. `#[cfg(target_os = "macos")]`
1. thread::spawn
1. unsafe.
1. foreign function interface (FFI)
1. cargo bench, [Continuous Benchmark](https://github.com/marketplace/actions/continuous-benchmark)

## Extras

1. 📚 A half-hour to `READ` Rust: https://fasterthanli.me/articles/a-half-hour-to-learn-rust
1. 🎮 Explore rust with playground: https://tourofrust.com/
1. 📚 Easier to read compare with doc: https://dhghomon.github.io/easy_rust/Chapter_1.html
1. 🎮 Fill in the blank quiz: https://github.com/sunface/rust-by-practice
1. 🎮 Feel like fixing bugs: https://github.com/rust-lang/rustlings
