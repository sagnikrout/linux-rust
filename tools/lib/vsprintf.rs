//! Automatically rewritten from C to Rust
//! Source: tools/lib/vsprintf.c
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
pub unsafe extern "C" fn vscnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: va_list) -> c_int {
    int vscnprintf(char *buf, size_t size, const char *fmt, va_list args)
    {
    let mut i: c_int = vsnprintf(buf, size, fmt, args);
    let mut ssize: isize = size;
    return (i >= ssize) ? (ssize - 1) : i;
    }
#[no_mangle]
pub unsafe extern "C" fn scnprintf(buf: *mut *mut c_char, size: usize, fmt: *const *const c_char, ...) -> c_int {
    int scnprintf(char * buf, size_t size, const char * fmt, ...)
    {
    let mut ssize: isize = size;
    va_list args;
    int i;
    va_start(args, fmt);
    i = vsnprintf(buf, size, fmt, args);
    va_end(args);
    return (i >= ssize) ? (ssize - 1) : i;
    }
#[no_mangle]
pub unsafe extern "C" fn scnprintf_pad(buf: *mut *mut c_char, size: usize, fmt: *const *const c_char, ...) -> c_int {
    int scnprintf_pad(char * buf, size_t size, const char * fmt, ...)
    {
    let mut ssize: isize = size;
    va_list args;
    int i;
    va_start(args, fmt);
    i = vscnprintf(buf, size, fmt, args);
    va_end(args);
    if (i < (int) size) {
    for (; i < (int) size; i++)
    buf[i] = ' ';
    buf[i] = 0x0;
    }
    return (i >= ssize) ? (ssize - 1) : i;
    }
