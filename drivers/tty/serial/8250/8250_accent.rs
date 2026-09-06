//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_accent.c
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
// Copyright (C) 2005 Russell King.
// Data taken from include/asm-i386/serial.h
//

    static struct plat_serial8250_port accent_data[] = {
    SERIAL8250_PORT(0x330, 4),
    SERIAL8250_PORT(0x338, 4),
    { },
    };
    static struct platform_device accent_device = {
    .name			= "serial8250",
    .id			= PLAT8250_DEV_ACCENT,
    .dev			= {
    .platform_data	= accent_data,
    },
    };
#[no_mangle]
unsafe extern "C" fn accent_init() -> int __init {
    static int __init accent_init(void)
    {
    return platform_device_register(&accent_device);
    }
    module_init(accent_init);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("8250 serial probe module for Accent Async cards");
    MODULE_LICENSE("GPL");
