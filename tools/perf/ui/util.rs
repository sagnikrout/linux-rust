//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/ui/util.h
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
pub const _PERF_UI_UTIL_H_: c_int = 1;

extern "C" {
    pub fn ui__getch(delay_secs: c_int) -> c_int;
}
extern "C" {
    pub fn ui__popup_menu(argc: c_int, argv[]: *const *const c_char, keyp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ui__help_window(text: *const c_char) -> c_int;
}
extern "C" {
    pub fn ui__dialog_yesno(msg: *const c_char) -> c_int;
}
extern "C" {
    pub fn __ui__info_window(title: *const c_char, text: *const c_char, exit_msg: *const c_char);
}
extern "C" {
    pub fn ui__info_window(title: *const c_char, text: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_error_ops {
    pub args): *const *const *const int (error)(char format, va_list,
    pub args): *const *const *const int (warning)(char format, va_list,
}

extern "C" {
    pub fn perf_error__register(eops: *mut perf_error_ops) -> c_int;
}
extern "C" {
    pub fn perf_error__unregister(eops: *mut perf_error_ops) -> c_int;
}
