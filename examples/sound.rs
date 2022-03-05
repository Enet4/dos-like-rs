//! Sound and music example. The code is public domain, the
//! wav file is from Warcraft 2 audio setup, the midi file
//! is from Simon the Sorcerer, the mus file is from Doom.
//! Direct Rust port.

#![no_main]

use dos_like::dos_like_sys::*;

#[no_mangle]
pub extern "C" fn dosmain() -> i32 {
    unsafe {
        setvideomode(videomode_t_videomode_80x25_8x16);

        let mus = loadmus("dos-like-sys/dos-like/files/sound/doom.mus\0".as_ptr() as *const _);
        let mid = loadmid("dos-like-sys/dos-like/files/sound/simon.mid\0".as_ptr() as *const _);
        let r#mod = loadmod("dos-like-sys/dos-like/files/sound/cfodder.mod\0".as_ptr() as *const _);
        let opb = loadopb("dos-like-sys/dos-like/files/sound/doom.opb\0".as_ptr() as *const _);
        let wav = loadwav("dos-like-sys/dos-like/files/sound/soundcard.wav\0".as_ptr() as *const _);
        let doom_soundbank = installusersoundbank(
            "dos-like-sys/dos-like/files/sound/doom.op2\0".as_ptr() as *const _,
        );

        let mut use_awe32 = true;

        cputs("SOUND DEMO\0".as_ptr() as *const _);
        gotoxy(0, 2);
        cputs("1 - Play MIDI song\0".as_ptr() as *const _);
        gotoxy(0, 3);
        cputs("2 - Play MUS song\0".as_ptr() as *const _);
        gotoxy(0, 4);
        cputs("3 - Play MOD song\0".as_ptr() as *const _);
        gotoxy(0, 5);
        cputs("4 - Play WAV sound on channel 1\0".as_ptr() as *const _);
        gotoxy(0, 6);
        cputs("5 - Play WAV sound on channel 2\0".as_ptr() as *const _);
        gotoxy(0, 7);
        cputs("6 - Play WAV sound on channel 3\0".as_ptr() as *const _);
        gotoxy(0, 8);
        cputs("7 - Stop sound and music\0".as_ptr() as *const _);
        gotoxy(0, 9);
        cputs("8 - Sound mode 11khz 8bit mono (default)\0".as_ptr() as *const _);
        gotoxy(0, 10);
        cputs("9 - Sound mode 44khz 16bit stereo\0".as_ptr() as *const _);
        gotoxy(0, 11);
        cputs("0 - Sound mode 5khz 8bit mono\0".as_ptr() as *const _);
        gotoxy(0, 12);
        cputs("A - Use AWE32 for MIDI/MUS (default)\0".as_ptr() as *const _);
        gotoxy(0, 13);
        cputs("S - Use SoundBlaster16 for MIDI/MUS\0".as_ptr() as *const _);
        gotoxy(0, 14);
        cputs("O - Play OPB song\0".as_ptr() as *const _);
        gotoxy(0, 16);
        cputs("ESC - quit\0".as_ptr() as *const _);
        cursoff();
        while shuttingdown() == 0 {
            let key = *readchars() as u8;
            match key {
                b'1' => {
                    if use_awe32 {
                        setsoundbank(DEFAULT_SOUNDBANK_AWE32 as i32);
                    } else {
                        setsoundbank(DEFAULT_SOUNDBANK_SB16 as i32);
                    }
                    playmusic(mid, 0, 255);
                }
                b'2' => {
                    if use_awe32 {
                        setsoundbank(DEFAULT_SOUNDBANK_AWE32 as i32);
                    } else {
                        setsoundbank(doom_soundbank);
                    }
                    playmusic(mus, 0, 255);
                }
                b'3' => {
                    playmusic(r#mod, 0, 255);
                }
                b'O' | b'o' => {
                    playmusic(opb, 0, 255);
                }
                b'4' => {
                    playsound(0, wav, 0, 128);
                }
                b'5' => {
                    playsound(1, wav, 0, 128);
                }
                b'6' => {
                    playsound(2, wav, 0, 128);
                }
                b'7' => {
                    stopmusic();
                    stopsound(0);
                    stopsound(1);
                    stopsound(2);
                }
                b'8' => {
                    setsoundmode(soundmode_t_soundmode_8bit_mono_11025);
                }
                b'9' => {
                    setsoundmode(soundmode_t_soundmode_16bit_mono_44100);
                }
                b'0' => {
                    setsoundmode(soundmode_t_soundmode_8bit_mono_5000);
                }
                b'A' | b'a' => {
                    if !use_awe32 {
                        use_awe32 = true;
                        stopmusic();
                    }
                }
                b'S' | b's' => {
                    if use_awe32 {
                        use_awe32 = false;
                        stopmusic();
                    }
                }
                _ => {}
            }
            if keystate(keycode_t_KEY_ESCAPE) != 0 {
                break;
            }
        }
    }
    0
}
