//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/xtkbd.c
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
// Copyright (c) 1999-2001 Vojtech Pavlik
//
// XT keyboard driver for Linux
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
pub const XTKBD_EMUL0: c_uint = 0xe0;
pub const XTKBD_EMUL1: c_uint = 0xe1;
pub const XTKBD_KEY: c_uint = 0x7f;
pub const XTKBD_RELEASE: c_uint = 0x80;
    static unsigned char xtkbd_keycode[256] = {
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83,  0,  0,  0, 87, 88,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0, 87, 88,  0,  0,  0,  0,110,111,103,108,105,
    106
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtkbd {
    pub keycode: [c_uchar; 256],
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub phys: [c_char; 32],
}

    static irqreturn_t xtkbd_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct xtkbd *xtkbd = serio_get_drvdata(serio);
    switch (data) {
    case XTKBD_EMUL0:
    case XTKBD_EMUL1:
    break;
    default:
    if (xtkbd.keycode[data & XTKBD_KEY]) {
    input_report_key(xtkbd.dev, xtkbd.keycode[data & XTKBD_KEY], !(data & XTKBD_RELEASE));
    input_sync(xtkbd.dev);
    } else {
    printk(KERN_WARNING "xtkbd.c: Unknown key (scancode %#x) %s.\n",
    data & XTKBD_KEY, data & XTKBD_RELEASE ? "released" : "pressed");
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xtkbd_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int xtkbd_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct xtkbd *xtkbd;
    struct input_dev *input_dev;
    let mut err: c_int = -ENOMEM;
    int i;
    xtkbd = kmalloc_obj(*xtkbd);
    input_dev = input_allocate_device();
    if (!xtkbd || !input_dev)
    goto fail1;
    xtkbd.serio = serio;
    xtkbd.dev = input_dev;
    snprintf(xtkbd.phys, sizeof(xtkbd.phys), "%s/input0", serio.phys);
    memcpy(xtkbd.keycode, xtkbd_keycode, sizeof(xtkbd.keycode));
    input_dev.name = "XT Keyboard";
    input_dev.phys = xtkbd.phys;
    input_dev.id.bustype = BUS_XTKBD;
    input_dev.id.vendor  = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    input_dev.keycode = xtkbd.keycode;
    input_dev.keycodesize = sizeof(unsigned char);
    input_dev.keycodemax = ARRAY_SIZE(xtkbd_keycode);
    for (i = 0; i < 255; i++)
    set_bit(xtkbd.keycode[i], input_dev.keybit);
    clear_bit(0, input_dev.keybit);
    serio_set_drvdata(serio, xtkbd);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(xtkbd.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(xtkbd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xtkbd_disconnect(serio: *mut serio) {
    static void xtkbd_disconnect(struct serio *serio)
    {
    struct xtkbd *xtkbd = serio_get_drvdata(serio);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_unregister_device(xtkbd.dev);
    kfree(xtkbd);
    }
    static const struct serio_device_id xtkbd_serio_ids[] = {
    {
    .type	= SERIO_XT,
    .proto	= SERIO_ANY,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, xtkbd_serio_ids);
    static struct serio_driver xtkbd_drv = {
    .driver		= {
    .name	= "xtkbd",
    },
    .description	= DRIVER_DESC,
    .id_table	= xtkbd_serio_ids,
    .interrupt	= xtkbd_interrupt,
    .connect	= xtkbd_connect,
    .disconnect	= xtkbd_disconnect,
    };
    module_serio_driver(xtkbd_drv);
