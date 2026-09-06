//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/debug.h
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
// For debugging general purposes

// Special macro to print perf_event_open arguments/return value.

extern "C" {
    pub fn dump_printf(fmt: *const c_char, __printf(1: ...), _arg: 2) -> c_int;
}
extern "C" {
    pub fn trace_event(event: *mut perf_event);
}
extern "C" {
    pub fn ui__error(format: *const c_char, __printf(1: ...), _arg: 2) -> c_int;
}
extern "C" {
    pub fn ui__warning(format: *const c_char, __printf(1: ...), _arg: 2) -> c_int;
}

extern "C" {
    pub fn pr_stat(fmt: *const c_char, ...);
}
extern "C" {
    pub fn eprintf(level: c_int, var: c_int, fmt: *const c_char, __printf(3: ...), _arg: 4) -> c_int;
}
extern "C" {
    pub fn eprintf_time(level: c_int, var: c_int, t: u64, fmt: *const c_char, __printf(4: ...), _arg: 5) -> c_int;
}
extern "C" {
    pub fn veprintf(level: c_int, var: c_int, fmt: *const c_char, args: va_list) -> c_int;
}
extern "C" {
    pub fn perf_debug_option(str: *const c_char) -> c_int;
}
extern "C" {
    pub fn debug_set_file(file: *mut FILE);
}
extern "C" {
    pub fn debug_set_display_time(set: bool);
}
extern "C" {
    pub fn perf_debug_setup();
}
extern "C" {
    pub fn perf_quiet_option() -> c_int;
}
extern "C" {
    pub fn __dump_stack(file: *mut FILE, stackdump: *mut c_void, stackdump_size: usize);
}
extern "C" {
    pub fn dump_stack();
}
extern "C" {
    pub fn sighandler_dump_stack(sig: c_int);
}
