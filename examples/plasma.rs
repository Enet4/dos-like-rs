// Rust port of plasma tutorial code by Lode Vandevenne
// https://lodev.org/cgtutor/plasma.html
// See end of file for license
#![no_main]

use dos_like::{
    key_state, screen_buffer, set_double_buffer, set_pal, set_video_mode, shutting_down,
    swap_buffers_and_get, wait_vbl, KeyCode, VideoMode,
};
use std::os::raw::c_int;

dos_like::dos_main! {
    // Y-coordinate first because we use horizontal scanlines
    let mut plasma = [[0 as c_int; 320]; 200];

    set_video_mode(VideoMode::Graphics320x200);
    set_double_buffer(true);
    let w = 320;
    let h = 200;

    //generate the palette
    for x in 0..256 {
        let r = (128. + 128. * (x as f64 * 3.1415 / 32.).sin()) as u8;
        let g = (128. + 128. * (x as f64 * 3.1415 / 64.).sin()) as u8;
        let b = (128. + 128. * (x as f64 * 3.1415 / 128.).sin()) as u8;
        set_pal(x, r >> 2, g >> 2, b >> 2);
    }

    //generate the plasma once
    for y in 0..h {
        for x in 0..w {
            //the plasma buffer is a sum of sines
            let color = (128.0
                + (128.0 * (x as f32 / 32.).sin())
                + 128.0
                + (128.0 * (y as f32 / 16.).sin())
                + 128.0
                + (128.0 * ((x as f32 + y as f32) / 32.).sin())
                + 128.0
                + (128.0 * (x * x + y * y) as f64 / 16.).sqrt().sin() as f32)
                as c_int
                / 4;
                plasma[y][x] = color;
        }
    }

    let mut palette_shift = 0;

    unsafe {
        let mut buffer = screen_buffer();

        //start the animation loop, it rotates the palette
        while !shutting_down() {
            wait_vbl();

            //the parameter to shift the palette varies with time
            palette_shift += 1;

            //draw every pixel again, with the shifted palette color
            for y in 0..h {
                for x in 0..w {
                    buffer[x + y * 320] = (plasma[y][x] + palette_shift) as u8;
                }
            }

            //make everything visible
            buffer = swap_buffers_and_get();

            if key_state(KeyCode::KEY_ESCAPE) {
                break;
            }
        }
    }
}

/*
Lode's Computer Graphics Tutorial
Legal Stuff
This tutorial (including all the separate articles) is Copyright (c) 2004-2007 by Lode Vandevenne. All rights reserved. Do not copy/translate any of the content of this tutorial to a site/book/whatever without my permission.

Some photos are taken from the free photo archive at morguefile.com. These are of course copyright by the authors and contributers of Morguefile. A thank you goes to the photographers!

The source code of QuickCG and all the source code of the examples given in this tutorial and all its articles is released under the following license:

Copyright (c) 2004-2007, Lode Vandevenne

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
