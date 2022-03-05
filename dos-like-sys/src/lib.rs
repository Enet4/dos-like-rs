//! Rust high level bindings to [`dos-like`][1],
//! the library/framework for writing applications that look
//! like MS-DOS programs from the 1990's.
//! 
//! [1]: https://github.com/mattiasgustavsson/dos-like
//! 
//! The bindings are directly generated from the original source code.

#![allow(nonstandard_style)]
mod bindings;

pub use bindings::*;
