# Eithr
A small personal Rust library aiming to implement the `Either` monad (inspired by the Rust `Option` and `Result` type and the Haskell `Either` type). 

The `Either` type is defined here as the sum type of order _2_, i.e. the sum of two types. It is implemented as an enum with variants `Left` and `Right`.

I found myself writing this type and variations of its functionalities on multiple occasions as I was learning Rust. 
There is likely a better library out there for this (i.e., the actual [either](https://crates.io/crates/either) crate, hence the use of the name `eithr`), but any improvements are welcome! 

Documentation still not complete.

# Cargo
```toml
[dependencies]
eithr = "*"
```

# License
Licensed under (the MIT license)[https://opensource.org/licenses/MIT].