//! # backtesting-framework
//!
//! A simple framework for testing quantitative trading strategies.
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::todo
)]

mod portfolio;

pub use portfolio::*;
