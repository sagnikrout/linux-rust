//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvc_udbg.c
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
// udbg interface to hvc_console.c
//
// (C) Copyright David Gibson, IBM Corporation 2008.
//

    static struct hvc_struct *hvc_udbg_dev;
#[no_mangle]
unsafe extern "C" fn hvc_udbg_put(vtermno: u32, buf: *const u8, count: usize) -> isize {
    static ssize_t hvc_udbg_put(uint32_t vtermno, const u8 *buf, size_t count)
    {
    size_t i;
    for (i = 0; i < count && udbg_putc; i++)
    udbg_putc(buf[i]);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn hvc_udbg_get(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    static ssize_t hvc_udbg_get(uint32_t vtermno, u8 *buf, size_t count)
    {
    size_t i;
    int c;
    if (!udbg_getc_poll)
    return 0;
    for (i = 0; i < count; i++) {
    if ((c = udbg_getc_poll()) == -1)
    break;
    buf[i] = c;
    }
    return i;
    }
    static const struct hv_ops hvc_udbg_ops = {
    .get_chars = hvc_udbg_get,
    .put_chars = hvc_udbg_put,
    };
#[no_mangle]
unsafe extern "C" fn hvc_udbg_init() -> int __init {
    static int __init hvc_udbg_init(void)
    {
    struct hvc_struct *hp;
    if (!udbg_putc)
    return -ENODEV;
    BUG_ON(hvc_udbg_dev);
    hp = hvc_alloc(0, 0, &hvc_udbg_ops, 16);
    if (IS_ERR(hp))
    return PTR_ERR(hp);
    hvc_udbg_dev = hp;
    return 0;
    }
    device_initcall(hvc_udbg_init);
#[no_mangle]
unsafe extern "C" fn hvc_udbg_console_init() -> int __init {
    static int __init hvc_udbg_console_init(void)
    {
    if (!udbg_putc)
    return -ENODEV;
    hvc_instantiate(0, 0, &hvc_udbg_ops);
    add_preferred_console("hvc", 0, core::ptr::null_mut());
    return 0;
    }
    console_initcall(hvc_udbg_console_init);
