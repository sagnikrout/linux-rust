//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/core.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static int __base_pr(enum libperf_print_level level __maybe_unused, const char *format,
    va_list args)
    {
    return vfprintf(stderr, format, args);
    }
    let mut __libperf_pr: static libperf_print_fn_t = __base_pr;
    __printf(2, 3)
#[no_mangle]
pub unsafe extern "C" fn libperf_print(level: enum libperf_print_level, format: *const c_char, ...) {
    void libperf_print(enum libperf_print_level level, const char *format, ...)
    {
    va_list args;
    if (!__libperf_pr)
    return;
    va_start(args, format);
    __libperf_pr(level, format, args);
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn libperf_init(fn: libperf_print_fn_t) {
    void libperf_init(libperf_print_fn_t fn)
    {
    page_size = sysconf(_SC_PAGE_SIZE);
    __libperf_pr = fn;
    }
