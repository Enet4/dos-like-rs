//! Module for sound related functions and constructs.
//!
//! See also [`music`](super::music) for the music module.

use std::{
    ffi::CString,
    os::raw::{c_int, c_short, c_uint},
};

use dos_like_sys::sound_t;

use crate::FileError;

/// The total number of sound channels supported by the engine.
pub const SOUND_CHANNELS: u32 = dos_like_sys::SOUND_CHANNELS;

/// A sound mode.
///
/// This type maps each variant
/// to the `soundmode_t` enum in the original framework,
/// with idiomatic naming.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum SoundMode {
    /// 8-bit mono, 5000 Hz
    Mono8bit5000 = dos_like_sys::soundmode_t_soundmode_8bit_mono_5000,
    /// 8-bit mono, 8000 Hz
    Mono8bit8000 = dos_like_sys::soundmode_t_soundmode_8bit_mono_8000,
    /// 8-bit mono, 11025 Hz
    Mono8bit11025 = dos_like_sys::soundmode_t_soundmode_8bit_mono_11025,
    /// 8-bit mono, 16000 Hz
    Mono8bit16000 = dos_like_sys::soundmode_t_soundmode_8bit_mono_16000,
    /// 8-bit mono, 22050 Hz
    Mono8bit22050 = dos_like_sys::soundmode_t_soundmode_8bit_mono_22050,
    /// 8-bit mono, 32000 Hz
    Mono8bit32000 = dos_like_sys::soundmode_t_soundmode_8bit_mono_32000,
    /// 8-bit mono, 44100 Hz
    Mono8bit44100 = dos_like_sys::soundmode_t_soundmode_8bit_mono_44100,
    /// 16-bit mono, 5000 Hz
    Mono16Bit5000 = dos_like_sys::soundmode_t_soundmode_16bit_mono_5000,
    /// 16-bit mono, 8000 Hz
    Mono16Bit8000 = dos_like_sys::soundmode_t_soundmode_16bit_mono_8000,
    /// 16-bit mono, 11025 Hz
    Mono16Bit11025 = dos_like_sys::soundmode_t_soundmode_16bit_mono_11025,
    /// 16-bit mono, 16000 Hz
    Mono16Bit16000 = dos_like_sys::soundmode_t_soundmode_16bit_mono_16000,
    /// 16-bit mono, 22050 Hz
    Mono16Bit22050 = dos_like_sys::soundmode_t_soundmode_16bit_mono_22050,
    /// 16-bit mono, 32000 Hz
    Mono16Bit32000 = dos_like_sys::soundmode_t_soundmode_16bit_mono_32000,
    /// 16-bit mono, 44100 Hz
    Mono16Bit44100 = dos_like_sys::soundmode_t_soundmode_16bit_mono_44100,
    /// 8-bit stereo, 5000 Hz
    Stereo8Bit5000 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_5000,
    /// 8-bit stereo, 8000 Hz
    Stereo8Bit8000 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_8000,
    /// 8-bit stereo, 11025 Hz
    Stereo8Bit11025 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_11025,
    /// 8-bit stereo, 16000 Hz
    Stereo8Bit16000 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_16000,
    /// 8-bit stereo, 22050 Hz
    Stereo8Bit22050 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_22050,
    /// 8-bit stereo, 32000 Hz
    Stereo8Bit32000 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_32000,
    /// 8-bit stereo, 44100 Hz
    Stereo8Bit44100 = dos_like_sys::soundmode_t_soundmode_8bit_stereo_44100,
    /// 16-bit stereo, 5000 Hz
    Stereo16Bit5000 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_5000,
    /// 16-bit stereo, 8000 Hz
    Stereo16Bit8000 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_8000,
    /// 16-bit stereo, 11025 Hz
    Stereo16Bit11025 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_11025,
    /// 16-bit stereo, 16000 Hz
    Stereo16Bit16000 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_16000,
    /// 16-bit stereo, 22050 Hz
    Stereo16Bit22050 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_22050,
    /// 16-bit stereo, 32000 Hz
    Stereo16Bit32000 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_32000,
    /// 16-bit stereo, 44100 Hz
    Stereo16Bit44100 = dos_like_sys::soundmode_t_soundmode_16bit_stereo_44100,
}

/// Sets the application sound mode.
pub fn set_sound_mode(sound_mode: SoundMode) {
    unsafe {
        dos_like_sys::setsoundmode(sound_mode as c_uint);
    }
}

/// A sound object.
///
/// This is a wrapper around the [`dos_like_sys::sound_t`] struct.
#[derive(Debug)]
#[repr(transparent)]
pub struct Sound(*mut sound_t);

impl Sound {
    /// Loads a new sound from a file.
    ///
    /// This is equivalent to the module's [`load_wav`] function.
    #[inline]
    pub fn load_wav(path: impl AsRef<str>) -> Result<Sound, FileError> {
        load_wav(path)
    }

    /// Creates a new sound from a buffer.
    /// 
    /// Note that this copies the samples internally,
    /// so there is effectively no lifetime dependency with the buffer.
    #[inline]
    pub fn create_sound(channels: u32, sample_rate: u32, samples: &[u16]) -> Sound {
        create_sound(channels, sample_rate, samples)
    }

    /// Plays this sound.
    #[inline]
    pub fn play(&self, channel: u8, loop_: bool, volume: u8) {
        play_sound(channel, self, loop_, volume);
    }
}

unsafe impl Send for Sound {}

/// Loads a new sound from a file.
pub fn load_wav(path: impl AsRef<str>) -> Result<Sound, FileError> {
    let path = CString::new(path.as_ref()).map_err(|_| FileError::BadFilePath)?;
    let p = unsafe { dos_like_sys::loadwav(path.as_ptr() as *const i8) };
    if p.is_null() {
        Err(FileError::FileNotFound)
    } else {
        Ok(Sound(p))
    }
}

/// Creates a new sound from a buffer.
pub fn create_sound(channels: u32, sample_rate: u32, samples: &[u16]) -> Sound {
    // safety: although we're passing a *mut,
    // nothing is ever written to samples
    unsafe {
        Sound(dos_like_sys::createsound(
            channels as c_int,
            sample_rate as c_int,
            samples.len() as c_int,
            samples.as_ptr() as *mut c_short,
        ))
    }
}

/// Plays the sound specified.
pub fn play_sound(channel: u8, sound: &Sound, loop_: bool, volume: u8) {
    unsafe {
        dos_like_sys::playsound(channel as c_int, sound.0, loop_ as c_int, volume as c_int);
    }
}

/// Stops any sound currently playing in the given channel.
pub fn stop_sound(channel: u8) {
    unsafe {
        dos_like_sys::stopsound(channel as c_int);
    }
}

/// Checks whether any sound is playing in the given channel.
pub fn is_sound_playing(channel: u8) -> bool {
    unsafe { dos_like_sys::soundplaying(channel as c_int) != 0 }
}

/// Sets the stereo volume of a channel.
pub fn set_sound_volume(channel: u8, left: u8, right: u8) {
    unsafe {
        dos_like_sys::soundvolume(channel as c_int, left as c_int, right as c_int);
    }
}
