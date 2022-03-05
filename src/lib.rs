//! Rust high level bindings to [`dos-like`][1],
//! the library/framework for writing applications that look
//! like MS-DOS programs from the 1990's.
//! 
//! [1]: https://github.com/mattiasgustavsson/dos-like
//! 
//! The Rust API was designed to expose the original C API
//! while maintaining Rust's safety guarantees.
//! Howeve, it is currently incomplete.
//! When such an abstraction is not possible or not yet available,
//! the low level unsafe bindings are available in [`dos_like_sys`].

pub use dos_like_sys;

/// Calls `waitvbl`, which waits for a vertical blanking signal.
/// 
/// This should usually be called once per frame.
pub fn wait_vbl() {
    unsafe {
        dos_like_sys::waitvbl();
    }
}

/// Checks whether the application should shut down.
pub fn shutting_down() -> bool {
    unsafe {
        dos_like_sys::shuttingdown() != 0
    }
}

/// Declare and define the main application function.
/// 
/// This macro can be used as an alternative to declaring `dosmain` manually.
/// 
/// # Example
/// 
/// This:
/// 
/// ```no_run
/// #![no_main]
/// 
/// dos_like::dos_main! {
///    println!("Hello")
/// }
/// ```
/// 
/// Expands to this:
/// 
/// ```no_run
/// #![no_main]
/// # use std::os::raw::{c_char, c_int};
/// 
/// #[no_mangle]
/// pub extern "C" fn dosmain(_argc: c_int, _argv: *const *const c_char) -> c_int {
///     println!("Hello");
///     0
/// }
/// ```
#[macro_export]
macro_rules! dos_main {
    ($($t:tt)*) => {
        #[no_mangle]
        pub extern "C" fn dosmain(_argc: std::os::raw::c_int, _argv: *const *const std::os::raw::c_char) -> std::os::raw::c_int {
            $($t)*;
            0
        }
    };
}

#[cfg(test)]
mod tests {
}
