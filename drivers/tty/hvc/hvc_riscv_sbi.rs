//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvc_riscv_sbi.c
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
// Copyright (C) 2008 David Gibson, IBM Corporation
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
//

#[no_mangle]
unsafe extern "C" fn hvc_sbi_tty_put(vtermno: u32, buf: *const u8, count: usize) -> isize {
    static ssize_t hvc_sbi_tty_put(uint32_t vtermno, const u8 *buf, size_t count)
    {
    size_t i;
    for (i = 0; i < count; i++)
    sbi_console_putchar(buf[i]);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn hvc_sbi_tty_get(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    static ssize_t hvc_sbi_tty_get(uint32_t vtermno, u8 *buf, size_t count)
    {
    size_t i;
    int c;
    for (i = 0; i < count; i++) {
    c = sbi_console_getchar();
    if (c < 0)
    break;
    buf[i] = c;
    }
    return i;
    }
    static const struct hv_ops hvc_sbi_v01_ops = {
    .get_chars = hvc_sbi_tty_get,
    .put_chars = hvc_sbi_tty_put,
    };
#[no_mangle]
unsafe extern "C" fn hvc_sbi_dbcn_tty_put(vtermno: u32, buf: *const u8, count: usize) -> isize {
    static ssize_t hvc_sbi_dbcn_tty_put(uint32_t vtermno, const u8 *buf, size_t count)
    {
    return sbi_debug_console_write(buf, count);
    }
#[no_mangle]
unsafe extern "C" fn hvc_sbi_dbcn_tty_get(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    static ssize_t hvc_sbi_dbcn_tty_get(uint32_t vtermno, u8 *buf, size_t count)
    {
    return sbi_debug_console_read(buf, count);
    }
    static const struct hv_ops hvc_sbi_dbcn_ops = {
    .put_chars = hvc_sbi_dbcn_tty_put,
    .get_chars = hvc_sbi_dbcn_tty_get,
    };
#[no_mangle]
unsafe extern "C" fn hvc_sbi_init() -> int __init {
    static int __init hvc_sbi_init(void)
    {
    int err;
    if (sbi_debug_console_available) {
    err = PTR_ERR_OR_ZERO(hvc_alloc(0, 0, &hvc_sbi_dbcn_ops, 256));
    if (err)
    return err;
    hvc_instantiate(0, 0, &hvc_sbi_dbcn_ops);
    } else if (IS_ENABLED(CONFIG_RISCV_SBI_V01)) {
    err = PTR_ERR_OR_ZERO(hvc_alloc(0, 0, &hvc_sbi_v01_ops, 256));
    if (err)
    return err;
    hvc_instantiate(0, 0, &hvc_sbi_v01_ops);
    } else {
    return -ENODEV;
    }
    return 0;
    }
    device_initcall(hvc_sbi_init);
