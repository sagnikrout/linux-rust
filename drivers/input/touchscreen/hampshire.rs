//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/hampshire.c
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
// Hampshire serial touchscreen driver
//
// Copyright (c) 2010 Adam Bennett
// Based on the dynapro driver (c) Tias Guns
//
// 2010/04/08 Adam Bennett <abennett72@gmail.com>
// Copied dynapro.c and edited for Hampshire 4-byte protocol
//

    MODULE_AUTHOR("Adam Bennett <abennett72@gmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const HAMPSHIRE_FORMAT_TOUCH_BIT: c_uint = 0x40;
pub const HAMPSHIRE_FORMAT_LENGTH: c_int = 4;
pub const HAMPSHIRE_RESPONSE_BEGIN_BYTE: c_uint = 0x80;
pub const HAMPSHIRE_MIN_XC: c_int = 0;
pub const HAMPSHIRE_MAX_XC: c_uint = 0x1000;
pub const HAMPSHIRE_MIN_YC: c_int = 0;
pub const HAMPSHIRE_MAX_YC: c_uint = 0x1000;

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hampshire {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; HAMPSHIRE_FORMAT_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn hampshire_process_data(phampshire: *mut hampshire) {
    static void hampshire_process_data(struct hampshire *phampshire)
    {
    struct input_dev *dev = phampshire.dev;
    if (HAMPSHIRE_FORMAT_LENGTH == ++phampshire.idx) {
    input_report_abs(dev, ABS_X, HAMPSHIRE_GET_XC(phampshire.data));
    input_report_abs(dev, ABS_Y, HAMPSHIRE_GET_YC(phampshire.data));
    input_report_key(dev, BTN_TOUCH,
    HAMPSHIRE_GET_TOUCHED(phampshire.data));
    input_sync(dev);
    phampshire.idx = 0;
    }
    }
    static irqreturn_t hampshire_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct hampshire *phampshire = serio_get_drvdata(serio);
    phampshire.data[phampshire.idx] = data;
    if (HAMPSHIRE_RESPONSE_BEGIN_BYTE & phampshire.data[0])
    hampshire_process_data(phampshire);
    else
    dev_dbg(&serio.dev, "unknown/unsynchronized data: %x\n",
    phampshire.data[0]);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hampshire_disconnect(serio: *mut serio) {
    static void hampshire_disconnect(struct serio *serio)
    {
    struct hampshire *phampshire = serio_get_drvdata(serio);
    input_get_device(phampshire.dev);
    input_unregister_device(phampshire.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(phampshire.dev);
    kfree(phampshire);
    }
//
// hampshire_connect() is the routine that is called when someone adds a
// new serio device that supports hampshire protocol and registers it as
// an input device. This is usually accomplished using inputattach.
//
#[no_mangle]
unsafe extern "C" fn hampshire_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int hampshire_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct hampshire *phampshire;
    struct input_dev *input_dev;
    int err;
    phampshire = kzalloc_obj(*phampshire);
    input_dev = input_allocate_device();
    if (!phampshire || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    phampshire.serio = serio;
    phampshire.dev = input_dev;
    scnprintf(phampshire.phys, sizeof(phampshire.phys),
    "%s/input0", serio.phys);
    input_dev.name = "Hampshire Serial TouchScreen";
    input_dev.phys = phampshire.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_HAMPSHIRE;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0001;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(phampshire.dev, ABS_X,
    HAMPSHIRE_MIN_XC, HAMPSHIRE_MAX_XC, 0, 0);
    input_set_abs_params(phampshire.dev, ABS_Y,
    HAMPSHIRE_MIN_YC, HAMPSHIRE_MAX_YC, 0, 0);
    serio_set_drvdata(serio, phampshire);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(phampshire.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(phampshire);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id hampshire_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_HAMPSHIRE,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, hampshire_serio_ids);
    static struct serio_driver hampshire_drv = {
    .driver		= {
    .name	= "hampshire",
    },
    .description	= DRIVER_DESC,
    .id_table	= hampshire_serio_ids,
    .interrupt	= hampshire_interrupt,
    .connect	= hampshire_connect,
    .disconnect	= hampshire_disconnect,
    };
    module_serio_driver(hampshire_drv);
