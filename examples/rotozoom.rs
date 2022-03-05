//! Rust port of rotozoom code
//!
//! <https://seancode.com/demofx/>
//!
//! See end of file for license
#![no_main]

use dos_like::{dos_main, wait_vbl, shutting_down};
use dos_like::dos_like_sys::*;
use std::f32::consts::PI;
use std::os::raw::c_int;

dos_main! {
    unsafe {
        setvideomode(videomode_t_videomode_320x200);
        setdoublebuffer(1);

        let mut palette = [0_u8; 768];
        let mut gif_width = 0;
        let mut gif_height = 0;
        let mut palcount = 0;
        let gif = loadgif("assets/rotozoom.gif\0".as_ptr() as *const i8,
            &mut gif_width,
            &mut gif_height,
            &mut palcount,
            palette.as_mut_ptr(),
        );
        if gif.is_null() {
            eprintln!("Could not load rotozoom.gif");
            std::process::exit(-2);
        }

        for i in 0..palcount as usize {
            setpal(
                i as c_int,
                palette[3 * i + 0] as c_int,
                palette[3 * i + 1] as c_int,
                palette[3 * i + 2] as c_int,
            );
        }

        let mut buffer = screenbuffer();
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
                    buffer.offset(dest_ofs as isize).write(*gif.offset(src_ofs as isize));
                    dest_ofs += 1;
                }
            }
            drop(buffer);
            buffer = swapbuffers();

            if keystate(keycode_t_KEY_ESCAPE) != 0 {
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
