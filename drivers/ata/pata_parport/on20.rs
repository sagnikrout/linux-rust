//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_parport/on20.c
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
// (c) 1996-1998  Grant R. Guenther <grant@torque.net>
//
// on20.c is a low-level protocol driver for the
// Onspec 90c20 parallel to IDE adapter.
//

    do {					\
    w2(4); w0(f); w2(5); w2(0xd);	\
    w2(5); w2(0xd); w2(5); w2(4);	\
    } while (0)

    do {					\
    w2(4); w0(v); w2(5);		\
    w2(7); w2(5); w2(4);		\
    } while (0)

//
// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set
//
#[no_mangle]
unsafe extern "C" fn on20_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    static int on20_read_regr(struct pi_adapter *pi, int cont, int regr)
    {
    int h, l, r;
    r = (regr << 2) + 1 + cont;
    op(1); vl(r); op(0);
    switch (pi.mode) {
    case 0:
    w2(4); w2(6); l = r1();
    w2(4); w2(6); h = r1();
    w2(4); w2(6); w2(4); w2(6); w2(4);
    return j44(l, h);
    case 1:
    w2(4); w2(0x26); r = r0();
    w2(4); w2(0x26); w2(4);
    return r;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn on20_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    static void on20_write_regr(struct pi_adapter *pi, int cont, int regr, int val)
    {
    let mut r: c_int = (regr << 2) + 1 + cont;
    op(1); vl(r);
    op(0); vl(val);
    op(0); vl(val);
    }
#[no_mangle]
unsafe extern "C" fn on20_connect(pi: *mut pi_adapter) {
    static void on20_connect(struct pi_adapter *pi)
    {
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    w2(4); w0(0); w2(0xc); w2(4); w2(6); w2(4); w2(6); w2(4);
    if (pi.mode) {
    op(2); vl(8); op(2); vl(9);
    } else {
    op(2); vl(0); op(2); vl(8);
    }
    }
#[no_mangle]
unsafe extern "C" fn on20_disconnect(pi: *mut pi_adapter) {
    static void on20_disconnect(struct pi_adapter *pi)
    {
    w2(4); w0(7); w2(4); w2(0xc); w2(4);
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    }
#[no_mangle]
unsafe extern "C" fn on20_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void on20_read_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k, l, h;
    op(1); vl(1); op(0);
    for (k = 0; k < count; k++) {
    if (pi.mode) {
    w2(4); w2(0x26); buf[k] = r0();
    } else {
    w2(6); l = r1(); w2(4);
    w2(6); h = r1(); w2(4);
    buf[k] = j44(l, h);
    }
    }
    w2(4);
    }
#[no_mangle]
unsafe extern "C" fn on20_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void on20_write_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k;
    op(1); vl(1); op(0);
    for (k = 0; k < count; k++) {
    w2(5); w0(buf[k]); w2(7);
    }
    w2(4);
    }
#[no_mangle]
unsafe extern "C" fn on20_log_adapter(pi: *mut pi_adapter) {
    static void on20_log_adapter(struct pi_adapter *pi)
    {
    char *mode_string[2] = { "4-bit", "8-bit" };
    dev_info(&pi.dev,
    "OnSpec 90c20 at 0x%x, mode %d (%s), delay %d\n",
    pi.port, pi.mode, mode_string[pi.mode], pi.delay);
    }
    static struct pi_protocol on20 = {
    .owner		= THIS_MODULE,
    .name		= "on20",
    .max_mode	= 2,
    .epp_first	= 2,
    .default_delay	= 1,
    .max_units	= 1,
    .write_regr	= on20_write_regr,
    .read_regr	= on20_read_regr,
    .write_block	= on20_write_block,
    .read_block	= on20_read_block,
    .connect	= on20_connect,
    .disconnect	= on20_disconnect,
    .log_adapter	= on20_log_adapter,
    };
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
    MODULE_DESCRIPTION("Onspec 90c20 parallel port IDE adapter protocol driver");
    module_pata_parport_driver(on20);
