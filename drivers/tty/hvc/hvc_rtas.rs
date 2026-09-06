//! Automatically rewritten from C to Rust
//! Source: drivers/tty/hvc/hvc_rtas.c
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
// IBM RTAS driver interface to hvc_console.c
//
// (C) Copyright IBM Corporation 2001-2005
// (C) Copyright Red Hat, Inc. 2005
//
// Author(s): Maximino Augilar <IBM STI Design Center>
// : Ryan S. Arnold <rsa@us.ibm.com>
// : Utz Bacher <utz.bacher@de.ibm.com>
// : David Woodhouse <dwmw2@infradead.org>
//
// inspired by drivers/char/hvc_console.c
// written by Anton Blanchard and Paul Mackerras
//

pub const hvc_rtas_cookie: c_uint = 0x67781e15;
    static struct hvc_struct *hvc_rtas_dev;
    let mut rtascons_put_char_token: static int = RTAS_UNKNOWN_SERVICE;
    let mut rtascons_get_char_token: static int = RTAS_UNKNOWN_SERVICE;
    static ssize_t hvc_rtas_write_console(uint32_t vtermno, const u8 *buf,
    size_t count)
    {
    size_t i;
    for (i = 0; i < count; i++) {
    if (rtas_call(rtascons_put_char_token, 1, 1, core::ptr::null_mut(), buf[i]))
    break;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn hvc_rtas_read_console(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    static ssize_t hvc_rtas_read_console(uint32_t vtermno, u8 *buf, size_t count)
    {
    size_t i;
    int c;
    for (i = 0; i < count; i++) {
    if (rtas_call(rtascons_get_char_token, 0, 2, &c))
    break;
    buf[i] = c;
    }
    return i;
    }
    static const struct hv_ops hvc_rtas_get_put_ops = {
    .get_chars = hvc_rtas_read_console,
    .put_chars = hvc_rtas_write_console,
    };
#[no_mangle]
unsafe extern "C" fn hvc_rtas_init() -> int __init {
    static int __init hvc_rtas_init(void)
    {
    struct hvc_struct *hp;
    if (rtascons_put_char_token == RTAS_UNKNOWN_SERVICE)
    rtascons_put_char_token = rtas_token("put-term-char");
    if (rtascons_put_char_token == RTAS_UNKNOWN_SERVICE)
    return -EIO;
    if (rtascons_get_char_token == RTAS_UNKNOWN_SERVICE)
    rtascons_get_char_token = rtas_token("get-term-char");
    if (rtascons_get_char_token == RTAS_UNKNOWN_SERVICE)
    return -EIO;
    BUG_ON(hvc_rtas_dev);
// Allocate an hvc_struct for the console device we instantiated
// earlier.  Save off hp so that we can return it on exit
    hp = hvc_alloc(hvc_rtas_cookie, 0, &hvc_rtas_get_put_ops, 16);
    if (IS_ERR(hp))
    return PTR_ERR(hp);
    hvc_rtas_dev = hp;
    return 0;
    }
    device_initcall(hvc_rtas_init);
// This will happen prior to module init.  There is no tty at this time?
#[no_mangle]
unsafe extern "C" fn hvc_rtas_console_init() -> int __init {
    static int __init hvc_rtas_console_init(void)
    {
    rtascons_put_char_token = rtas_token("put-term-char");
    if (rtascons_put_char_token == RTAS_UNKNOWN_SERVICE)
    return -EIO;
    rtascons_get_char_token = rtas_token("get-term-char");
    if (rtascons_get_char_token == RTAS_UNKNOWN_SERVICE)
    return -EIO;
    hvc_instantiate(hvc_rtas_cookie, 0, &hvc_rtas_get_put_ops);
    add_preferred_console("hvc", 0, core::ptr::null_mut());
    return 0;
    }
    console_initcall(hvc_rtas_console_init);
