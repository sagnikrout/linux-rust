//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/atafb_mfb.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// linux/drivers/video/mfb.c -- Low level frame buffer operations for
// monochrome
//
// Created 5 Apr 1997 by Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

//
// Monochrome
//
    void atafb_mfb_copyarea(struct fb_info *info, u_long next_line,
    int sy, int sx, int dy, int dx,
    int height, int width)
    {
    u8 *src, *dest;
    u_int rows;
    if (sx == 0 && dx == 0 && width == next_line) {
    src = (u8 *)info.screen_base + sy * (width >> 3);
    dest = (u8 *)info.screen_base + dy * (width >> 3);
    fb_memmove(dest, src, height * (width >> 3));
    } else if (dy <= sy) {
    src = (u8 *)info.screen_base + sy * next_line + (sx >> 3);
    dest = (u8 *)info.screen_base + dy * next_line + (dx >> 3);
    for (rows = height; rows--;) {
    fb_memmove(dest, src, width >> 3);
    src += next_line;
    dest += next_line;
    }
    } else {
    src = (u8 *)info.screen_base + (sy + height - 1) * next_line + (sx >> 3);
    dest = (u8 *)info.screen_base + (dy + height - 1) * next_line + (dx >> 3);
    for (rows = height; rows--;) {
    fb_memmove(dest, src, width >> 3);
    src -= next_line;
    dest -= next_line;
    }
    }
    }
    void atafb_mfb_fillrect(struct fb_info *info, u_long next_line, u32 color,
    int sy, int sx, int height, int width)
    {
    u8 *dest;
    u_int rows;
    dest = (u8 *)info.screen_base + sy * next_line + (sx >> 3);
    if (sx == 0 && width == next_line) {
    if (color)
    fb_memset255(dest, height * (width >> 3));
    else
    fb_memclear(dest, height * (width >> 3));
    } else {
    for (rows = height; rows--; dest += next_line) {
    if (color)
    fb_memset255(dest, width >> 3);
    else
    fb_memclear_small(dest, width >> 3);
    }
    }
    }
    void atafb_mfb_linefill(struct fb_info *info, u_long next_line,
    int dy, int dx, u32 width,
    const u8 *data, u32 bgcolor, u32 fgcolor)
    {
    u8 *dest;
    u_int rows;
    dest = (u8 *)info.screen_base + dy * next_line + (dx >> 3);
    for (rows = width / 8; rows--; /* check margins */ ) {
// use fast_memmove or fb_memmove
// dest++ = *data++;
    }
    }
