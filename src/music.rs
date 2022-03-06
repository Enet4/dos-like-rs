//! Music module
//! 

use std::{ffi::CString, os::raw::c_int};

use crate::FileError;

/// Stops any music that is currently playing.
pub fn stop_music() {
    unsafe {
        dos_like_sys::stopmusic()
    }
}

/// A music object.
/// 
/// This is a wrapper around the [`dos_like_sys::music_t`] struct.
#[derive(Debug)]
pub struct Music(*mut dos_like_sys::music_t);

unsafe impl Send for Music {}

impl Music {

    /// Loads a music from a MIDI file.
    pub fn load_mid(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref())
            .map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmid(filename.as_ptr() as *const _);
            if music.is_null() {
                return Err(FileError::FileNotFound);
            }
            Ok(Music(music))
        }
    }

    /// Loads a music from a MUS file.
    pub fn load_mus(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref())
            .map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmus(filename.as_ptr() as *const _);
            if music.is_null() {
                return Err(FileError::FileNotFound);
            }
            Ok(Music(music))
        }
    }

    /// Loads a music from a MOD file.
    pub fn load_mod(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref())
            .map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmod(filename.as_ptr() as *const _);
            if music.is_null() {
                return Err(FileError::FileNotFound);
            }
            Ok(Music(music))
        }
    }

    /// Loads a music from a OPB file.
    pub fn load_opb(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref())
            .map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadopb(filename.as_ptr() as *const _);
            if music.is_null() {
                return Err(FileError::FileNotFound);
            }
            Ok(Music(music))
        }
    }

    /// Creates a music object from the byte data of a MUS file.
    pub fn create_mus(data: &[u8]) -> Music {
        // safety: although pointer type is *mut void_t,
        // the data is never written via the pointer.
        unsafe {
            let music = dos_like_sys::createmus(data.as_ptr() as *mut _, data.len() as c_int);
            Music(music)
        }
    }
    
    /// Plays this music,
    /// stopping any other music currently playing.
    /// 
    /// If `loop_` is true, the music will loop forever.
    /// `volume` is a number between 0 (silent) and 255 (full volume).
    pub fn play(&self, loop_: bool, volume: u8) {
        unsafe {
            dos_like_sys::playmusic(self.0, loop_ as c_int, volume as c_int);
        }
    }
}
