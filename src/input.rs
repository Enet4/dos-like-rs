//! Module for keyboard and mouse input functions.

use dos_like_sys::keycode_t;
use smallvec::SmallVec;

/// A key code object.
///
/// See the various associated constants for specific keys.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct KeyCode(keycode_t);

impl From<KeyCode> for u32 {
    fn from(key: KeyCode) -> u32 {
        key.0 as u32
    }
}

/// Checks whether a key is currently pushed (down).
pub fn key_state(key: KeyCode) -> bool {
    unsafe { dos_like_sys::keystate(key.0) != 0 }
}

/// Reads the key press events available
/// and saves them in an array.
///
/// This creates an independent copy of the keys,
/// consuming the underlying buffer in the process.
pub fn read_keys() -> SmallVec<[KeyCode; 2]> {
    let mut keys = SmallVec::new();

    // Safety: readkeys is a valid pointer
    // to a null terminated sequence of keycode_t
    unsafe {
        let p = dos_like_sys::readkeys();
        for i in 0..=255 {
            let c = *p.offset(i);
            if c == 0 {
                break;
            }
            keys.push(KeyCode(c));
        }
    }

    keys
}

/// Reads the character input events available
/// and saves them in an array.
///
/// This creates an independent copy of the characters,
/// consuming the underlying buffer in the process.
pub fn read_chars() -> SmallVec<[u8; 4]> {
    let mut keys = SmallVec::new();

    // Safety: readchars is a valid pointer
    // to a null terminated sequence of bytes
    unsafe {
        let p = dos_like_sys::readchars();
        for i in 0..=255 {
            let c = *p.offset(i);
            if c == 0 {
                break;
            }
            keys.push(c as u8);
        }
    }

    keys
}

/// Gets the absolute mouse position on the X axis.
pub fn mouse_x() -> u16 {
    unsafe { dos_like_sys::mousex() as u16 }
}

/// Gets the absolute mouse position on the Y axis.
pub fn mouse_y() -> u16 {
    unsafe { dos_like_sys::mousey() as u16 }
}

/// Gets the mouse relative position
/// since the last internal application loop
/// on the X axis.
pub fn mouse_rel_x() -> u16 {
    unsafe { dos_like_sys::mouserelx() as u16 }
}

/// Gets the mouse relative position
/// since the last internal application loop
/// on the Y axis.
pub fn mouse_rel_y() -> u16 {
    unsafe { dos_like_sys::mouserely() as u16 }
}

