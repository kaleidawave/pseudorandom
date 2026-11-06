# Psuedorandom - pseudorandom number generators

[![crates.io badge](https://img.shields.io/crates/v/pseudorandom)](https://crates.io/crates/pseudorandom)
[![docs.rs badge](https://img.shields.io/docsrs/pseudorandom)](https://docs.rs/syraise/latest/pseudorandom/)
[![compiler-club](https://f005.backblazeb2.com/file/6948446f806c172a098591088e0715b8258b380b/join-compiler-club.svg)](https://kaleidawave-public-pages.val.run/r/cc-discord)

Algorithms from <https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript>

```rust
use pseudorandom::RandomAlgorithm;
use pseudorandom::algorithms::mulberry32::Mulberry32;

let mut mul = Mulberry32::from_seed(0x1234);
assert_eq!(mul.next_value(), 2370005675);
assert_eq!(mul.next_value(), 1722777958);
assert_eq!(mul.next_value(), 1378939303);
assert_eq!(mul.next_value(), 420640238);
assert_eq!(mul.next_value(), 261832786);
```
