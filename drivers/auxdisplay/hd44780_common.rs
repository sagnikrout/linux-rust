//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/auxdisplay/hd44780_common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
pub const DEFAULT_LCD_BWIDTH: c_int = 40;
pub const DEFAULT_LCD_HWIDTH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd44780_common {
    pub /: *mut *mut int ifwidth; / 4-bit or 8-bit (default),
    pub /: *mut *mut int bwidth; / Default set by hd44780_alloc(),
    pub /: *mut *mut int hwidth; / Default set by hd44780_alloc(),
    pub hd44780_common_flags: c_ulong,
    pub data): *mut *mut *mut void (write_data)(struct hd44780_common hdc, int,
    pub cmd): *mut *mut *mut void (write_cmd)(struct hd44780_common hdc, int,
// write_cmd_raw4 is for 4-bit connected displays only
    pub cmd): *mut *mut *mut void (write_cmd_raw4)(struct hd44780_common hdc, int,
    pub hd44780: *mut c_void,
}

extern "C" {
    pub fn hd44780_common_print(lcd: *mut charlcd, c: c_int) -> c_int;
}
extern "C" {
    pub fn hd44780_common_gotoxy(lcd: *mut charlcd, x: c_uint, y: c_uint) -> c_int;
}
extern "C" {
    pub fn hd44780_common_home(lcd: *mut charlcd) -> c_int;
}
extern "C" {
    pub fn hd44780_common_clear_display(lcd: *mut charlcd) -> c_int;
}
extern "C" {
    pub fn hd44780_common_init_display(lcd: *mut charlcd) -> c_int;
}
extern "C" {
    pub fn hd44780_common_display(lcd: *mut charlcd, on: charlcd_onoff) -> c_int;
}
extern "C" {
    pub fn hd44780_common_cursor(lcd: *mut charlcd, on: charlcd_onoff) -> c_int;
}
extern "C" {
    pub fn hd44780_common_blink(lcd: *mut charlcd, on: charlcd_onoff) -> c_int;
}
extern "C" {
    pub fn hd44780_common_fontsize(lcd: *mut charlcd, size: charlcd_fontsize) -> c_int;
}
extern "C" {
    pub fn hd44780_common_lines(lcd: *mut charlcd, lines: charlcd_lines) -> c_int;
}
extern "C" {
    pub fn hd44780_common_redefine_char(lcd: *mut charlcd, esc: *mut c_char) -> c_int;
}
extern "C" {
    pub fn hd44780_common_free(lcd: *mut charlcd);
}
