// make `std` available when testing
#![cfg_attr(not(test), no_std)]
#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::multiple_crate_versions)]

pub mod encoder;
