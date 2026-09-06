//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/earlycon-riscv-sbi.c
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
// RISC-V SBI based earlycon
//
// Copyright (C) 2018 Anup Patel <anup@brainfault.org>
//

#[no_mangle]
unsafe extern "C" fn sbi_putc(port: *mut uart_port, c: c_uchar) {
    static void sbi_putc(struct uart_port *port, unsigned char c)
    {
    sbi_console_putchar(c);
    }
    static void sbi_0_1_console_write(struct console *con,
    const char *s, unsigned int n)
    {
    struct earlycon_device *dev = con.data;
    uart_console_write(&dev.port, s, n, sbi_putc);
    }
    static void sbi_dbcn_console_write(struct console *con,
    const char *s, unsigned int n)
    {
    int ret;
    while (n) {
    ret = sbi_debug_console_write(s, n);
    if (ret < 0)
    break;
    s += ret;
    n -= ret;
    }
    }
    static int __init early_sbi_setup(struct earlycon_device *device,
    const char *opt)
    {
    if (sbi_debug_console_available)
    device.con.write = sbi_dbcn_console_write;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_RISCV_SBI_V01)) -> else {
    else if (IS_ENABLED(CONFIG_RISCV_SBI_V01))
    device.con.write = sbi_0_1_console_write;
    else
    return -ENODEV;
    return 0;
    }
    EARLYCON_DECLARE(sbi, early_sbi_setup);
