//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/imx_earlycon.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2020 NXP
//

pub const URTX0: c_uint = 0x40 /* Transmitter Register */;

pub const IMX21_UTS: c_uint = 0xb4 /* UART Test Register on all other i.mx*/;
#[no_mangle]
unsafe extern "C" fn imx_uart_console_early_putchar(port: *mut uart_port, ch: c_uchar) {
    static void imx_uart_console_early_putchar(struct uart_port *port, unsigned char ch)
    {
    while (readl_relaxed(port.membase + IMX21_UTS) & UTS_TXFULL)
    cpu_relax();
    writel_relaxed(ch, port.membase + URTX0);
    }
    static void imx_uart_console_early_write(struct console *con, const char *s,
    unsigned count)
    {
    struct earlycon_device *dev = con.data;
    uart_console_write(&dev.port, s, count, imx_uart_console_early_putchar);
    }
    static int __init
    imx_console_early_setup(struct earlycon_device *dev, const char *opt)
    {
    if (!dev.port.membase)
    return -ENODEV;
    dev.con.write = imx_uart_console_early_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(ec_imx6q, "fsl,imx6q-uart", imx_console_early_setup);
    OF_EARLYCON_DECLARE(ec_imx21, "fsl,imx21-uart", imx_console_early_setup);
    MODULE_AUTHOR("NXP");
    MODULE_DESCRIPTION("IMX earlycon driver");
    MODULE_LICENSE("GPL");
