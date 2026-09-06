//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/mtouch.c
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
// MicroTouch (3M) serial touchscreen driver
//
// Copyright (c) 2004 Vojtech Pavlik
//
// 2005/02/19 Dan Streetman <ddstreet@ieee.org>
// Copied elo.c and edited for MicroTouch protocol
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const MTOUCH_FORMAT_TABLET_STATUS_BIT: c_uint = 0x80;
pub const MTOUCH_FORMAT_TABLET_TOUCH_BIT: c_uint = 0x40;
pub const MTOUCH_FORMAT_TABLET_LENGTH: c_int = 5;
pub const MTOUCH_RESPONSE_BEGIN_BYTE: c_uint = 0x01;
pub const MTOUCH_RESPONSE_END_BYTE: c_uint = 0x0d;
// todo: check specs for max length of all responses
pub const MTOUCH_MAX_LENGTH: c_int = 16;
pub const MTOUCH_MIN_XC: c_int = 0;
pub const MTOUCH_MAX_XC: c_uint = 0x3fff;
pub const MTOUCH_MIN_YC: c_int = 0;
pub const MTOUCH_MAX_YC: c_uint = 0x3fff;

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtouch {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; MTOUCH_MAX_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn mtouch_process_format_tablet(mtouch: *mut mtouch) {
    static void mtouch_process_format_tablet(struct mtouch *mtouch)
    {
    struct input_dev *dev = mtouch.dev;
    if (MTOUCH_FORMAT_TABLET_LENGTH == ++mtouch.idx) {
    input_report_abs(dev, ABS_X, MTOUCH_GET_XC(mtouch.data));
    input_report_abs(dev, ABS_Y, MTOUCH_MAX_YC - MTOUCH_GET_YC(mtouch.data));
    input_report_key(dev, BTN_TOUCH, MTOUCH_GET_TOUCHED(mtouch.data));
    input_sync(dev);
    mtouch.idx = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn mtouch_process_response(mtouch: *mut mtouch) {
    static void mtouch_process_response(struct mtouch *mtouch)
    {
    if (MTOUCH_RESPONSE_END_BYTE == mtouch.data[mtouch.idx++]) {
// FIXME - process response
    mtouch.idx = 0;
    } else if (MTOUCH_MAX_LENGTH == mtouch.idx) {
    printk(KERN_ERR "mtouch.c: too many response bytes\n");
    mtouch.idx = 0;
    }
    }
    static irqreturn_t mtouch_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct mtouch *mtouch = serio_get_drvdata(serio);
    mtouch.data[mtouch.idx] = data;
    if (MTOUCH_FORMAT_TABLET_STATUS_BIT & mtouch.data[0])
    mtouch_process_format_tablet(mtouch);
#[no_mangle]
pub unsafe extern "C" fn if(mtouch->data[0]: MTOUCH_RESPONSE_BEGIN_BYTE ==) -> else {
    else if (MTOUCH_RESPONSE_BEGIN_BYTE == mtouch.data[0])
    mtouch_process_response(mtouch);
    else
    printk(KERN_DEBUG "mtouch.c: unknown/unsynchronized data from device, byte %x\n",mtouch.data[0]);
    return IRQ_HANDLED;
    }
//
// mtouch_disconnect() is the opposite of mtouch_connect()
//
#[no_mangle]
unsafe extern "C" fn mtouch_disconnect(serio: *mut serio) {
    static void mtouch_disconnect(struct serio *serio)
    {
    struct mtouch *mtouch = serio_get_drvdata(serio);
    input_get_device(mtouch.dev);
    input_unregister_device(mtouch.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(mtouch.dev);
    kfree(mtouch);
    }
//
// mtouch_connect() is the routine that is called when someone adds a
// new serio device that supports MicroTouch (Format Tablet) protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn mtouch_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int mtouch_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct mtouch *mtouch;
    struct input_dev *input_dev;
    int err;
    mtouch = kzalloc_obj(*mtouch);
    input_dev = input_allocate_device();
    if (!mtouch || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    mtouch.serio = serio;
    mtouch.dev = input_dev;
    scnprintf(mtouch.phys, sizeof(mtouch.phys), "%s/input0", serio.phys);
    input_dev.name = "MicroTouch Serial TouchScreen";
    input_dev.phys = mtouch.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_MICROTOUCH;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(mtouch.dev, ABS_X, MTOUCH_MIN_XC, MTOUCH_MAX_XC, 0, 0);
    input_set_abs_params(mtouch.dev, ABS_Y, MTOUCH_MIN_YC, MTOUCH_MAX_YC, 0, 0);
    serio_set_drvdata(serio, mtouch);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(mtouch.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(mtouch);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id mtouch_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_MICROTOUCH,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, mtouch_serio_ids);
    static struct serio_driver mtouch_drv = {
    .driver		= {
    .name	= "mtouch",
    },
    .description	= DRIVER_DESC,
    .id_table	= mtouch_serio_ids,
    .interrupt	= mtouch_interrupt,
    .connect	= mtouch_connect,
    .disconnect	= mtouch_disconnect,
    };
    module_serio_driver(mtouch_drv);
