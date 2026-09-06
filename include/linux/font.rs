//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/font.h
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
// font.h -- `Soft' font definitions
//
// Created 1995 by Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Glyphs
//
// font_glyph_pitch - Calculates the number of bytes per scanline
// @width: The glyph width in bits per scanline
//
// A glyph's pitch is the number of bytes in a single scanline, rounded
// up to the next full byte. The parameter @width receives the number
// of visible bits per scanline. For example, if width is 14 bytes per
// scanline, the pitch is 2 bytes per scanline. If width is 8 bits per
// scanline, the pitch is 1 byte per scanline.
//
// Returns:
// The number of bytes in a single scanline of the glyph
//
extern "C" {
    pub fn DIV_ROUND_UP(_arg: width, _arg: 8) -> return;
}
//
// font_glyph_size - Calculates the number of bytes per glyph
// @width: The glyph width in bits per scanline
// @vpitch: The number of scanlines in the glyph
//
// The number of bytes in a glyph depends on the pitch and the number
// of scanlines. font_glyph_size automatically calculates the pitch
// from the given width. The parameter @vpitch gives the number of
// scanlines, which is usually the glyph's height in scanlines. Fonts
// coming from user space can sometimes have a different vertical pitch
// with empty scanlines between two adjacent glyphs.
//
// Returns: the number of bytes per glyph
//
// font_data_t and helpers
//
// typedef font_data_t - Raw font data
//
// Values of type font_data_t store a pointer to raw font data. The format
// is monochrome. Each bit sets a pixel of a stored glyph. Font data does
// not store geometry information for the individual glyphs. Users of the
// font have to store glyph size, pitch and character count separately.
//
// Font data in font_data_t is not equivalent to raw u8. Each pointer stores
// an additional hidden header before the font data. The layout is
//
// +------+-----------------------------+
// | -16  |  CRC32 Checksum (optional)  |
// | -12  |  <Unused>                   |
// |  -8  |  Number of data bytes       |
// |  -4  |  Reference count            |
// +------+-----------------------------+
// |   0  |  Data buffer                |
// |  ... |                             |
// +------+-----------------------------+
//
// Use helpers to access font_data_t. Use font_data_buf() to get the stored data.
//
pub type font_data_t = c_uchar;
//
// font_data_buf() - Returns the font data as raw bytes
// @fd: The font data
//
// Returns:
// The raw font data. The provided buffer is read-only.
//
extern "C" {
    pub fn font_data_get(fd: *mut font_data_t);
}
extern "C" {
    pub fn font_data_put(fd: *mut font_data_t) -> bool;
}
extern "C" {
    pub fn font_data_size(fd: *mut font_data_t) -> c_uint;
}
extern "C" {
    pub fn font_data_is_equal(lhs: *mut font_data_t, rhs: *mut font_data_t) -> bool;
}
extern "C" {
    pub fn font_data_export(fd: *mut font_data_t, font: *mut console_font, vpitch: c_uint) -> c_int;
}
// font_rotate.c
//
// Font description
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct font_desc {
    pub idx: c_int,
    pub name: *const c_char,
    pub height: unsigned int width,,
    pub charcount: c_uint,
    pub data: *mut font_data_t,
    pub pref: c_int,
}

// Find a font with a specific name
// Get the default font for a specific screen size
// Max. length for the name of a predefined font
pub const MAX_FONT_NAME: c_int = 32;
//
// Built-in fonts
//
