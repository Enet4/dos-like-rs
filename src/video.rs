//! Video module

use std::{os::raw::{c_int, c_uint}, ffi::CString};

use crate::FileError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VideoType {
    /// Text mode, where each cell is a character or glyph.
    /// and the resolution is defined via cell matrix and font size.
    Text,
    /// Graphics mode, where the resolution is specified directly
    /// and the graphics are drawn to the screen in indexed color mode.
    Graphics,
}

/// A video mode.
///
/// This type maps to the `videomode_t` struct in the original framework.
/// Each variant is either in text mode or graphics mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum VideoMode {
    /// Text mode, 40 columns and 25 rows, 8x8 font size.
    Text40x25_8x8 = dos_like_sys::videomode_t_videomode_40x25_8x8,
    /// Text mode, 40 columns and 25 rows, 9x16 font size.
    Text40x25_9x16 = dos_like_sys::videomode_t_videomode_40x25_9x16,
    /// Text mode, 80 columns and 25 rows, 8x8 font size.
    Text80x25_8x8 = dos_like_sys::videomode_t_videomode_80x25_8x8,
    /// Text mode, 80 columns and 25 rows, 8x16 font size.
    ///
    /// This is the mode set by default.
    Text80x25_8x16 = dos_like_sys::videomode_t_videomode_80x25_8x16,
    /// Text mode, 80 columns and 25 rows, 9x16 font size.
    Text80x25_9x16 = dos_like_sys::videomode_t_videomode_80x25_9x16,
    /// Text mode, 80 columns and 43 rows, 8x8 font size.
    Text80x43_8x8 = dos_like_sys::videomode_t_videomode_80x43_8x8,
    /// Text mode, 80 columns and 50 rows, 8x8 font size.
    Text80x50_8x8 = dos_like_sys::videomode_t_videomode_80x50_8x8,
    /// Graphics mode, 320x200 pixels.
    Graphics320x200 = dos_like_sys::videomode_t_videomode_320x200,
    /// Graphics mode, 320x240 pixels.
    Graphics320x240 = dos_like_sys::videomode_t_videomode_320x240,
    /// Graphics mode, 320x400 pixels.
    Graphics320x400 = dos_like_sys::videomode_t_videomode_320x400,
    /// Graphics mode, 640x200 pixels.
    Graphics640x200 = dos_like_sys::videomode_t_videomode_640x200,
    /// Graphics mode, 640x350 pixels.
    Graphics640x350 = dos_like_sys::videomode_t_videomode_640x350,
    /// Graphics mode, 640x400 pixels.
    Graphics640x400 = dos_like_sys::videomode_t_videomode_640x400,
    /// Graphics mode, 640x480 pixels.
    Graphics640x480 = dos_like_sys::videomode_t_videomode_640x480,
}

impl VideoMode {

    /// Gets whether the given video mode is in graphics mode.
    pub fn is_graphics(self) -> bool {
        match self {
            VideoMode::Graphics320x200
            | VideoMode::Graphics320x240
            | VideoMode::Graphics320x400
            | VideoMode::Graphics640x200
            | VideoMode::Graphics640x350
            | VideoMode::Graphics640x400
            | VideoMode::Graphics640x480 => true,
            VideoMode::Text40x25_8x8
            | VideoMode::Text40x25_9x16
            | VideoMode::Text80x25_8x8
            | VideoMode::Text80x25_8x16
            | VideoMode::Text80x25_9x16
            | VideoMode::Text80x43_8x8
            | VideoMode::Text80x50_8x8 => false,
        }
    }

    /// Gets whether the given video mode is in text mode.
    #[inline]
    pub fn is_text(self) -> bool {
        !self.is_graphics()
    }
}

/// Sets the video mode.
#[inline]
pub fn set_video_mode(mode: VideoMode) {
    unsafe {
        dos_like_sys::setvideomode(mode as c_uint);
    }
}

/// Enables or disables screen double buffering.
#[inline]
pub fn set_double_buffer(enabled: bool) {
    unsafe {
        dos_like_sys::setdoublebuffer(enabled as c_int);
    }
}

/// Obtains the screen width in pixels.
#[inline]
pub fn screen_width() -> u16 {
    unsafe { dos_like_sys::screenwidth() as u16 }
}

/// Obtains the screen height in pixels.
#[inline]
pub fn screen_height() -> u16 {
    unsafe { dos_like_sys::screenheight() as u16 }
}