impl KeyCode {
    pub const KEY_INVALID: Self = KeyCode(dos_like_sys::keycode_t_KEY_INVALID);
    pub const KEY_LBUTTON: Self = KeyCode(dos_like_sys::keycode_t_KEY_LBUTTON);
    pub const KEY_RBUTTON: Self = KeyCode(dos_like_sys::keycode_t_KEY_RBUTTON);
    pub const KEY_CANCEL: Self = KeyCode(dos_like_sys::keycode_t_KEY_CANCEL);
    pub const KEY_MBUTTON: Self = KeyCode(dos_like_sys::keycode_t_KEY_MBUTTON);
    pub const KEY_XBUTTON1: Self = KeyCode(dos_like_sys::keycode_t_KEY_XBUTTON1);
    pub const KEY_XBUTTON2: Self = KeyCode(dos_like_sys::keycode_t_KEY_XBUTTON2);
    pub const KEY_BACK: Self = KeyCode(dos_like_sys::keycode_t_KEY_BACK);
    pub const KEY_TAB: Self = KeyCode(dos_like_sys::keycode_t_KEY_TAB);
    pub const KEY_CLEAR: Self = KeyCode(dos_like_sys::keycode_t_KEY_CLEAR);
    pub const KEY_RETURN: Self = KeyCode(dos_like_sys::keycode_t_KEY_RETURN);
    pub const KEY_SHIFT: Self = KeyCode(dos_like_sys::keycode_t_KEY_SHIFT);
    pub const KEY_CONTROL: Self = KeyCode(dos_like_sys::keycode_t_KEY_CONTROL);
    pub const KEY_MENU: Self = KeyCode(dos_like_sys::keycode_t_KEY_MENU);
    pub const KEY_PAUSE: Self = KeyCode(dos_like_sys::keycode_t_KEY_PAUSE);
    pub const KEY_CAPITAL: Self = KeyCode(dos_like_sys::keycode_t_KEY_CAPITAL);
    pub const KEY_KANA: Self = KeyCode(dos_like_sys::keycode_t_KEY_KANA);
    pub const KEY_HANGUL: Self = KeyCode(dos_like_sys::keycode_t_KEY_HANGUL);
    pub const KEY_JUNJA: Self = KeyCode(dos_like_sys::keycode_t_KEY_JUNJA);
    pub const KEY_FINAL: Self = KeyCode(dos_like_sys::keycode_t_KEY_FINAL);
    pub const KEY_HANJA: Self = KeyCode(dos_like_sys::keycode_t_KEY_HANJA);
    pub const KEY_KANJI: Self = KeyCode(dos_like_sys::keycode_t_KEY_KANJI);
    pub const KEY_ESCAPE: Self = KeyCode(dos_like_sys::keycode_t_KEY_ESCAPE);
    pub const KEY_CONVERT: Self = KeyCode(dos_like_sys::keycode_t_KEY_CONVERT);
    pub const KEY_NONCONVERT: Self = KeyCode(dos_like_sys::keycode_t_KEY_NONCONVERT);
    pub const KEY_ACCEPT: Self = KeyCode(dos_like_sys::keycode_t_KEY_ACCEPT);
    pub const KEY_MODECHANGE: Self = KeyCode(dos_like_sys::keycode_t_KEY_MODECHANGE);
    pub const KEY_SPACE: Self = KeyCode(dos_like_sys::keycode_t_KEY_SPACE);
    pub const KEY_PRIOR: Self = KeyCode(dos_like_sys::keycode_t_KEY_PRIOR);
    pub const KEY_NEXT: Self = KeyCode(dos_like_sys::keycode_t_KEY_NEXT);
    pub const KEY_END: Self = KeyCode(dos_like_sys::keycode_t_KEY_END);
    pub const KEY_HOME: Self = KeyCode(dos_like_sys::keycode_t_KEY_HOME);
    pub const KEY_LEFT: Self = KeyCode(dos_like_sys::keycode_t_KEY_LEFT);
    pub const KEY_UP: Self = KeyCode(dos_like_sys::keycode_t_KEY_UP);
    pub const KEY_RIGHT: Self = KeyCode(dos_like_sys::keycode_t_KEY_RIGHT);
    pub const KEY_DOWN: Self = KeyCode(dos_like_sys::keycode_t_KEY_DOWN);
    pub const KEY_SELECT: Self = KeyCode(dos_like_sys::keycode_t_KEY_SELECT);
    pub const KEY_PRINT: Self = KeyCode(dos_like_sys::keycode_t_KEY_PRINT);
    pub const KEY_EXEC: Self = KeyCode(dos_like_sys::keycode_t_KEY_EXEC);
    pub const KEY_SNAPSHOT: Self = KeyCode(dos_like_sys::keycode_t_KEY_SNAPSHOT);
    pub const KEY_INSERT: Self = KeyCode(dos_like_sys::keycode_t_KEY_INSERT);
    pub const KEY_DELETE: Self = KeyCode(dos_like_sys::keycode_t_KEY_DELETE);
    pub const KEY_HELP: Self = KeyCode(dos_like_sys::keycode_t_KEY_HELP);
    pub const KEY_0: Self = KeyCode(dos_like_sys::keycode_t_KEY_0);
    pub const KEY_1: Self = KeyCode(dos_like_sys::keycode_t_KEY_1);
    pub const KEY_2: Self = KeyCode(dos_like_sys::keycode_t_KEY_2);
    pub const KEY_3: Self = KeyCode(dos_like_sys::keycode_t_KEY_3);
    pub const KEY_4: Self = KeyCode(dos_like_sys::keycode_t_KEY_4);
    pub const KEY_5: Self = KeyCode(dos_like_sys::keycode_t_KEY_5);
    pub const KEY_6: Self = KeyCode(dos_like_sys::keycode_t_KEY_6);
    pub const KEY_7: Self = KeyCode(dos_like_sys::keycode_t_KEY_7);
    pub const KEY_8: Self = KeyCode(dos_like_sys::keycode_t_KEY_8);
    pub const KEY_9: Self = KeyCode(dos_like_sys::keycode_t_KEY_9);
    pub const KEY_A: Self = KeyCode(dos_like_sys::keycode_t_KEY_A);
    pub const KEY_B: Self = KeyCode(dos_like_sys::keycode_t_KEY_B);
    pub const KEY_C: Self = KeyCode(dos_like_sys::keycode_t_KEY_C);
    pub const KEY_D: Self = KeyCode(dos_like_sys::keycode_t_KEY_D);
    pub const KEY_E: Self = KeyCode(dos_like_sys::keycode_t_KEY_E);
    pub const KEY_F: Self = KeyCode(dos_like_sys::keycode_t_KEY_F);
    pub const KEY_G: Self = KeyCode(dos_like_sys::keycode_t_KEY_G);
    pub const KEY_H: Self = KeyCode(dos_like_sys::keycode_t_KEY_H);
    pub const KEY_I: Self = KeyCode(dos_like_sys::keycode_t_KEY_I);
    pub const KEY_J: Self = KeyCode(dos_like_sys::keycode_t_KEY_J);
    pub const KEY_K: Self = KeyCode(dos_like_sys::keycode_t_KEY_K);
    pub const KEY_L: Self = KeyCode(dos_like_sys::keycode_t_KEY_L);
    pub const KEY_M: Self = KeyCode(dos_like_sys::keycode_t_KEY_M);
    pub const KEY_N: Self = KeyCode(dos_like_sys::keycode_t_KEY_N);
    pub const KEY_O: Self = KeyCode(dos_like_sys::keycode_t_KEY_O);
    pub const KEY_P: Self = KeyCode(dos_like_sys::keycode_t_KEY_P);
    pub const KEY_Q: Self = KeyCode(dos_like_sys::keycode_t_KEY_Q);
    pub const KEY_R: Self = KeyCode(dos_like_sys::keycode_t_KEY_R);
    pub const KEY_S: Self = KeyCode(dos_like_sys::keycode_t_KEY_S);
    pub const KEY_T: Self = KeyCode(dos_like_sys::keycode_t_KEY_T);
    pub const KEY_U: Self = KeyCode(dos_like_sys::keycode_t_KEY_U);
    pub const KEY_V: Self = KeyCode(dos_like_sys::keycode_t_KEY_V);
    pub const KEY_W: Self = KeyCode(dos_like_sys::keycode_t_KEY_W);
    pub const KEY_X: Self = KeyCode(dos_like_sys::keycode_t_KEY_X);
    pub const KEY_Y: Self = KeyCode(dos_like_sys::keycode_t_KEY_Y);
    pub const KEY_Z: Self = KeyCode(dos_like_sys::keycode_t_KEY_Z);
    pub const KEY_LWIN: Self = KeyCode(dos_like_sys::keycode_t_KEY_LWIN);
    pub const KEY_RWIN: Self = KeyCode(dos_like_sys::keycode_t_KEY_RWIN);
    pub const KEY_APPS: Self = KeyCode(dos_like_sys::keycode_t_KEY_APPS);
    pub const KEY_SLEEP: Self = KeyCode(dos_like_sys::keycode_t_KEY_SLEEP);
    pub const KEY_NUMPAD0: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD0);
    pub const KEY_NUMPAD1: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD1);
    pub const KEY_NUMPAD2: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD2);
    pub const KEY_NUMPAD3: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD3);
    pub const KEY_NUMPAD4: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD4);
    pub const KEY_NUMPAD5: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD5);
    pub const KEY_NUMPAD6: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD6);
    pub const KEY_NUMPAD7: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD7);
    pub const KEY_NUMPAD8: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD8);
    pub const KEY_NUMPAD9: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMPAD9);
    pub const KEY_MULTIPLY: Self = KeyCode(dos_like_sys::keycode_t_KEY_MULTIPLY);
    pub const KEY_ADD: Self = KeyCode(dos_like_sys::keycode_t_KEY_ADD);
    pub const KEY_SEPARATOR: Self = KeyCode(dos_like_sys::keycode_t_KEY_SEPARATOR);
    pub const KEY_SUBTRACT: Self = KeyCode(dos_like_sys::keycode_t_KEY_SUBTRACT);
    pub const KEY_DECIMAL: Self = KeyCode(dos_like_sys::keycode_t_KEY_DECIMAL);
    pub const KEY_DIVIDE: Self = KeyCode(dos_like_sys::keycode_t_KEY_DIVIDE);
    pub const KEY_F1: Self = KeyCode(dos_like_sys::keycode_t_KEY_F1);
    pub const KEY_F2: Self = KeyCode(dos_like_sys::keycode_t_KEY_F2);
    pub const KEY_F3: Self = KeyCode(dos_like_sys::keycode_t_KEY_F3);
    pub const KEY_F4: Self = KeyCode(dos_like_sys::keycode_t_KEY_F4);
    pub const KEY_F5: Self = KeyCode(dos_like_sys::keycode_t_KEY_F5);
    pub const KEY_F6: Self = KeyCode(dos_like_sys::keycode_t_KEY_F6);
    pub const KEY_F7: Self = KeyCode(dos_like_sys::keycode_t_KEY_F7);
    pub const KEY_F8: Self = KeyCode(dos_like_sys::keycode_t_KEY_F8);
    pub const KEY_F9: Self = KeyCode(dos_like_sys::keycode_t_KEY_F9);
    pub const KEY_F10: Self = KeyCode(dos_like_sys::keycode_t_KEY_F10);
    pub const KEY_F11: Self = KeyCode(dos_like_sys::keycode_t_KEY_F11);
    pub const KEY_F12: Self = KeyCode(dos_like_sys::keycode_t_KEY_F12);
    pub const KEY_F13: Self = KeyCode(dos_like_sys::keycode_t_KEY_F13);
    pub const KEY_F14: Self = KeyCode(dos_like_sys::keycode_t_KEY_F14);
    pub const KEY_F15: Self = KeyCode(dos_like_sys::keycode_t_KEY_F15);
    pub const KEY_F16: Self = KeyCode(dos_like_sys::keycode_t_KEY_F16);
    pub const KEY_F17: Self = KeyCode(dos_like_sys::keycode_t_KEY_F17);
    pub const KEY_F18: Self = KeyCode(dos_like_sys::keycode_t_KEY_F18);
    pub const KEY_F19: Self = KeyCode(dos_like_sys::keycode_t_KEY_F19);
    pub const KEY_F20: Self = KeyCode(dos_like_sys::keycode_t_KEY_F20);
    pub const KEY_F21: Self = KeyCode(dos_like_sys::keycode_t_KEY_F21);
    pub const KEY_F22: Self = KeyCode(dos_like_sys::keycode_t_KEY_F22);
    pub const KEY_F23: Self = KeyCode(dos_like_sys::keycode_t_KEY_F23);
    pub const KEY_F24: Self = KeyCode(dos_like_sys::keycode_t_KEY_F24);
    pub const KEY_NUMLOCK: Self = KeyCode(dos_like_sys::keycode_t_KEY_NUMLOCK);
    pub const KEY_SCROLL: Self = KeyCode(dos_like_sys::keycode_t_KEY_SCROLL);
    pub const KEY_LSHIFT: Self = KeyCode(dos_like_sys::keycode_t_KEY_LSHIFT);
    pub const KEY_RSHIFT: Self = KeyCode(dos_like_sys::keycode_t_KEY_RSHIFT);
    pub const KEY_LCONTROL: Self = KeyCode(dos_like_sys::keycode_t_KEY_LCONTROL);
    pub const KEY_RCONTROL: Self = KeyCode(dos_like_sys::keycode_t_KEY_RCONTROL);
    pub const KEY_LMENU: Self = KeyCode(dos_like_sys::keycode_t_KEY_LMENU);
    pub const KEY_RMENU: Self = KeyCode(dos_like_sys::keycode_t_KEY_RMENU);
    pub const KEY_BROWSER_BACK: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_BACK);
    pub const KEY_BROWSER_FORWARD: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_FORWARD);
    pub const KEY_BROWSER_REFRESH: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_REFRESH);
    pub const KEY_BROWSER_STOP: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_STOP);
    pub const KEY_BROWSER_SEARCH: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_SEARCH);
    pub const KEY_BROWSER_FAVORITES: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_FAVORITES);
    pub const KEY_BROWSER_HOME: Self = KeyCode(dos_like_sys::keycode_t_KEY_BROWSER_HOME);
    pub const KEY_VOLUME_MUTE: Self = KeyCode(dos_like_sys::keycode_t_KEY_VOLUME_MUTE);
    pub const KEY_VOLUME_DOWN: Self = KeyCode(dos_like_sys::keycode_t_KEY_VOLUME_DOWN);
    pub const KEY_VOLUME_UP: Self = KeyCode(dos_like_sys::keycode_t_KEY_VOLUME_UP);
    pub const KEY_MEDIA_NEXT_TRACK: Self = KeyCode(dos_like_sys::keycode_t_KEY_MEDIA_NEXT_TRACK);
    pub const KEY_MEDIA_PREV_TRACK: Self = KeyCode(dos_like_sys::keycode_t_KEY_MEDIA_PREV_TRACK);
    pub const KEY_MEDIA_STOP: Self = KeyCode(dos_like_sys::keycode_t_KEY_MEDIA_STOP);
    pub const KEY_MEDIA_PLAY_PAUSE: Self = KeyCode(dos_like_sys::keycode_t_KEY_MEDIA_PLAY_PAUSE);
    pub const KEY_LAUNCH_MAIL: Self = KeyCode(dos_like_sys::keycode_t_KEY_LAUNCH_MAIL);
    pub const KEY_LAUNCH_MEDIA_SELECT: Self =
        KeyCode(dos_like_sys::keycode_t_KEY_LAUNCH_MEDIA_SELECT);
    pub const KEY_LAUNCH_APP1: Self = KeyCode(dos_like_sys::keycode_t_KEY_LAUNCH_APP1);
    pub const KEY_LAUNCH_APP2: Self = KeyCode(dos_like_sys::keycode_t_KEY_LAUNCH_APP2);
    pub const KEY_OEM_1: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_1);
    pub const KEY_OEM_PLUS: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_PLUS);
    pub const KEY_OEM_COMMA: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_COMMA);
    pub const KEY_OEM_MINUS: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_MINUS);
    pub const KEY_OEM_PERIOD: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_PERIOD);
    pub const KEY_OEM_2: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_2);
    pub const KEY_OEM_3: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_3);
    pub const KEY_OEM_4: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_4);
    pub const KEY_OEM_5: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_5);
    pub const KEY_OEM_6: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_6);
    pub const KEY_OEM_7: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_7);
    pub const KEY_OEM_8: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_8);
    pub const KEY_OEM_102: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_102);
    pub const KEY_PROCESSKEY: Self = KeyCode(dos_like_sys::keycode_t_KEY_PROCESSKEY);
    pub const KEY_ATTN: Self = KeyCode(dos_like_sys::keycode_t_KEY_ATTN);
    pub const KEY_CRSEL: Self = KeyCode(dos_like_sys::keycode_t_KEY_CRSEL);
    pub const KEY_EXSEL: Self = KeyCode(dos_like_sys::keycode_t_KEY_EXSEL);
    pub const KEY_EREOF: Self = KeyCode(dos_like_sys::keycode_t_KEY_EREOF);
    pub const KEY_PLAY: Self = KeyCode(dos_like_sys::keycode_t_KEY_PLAY);
    pub const KEY_ZOOM: Self = KeyCode(dos_like_sys::keycode_t_KEY_ZOOM);
    pub const KEY_NONAME: Self = KeyCode(dos_like_sys::keycode_t_KEY_NONAME);
    pub const KEY_PA1: Self = KeyCode(dos_like_sys::keycode_t_KEY_PA1);
    pub const KEY_OEM_CLEAR: Self = KeyCode(dos_like_sys::keycode_t_KEY_OEM_CLEAR);
    pub const KEYCOUNT: Self = KeyCode(dos_like_sys::keycode_t_KEYCOUNT);
}
