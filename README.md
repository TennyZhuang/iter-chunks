# iter-chunks

Yet another crate to provides `chunks` method to rust `Iterator`.

Please read the [API documentation here](https://docs.rs/iter-chunks/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iter_chunks = "0.1"
```

## Examples

Currently, only while loop over `Chunks` is supported.

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

[itertools](https://crates.io/crates/itertools) provides many awesome extensions, including [`chunks`](https://docs.rs/itertools/0.10.3/itertools/trait.Itertools.html#method.chunks). It's really useful, but it use `RefCell` internally, causing it's not `Send`.

This crate implements `chunks` without `RefCell`, so `Chunks` is both Send and Sync. As a price, `Chunks` cannot implements `Iterator` (which can be resolved later by [GAT][GAT] and LendingIterator).

## Future works

The lack of the `Iterator` implementation is very difficult to use, and the best solution is to wait for [GAT][GAT] and a suitable LendingIterator crate. But in the short term, we can consider providing some common methods such as `nth`, `for_each`, `try_for_each`, and so on.

Contributions are welcome.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


[GAT]: https://github.com/rust-lang/rust/issues/44265
