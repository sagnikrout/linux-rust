//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_parport/epia.c
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
// epia.c is a low-level protocol driver for Shuttle Technologies
// EPIA parallel to IDE adapter chip.  This device is now obsolete
// and has been replaced with the EPAT chip, which is supported
// by epat.c, however, some devices based on EPIA are still
// available.
//

//
// mode codes:  0  nybble reads on port 1, 8-bit writes
// 1  5/3 reads on ports 1 & 2, 8-bit writes
// 2  8-bit reads and writes
// 3  8-bit EPP mode
// 4  16-bit EPP
// 5  32-bit EPP
//

//
// cont =  0   IDE register file
// cont =  1   IDE control registers
//
    static int cont_map[2] = { 0, 0x80 };
#[no_mangle]
unsafe extern "C" fn epia_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    static int epia_read_regr(struct pi_adapter *pi, int cont, int regr)
    {
    int a, b, r;
    regr += cont_map[cont];
    switch (pi.mode)  {
    case 0:
    r = regr ^ 0x39;
    w0(r); w2(1); w2(3); w0(r);
    a = r1(); w2(1); b = r1(); w2(4);
    return j44(a, b);
    case 1:
    r = regr ^ 0x31;
    w0(r); w2(1); w0(r & 0x37);
    w2(3); w2(5); w0(r | 0xf0);
    a = r1(); b = r2(); w2(4);
    return j53(a, b);
    case 2:
    r = regr^0x29;
    w0(r); w2(1); w2(0X21); w2(0x23);
    a = r0(); w2(4);
    return a;
    case 3:
    case 4:
    case 5:
    w3(regr); w2(0x24); a = r4(); w2(4);
    return a;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn epia_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    static void epia_write_regr(struct pi_adapter *pi, int cont, int regr, int val)
    {
    int  r;
    regr += cont_map[cont];
    switch (pi.mode)  {
    case 0:
    case 1:
    case 2:
    r = regr ^ 0x19;
    w0(r); w2(1); w0(val); w2(3); w2(4);
    break;
    case 3:
    case 4:
    case 5:
    r = regr ^ 0x40;
    w3(r); w4(val); w2(4);
    break;
    }
    }

//
// The use of register 0x84 is entirely unclear - it seems to control
// some EPP counters ...  currently we know about 3 different block
// sizes:  the standard 512 byte reads and writes, 12 byte writes and
// 2048 byte reads (the last two being used in the CDrom drivers.
//
#[no_mangle]
unsafe extern "C" fn epia_connect(pi: *mut pi_adapter) {
    static void epia_connect(struct pi_adapter *pi)
    {
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    w2(4); w0(0xa0); w0(0x50); w0(0xc0); w0(0x30); w0(0xa0); w0(0);
    w2(1); w2(4);
    if (pi.mode >= 3) {
    w0(0xa); w2(1); w2(4); w0(0x82); w2(4); w2(0xc); w2(4);
    w2(0x24); w2(0x26); w2(4);
    }
    WR(0x86, 8);
    }
#[no_mangle]
unsafe extern "C" fn epia_disconnect(pi: *mut pi_adapter) {
    static void epia_disconnect(struct pi_adapter *pi)
    {
// WR(0x84,0x10);
    w0(pi.saved_r0);
    w2(1); w2(4);
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    }
#[no_mangle]
unsafe extern "C" fn epia_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void epia_read_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k, ph, a, b;
    switch (pi.mode) {
    case 0:
    w0(0x81); w2(1); w2(3); w0(0xc1);
    ph = 1;
    for (k = 0; k < count; k++) {
    w2(2+ph); a = r1();
    w2(4+ph); b = r1();
    buf[k] = j44(a, b);
    ph = 1 - ph;
    }
    w0(0); w2(4);
    break;
    case 1:
    w0(0x91); w2(1); w0(0x10); w2(3);
    w0(0x51); w2(5); w0(0xd1);
    ph = 1;
    for (k = 0; k < count; k++) {
    w2(4 + ph);
    a = r1(); b = r2();
    buf[k] = j53(a, b);
    ph = 1 - ph;
    }
    w0(0); w2(4);
    break;
    case 2:
    w0(0x89); w2(1); w2(0x23); w2(0x21);
    ph = 1;
    for (k = 0; k < count; k++) {
    w2(0x24 + ph);
    buf[k] = r0();
    ph = 1 - ph;
    }
    w2(6); w2(4);
    break;
    case 3:
    if (count > 512)
    WR(0x84, 3);
    w3(0); w2(0x24);
    for (k = 0; k < count; k++)
    buf[k] = r4();
    w2(4); WR(0x84, 0);
    break;
    case 4:
    if (count > 512)
    WR(0x84, 3);
    w3(0); w2(0x24);
    for (k = 0; k < count / 2; k++)
    ((u16 *)buf)[k] = r4w();
    w2(4); WR(0x84, 0);
    break;
    case 5:
    if (count > 512)
    WR(0x84, 3);
    w3(0); w2(0x24);
    for (k = 0; k < count / 4; k++)
    ((u32 *)buf)[k] = r4l();
    w2(4); WR(0x84, 0);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn epia_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void epia_write_block(struct pi_adapter *pi, char *buf, int count)
    {
    int ph, k, last, d;
    switch (pi.mode) {
    case 0:
    case 1:
    case 2:
    w0(0xa1); w2(1); w2(3); w2(1); w2(5);
    ph = 0;  last = 0x8000;
    for (k = 0; k < count; k++) {
    d = buf[k];
    if (d != last) {
    last = d;
    w0(d);
    }
    w2(4 + ph);
    ph = 1 - ph;
    }
    w2(7); w2(4);
    break;
    case 3:
    if (count < 512)
    WR(0x84, 1);
    w3(0x40);
    for (k = 0; k < count; k++)
    w4(buf[k]);
    if (count < 512)
    WR(0x84, 0);
    break;
    case 4:
    if (count < 512)
    WR(0x84, 1);
    w3(0x40);
    for (k = 0; k < count / 2; k++)
    w4w(((u16 *)buf)[k]);
    if (count < 512)
    WR(0x84, 0);
    break;
    case 5:
    if (count < 512)
    WR(0x84, 1);
    w3(0x40);
    for (k = 0; k < count / 4; k++)
    w4l(((u32 *)buf)[k]);
    if (count < 512)
    WR(0x84, 0);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn epia_test_proto(pi: *mut pi_adapter) -> c_int {
    static int epia_test_proto(struct pi_adapter *pi)
    {
    int j, k, f;
    int e[2] = { 0, 0 };
    char scratch[512];
    epia_connect(pi);
    for (j = 0; j < 2; j++) {
    WR(6, 0xa0 + j * 0x10);
    for (k = 0; k < 256; k++) {
    WR(2, k ^ 0xaa);
    WR(3, k ^ 0x55);
    if (RR(2) != (k ^ 0xaa))
    e[j]++;
    }
    WR(2, 1); WR(3, 1);
    }
    epia_disconnect(pi);
    f = 0;
    epia_connect(pi);
    WR(0x84, 8);
    epia_read_block(pi, scratch, 512);
    for (k = 0; k < 256; k++) {
    if ((scratch[2 * k] & 0xff) != ((k + 1) & 0xff))
    f++;
    if ((scratch[2 * k + 1] & 0xff) != ((-2 - k) & 0xff))
    f++;
    }
    WR(0x84, 0);
    epia_disconnect(pi);
    dev_dbg(&pi.dev, "epia: port 0x%x, mode %d, test=(%d,%d,%d)\n",
    pi.port, pi.mode, e[0], e[1], f);
    return (e[0] && e[1]) || f;
    }
#[no_mangle]
unsafe extern "C" fn epia_log_adapter(pi: *mut pi_adapter) {
    static void epia_log_adapter(struct pi_adapter *pi)
    {
    char *mode[6] = { "4-bit", "5/3", "8-bit", "EPP-8", "EPP-16", "EPP-32"};
    dev_info(&pi.dev,
    "Shuttle EPIA at 0x%x, mode %d (%s), delay %d\n",
    pi.port, pi.mode, mode[pi.mode], pi.delay);
    }
    static struct pi_protocol epia = {
    .owner		= THIS_MODULE,
    .name		= "epia",
    .max_mode	= 6,
    .epp_first	= 3,
    .default_delay	= 1,
    .max_units	= 1,
    .write_regr	= epia_write_regr,
    .read_regr	= epia_read_regr,
    .write_block	= epia_write_block,
    .read_block	= epia_read_block,
    .connect	= epia_connect,
    .disconnect	= epia_disconnect,
    .test_proto	= epia_test_proto,
    .log_adapter	= epia_log_adapter,
    };
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
    MODULE_DESCRIPTION("Shuttle Technologies EPIA parallel port IDE adapter "
    "protocol driver");
    module_pata_parport_driver(epia);
