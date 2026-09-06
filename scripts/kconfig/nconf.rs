//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/nconf.h
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
// Copyright (C) 2008 Nir Tzachar <nir.tzachar@gmail.com>
//
// Derived from menuconfig.
//

extern "C" {
    pub fn set_colors();
}
extern "C" {
    pub fn int(_arg: *mut extra_key_cb_fn)(int, _arg: usize, _arg: usize, : *mut c_void) -> typedef;
}
// this changes the windows attributes !!!
extern "C" {
    pub fn print_in_middle(win: *mut WINDOW, y: c_int, width: c_int, str: *const c_char, attrs: c_int);
}
extern "C" {
    pub fn get_line_length(line: *const c_char) -> c_int;
}
extern "C" {
    pub fn get_line_no(text: *const c_char) -> c_int;
}
extern "C" {
    pub fn fill_window(win: *mut WINDOW, text: *const c_char);
}
extern "C" {
    pub fn btn_dialog(main_window: *mut WINDOW, msg: *const c_char, btn_num: c_int, ...) -> c_int;
}
extern "C" {
    pub fn refresh_all_windows(main_window: *mut WINDOW);
}