/// Sets a palette color by index.
#[inline]
pub fn set_pal(index: usize, r: u8, g: u8, b: u8) {
    unsafe {
        dos_like_sys::setpal(index as c_int, r as c_int, g as c_int, b as c_int);
    }
}

/// Gets a palette color by index.
#[inline]
pub fn pal(index: usize) -> (u8, u8, u8) {
    let (mut r, mut g, mut b) = (0, 0, 0);
    unsafe {
        dos_like_sys::getpal(index as c_int, &mut r, &mut g, &mut b);
        (r as u8, g as u8, b as u8)
    }
}

// -- Graphics buffer manipulation functions
// Due to the way the original framework works,
// some operations are hard to be marked as safe by the compiler.

/// Gets a mutable slice of the current screen buffer.
/// 
/// Only makes sense in graphics mode.
/// The length of the slice is equal to the number of pixels on the screen.
/// 
/// # Safety
/// 
/// It is unsafe because access to the screen is not guaranteed to be exclusive.
/// Calling _any function_ that blits _anything_ to the screen
/// during the returned slice's lifetime
/// is _undefined behavior_.
/// It is also undefined behavior to call this function multiple times
/// without dropping the previous slices first.
/// However, if _double buffering_ is enabled
/// (via [`set_double_buffer`]),
/// then it is safe to call [`swap_buffers`]
/// and immediately drop this slice in favor of the new buffer slice.
pub unsafe fn screen_buffer() -> &'static mut [u8] {
    // Safety: it is documented that the user
    // must not draw anything through other functions,
    // so that buffer access is truly exclusive.
    #[allow(unused_unsafe)]
    unsafe {
        let buf = dos_like_sys::screenbuffer();
        let width = dos_like_sys::screenwidth() as usize;
        let height = dos_like_sys::screenheight() as usize;
        std::slice::from_raw_parts_mut(buf, width * height)
    }
}

/// Swaps the current buffer a mutable slice of the current screen buffer.
/// 
/// Only makes sense in graphics mode.
/// 
/// # Safety
/// 
/// It is unsafe because access to the screen is not guaranteed to be exclusive.
/// Calling _any function_ that blits _anything_ to the screen
/// during the returned slice's lifetime
/// is _undefined behavior_.
///
/// Moreover, if _double buffering_ is disabled,
/// any buffer slice obtained through this function or [`screen_buffer`]
/// _must_ be dropped before calling this one.
/// That is,
/// it is only safe to call `swap_buffers` with an existing buffer slice
/// if double buffering is enabled
/// (via [`set_double_buffer`]).
/// and the other slice is immediately dropped.
/// 
/// # Example
/// 
/// ```no_run
/// # use dos_like::*;
/// set_double_buffer(true);
/// // safety: I solemnly swear that I will not draw anything through other functions
/// let mut buffer = unsafe {
///     screen_buffer()
/// };
/// 
/// loop {
///     // do things with buffer
///     buffer[0] = 1;
/// 
///     // safety: previous buffer was dropped
///     buffer = unsafe {
///         swap_buffers()
///     };
/// }
/// ```
pub unsafe fn swap_buffers() -> &'static mut [u8] {
    // Safety: it is documented that the user
    // must drop other buffer slices
    // and must not draw anything through other functions,
    // so that buffer access is truly exclusive and not aliased.
    #[allow(unused_unsafe)]
    unsafe {
        let buf = dos_like_sys::swapbuffers();
        let width = dos_like_sys::screenwidth() as usize;
        let height = dos_like_sys::screenheight() as usize;
        std::slice::from_raw_parts_mut(buf, width * height)
    }
}


// -- Graphics manipulation functions

/// Clears the screen when in graphics mode.
#[inline]
pub fn clear_screen() {
    unsafe {
        dos_like_sys::clearscreen();
    }
}

/// Gets the color of a single pixel on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn pixel(x: u16, y: u16) -> u8 {
    unsafe {
        dos_like_sys::getpixel(x as c_int, y as c_int) as u8
    }
}


/// Puts a color on a single pixel.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn put_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        dos_like_sys::putpixel(x as c_int, y as c_int, color as c_int);
    }
}

/// Draws a horizonal line.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn h_line(x: u16, y: u16, len: u16, color: u8) {
    unsafe {
        dos_like_sys::hline( x as c_int, y as c_int, len as c_int, color as c_int);
    }
}

/// Sets the foreground color to the given palette color index
/// for subsequent drawing operations.
/// 
/// Only works in graphics mode.
#[inline]
pub fn set_color(color: u8) {
    unsafe {
        dos_like_sys::setcolor(color as c_int);
    }
}

