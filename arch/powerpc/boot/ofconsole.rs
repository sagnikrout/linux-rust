//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/ofconsole.c
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
//
// OF console routines
//
// Copyright (C) Paul Mackerras 1997.
//

    static unsigned int of_stdout_handle;
#[no_mangle]
unsafe extern "C" fn of_console_open() -> c_int {
    static int of_console_open(void)
    {
    void *devp;
    if (((devp = of_finddevice("/chosen")) != core::ptr::null_mut())
    && (of_getprop(devp, "stdout", &of_stdout_handle,
    sizeof(of_stdout_handle))
    == sizeof(of_stdout_handle))) {
    of_stdout_handle = be32_to_cpu(of_stdout_handle);
    return 0;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn of_console_write(buf: *const c_char, len: c_int) {
    static void of_console_write(const char *buf, int len)
    {
    of_call_prom("write", 3, 1, of_stdout_handle, buf, len);
    }
#[no_mangle]
pub unsafe extern "C" fn of_console_init() {
    void of_console_init(void)
    {
    console_ops.open = of_console_open;
    console_ops.write = of_console_write;
    }
