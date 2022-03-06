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
//! ```no_run
//! #![no_main]
//!
//! #[no_mangle]
//! pub extern "C" fn dosmain() -> i32 {
//!     // your code here
//! 
//!     0
//! }
//! ```
//! 
//! A utility macro is available as an alternative to declaring the function:
//! 
//! ```no_run
//! #![no_main]
//! 
//! dos_like::dos_main! {
//!     // your code here
//! }
//! ```

pub mod video;
pub mod music;

pub use video::*;
pub use music::*;

use std::ffi::CString;

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

/// An image loaded from a file.
#[derive(Debug)]
pub struct Image {
    /// The color palette of the image.
    palette: [u8; 768],
    /// The real size of the palette
    palette_count: u32,
    /// The width of the image.
    width: u32,
    /// The height of the image.
    height: u32,
    /// Pointer to the indexed pixel data.
    data: *mut u8,
}

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl Image {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Gets the image data as a slice of bytes,
    /// each byte representing a pixel indexed by the image's palette.
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.data, self.width as usize * self.height as usize)
        }
    }

    /// Gets the image data as a mutable slice of bytes,
    /// each byte representing a pixel indexed by the image's palette.
    pub fn data_mut(&self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.data, self.width as usize * self.height as usize)
        }
    }

    /// Gets the number of pixel colors in the palette.
    /// The full size of the palette is `palette_count * 3` bytes.
    pub fn palette_count(&self) -> u32 {
        self.palette_count
    }

    /// Gets the image's color palette as a slice of bytes, in RGB
    /// (8 bits per channel).
    pub fn palette(&self) -> &[u8] {
        &self.palette[..self.palette_count as usize * 3]
    }

    /// Gets the image's color palette as a mutable slice of bytes, in RGB
    /// (8 bits per channel).
    pub fn palette_mut(&mut self) -> &mut [u8] {
        &mut self.palette[..self.palette_count as usize]
    }

    /// Gets the image's color palette as a reference to the underlying array,
    /// in RGB (8 bits per channel).
    /// 
    /// The maximum expected size of any palette is 768 bytes
    /// (256 colors * 3 bytes per color).
    /// Note that the real palette used might be smaller than
    /// the full size of the array,
    /// see [`palette`][self::Image::palette] for an accurate slice.
    pub fn raw_palette(&self) -> &[u8; 768] {
        &self.palette
    }

}

/// General error type for file loading functions which can fail
#[derive(Debug)]
pub enum FileError {
    /// Invalid file path (typically due to the presence of null bytes in the string)
    BadFilePath,
    /// File not found, or failed to read/write
    FileNotFound,
}

/// Loads an image from a GIF file.
pub fn load_gif(path: impl AsRef<str>) -> Result<Image, FileError> {
    let filename = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;
    let mut width = 0;
    let mut height = 0;
    let mut palcount = 0;
    let mut palette = [0; 768];

    unsafe {
        let data = dos_like_sys::loadgif(filename.as_ptr(), &mut width, &mut height, &mut palcount, palette.as_mut_ptr());

        if data.is_null() {
            return Err(FileError::FileNotFound);
        }

        Ok(Image {
            width: width as u32,
            height: height as u32,
            palette_count: palcount as u32,
            palette,
            data,
        })
    }
}

/// Declares and defines the main application function.
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
