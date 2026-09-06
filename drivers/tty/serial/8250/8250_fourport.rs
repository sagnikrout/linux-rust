//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_fourport.c
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

    SERIAL8250_PORT_FLAGS(_base, _irq, UPF_FOURPORT)
    static struct plat_serial8250_port fourport_data[] = {
    SERIAL8250_FOURPORT(0x1a0, 9),
    SERIAL8250_FOURPORT(0x1a8, 9),
    SERIAL8250_FOURPORT(0x1b0, 9),
    SERIAL8250_FOURPORT(0x1b8, 9),
    SERIAL8250_FOURPORT(0x2a0, 5),
    SERIAL8250_FOURPORT(0x2a8, 5),
    SERIAL8250_FOURPORT(0x2b0, 5),
    SERIAL8250_FOURPORT(0x2b8, 5),
    { },
    };
    static struct platform_device fourport_device = {
    .name			= "serial8250",
    .id			= PLAT8250_DEV_FOURPORT,
    .dev			= {
    .platform_data	= fourport_data,
    },
    };
#[no_mangle]
unsafe extern "C" fn fourport_init() -> int __init {
    static int __init fourport_init(void)
    {
    return platform_device_register(&fourport_device);
    }
    module_init(fourport_init);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("8250 serial probe module for AST Fourport cards");
    MODULE_LICENSE("GPL");
