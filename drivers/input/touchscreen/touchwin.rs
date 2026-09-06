//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/touchwin.c
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
// Touchwindow serial touchscreen driver
//
// Copyright (c) 2006 Rick Koch <n1gp@hotmail.com>
//
// Based on MicroTouch driver (drivers/input/touchscreen/mtouch.c)
// Copyright (c) 2004 Vojtech Pavlik
// and Dan Streetman <ddstreet@ieee.org>
//
// 2005/02/19 Rick Koch:
// The Touchwindow I used is made by Edmark Corp. and
// constantly outputs a stream of 0's unless it is touched.
// It then outputs 3 bytes: X, Y, and a copy of Y.
//

    MODULE_AUTHOR("Rick Koch <n1gp@hotmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const TW_LENGTH: c_int = 3;
pub const TW_MIN_XC: c_int = 0;
pub const TW_MAX_XC: c_uint = 0xff;
pub const TW_MIN_YC: c_int = 0;
pub const TW_MAX_YC: c_uint = 0xff;
//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub touched: c_int,
    pub data: [c_uchar; TW_LENGTH],
    pub phys: [c_char; 32],
}

    static irqreturn_t tw_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct tw *tw = serio_get_drvdata(serio);
    struct input_dev *dev = tw.dev;
    if (data) {		/* touch */
    tw.touched = 1;
    tw.data[tw.idx++] = data;
// a full packet ends the accumulation, valid or not
    if (tw.idx == TW_LENGTH) {
// report only if the two Y's are the same
    if (tw.data[1] == tw.data[2]) {
    input_report_abs(dev, ABS_X, tw.data[0]);
    input_report_abs(dev, ABS_Y, tw.data[1]);
    input_report_key(dev, BTN_TOUCH, 1);
    input_sync(dev);
    }
    tw.idx = 0;
    }
    } else if (tw.touched) {	/* untouch */
    input_report_key(dev, BTN_TOUCH, 0);
    input_sync(dev);
    tw.idx = 0;
    tw.touched = 0;
    }
    return IRQ_HANDLED;
    }
//
// tw_disconnect() is the opposite of tw_connect()
//
#[no_mangle]
unsafe extern "C" fn tw_disconnect(serio: *mut serio) {
    static void tw_disconnect(struct serio *serio)
    {
    struct tw *tw = serio_get_drvdata(serio);
    input_get_device(tw.dev);
    input_unregister_device(tw.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(tw.dev);
    kfree(tw);
    }
//
// tw_connect() is the routine that is called when someone adds a
// new serio device that supports the Touchwin protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn tw_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int tw_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct tw *tw;
    struct input_dev *input_dev;
    int err;
    tw = kzalloc_obj(*tw);
    input_dev = input_allocate_device();
    if (!tw || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    tw.serio = serio;
    tw.dev = input_dev;
    scnprintf(tw.phys, sizeof(tw.phys), "%s/input0", serio.phys);
    input_dev.name = "Touchwindow Serial TouchScreen";
    input_dev.phys = tw.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_TOUCHWIN;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(tw.dev, ABS_X, TW_MIN_XC, TW_MAX_XC, 0, 0);
    input_set_abs_params(tw.dev, ABS_Y, TW_MIN_YC, TW_MAX_YC, 0, 0);
    serio_set_drvdata(serio, tw);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(tw.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(tw);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id tw_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_TOUCHWIN,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, tw_serio_ids);
    static struct serio_driver tw_drv = {
    .driver		= {
    .name	= "touchwin",
    },
    .description	= DRIVER_DESC,
    .id_table	= tw_serio_ids,
    .interrupt	= tw_interrupt,
    .connect	= tw_connect,
    .disconnect	= tw_disconnect,
    };
    module_serio_driver(tw_drv);
