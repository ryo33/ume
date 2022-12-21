# ume

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/ume-222222)](https://github.com/ryo33/ume)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/ume)](https://crates.io/crates/ume)
[![docs.rs](https://img.shields.io/docsrs/ume)](https://docs.rs/ume)

`ume` means "Embed" or "Japanese apricot" in Japanese.

`ume` is a simple macro to generate Rust source code.

## How to use

```rust
let use_something = ume!(
    use something::prelude::*;
);
let let_one = ume!(let one = 1;);
let let_two = ume!(let two = 2;);
let source_code = ume! {
    #use_something
    fn main() {
        #let_one
        #let_two
        println!("{one} {two}");
    }
}
```
