[![Rust](https://github.com/MRandl/BlackJack/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/MRandl/BlackJack/actions/workflows/rust.yml)
# BlackJack 
This project is an implementation of the BlackJack card game in Rust. It supports same-host multiplayer as well as automatic bot players, and is entirely based on a terminal command line interface.

It has minimal dependencies (apart from the Rust standard library, it only uses the *rand* crate for card shuffling).

## Game variant

This blackjack games implements the basic blackjack rules, along with the following variants :

- The dealer stands on soft 17.
- There is no hole card ("European variant").
- There are four card packs being played at all times.
- The player may split only once.
- The player may double on a split hand.
- The player may not surrender.
- There are no side bets.

## Supported Architectures

Your architecture must be supported by the Rust compiler, and must have access to the standard `std` crate.

## How to build

This project requires cargo. To build an executable, clone this repository and run `cargo build --release`. To build and run the executable, use `cargo run --release`. Note that if the `rand` library is not in your cargo cache, cargo will need an Internet connection to download it.

## License

This project is licensed under the MIT license. See the [license](LICENSE.md) for more details.

#

Software written by Mathis Randl ([@MRandl](https://www.github.com/MRandl))
