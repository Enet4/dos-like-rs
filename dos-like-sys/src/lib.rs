//! Rust high level bindings to [`dos-like`][1],
//! the library/framework for writing applications that look
//! like MS-DOS programs from the 1990's.
//! 
//! [1]: https://github.com/mattiasgustavsson/dos-like
//! 
//! The bindings are directly generated from the original source code.
//!
//! ## Using
//! 
//! **This crate does not function as a regular library,**
//! because it already defines a `main` function by itself.
//! Attempting to create your own executable with its own `main` function
//! will result in a linker error.
//! For the building process to work,
//! the main source file needs the `no_main` attribute
//! and to define an extern C function `dosmain` instead.
//! 
//! ```rust
//! #![no_main]
//!
//! #[no_mangle]
//! pub extern "C" fn dosmain() -> i32 {
//!     // your code here
//! 
//!     0
//! }
//! ```

#![allow(nonstandard_style)]
mod bindings;

pub use bindings::*;
