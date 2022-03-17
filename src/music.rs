//! Module for music related functions and constructs.
//!
//! See also [`sound`](super::sound) for the sound module.

use std::{ffi::CString, num::NonZeroU32, os::raw::c_int, ptr::NonNull};

use crate::FileError;

/// A music object.
///
/// This is a wrapper around the [`dos_like_sys::music_t`] struct.
#[derive(Debug)]
pub struct Music(NonNull<dos_like_sys::music_t>);

unsafe impl Send for Music {}

impl Music {
    /// Loads a music from a MIDI file.
    pub fn load_mid(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmid(filename.as_ptr() as *const _);
            if let Some(music) = NonNull::new(music) {
                Ok(Music(music))
            } else {
                Err(FileError::FileNotFound)
            }
        }
    }

    /// Loads a music from a MUS file.
    pub fn load_mus(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmus(filename.as_ptr() as *const _);
            if let Some(music) = NonNull::new(music) {
                Ok(Music(music))
            } else {
                Err(FileError::FileNotFound)
            }
        }
    }

    /// Loads a music from a MOD file.
    pub fn load_mod(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadmod(filename.as_ptr() as *const _);
            if let Some(music) = NonNull::new(music) {
                Ok(Music(music))
            } else {
                Err(FileError::FileNotFound)
            }
        }
    }

    /// Loads a music from a OPB file.
    pub fn load_opb(path: impl AsRef<str>) -> Result<Music, FileError> {
        let filename = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;

        unsafe {
            let music = dos_like_sys::loadopb(filename.as_ptr() as *const _);
            if let Some(music) = NonNull::new(music) {
                Ok(Music(music))
            } else {
                Err(FileError::FileNotFound)
            }
        }
    }

    /// Creates a music object from the byte data of a MUS file.
    ///
    /// # Panic
    ///
    /// This function panics if the data is not a valid MUS file.
    /// See [`Music::try_create_mus`] to handle this gracefully.
    #[inline]
    pub fn create_mus(data: &[u8]) -> Music {
        Music::try_create_mus(data).expect("Invalid MUS data")
    }

    /// Creates a music object from the byte data of a MUS file,
    /// returning `None` if the contents could not be read as such.
    pub fn try_create_mus(data: &[u8]) -> Option<Music> {
        // safety: although pointer type is *mut void_t,
        // no data is never written via the pointer.
        unsafe {
            let music = dos_like_sys::createmus(data.as_ptr() as *mut _, data.len() as c_int);
            if let Some(music) = NonNull::new(music) {
                Some(Music(music))
            } else {
                None
            }
        }
    }

    /// Plays this music,
    /// stopping any other music currently playing.
    ///
    /// If `loop_` is true, the music will loop forever.
    /// `volume` is a number between 0 (silent) and 255 (full volume).
    pub fn play(&self, loop_: bool, volume: u8) {
        play_music(self, loop_, volume)
    }
}

/// Plays this music,
/// stopping any other music currently playing.
///
/// If `loop_` is true, the music will loop forever.
/// `volume` is a number between 0 (silent) and 255 (full volume).
pub fn play_music(music: &Music, loop_: bool, volume: u8) {
    unsafe {
        dos_like_sys::playmusic(music.0.as_ptr(), loop_ as c_int, volume as c_int);
    }
}

/// Stops any music that is currently playing.
pub fn stop_music() {
    unsafe { dos_like_sys::stopmusic() }
}

/// Checks whether the application is currently playing any music.
pub fn is_music_playing() -> bool {
    unsafe { dos_like_sys::musicplaying() != 0 }
}

/// Sets the music volume.
pub fn set_music_volume(volume: u8) {
    unsafe { dos_like_sys::musicvolume(volume as i32) }
}

/// A soundbank identifier.
///
/// Use [`install_user_soundbank`] to obtain a font,
/// or take one of the associated constants for the default soundbanks.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Soundbank(NonZeroU32);

impl Soundbank {
    /// Default soundbank: Sound Blaster AWE32
    pub const DEFAULT_AWE32: Self = Soundbank(
        // safety: definitely not 0
        unsafe { NonZeroU32::new_unchecked(dos_like_sys::DEFAULT_SOUNDBANK_AWE32) },
    );

    /// Default soundbank: Sound Blaster 16
    pub const DEFAULT_SB16: Self = Soundbank(
        // safety: definitely not 0
        unsafe { NonZeroU32::new_unchecked(dos_like_sys::DEFAULT_SOUNDBANK_SB16) },
    );

    /// Obtains a soundbank identifier by its internal index.
    ///
    /// This operation does not check whether the font really exists.
    #[inline]
    pub fn from_id(id: u32) -> Option<Self> {
        Self::from_raw_id(id as c_int)
    }

    #[inline]
    fn from_raw_id(id: c_int) -> Option<Self> {
        NonZeroU32::new(id as u32).map(Soundbank)
    }

    #[inline]
    fn to_id(self) -> c_int {
        self.0.get() as c_int
    }

    /// Installs a soundbank from an .sf2 or .op2 file.
    ///
    /// Equivalent to the module's [`install_user_soundbank`] function.
    #[inline]
    pub fn install_user_soundbank(filename: impl AsRef<str>) -> Result<Self, FileError> {
        install_user_soundbank(filename)
    }

    /// Sets this soundbank for subsequent audio operations.
    ///
    /// Equivalent to the module's [`set_soundbank`] function.
    #[inline]
    pub fn set_soundbank(&self) {
        set_soundbank(self);
    }
}

/// Installs a soundbank from an .sf2 or .op2 file.
///
/// Returns the identifier of the soundbank.
pub fn install_user_soundbank(filename: impl AsRef<str>) -> Result<Soundbank, FileError> {
    let filename = CString::new(filename.as_ref()).map_err(|_| FileError::BadFilePath)?;

    unsafe {
        let soundbank_id = dos_like_sys::installusersoundbank(filename.as_ptr() as *const _);

        Soundbank::from_id(soundbank_id as u32).ok_or(FileError::FileNotFound)
    }
}

/// Sets this soundbank for subsequent audio operations.
#[inline]
pub fn set_soundbank(soundbank: &Soundbank) {
    unsafe { dos_like_sys::setsoundbank(soundbank.to_id()) }
}

// -- music channel manipulation functions

/// The total number of music channels supported by the engine.
pub const MUSIC_CHANNELS: u32 = dos_like_sys::MUSIC_CHANNELS;

/// Pushes a note on the given music channel.
///
/// `note` is a number between 0 and 127 representing the note's pitch.
/// `velocity` is a number between 0 and 127.
pub fn note_on(channel: u8, note: u8, velocity: u8) {
    unsafe { dos_like_sys::noteon(channel as c_int, note as c_int, velocity as c_int) }
}

/// Releases a note on the given music channel.
///
/// `note` is a number between 0 and 127 representing the note's pitch.
pub fn note_off(channel: u8, note: u8) {
    unsafe { dos_like_sys::noteoff(channel as c_int, note as c_int) }
}

/// Releases all notes on the given music channel.
pub fn all_notes_off(channel: u8) {
    unsafe { dos_like_sys::allnotesoff(channel as c_int) }
}

/// Sets the current instrument on the given music channel.
pub fn set_instrument(channel: u8, instrument: u8) {
    unsafe { dos_like_sys::setinstrument(channel as c_int, instrument as c_int) }
}
