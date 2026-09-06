//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cx88/cx88-i2c.c
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
// cx88-i2c.c  --  all the i2c code is here
//
// Copyright (C) 1996,97,98 Ralph  Metzler (rjkm@thp.uni-koeln.de)
// & Marcus Metzler (mocm@thp.uni-koeln.de)
// (c) 2002 Yurij Sysoev <yurij@naturesoft.net>
// (c) 1999-2003 Gerd Knorr <kraxel@bytesex.org>
// (c) 2005 Mauro Carvalho Chehab <mchehab@kernel.org>
// - Multituner support and i2c address binding
//

    static unsigned int i2c_debug;
    module_param(i2c_debug, int, 0644);
    MODULE_PARM_DESC(i2c_debug, "enable debug messages [i2c]");
    static unsigned int i2c_scan;
    module_param(i2c_scan, int, 0444);
    MODULE_PARM_DESC(i2c_scan, "scan i2c bus at insmod time");
    let mut i2c_udelay: static unsigned int = 5;
    module_param(i2c_udelay, int, 0644);
    MODULE_PARM_DESC(i2c_udelay,
    "i2c delay at insmod time, in usecs (should be 5 or higher). Lower value means higher bus speed.");

    if (i2c_debug >= level)						\
    printk(KERN_DEBUG pr_fmt("%s: i2c:" fmt),		\
    __func__, ##arg);				\
    } while (0)
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn cx8800_bit_setscl(data: *mut c_void, state: c_int) {
    static void cx8800_bit_setscl(void *data, int state)
    {
    struct cx88_core *core = data;
    if (state)
    core.i2c_state |= 0x02;
    else
    core.i2c_state &= ~0x02;
    cx_write(MO_I2C, core.i2c_state);
    cx_read(MO_I2C);
    }
#[no_mangle]
unsafe extern "C" fn cx8800_bit_setsda(data: *mut c_void, state: c_int) {
    static void cx8800_bit_setsda(void *data, int state)
    {
    struct cx88_core *core = data;
    if (state)
    core.i2c_state |= 0x01;
    else
    core.i2c_state &= ~0x01;
    cx_write(MO_I2C, core.i2c_state);
    cx_read(MO_I2C);
    }
#[no_mangle]
unsafe extern "C" fn cx8800_bit_getscl(data: *mut c_void) -> c_int {
    static int cx8800_bit_getscl(void *data)
    {
    struct cx88_core *core = data;
    u32 state;
    state = cx_read(MO_I2C);
    return state & 0x02 ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn cx8800_bit_getsda(data: *mut c_void) -> c_int {
    static int cx8800_bit_getsda(void *data)
    {
    struct cx88_core *core = data;
    u32 state;
    state = cx_read(MO_I2C);
    return state & 0x01;
    }
// -----------------------------------------------------------------------
    static const struct i2c_algo_bit_data cx8800_i2c_algo_template = {
    .setsda  = cx8800_bit_setsda,
    .setscl  = cx8800_bit_setscl,
    .getsda  = cx8800_bit_getsda,
    .getscl  = cx8800_bit_getscl,
    .udelay  = 16,
    .timeout = 200,
    };
// -----------------------------------------------------------------------
    static const char * const i2c_devs[128] = {
    [0x1c >> 1] = "lgdt330x",
    [0x86 >> 1] = "tda9887/cx22702",
    [0xa0 >> 1] = "eeprom",
    [0xc0 >> 1] = "tuner (analog)",
    [0xc2 >> 1] = "tuner (analog/dvb)",
    [0xc8 >> 1] = "xc5000",
    };
#[no_mangle]
unsafe extern "C" fn do_i2c_scan(name: *const c_char, c: *mut i2c_client) {
    static void do_i2c_scan(const char *name, struct i2c_client *c)
    {
    unsigned char buf;
    int i, rc;
    for (i = 0; i < ARRAY_SIZE(i2c_devs); i++) {
    c.addr = i;
    rc = i2c_master_recv(c, &buf, 0);
    if (rc < 0)
    continue;
    pr_info("i2c scan: found device @ 0x%x  [%s]\n",
    i << 1, i2c_devs[i] ? i2c_devs[i] : "???");
    }
    }
// init + register i2c adapter
#[no_mangle]
pub unsafe extern "C" fn cx88_i2c_init(core: *mut cx88_core, pci: *mut pci_dev) -> c_int {
    int cx88_i2c_init(struct cx88_core *core, struct pci_dev *pci)
    {
// Prevents usage of invalid delay values
    if (i2c_udelay < 5)
    i2c_udelay = 5;
    core.i2c_algo = cx8800_i2c_algo_template;
    core.i2c_adap.dev.parent = &pci.dev;
    strscpy(core.i2c_adap.name, core.name, sizeof(core.i2c_adap.name));
    core.i2c_adap.owner = THIS_MODULE;
    core.i2c_algo.udelay = i2c_udelay;
    core.i2c_algo.data = core;
    i2c_set_adapdata(&core.i2c_adap, &core.v4l2_dev);
    core.i2c_adap.algo_data = &core.i2c_algo;
    core.i2c_client.adapter = &core.i2c_adap;
    strscpy(core.i2c_client.name, "cx88xx internal", I2C_NAME_SIZE);
    cx8800_bit_setscl(core, 1);
    cx8800_bit_setsda(core, 1);
    core.i2c_rc = i2c_bit_add_bus(&core.i2c_adap);
    if (core.i2c_rc == 0) {
    static u8 tuner_data[] = {
    0x0b, 0xdc, 0x86, 0x52 };
    static struct i2c_msg tuner_msg = {
    .flags = 0,
    .addr = 0xc2 >> 1,
    .buf = tuner_data,
    .len = 4
    };
    dprintk(1, "i2c register ok\n");
    switch (core.boardnr) {
    case CX88_BOARD_HAUPPAUGE_HVR1300:
    case CX88_BOARD_HAUPPAUGE_HVR3000:
    case CX88_BOARD_HAUPPAUGE_HVR4000:
    pr_info("i2c init: enabling analog demod on HVR1300/3000/4000 tuner\n");
    i2c_transfer(core.i2c_client.adapter, &tuner_msg, 1);
    break;
    default:
    break;
    }
    if (i2c_scan)
    do_i2c_scan(core.name, &core.i2c_client);
    } else
    pr_err("i2c register FAILED\n");
    return core.i2c_rc;
    }
