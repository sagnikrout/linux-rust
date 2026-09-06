//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/selection.h
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


// SPDX-License-Identifier: GPL-2.0
//
// selection.h
//
// Interface between console.c, tty_io.c, vt.c, vc_screen.c and selection.c
//

extern "C" {
    pub fn clear_selection();
}
extern "C" {
    pub fn set_selection_kernel(v: *mut tiocl_selection, tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn paste_selection(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn sel_loadlut(lut: *mut u32 __user) -> c_int;
}
extern "C" {
    pub fn mouse_reporting() -> c_int;
}
extern "C" {
    pub fn mouse_report(tty: *mut tty_struct, butt: c_int, mrx: c_int, mry: c_int);
}
extern "C" {
    pub fn vc_is_sel(vc: *const vc_data) -> bool;
}
extern "C" {
    pub fn screen_glyph(vc: *const vc_data, offset: c_int) -> u16;
}
extern "C" {
    pub fn screen_glyph_unicode(vc: *const vc_data, offset: c_int) -> u32;
}
extern "C" {
    pub fn complement_pos(vc: *mut vc_data, offset: c_int);
}
extern "C" {
    pub fn invert_screen(vc: *mut vc_data, offset: c_int, count: c_int, viewed: bool);
}
extern "C" {
    pub fn getconsxy(vc: *const vc_data, 2]: unsigned char xy[static);
}
extern "C" {
    pub fn putconsxy(vc: *mut vc_data, 2]: unsigned char xy[static const);
}
extern "C" {
    pub fn vcs_scr_readw(vc: *const vc_data, org: *const u16) -> u16;
}
extern "C" {
    pub fn vcs_scr_writew(vc: *mut vc_data, val: u16, org: *mut u16);
}
extern "C" {
    pub fn vcs_scr_updated(vc: *mut vc_data);
}
extern "C" {
    pub fn vc_uniscr_check(vc: *mut vc_data) -> c_int;
}
