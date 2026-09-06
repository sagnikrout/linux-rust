//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_hub6.c
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

    {								\
    .iobase		= 0x302,				\
    .irq		= 3,					\
    .uartclk	= 1843200,				\
    .iotype		= UPIO_HUB6,				\
    .flags		= UPF_BOOT_AUTOCONF,			\
    .hub6		= (card) << 6 | (port) << 3 | 1,	\
    }
    static struct plat_serial8250_port hub6_data[] = {
    HUB6(0, 0),
    HUB6(0, 1),
    HUB6(0, 2),
    HUB6(0, 3),
    HUB6(0, 4),
    HUB6(0, 5),
    HUB6(1, 0),
    HUB6(1, 1),
    HUB6(1, 2),
    HUB6(1, 3),
    HUB6(1, 4),
    HUB6(1, 5),
    { },
    };
    static struct platform_device hub6_device = {
    .name			= "serial8250",
    .id			= PLAT8250_DEV_HUB6,
    .dev			= {
    .platform_data	= hub6_data,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn hub6_match_port(port1: *const uart_port, port2: *const uart_port) -> bool {
    bool hub6_match_port(const struct uart_port *port1, const struct uart_port *port2)
    {
    return port1.iobase == port2.iobase && port1.hub6 == port2.hub6;
    }
    EXPORT_SYMBOL_GPL(hub6_match_port);
#[no_mangle]
unsafe extern "C" fn hub6_init() -> int __init {
    static int __init hub6_init(void)
    {
    return platform_device_register(&hub6_device);
    }
    module_init(hub6_init);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("8250 serial probe module for Hub6 cards");
    MODULE_LICENSE("GPL");
