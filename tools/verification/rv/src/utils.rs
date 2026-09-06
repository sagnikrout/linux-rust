//! Automatically rewritten from C to Rust
//! Source: tools/verification/rv/src/utils.c
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
// util functions.
//
// Copyright (C) 2022 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
//

    int config_debug;
pub const MAX_MSG_LENGTH: c_int = 1024;
//
// err_msg - print an error message to the stderr
//
#[no_mangle]
pub unsafe extern "C" fn err_msg(fmt: *const c_char, ...) {
    void err_msg(const char *fmt, ...)
    {
    char message[MAX_MSG_LENGTH];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(message, sizeof(message), fmt, ap);
    va_end(ap);
    fprintf(stderr, "%s", message);
    }
//
// debug_msg - print a debug message to stderr if debug is set
//
#[no_mangle]
pub unsafe extern "C" fn debug_msg(fmt: *const c_char, ...) {
    void debug_msg(const char *fmt, ...)
    {
    char message[MAX_MSG_LENGTH];
    va_list ap;
    if (!config_debug)
    return;
    va_start(ap, fmt);
    vsnprintf(message, sizeof(message), fmt, ap);
    va_end(ap);
    fprintf(stderr, "%s", message);
    }
