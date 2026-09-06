//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/touchright.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Touchright serial touchscreen driver
//
// Copyright (c) 2006 Rick Koch <n1gp@hotmail.com>
//
// Based on MicroTouch driver (drivers/input/touchscreen/mtouch.c)
// Copyright (c) 2004 Vojtech Pavlik
// and Dan Streetman <ddstreet@ieee.org>
//

    MODULE_AUTHOR("Rick Koch <n1gp@hotmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const TR_FORMAT_TOUCH_BIT: c_uint = 0x01;
pub const TR_FORMAT_STATUS_BYTE: c_uint = 0x40;

pub const TR_LENGTH: c_int = 5;
pub const TR_MIN_XC: c_int = 0;
pub const TR_MAX_XC: c_uint = 0x1ff;
pub const TR_MIN_YC: c_int = 0;
pub const TR_MAX_YC: c_uint = 0x1ff;
//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tr {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; TR_LENGTH],
    pub phys: [c_char; 32],
}

    static irqreturn_t tr_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct tr *tr = serio_get_drvdata(serio);
    struct input_dev *dev = tr.dev;
    tr.data[tr.idx] = data;
    if ((tr.data[0] & TR_FORMAT_STATUS_MASK) == TR_FORMAT_STATUS_BYTE) {
    if (++tr.idx == TR_LENGTH) {
    input_report_abs(dev, ABS_X,
    (tr.data[1] << 5) | (tr.data[2] >> 1));
    input_report_abs(dev, ABS_Y,
    (tr.data[3] << 5) | (tr.data[4] >> 1));
    input_report_key(dev, BTN_TOUCH,
    tr.data[0] & TR_FORMAT_TOUCH_BIT);
    input_sync(dev);
    tr.idx = 0;
    }
    }
    return IRQ_HANDLED;
    }
//
// tr_disconnect() is the opposite of tr_connect()
//
#[no_mangle]
unsafe extern "C" fn tr_disconnect(serio: *mut serio) {
    static void tr_disconnect(struct serio *serio)
    {
    struct tr *tr = serio_get_drvdata(serio);
    input_get_device(tr.dev);
    input_unregister_device(tr.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(tr.dev);
    kfree(tr);
    }
//
// tr_connect() is the routine that is called when someone adds a
// new serio device that supports the Touchright protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn tr_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int tr_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct tr *tr;
    struct input_dev *input_dev;
    int err;
    tr = kzalloc_obj(*tr);
    input_dev = input_allocate_device();
    if (!tr || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    tr.serio = serio;
    tr.dev = input_dev;
    scnprintf(tr.phys, sizeof(tr.phys), "%s/input0", serio.phys);
    input_dev.name = "Touchright Serial TouchScreen";
    input_dev.phys = tr.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_TOUCHRIGHT;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(tr.dev, ABS_X, TR_MIN_XC, TR_MAX_XC, 0, 0);
    input_set_abs_params(tr.dev, ABS_Y, TR_MIN_YC, TR_MAX_YC, 0, 0);
    serio_set_drvdata(serio, tr);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(tr.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(tr);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id tr_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_TOUCHRIGHT,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, tr_serio_ids);
    static struct serio_driver tr_drv = {
    .driver		= {
    .name	= "touchright",
    },
    .description	= DRIVER_DESC,
    .id_table	= tr_serio_ids,
    .interrupt	= tr_interrupt,
    .connect	= tr_connect,
    .disconnect	= tr_disconnect,
    };
    module_serio_driver(tr_drv);
