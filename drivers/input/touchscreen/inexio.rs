//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/inexio.c
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
// iNexio serial touchscreen driver
//
// Copyright (c) 2008 Richard Lemon
// Based on the mtouch driver (c) Vojtech Pavlik and Dan Streetman
//
// 2008/06/19 Richard Lemon <richard@codelemon.com>
// Copied mtouch.c and edited for iNexio protocol
//

    MODULE_AUTHOR("Richard Lemon <richard@codelemon.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const INEXIO_FORMAT_TOUCH_BIT: c_uint = 0x01;
pub const INEXIO_FORMAT_LENGTH: c_int = 5;
pub const INEXIO_RESPONSE_BEGIN_BYTE: c_uint = 0x80;
// todo: check specs for max length of all responses
pub const INEXIO_MAX_LENGTH: c_int = 16;
pub const INEXIO_MIN_XC: c_int = 0;
pub const INEXIO_MAX_XC: c_uint = 0x3fff;
pub const INEXIO_MIN_YC: c_int = 0;
pub const INEXIO_MAX_YC: c_uint = 0x3fff;

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inexio {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; INEXIO_MAX_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn inexio_process_data(pinexio: *mut inexio) {
    static void inexio_process_data(struct inexio *pinexio)
    {
    struct input_dev *dev = pinexio.dev;
    if (INEXIO_FORMAT_LENGTH == ++pinexio.idx) {
    input_report_abs(dev, ABS_X, INEXIO_GET_XC(pinexio.data));
    input_report_abs(dev, ABS_Y, INEXIO_GET_YC(pinexio.data));
    input_report_key(dev, BTN_TOUCH, INEXIO_GET_TOUCHED(pinexio.data));
    input_sync(dev);
    pinexio.idx = 0;
    }
    }
    static irqreturn_t inexio_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct inexio *pinexio = serio_get_drvdata(serio);
    pinexio.data[pinexio.idx] = data;
    if (INEXIO_RESPONSE_BEGIN_BYTE&pinexio.data[0])
    inexio_process_data(pinexio);
    else
    dev_dbg(&serio.dev,
    "unknown/unsynchronized data from device, byte %x\n",
    pinexio.data[0]);
    return IRQ_HANDLED;
    }
//
// inexio_disconnect() is the opposite of inexio_connect()
//
#[no_mangle]
unsafe extern "C" fn inexio_disconnect(serio: *mut serio) {
    static void inexio_disconnect(struct serio *serio)
    {
    struct inexio *pinexio = serio_get_drvdata(serio);
    input_get_device(pinexio.dev);
    input_unregister_device(pinexio.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(pinexio.dev);
    kfree(pinexio);
    }
//
// inexio_connect() is the routine that is called when someone adds a
// new serio device that supports iNexio protocol and registers it as
// an input device. This is usually accomplished using inputattach.
//
#[no_mangle]
unsafe extern "C" fn inexio_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int inexio_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct inexio *pinexio;
    struct input_dev *input_dev;
    int err;
    pinexio = kzalloc_obj(*pinexio);
    input_dev = input_allocate_device();
    if (!pinexio || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    pinexio.serio = serio;
    pinexio.dev = input_dev;
    scnprintf(pinexio.phys, sizeof(pinexio.phys), "%s/input0", serio.phys);
    input_dev.name = "iNexio Serial TouchScreen";
    input_dev.phys = pinexio.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_INEXIO;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0001;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(pinexio.dev, ABS_X, INEXIO_MIN_XC, INEXIO_MAX_XC, 0, 0);
    input_set_abs_params(pinexio.dev, ABS_Y, INEXIO_MIN_YC, INEXIO_MAX_YC, 0, 0);
    serio_set_drvdata(serio, pinexio);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(pinexio.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(pinexio);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id inexio_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_INEXIO,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, inexio_serio_ids);
    static struct serio_driver inexio_drv = {
    .driver		= {
    .name	= "inexio",
    },
    .description	= DRIVER_DESC,
    .id_table	= inexio_serio_ids,
    .interrupt	= inexio_interrupt,
    .connect	= inexio_connect,
    .disconnect	= inexio_disconnect,
    };
    module_serio_driver(inexio_drv);
