//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/early_printk.c
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
//
// Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
//

#[no_mangle]
unsafe extern "C" fn early_console_write(con: *mut console, s: *const c_char, n: c_uint) {
    static void early_console_write(struct console *con, const char *s, unsigned int n)
    {
    um_early_printk(s, n);
    }
    static struct console early_console_dev = {
    .name = "earlycon",
    .write = early_console_write,
    .flags = CON_BOOT,
    .index = -1,
    };
#[no_mangle]
unsafe extern "C" fn setup_early_printk(buf: *mut c_char) -> int __init {
    static int __init setup_early_printk(char *buf)
    {
    if (!early_console) {
    early_console = &early_console_dev;
    register_console(&early_console_dev);
    }
    return 0;
    }
    early_param("earlyprintk", setup_early_printk);
