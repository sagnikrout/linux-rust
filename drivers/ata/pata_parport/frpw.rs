//! Automatically rewritten from C to Rust
//! Source: drivers/ata/pata_parport/frpw.c
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
// frpw.c is a low-level protocol driver for the Freecom "Power" parallel port
// IDE adapter.
//
// Some applications of this adapter may require a "printer" reset prior to
// loading the driver.  This can be done by loading and unloading the "lp"
// driver, or it can be done by this driver if you define FRPW_HARD_RESET.
// The latter is not recommended as it may upset devices on other ports.
//

//
// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set
//
    static int  cont_map[2] = { 0x08, 0x10 };
#[no_mangle]
unsafe extern "C" fn frpw_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    static int frpw_read_regr(struct pi_adapter *pi, int cont, int regr)
    {
    int h, l, r;
    r = regr + cont_map[cont];
    w2(4);
    w0(r); cec4;
    w2(6); l = r1();
    w2(4); h = r1();
    w2(4);
    return j44(l, h);
    }
#[no_mangle]
unsafe extern "C" fn frpw_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    static void frpw_write_regr(struct pi_adapter *pi, int cont, int regr, int val)
    {
    let mut r: c_int = regr + cont_map[cont];
    w2(4); w0(r); cec4;
    w0(val);
    w2(5); w2(7); w2(5); w2(4);
    }
    static void frpw_read_block_int(struct pi_adapter *pi, char *buf, int count,
    int regr)
    {
    int h, l, k, ph;
    switch (pi.mode) {
    case 0:
    w2(4); w0(regr); cec4;
    for (k = 0; k < count; k++) {
    w2(6); l = r1();
    w2(4); h = r1();
    buf[k] = j44(l, h);
    }
    w2(4);
    break;
    case 1:
    ph = 2;
    w2(4); w0(regr + 0xc0); cec4;
    w0(0xff);
    for (k = 0; k < count; k++) {
    w2(0xa4 + ph);
    buf[k] = r0();
    ph = 2 - ph;
    }
    w2(0xac); w2(0xa4); w2(4);
    break;
    case 2:
    w2(4); w0(regr + 0x80); cec4;
    for (k = 0; k < count; k++)
    buf[k] = r4();
    w2(0xac); w2(0xa4);
    w2(4);
    break;
    case 3:
    w2(4); w0(regr + 0x80); cec4;
    for (k = 0; k < count - 2; k++)
    buf[k] = r4();
    w2(0xac); w2(0xa4);
    buf[count - 2] = r4();
    buf[count - 1] = r4();
    w2(4);
    break;
    case 4:
    w2(4); w0(regr + 0x80); cec4;
    for (k = 0; k < count / 2 - 1; k++)
    ((u16 *)buf)[k] = r4w();
    w2(0xac); w2(0xa4);
    buf[count - 2] = r4();
    buf[count - 1] = r4();
    w2(4);
    break;
    case 5:
    w2(4); w0(regr + 0x80); cec4;
    for (k = 0; k < count / 4 - 1; k++)
    ((u32 *)buf)[k] = r4l();
    buf[count - 4] = r4();
    buf[count - 3] = r4();
    w2(0xac); w2(0xa4);
    buf[count - 2] = r4();
    buf[count - 1] = r4();
    w2(4);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn frpw_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void frpw_read_block(struct pi_adapter *pi, char *buf, int count)
    {
    frpw_read_block_int(pi, buf, count, 0x08);
    }
#[no_mangle]
unsafe extern "C" fn frpw_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    static void frpw_write_block(struct pi_adapter *pi, char *buf, int count)
    {
    int k;
    switch (pi.mode) {
    case 0:
    case 1:
    case 2:
    w2(4); w0(8); cec4; w2(5);
    for (k = 0; k < count; k++) {
    w0(buf[k]);
    w2(7); w2(5);
    }
    w2(4);
    break;
    case 3:
    w2(4); w0(0xc8); cec4; w2(5);
    for (k = 0; k < count; k++)
    w4(buf[k]);
    w2(4);
    break;
    case 4:
    w2(4); w0(0xc8); cec4; w2(5);
    for (k = 0; k < count / 2; k++)
    w4w(((u16 *)buf)[k]);
    w2(4);
    break;
    case 5:
    w2(4); w0(0xc8); cec4; w2(5);
    for (k = 0; k < count / 4; k++)
    w4l(((u32 *)buf)[k]);
    w2(4);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn frpw_connect(pi: *mut pi_adapter) {
    static void frpw_connect(struct pi_adapter *pi)
    {
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    w2(4);
    }
#[no_mangle]
unsafe extern "C" fn frpw_disconnect(pi: *mut pi_adapter) {
    static void frpw_disconnect(struct pi_adapter *pi)
    {
    w2(4); w0(0x20); cec4;
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    }
//
// Stub logic to see if PNP string is available - used to distinguish
// between the Xilinx and ASIC implementations of the Freecom adapter.
// returns chip_type:   0 = Xilinx, 1 = ASIC
//
#[no_mangle]
unsafe extern "C" fn frpw_test_pnp(pi: *mut pi_adapter) -> c_int {
    static int frpw_test_pnp(struct pi_adapter *pi)
    {
    int olddelay, a, b;

    w0(0); w2(8); udelay(50); w2(0xc);   /* parallel bus reset */
    mdelay(1500);

    olddelay = pi.delay;
    pi.delay = 10;
    pi.saved_r0 = r0();
    pi.saved_r2 = r2();
    w2(4); w0(4); w2(6); w2(7);
    a = r1() & 0xff; w2(4); b = r1() & 0xff;
    w2(0xc); w2(0xe); w2(4);
    pi.delay = olddelay;
    w0(pi.saved_r0);
    w2(pi.saved_r2);
    return ((~a & 0x40) && (b & 0x40));
    }
//
// We use the pi->private to remember the result of the PNP test.
// To make this work, private = port*2 + chip.  Yes, I know it's a hack :-(
//
#[no_mangle]
unsafe extern "C" fn frpw_test_proto(pi: *mut pi_adapter) -> c_int {
    static int frpw_test_proto(struct pi_adapter *pi)
    {
    int j, k, r;
    int e[2] = { 0, 0 };
    char scratch[512];
    if ((pi.private >> 1) != pi.port)
    pi.private = frpw_test_pnp(pi) + 2*pi.port;
    if (((pi.private & 0x1) == 0) && (pi.mode > 2)) {
    dev_dbg(&pi.dev,
    "frpw: Xilinx does not support mode %d\n", pi.mode);
    return 1;
    }
    if (((pi.private & 0x1) == 1) && (pi.mode == 2)) {
    dev_dbg(&pi.dev, "frpw: ASIC does not support mode 2\n");
    return 1;
    }
    frpw_connect(pi);
    for (j = 0; j < 2; j++) {
    frpw_write_regr(pi, 0, 6, 0xa0 + j * 0x10);
    for (k = 0; k < 256; k++) {
    frpw_write_regr(pi, 0, 2, k ^ 0xaa);
    frpw_write_regr(pi, 0, 3, k ^ 0x55);
    if (frpw_read_regr(pi, 0, 2) != (k ^ 0xaa))
    e[j]++;
    }
    }
    frpw_disconnect(pi);
    frpw_connect(pi);
    frpw_read_block_int(pi, scratch, 512, 0x10);
    r = 0;
    for (k = 0; k < 128; k++) {
    if (scratch[k] != k)
    r++;
    }
    frpw_disconnect(pi);
    dev_dbg(&pi.dev,
    "frpw: port 0x%x, chip %ld, mode %d, test=(%d,%d,%d)\n",
    pi.port, (pi.private%2), pi.mode, e[0], e[1], r);
    return r || (e[0] && e[1]);
    }
#[no_mangle]
unsafe extern "C" fn frpw_log_adapter(pi: *mut pi_adapter) {
    static void frpw_log_adapter(struct pi_adapter *pi)
    {
    char *mode[6] = { "4-bit", "8-bit", "EPP", "EPP-8", "EPP-16", "EPP-32"};
    dev_info(&pi.dev,
    "Freecom (%s) adapter at 0x%x, mode %d (%s), delay %d\n",
    ((pi.private & 0x1) == 0) ? "Xilinx" : "ASIC",
    pi.port, pi.mode, mode[pi.mode], pi.delay);
    }
    static struct pi_protocol frpw = {
    .owner		= THIS_MODULE,
    .name		= "frpw",
    .max_mode	= 6,
    .epp_first	= 2,
    .default_delay	= 2,
    .max_units	= 1,
    .write_regr	= frpw_write_regr,
    .read_regr	= frpw_read_regr,
    .write_block	= frpw_write_block,
    .read_block	= frpw_read_block,
    .connect	= frpw_connect,
    .disconnect	= frpw_disconnect,
    .test_proto	= frpw_test_proto,
    .log_adapter	= frpw_log_adapter,
    };
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
    MODULE_DESCRIPTION("Freecom Power parallel port IDE adapter protocol driver");
    module_pata_parport_driver(frpw);
