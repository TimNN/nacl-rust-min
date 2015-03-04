The goal of this repository is to allow you to verify that [DiamondLovesYou/rust](https://github.com/DiamondLovesYou/rust) (Rust PNaCl fork) works correctly and familarize yourself with it's use without relying on any external dependecies.

The code implements a bare minimum native client module that succesfully loads but does nothing else except printing some messages to chrome's stdout.

To compile run `./build.sh` or `rustc --target le32-unknown-nacl nacl-rust-min.rs`.
