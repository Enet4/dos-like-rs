#![no_main]

use dos_like::{
    curs_off, goto_xy, key_state, put_str, read_keys, shutting_down, wait_vbl, KeyCode,
};

#[no_mangle]
pub extern "C" fn dosmain() -> i32 {
    goto_xy(2, 2);
    put_str("KEYBOARD DEMO");
    goto_xy(0, 16);
    put_str("ESC - quit");
    curs_off();
    while !shutting_down() {
        wait_vbl();
        let keys = read_keys();

        if !keys.is_empty() {
            goto_xy(5, 5);
            dos_like::text_color(3);
            put_str("!");
            dos_like::text_color(7);

            for (i, key) in keys.iter().enumerate() {
                goto_xy(2, 8 + i as u16);
                put_str("                  ");
                goto_xy(2, 8 + i as u16);
                if key.is_pressed() {
                    put_str(format!("{:?} pressed             ", key.key_code()));
                } else {
                    put_str(format!("{:?} released            ", key.key_code()));
                }
            }
            for i in keys.len()..8 {
                goto_xy(2, 8 + i as u16);
                put_str("                              ");
            }
        } else {
            goto_xy(5, 5);
            put_str(" ");
        }

        if key_state(KeyCode::KEY_ESCAPE) {
            break;
        }
    }
    0
}
