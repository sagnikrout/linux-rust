//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/dynapro.c
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
// Dynapro serial touchscreen driver
//
// Copyright (c) 2009 Tias Guns
// Based on the inexio driver (c) Vojtech Pavlik and Dan Streetman and
// Richard Lemon
//
// 2009/09/19 Tias Guns <tias@ulyssis.org>
// Copied inexio.c and edited for Dynapro protocol (from retired Xorg module)
//

    MODULE_AUTHOR("Tias Guns <tias@ulyssis.org>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const DYNAPRO_FORMAT_TOUCH_BIT: c_uint = 0x40;
pub const DYNAPRO_FORMAT_LENGTH: c_int = 3;
pub const DYNAPRO_RESPONSE_BEGIN_BYTE: c_uint = 0x80;
pub const DYNAPRO_MIN_XC: c_int = 0;
pub const DYNAPRO_MAX_XC: c_uint = 0x3ff;
pub const DYNAPRO_MIN_YC: c_int = 0;
pub const DYNAPRO_MAX_YC: c_uint = 0x3ff;

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynapro {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; DYNAPRO_FORMAT_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn dynapro_process_data(pdynapro: *mut dynapro) {
    static void dynapro_process_data(struct dynapro *pdynapro)
    {
    struct input_dev *dev = pdynapro.dev;
    if (DYNAPRO_FORMAT_LENGTH == ++pdynapro.idx) {
    input_report_abs(dev, ABS_X, DYNAPRO_GET_XC(pdynapro.data));
    input_report_abs(dev, ABS_Y, DYNAPRO_GET_YC(pdynapro.data));
    input_report_key(dev, BTN_TOUCH,
    DYNAPRO_GET_TOUCHED(pdynapro.data));
    input_sync(dev);
    pdynapro.idx = 0;
    }
    }
    static irqreturn_t dynapro_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct dynapro *pdynapro = serio_get_drvdata(serio);
    pdynapro.data[pdynapro.idx] = data;
    if (DYNAPRO_RESPONSE_BEGIN_BYTE & pdynapro.data[0])
    dynapro_process_data(pdynapro);
    else
    dev_dbg(&serio.dev, "unknown/unsynchronized data: %x\n",
    pdynapro.data[0]);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dynapro_disconnect(serio: *mut serio) {
    static void dynapro_disconnect(struct serio *serio)
    {
    struct dynapro *pdynapro = serio_get_drvdata(serio);
    input_get_device(pdynapro.dev);
    input_unregister_device(pdynapro.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(pdynapro.dev);
    kfree(pdynapro);
    }
//
// dynapro_connect() is the routine that is called when someone adds a
// new serio device that supports dynapro protocol and registers it as
// an input device. This is usually accomplished using inputattach.
//
#[no_mangle]
unsafe extern "C" fn dynapro_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int dynapro_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct dynapro *pdynapro;
    struct input_dev *input_dev;
    int err;
    pdynapro = kzalloc_obj(*pdynapro);
    input_dev = input_allocate_device();
    if (!pdynapro || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    pdynapro.serio = serio;
    pdynapro.dev = input_dev;
    scnprintf(pdynapro.phys, sizeof(pdynapro.phys),
    "%s/input0", serio.phys);
    input_dev.name = "Dynapro Serial TouchScreen";
    input_dev.phys = pdynapro.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_DYNAPRO;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0001;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    input_set_abs_params(pdynapro.dev, ABS_X,
    DYNAPRO_MIN_XC, DYNAPRO_MAX_XC, 0, 0);
    input_set_abs_params(pdynapro.dev, ABS_Y,
    DYNAPRO_MIN_YC, DYNAPRO_MAX_YC, 0, 0);
    serio_set_drvdata(serio, pdynapro);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(pdynapro.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(pdynapro);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id dynapro_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_DYNAPRO,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, dynapro_serio_ids);
    static struct serio_driver dynapro_drv = {
    .driver		= {
    .name	= "dynapro",
    },
    .description	= DRIVER_DESC,
    .id_table	= dynapro_serio_ids,
    .interrupt	= dynapro_interrupt,
    .connect	= dynapro_connect,
    .disconnect	= dynapro_disconnect,
    };
    module_serio_driver(dynapro_drv);
