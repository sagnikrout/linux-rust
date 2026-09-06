//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/color_config.c
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

#[no_mangle]
pub unsafe extern "C" fn perf_config_colorbool(var: *const c_char, value: *const c_char, stdout_is_tty: c_int) -> c_int {
    int perf_config_colorbool(const char *var, const char *value, int stdout_is_tty)
    {
    if (value) {
    if (!strcasecmp(value, "never"))
    return 0;
    if (!strcasecmp(value, "always"))
    return 1;
    if (!strcasecmp(value, "auto"))
    goto auto_color;
    }
// Missing or explicit false to turn off colorization
    if (!perf_config_bool(var, value))
    return 0;
// any normal truth value defaults to 'auto'
    auto_color:
    if (stdout_is_tty < 0)
    stdout_is_tty = isatty(1);
    if (stdout_is_tty || pager_in_use()) {
    char *term = getenv("TERM");
    if (term && strcmp(term, "dumb"))
    return 1;
    }
    return 0;
    }
