# POC: `async_std` with `wasm`

## Known issues

- can't use `test` button, stuck at handles = never finish.
- can't pack as `wasm`.
- build failed because of `std`?

```
   Compiling mio v0.8.4
error[E0432]: unresolved import `crate::sys::IoSourceState`
  --> /Users/katopz/.cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.8.4/src/io_source.rs:12:5
   |
12 | use crate::sys::IoSourceState;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `IoSourceState` in `sys`
```
