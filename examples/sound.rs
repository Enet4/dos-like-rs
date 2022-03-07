//! Sound and music example. The code is public domain, the
//! wav file is from Warcraft 2 audio setup, the midi file
//! is from Simon the Sorcerer, the mus file is from Doom.
//! Direct Rust port.

#![no_main]

use dos_like::{
    curs_off, dos_like_sys::*, goto_xy, install_user_soundbank, key_state, load_wav, play_sound,
    put_str, set_sound_mode, set_soundbank, shutting_down, stop_music, stop_sound, KeyCode, Music,
    SoundMode, Soundbank,
};

#[no_mangle]
pub extern "C" fn dosmain() -> i32 {
    unsafe {
        let mus = Music::load_mus("dos-like-sys/dos-like/files/sound/doom.mus")
            .expect("Could not load doom.mus");
        let mid = Music::load_mid("dos-like-sys/dos-like/files/sound/simon.mid")
            .expect("Could not load simon.mid");
        let r#mod = Music::load_mod("dos-like-sys/dos-like/files/sound/cfodder.mod")
            .expect("Could not load cfodder.mod");
        let opb = Music::load_opb("dos-like-sys/dos-like/files/sound/doom.opb")
            .expect("Could not load doom.opb");
        let wav = load_wav("dos-like-sys/dos-like/files/sound/soundcard.wav")
            .expect("Could not load soundcard.wav");
        let doom_soundbank = install_user_soundbank("dos-like-sys/dos-like/files/sound/doom.op2")
            .expect("Could not load doom.op2");

        let mut use_awe32 = true;

        put_str("SOUND DEMO");
        goto_xy(0, 2);
        put_str("1 - Play MIDI song");
        goto_xy(0, 3);
        put_str("2 - Play MUS song");
        goto_xy(0, 4);
        put_str("3 - Play MOD song");
        goto_xy(0, 5);
        put_str("4 - Play WAV sound on channel 1");
        goto_xy(0, 6);
        put_str("5 - Play WAV sound on channel 2");
        goto_xy(0, 7);
        put_str("6 - Play WAV sound on channel 3");
        goto_xy(0, 8);
        put_str("7 - Stop sound and music");
        goto_xy(0, 9);
        put_str("8 - Sound mode 11khz 8bit mono (default)");
        goto_xy(0, 10);
        put_str("9 - Sound mode 44khz 16bit stereo");
        goto_xy(0, 11);
        put_str("0 - Sound mode 5khz 8bit mono");
        goto_xy(0, 12);
        put_str("A - Use AWE32 for MIDI/MUS (default)");
        goto_xy(0, 13);
        put_str("S - Use SoundBlaster16 for MIDI/MUS");
        goto_xy(0, 14);
        put_str("O - Play OPB song");
        goto_xy(0, 16);
        put_str("ESC - quit");
        curs_off();
        while !shutting_down() {
            let key = *readchars() as u8;
            match key {
                b'1' => {
                    if use_awe32 {
                        set_soundbank(&Soundbank::DEFAULT_AWE32);
                    } else {
                        set_soundbank(&Soundbank::DEFAULT_SB16);
                    }
                    mid.play(false, 255);
                }
                b'2' => {
                    if use_awe32 {
                        set_soundbank(&Soundbank::DEFAULT_AWE32);
                    } else {
                        set_soundbank(&doom_soundbank);
                    }
                    mus.play(false, 255);
                }
                b'3' => {
                    r#mod.play(false, 255);
                }
                b'O' | b'o' => {
                    opb.play(false, 255);
                }
                b'4' => {
                    play_sound(0, &wav, false, 128);
                }
                b'5' => {
                    play_sound(1, &wav, false, 128);
                }
                b'6' => {
                    play_sound(2, &wav, false, 128);
                }
                b'7' => {
                    stop_music();
                    stop_sound(0);
                    stop_sound(1);
                    stop_sound(2);
                }
                b'8' => {
                    set_sound_mode(SoundMode::Mono8bit11025);
                }
                b'9' => {
                    set_sound_mode(SoundMode::Mono16Bit44100);
                }
                b'0' => {
                    set_sound_mode(SoundMode::Mono8bit5000);
                }
                b'A' | b'a' => {
                    if !use_awe32 {
                        use_awe32 = true;
                        stop_music();
                    }
                }
                b'S' | b's' => {
                    if use_awe32 {
                        use_awe32 = false;
                        stop_music();
                    }
                }
                _ => {}
            }
            if key_state(KeyCode::KEY_ESCAPE) {
                break;
            }
        }
    }
    0
}
