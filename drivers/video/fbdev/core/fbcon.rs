//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fbcon.h
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
// linux/drivers/video/console/fbcon.h -- Low level frame buffer based console driver
//
// Copyright (C) 1997 Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// This is the interface between the low-level console driver and the
// low-level frame buffer device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbcon_display {
// Filled in by the low-level console driver
    pub fontdata: *mut font_data_t,

    pub /: *mut *mut u_short scrollmode; / Scroll Method, use fb_scrollmode(),

    pub /: *mut *mut short yscroll; / Hardware scrolling,
    pub /: *mut *mut int vrows; / number of virtual rows,
    pub cursor_shape: c_int,
    pub con_rotate: c_int,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub height: u32,
    pub width: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub nonstd: u32,
    pub accel_flags: u32,
    pub rotate: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub mode: *const fb_videomode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbcon_bitops {
    pub width): int sx, int dy, int dx, int height, int,
    pub bg): int sx, int height, int width, int fb, int,
    pub bg): int fg, int,
    pub bottom_only): int color, int,
    pub bg): bool enable, int fg, int,
    pub info): *mut *mut int (update_start)(struct fb_info,
    pub vc): *mut *mut *mut int (rotate_font)(struct fb_info info, struct vc_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbcon_par {
    pub /: *mut *mut fb_var_screeninfo var; / copy of the current fb_var_screeninfo,
    pub /: *mut *mut delayed_work cursor_work; / Cursor timer,
    pub cursor_state: fb_cursor,
    pub p: *mut fbcon_display,
    pub info: *mut fb_info,
    pub /: *mut *mut int currcon; / Current VC.,
    pub cur_blink_jiffies: c_int,
    pub cursor_flash: c_int,
    pub cursor_reset: c_int,
    pub blank_state: c_int,
    pub graphics: c_int,
    pub initialized: bool,
    pub rotate: c_int,
    pub cursor_data: *mut c_char,

    pub /: *mut *mut *mut font_data_t fontdata; / source font,
    pub /: *mut *mut *mut u8 buf; / rotated glyphs,
    pub bufsize: usize,
    pub /: *mut *mut int buf_rotate; / rotation of buf,
    pub rotated: },

    pub cursor_src: *mut u8,
    pub cursor_size: u32,
    pub bitops: *const fbcon_bitops,
}

//
// Attribute Decoding
//
// Color

// Monochrome

//
// Scroll Method
//
// There are several methods fbcon can use to move text around the screen:
//
// Operation   Pan    Wrap
// ---------------------------------------------
// SCROLL_MOVE         copyarea    No     No
// SCROLL_PAN_MOVE     copyarea    Yes    No
// SCROLL_WRAP_MOVE    copyarea    No     Yes
// SCROLL_REDRAW       imageblit   No     No
// SCROLL_PAN_REDRAW   imageblit   Yes    No
// SCROLL_WRAP_REDRAW  imageblit   No     Yes
//
// (SCROLL_WRAP_REDRAW is not implemented yet)
//
// In general, fbcon will choose the best scrolling
// method based on the rule below:
//
// Pan/Wrap > accel imageblit > accel copyarea >
// soft imageblit > (soft copyarea)
//
// Exception to the rule: Pan + accel copyarea is
// preferred over Pan + accel imageblit.
//
// The above is typical for PCI/AGP cards. Unless
// overridden, fbcon will never use soft copyarea.
//
// If you need to override the above rule, set the
// appropriate flags in fb_info->flags.  For example,
// to prefer copyarea over imageblit, set
// FBINFO_READS_FAST.
//
// Other notes:
// + use the hardware engine to move the text
// (hw-accelerated copyarea() and fillrect())
// + use hardware-supported panning on a large virtual screen
// + amifb can not only pan, but also wrap the display by N lines
// (i.e. visible line i = physical line (i+N) % yres).
// + read what's already rendered on the screen and
// write it in a different place (this is cfb_copyarea())
// + re-render the text to the screen
//
// Whether to use wrapping or panning can only be figured out at
// runtime (when we know whether our font height is a multiple
// of the pan/wrap step)
//
pub const SCROLL_MOVE: c_uint = 0x001;
pub const SCROLL_PAN_MOVE: c_uint = 0x002;
pub const SCROLL_WRAP_MOVE: c_uint = 0x003;
pub const SCROLL_REDRAW: c_uint = 0x004;
pub const SCROLL_PAN_REDRAW: c_uint = 0x005;

// hardcoded to SCROLL_REDRAW if acceleration was disabled.

extern "C" {
    pub fn fbcon_set_tileops(vc: *mut vc_data, info: *mut fb_info);
}

extern "C" {
    pub fn fbcon_set_bitops_ur(par: *mut fbcon_par);
}
extern "C" {
    pub fn soft_cursor(info: *mut fb_info, cursor: *mut fb_cursor) -> c_int;
}
extern "C" {
    pub fn fbcon_fill_cursor_mask(par: *mut fbcon_par, vc: *mut vc_data, mask: *mut c_uchar);
}
pub const FBCON_ATTRIBUTE_UNDERLINE: c_int = 1;
pub const FBCON_ATTRIBUTE_REVERSE: c_int = 2;
pub const FBCON_ATTRIBUTE_BOLD: c_int = 4;

extern "C" {
    pub fn fb_console_init() -> void __init;
}
extern "C" {
    pub fn fb_console_exit() -> void __exit;
}
extern "C" {
    pub fn fbcon_fb_registered(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fbcon_fb_unregistered(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_fb_unbind(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_suspended(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_resumed(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_delete_modelist(head: *mut list_head);
}
extern "C" {
    pub fn fbcon_new_modelist(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_fb_blanked(info: *mut fb_info, blank: c_int);
}
extern "C" {
    pub fn fbcon_update_vcs(info: *mut fb_info, all: bool);
}
extern "C" {
    pub fn fbcon_remap_all(info: *mut fb_info);
}
extern "C" {
    pub fn fbcon_set_con2fb_map_ioctl(argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn fbcon_get_con2fb_map_ioctl(argp: *mut void __user) -> c_int;
}

