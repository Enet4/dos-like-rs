//! Rust port of rotozoom code
//!
//! <https://seancode.com/demofx/>
//!
//! See end of file for license
#![no_main]

use dos_like::{
    dos_main, key_state, load_gif, screen_buffer, set_double_buffer, set_pal, set_video_mode,
    shutting_down, swap_buffers, wait_vbl, KeyCode, VideoMode,
};
use std::f32::consts::PI;

dos_main! {
    set_video_mode(VideoMode::Graphics320x200);
    set_double_buffer(true);
    let gif = load_gif("assets/rotozoom.gif").unwrap_or_else(|_| {
        eprintln!("Could not load rotozoom.gif");
        std::process::exit(-2);
    });


    let palette = gif.raw_palette();
    let palcount = gif.palette_count();
    let gif_width = gif.width() as i32;
    let gif_height = gif.height() as i32;
    let gif_data = gif.data();

    for i in 0..palcount as usize {
        set_pal(
            i,
            palette[3 * i + 0],
            palette[3 * i + 1],
            palette[3 * i + 2],
        );
    }

    unsafe {
        let mut buffer = screen_buffer();
        let mut angle = 0.;
        while !shutting_down() {
            wait_vbl();
            let s = (angle * PI / 180.).sin();
            let c = (angle * PI / 180.).cos();
            angle = ((angle + 1.) as i32 % 360) as f32;
            let mut dest_ofs = 0;
            for y in 0..200 {
                for x in 0..320 {
                    let x = x as f32;
                    let y = y as f32;
                    let mut u = ((x * c - y * s) * (s + 1.) + 64.) as i32 % gif_width;
                    let mut v = ((x * s + y * c) * (s + 1.) + 64.) as i32 % gif_height;
                    if u < 0 {
                        u += gif_width;
                    }
                    if v < 0 {
                        v += gif_height;
                    }
                    let src_ofs = u + v * gif_width;
                    buffer[dest_ofs] = gif_data[src_ofs as usize];
                    dest_ofs += 1;
                }
            }
            buffer = swap_buffers();

            if key_state(KeyCode::KEY_ESCAPE) {
                break;
            }
        }
    }
}

/*
Copyright (c) 2021, seancode
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
THE POSSIBILITY OF SUCH DAMAGE.
*/
