//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/color.h
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

// "\033[1;38;5;2xx;48;5;2xxm\0" is 23 bytes
pub const COLOR_MAXLEN: c_int = 24;

//
// This variable stores the value of color.ui
//
extern "C" {
    pub fn perf_config_colorbool(var: *const c_char, value: *const c_char, stdout_is_tty: c_int) -> c_int;
}
extern "C" {
    pub fn color_vfprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, __printf(3: ...), _arg: 4) -> c_int;
}
extern "C" {
    pub fn color_snprintf(bf: *mut c_char, size: usize, color: *const c_char, fmt: *const c_char, __printf(4: ...), _arg: 5) -> c_int;
}
extern "C" {
    pub fn value_color_snprintf(bf: *mut c_char, size: usize, fmt: *const c_char, value: double) -> c_int;
}
extern "C" {
    pub fn percent_color_snprintf(bf: *mut c_char, size: usize, fmt: *const c_char, __printf(3: ...), _arg: 4) -> c_int;
}
extern "C" {
    pub fn percent_color_len_snprintf(bf: *mut c_char, size: usize, fmt: *const c_char, __printf(3: ...), _arg: 4) -> c_int;
}
extern "C" {
    pub fn percent_color_fprintf(fp: *mut FILE, fmt: *const c_char, percent: double) -> c_int;
}
