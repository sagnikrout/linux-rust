//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_parport/on26.c
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
// (c) 1997-1998  Grant R. Guenther <grant@torque.net>
//
// on26.c is a low-level protocol driver for the
// OnSpec 90c26 parallel to IDE adapter chip.
//

//
// mode codes:  0  nybble reads, 8-bit writes
// 1  8-bit reads and writes
// 2  8-bit EPP mode
// 3  EPP-16
// 4  EPP-32
//

    do {						      \
    w2(5); w2(0xd); w2(5); w2(0xd); w2(5); w2(4); \
    } while (0)

    do {					\
    w2(5); w2(7); w2(5); w2(4);	\
    } while (0)
//
// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set
//
#[no_mangle]
unsafe extern "C" fn on26_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    static int on26_read_regr(struct pi_adapter *pi, int cont, int regr)
    {
    int a, b, r;
    r = (regr << 2) + 1 + cont;
    switch (pi.mode) {
    case 0:
    w0(1); P1; w0(r); P2; w0(0); P1;
    w2(6); a = r1(); w2(4);
    w2(6); b = r1(); w2(4);
    w2(6); w2(4); w2(6); w2(4);
    return j44(a, b);
    case 1:
    w0(1); P1; w0(r); P2; w0(0); P1;
    w2(0x26); a = r0(); w2(4); w2(0x26); w2(4);
    return a;
    case 2:
    case 3:
    case 4:
    w3(1); w3(1); w2(5); w4(r); w2(4);
    w3(0); w3(0); w2(0x24); a = r4(); w2(4);
    w2(0x24); (void)r4(); w2(4);
    return a;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn on26_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    static void on26_write_regr(struct pi_adapter *pi, int cont, int regr, int val)
    {
    let mut r: c_int = (regr << 2) + 1 + cont;
    switch (pi.mode) {
    case 0:
    case 1:
    w0(1); P1; w0(r); P2; w0(0); P1;
    w0(val); P2; w0(val); P2;
    break;
    case 2:
    case 3:
    case 4:
    w3(1); w3(1); w2(5); w4(r); w2(4);
    w3(0); w3(0);
    w2(5); w4(val); w2(4);
    w2(5); w4(val); w2(4);
    break;
    }
    }

    do {						\
    w0(0xfe); w0(0xaa); w0(0x55); w0(0);	\
    w0(0xff); w0(0x87); w0(0x78); w0(x);	\
    w2(4); w2(5); w2(4); w0(0xff);		\
    } while (0)
#[no_mangle]
unsafe extern "C" fn on26_connect(pi: *mut pi_adapter) {
    static void on26_connect(struct pi_adapter *pi)
    {
    int x;
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    CCP(0x20);
    if (pi.mode)
    x = 9;
    else
    x = 8;
    w0(2); P1; w0(8); P2;
    w0(2); P1; w0(x); P2;
    }
#[no_mangle]
unsafe extern "C" fn on26_disconnect(pi: *mut pi_adapter) {
    static void on26_disconnect(struct pi_adapter *pi)
    {
    if (pi.mode >= 2) {
    w3(4); w3(4); w3(4); w3(4);
    } else {
    w0(4); P1; w0(4); P1;
    }
    CCP(0x30);
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    }
pub const RESET_WAIT: c_int = 200;
// hard reset
#[no_mangle]
unsafe extern "C" fn on26_test_port(pi: *mut pi_adapter) -> c_int {
    static int on26_test_port(struct pi_adapter *pi)
    {
    int i, m, d, x = 0, y = 0;
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    d = pi.delay;
    m = pi.mode;
    pi.delay = 5;
    pi.mode = 0;
    w2(0xc);
    CCP(0x30); CCP(0);
    w0(0xfe); w0(0xaa); w0(0x55); w0(0); w0(0xff);
    i = ((r1() & 0xf0) << 4); w0(0x87);
    i |= (r1() & 0xf0); w0(0x78);
    w0(0x20); w2(4); w2(5);
    i |= ((r1() & 0xf0) >> 4);
    w2(4); w0(0xff);
    if (i == 0xb5f) {
    w0(2); P1; w0(0);   P2;
    w0(3); P1; w0(0);   P2;
    w0(2); P1; w0(8);   P2; udelay(100);
    w0(2); P1; w0(0xa); P2; udelay(100);
    w0(2); P1; w0(8);   P2; udelay(1000);
    on26_write_regr(pi, 0, 6, 0xa0);
    for (i = 0; i < RESET_WAIT; i++) {
    on26_write_regr(pi, 0, 6, 0xa0);
    x = on26_read_regr(pi, 0, 7);
    on26_write_regr(pi, 0, 6, 0xb0);
    y = on26_read_regr(pi, 0, 7);
    if (!((x & 0x80) || (y & 0x80)))
    break;
    mdelay(100);
    }
    if (i == RESET_WAIT)
    dev_err(&pi.dev,
    "on26: Device reset failed (%x,%x)\n", x, y);
    w0(4); P1; w0(4); P1;
    }
    CCP(0x30);
    pi.delay = d;
    pi.mode = m;
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    return 5;
    }
#[no_mangle]
unsafe extern "C" fn on26_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void on26_read_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k, a, b;
    switch (pi.mode) {
    case 0:
    w0(1); P1; w0(1); P2; w0(2); P1; w0(0x18); P2; w0(0); P1;
    udelay(10);
    for (k = 0; k < count; k++) {
    w2(6); a = r1();
    w2(4); b = r1();
    buf[k] = j44(a, b);
    }
    w0(2); P1; w0(8); P2;
    break;
    case 1:
    w0(1); P1; w0(1); P2; w0(2); P1; w0(0x19); P2; w0(0); P1;
    udelay(10);
    for (k = 0; k < count / 2; k++) {
    w2(0x26); buf[2 * k] = r0();
    w2(0x24); buf[2 * k + 1] = r0();
    }
    w0(2); P1; w0(9); P2;
    break;
    case 2:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0x24);
    udelay(10);
    for (k = 0; k < count; k++)
    buf[k] = r4();
    w2(4);
    break;
    case 3:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0x24);
    udelay(10);
    for (k = 0; k < count / 2; k++)
    ((u16 *)buf)[k] = r4w();
    w2(4);
    break;
    case 4:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0x24);
    udelay(10);
    for (k = 0; k < count / 4; k++)
    ((u32 *)buf)[k] = r4l();
    w2(4);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn on26_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void on26_write_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k;
    switch (pi.mode) {
    case 0:
    case 1:
    w0(1); P1; w0(1); P2;
    w0(2); P1; w0(0x18 + pi.mode); P2; w0(0); P1;
    udelay(10);
    for (k = 0; k < count / 2; k++) {
    w2(5); w0(buf[2 * k]);
    w2(7); w0(buf[2 * k + 1]);
    }
    w2(5); w2(4);
    w0(2); P1; w0(8 + pi.mode); P2;
    break;
    case 2:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0xc5);
    udelay(10);
    for (k = 0; k < count; k++)
    w4(buf[k]);
    w2(0xc4);
    break;
    case 3:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0xc5);
    udelay(10);
    for (k = 0; k < count / 2; k++)
    w4w(((u16 *)buf)[k]);
    w2(0xc4);
    break;
    case 4:
    w3(1); w3(1); w2(5); w4(1); w2(4);
    w3(0); w3(0); w2(0xc5);
    udelay(10);
    for (k = 0; k < count / 4; k++)
    w4l(((u32 *)buf)[k]);
    w2(0xc4);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn on26_log_adapter(pi: *mut pi_adapter) {
    static void on26_log_adapter(struct pi_adapter *pi)
    {
    char *mode_string[5] = { "4-bit", "8-bit", "EPP-8", "EPP-16", "EPP-32" };
    dev_info(&pi.dev,
    "OnSpec 90c26 at 0x%x, mode %d (%s), delay %d\n",
    pi.port, pi.mode, mode_string[pi.mode], pi.delay);
    }
    static struct pi_protocol on26 = {
    .owner		= THIS_MODULE,
    .name		= "on26",
    .max_mode	= 5,
    .epp_first	= 2,
    .default_delay	= 1,
    .max_units	= 1,
    .write_regr	= on26_write_regr,
    .read_regr	= on26_read_regr,
    .write_block	= on26_write_block,
    .read_block	= on26_read_block,
    .connect	= on26_connect,
    .disconnect	= on26_disconnect,
    .test_port	= on26_test_port,
    .log_adapter	= on26_log_adapter,
    };
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
    MODULE_DESCRIPTION("Onspec 90c26 parallel port IDE adapter protocol driver");
    module_pata_parport_driver(on26);