/// Gets the current foreground color by palette color index.
/// 
/// Returns 0 if video is not in graphics mode.
#[inline]
pub fn get_color() -> u8 {
    unsafe {
        dos_like_sys::getcolor() as u8
    }
}

/// Draws a line on the screen from one position to another.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn line(x1: u16, y1: u16, x2: u16, y2: u16) {
    unsafe {
        dos_like_sys::line(x1 as c_int,y1 as c_int,x2 as c_int,y2 as c_int);
    }
}

/// Draws a non-filled rectangle on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn rectangle(x1: u16, y1: u16, x2: u16, y2: u16) {
    unsafe {
        dos_like_sys::rectangle(x1 as c_int,y1 as c_int,x2 as c_int,y2 as c_int);
    }
}

/// Draws a filled rectangle on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn bar(x1: u16, y1: u16, x2: u16, y2: u16) {
    unsafe {
        dos_like_sys::bar(x1 as c_int,y1 as c_int,x2 as c_int,y2 as c_int);
    }
}

/// Draws a circle with no filling on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn circle(x: u16, y: u16, r: u16) {
    unsafe {
        dos_like_sys::circle(x as c_int, y as c_int, r as c_int);
    }
}

/// Draws a filled circle on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn fill_circle(x: u16, y: u16, r: u16) {
    unsafe {
        dos_like_sys::fillcircle(x as c_int, y as c_int, r as c_int);
    }
}

/// Draws a non-filled ellipse on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn ellipse(x: u16, y: u16, rx: u16, ry: u16) {
    unsafe {
        dos_like_sys::ellipse(x as c_int, y as c_int, rx as c_int, ry as c_int);
    }
}

/// Draws a filled ellipse on the screen.
/// 
/// Only makes sense in graphics mode.
#[inline]
pub fn fill_ellipse(x: u16, y: u16, rx: u16, ry: u16) {
    unsafe {
        dos_like_sys::fillellipse(x as c_int, y as c_int, rx as c_int, ry as c_int);
    }
}

/// Draws a non-filled polygon on the screen,
/// with the given flat list of XY coordinates in pixels.
/// 
/// Only makes sense in graphics mode.
/// 
/// # Panic
/// 
/// Panics if the given list of points is empty or not even.
#[inline]
pub fn draw_poly(points: &[u16]) {
    assert!(points.len() > 0 && points.len() % 2 == 0);
    
    // Safety: although the pointer type is *mut,
    // it never really writes via the pointer.
    unsafe {
        dos_like_sys::drawpoly(points.as_ptr() as *mut _, points.len() as c_int);
    }
}

/// Draws a filled polygon on the screen,
/// with the given flat list of XY coordinates in pixels.
/// 
/// Only makes sense in graphics mode.
/// 
/// # Panic
/// 
/// Panics if the given list of points is empty or not even.
#[inline]
pub fn fill_poly(points: &[u16]) {
    assert!(points.len() > 0 && points.len() % 2 == 0);
    
    // Safety: although the pointer type is *mut,
    // it never really writes via the pointer.
    unsafe {
        dos_like_sys::fillpoly(points.as_ptr() as *mut _, points.len() as c_int);
    }
}

/// Flood fills the screen from the given position.
/// 
/// Only makes sense in graphics mode.
pub fn flood_fill(x: u16, y: u16) {
    unsafe {
        dos_like_sys::floodfill(x as c_int, y as c_int);
    }
}

/// Flood fills the screen from the given position
/// with the given color as boundary.
/// 
/// Only makes sense in graphics mode.
pub fn boundary_fill(x: u16, y: u16, boundary: u8) {
    unsafe {
        dos_like_sys::boundaryfill(x as c_int, y as c_int, boundary as c_int);
    }
}

/// Blits a text to the screen at the given position.
/// 
/// XY coordinates are in pixels.
pub fn out_text_xy(x: u16, y: u16, text: impl AsRef<[u8]>) {
    let text = CString::new(text.as_ref()).unwrap();

    unsafe {
        dos_like_sys::outtextxy(x as c_int, y as c_int, text.as_ptr() as *const _);
    }
}

/// Blits a text to the screen at the given position,
/// wrapping around before it goes beyond the width specified.
/// 
/// XY coordinates and width are in pixels.
pub fn wrap_text_xy(x: u16, y: u16, text: impl AsRef<[u8]>, width: u16) {
    let text = CString::new(text.as_ref()).unwrap();

    unsafe {
        dos_like_sys::wraptextxy(x as c_int, y as c_int, text.as_ptr() as *const _, width as c_int);
    }
}

