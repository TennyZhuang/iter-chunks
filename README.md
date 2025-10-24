# iter-chunks

[![Crates.io](https://img.shields.io/crates/v/iter-chunks.svg)](https://crates.io/crates/iter-chunks)
[![Docs.rs](https://docs.rs/iter-chunks/badge.svg)](https://docs.rs/iter-chunks)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://github.com/rust-lang/rust/blob/master/RELEASES.md)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

`iter-chunks` extends the standard [`Iterator`] trait with a `chunks` method that yields lending iterators without resorting to interior mutability.

Please read the [API documentation on docs.rs](https://docs.rs/iter-chunks/) for details.

[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iter-chunks = "0.3"
```

## Examples

`Chunks` implements a lending iterator, so iteration currently happens via a `while let` loop.

```rust
use iter_chunks::IterChunks;

let arr = [1, 1, 2, 2, 3];
let expected = [vec![1, 1], vec![2, 2], vec![3]];
let mut chunks = arr.into_iter().chunks(2);
let mut i = 0;
while let Some(chunk) = chunks.next() {
    assert_eq!(chunk.collect::<Vec<_>>(), expected[i]);
    i += 1;
}
```

## Why create this crate?

[itertools](https://crates.io/crates/itertools) provides many awesome extensions, including [`chunks`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.chunks). It is very handy, but it uses `RefCell` internally, so the resulting iterator is not `Send`.

It's a very common usecase in async context, which requires `Chunks` to be `Send`:

```rust
async fn do_some_work(input: impl Iterator<Item = i32>) {
    for chunk in input.chunks(1024) {
        for v in chunk {
            handle(v).await
        }
        do_some_flush().await
    }
}
```


This crate implements `chunks` without `RefCell`, so `Chunks` is both `Send` and `Sync`. As a trade-off, `Chunks` cannot currently implement `Iterator` because the standard library still lacks a lending iterator abstraction, even though GAT is now stable.

## Future works

The lack of the `Iterator` implementation is inconvenient, and the best solution is to wait for a stable lending iterator trait (either in `std` or via crates such as [`lending-iterator`][lending-iterator]). In the short term we can provide more helper adaptors such as `nth`, `for_each`, `try_for_each`, and so on.

Contributions are welcome.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


[GAT]: https://github.com/rust-lang/rust/issues/44265
[lending-iterator]: https://crates.io/crates/lending-iterator