/// Blits a text to the screen at the given position,
/// wrapping around before it goes beyond the width specified.
/// 
/// XY coordinates and width are in pixels.
pub fn center_text_xy(x: u16, y: u16, text: impl AsRef<[u8]>, width: u16) {
    let text = CString::new(text.as_ref()).unwrap();

    unsafe {
        dos_like_sys::centertextxy(x as c_int, y as c_int, text.as_ptr() as *const _, width as c_int);
    }
}


// -- Font manipulation functions --

/// A font identifier.
/// 
/// Use [`install_user_font`] to obtain a font,
/// or take [`FontId::BASE`] for the default font.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FontId(c_int);

impl FontId {
    /// The default font.
    pub const BASE: FontId = FontId(0);

    /// Installs a font from a .fnt file
    /// 
    /// This is the same as the module's [`install_user_font`] function.
    pub fn install_user_font(filename: impl AsRef<str>) -> Result<Self, FileError> {
        install_user_font(filename)
    }

    /// Obtain a font identifier by its internal index.
    /// 
    /// This operation does not check whether the font really exists.
    #[inline]
    pub fn from_id(id: c_int) -> FontId {
        FontId(id)
    }
}

/// Installs a font from a .fnt file.
/// 
/// Returns the identifier of the font.
pub fn install_user_font(filename: impl AsRef<str>) -> Result<FontId, FileError> {
    let filename = CString::new(filename.as_ref()).map_err(|_| FileError::BadFilePath)?;

    unsafe {
        let font_id = dos_like_sys::installuserfont(filename.as_ptr() as *const _);

        if font_id == 0 {
            Err(FileError::FileNotFound)
        } else {
            Ok(FontId(font_id))
        }
        
    }
}

/// Sets the font and style of upcoming text blit operations.
/// 
/// This is only available in graphics mode with a font loaded.
/// The operations is ignored if `FontId` does not correspond to a valid font.
#[inline]
pub fn set_text_style(font: FontId, bold: bool, italic: bool, underline: bool) {
    unsafe {
        dos_like_sys::settextstyle(font.0, bold as c_int, italic as c_int, underline as c_int);
    }
}

// --- Pure text mode functions ---

/// Writes an ASCII string to the screen, at the current cursor position.
/// 
/// Does nothing unless the video is in text mode.
/// 
/// # Panics
/// 
/// Panics if the given string cannot be converted to be printed to the screen.
/// Always check for null characters (`\0`) in the string
/// before calling this function.
pub fn cputs(string: impl AsRef<[u8]>) {
    let text = CString::new(string.as_ref()).unwrap();
    unsafe {
        dos_like_sys::cputs(text.as_ptr() as *const _);
    }
}

/// Sets the color of the text.
/// 
/// Only works in text mode.
pub fn text_color(color: u32) {
    unsafe {
        dos_like_sys::textcolor(color as c_int);
    }
}

/// Sets the background color of the text by palette color index.
/// 
/// Only works in text mode.
pub fn text_background(color: u8) {
    unsafe {
        dos_like_sys::textbackground(color as c_int);
    }
}

/// Moves the cursor to the specified position.
/// 
/// Only works in text mode.
pub fn goto_xy(x: u16, y: u16) {
    unsafe {
        dos_like_sys::gotoxy(x as c_int, y as c_int);
    }
}

/// Gets the cursor's current X position.
/// 
/// Returns 0 if the video is not in text mode.
pub fn where_x() -> u16 {
    unsafe { dos_like_sys::wherex().max(0) as u16 }
}

/// Gets the cursor's current Y position.
/// 
/// Returns 0 if the video is not in text mode.
pub fn where_y() -> u16 {
    unsafe { dos_like_sys::wherex().max(0) as u16 }
}

/// Clears the screen when in text mode.
pub fn clr_scr() {
    unsafe {
        dos_like_sys::clrscr();
    }
}

/// Enables the blinking text cursor.
/// 
/// The cursor is visible to the user by default.
/// 
/// Only works in text mode.
pub fn curs_on() {
    unsafe {
        dos_like_sys::curson();
    }
}

/// Hides the text cursor.
/// 
/// Only works in text mode.
pub fn curs_off() {
    unsafe {
        dos_like_sys::cursoff();
    }
}
