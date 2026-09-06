//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/egalax_ts_serial.c
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
// EETI Egalax serial touchscreen driver
//
// Copyright (c) 2015 Zoltán Böszörményi <zboszor@pr.hu>
//
// based on the
//
// Hampshire serial touchscreen driver (Copyright (c) 2010 Adam Bennett)
//

//
// Definitions & global arrays.
//
pub const EGALAX_FORMAT_MAX_LENGTH: c_int = 6;

pub const EGALAX_FORMAT_RESOLUTION_MASK: c_uint = 0x06;
pub const EGALAX_MIN_XC: c_int = 0;
pub const EGALAX_MAX_XC: c_uint = 0x4000;
pub const EGALAX_MIN_YC: c_int = 0;
pub const EGALAX_MAX_YC: c_uint = 0x4000;
//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct egalax {
    pub input: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [u8; EGALAX_FORMAT_MAX_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn egalax_process_data(egalax: *mut egalax) {
    static void egalax_process_data(struct egalax *egalax)
    {
    struct input_dev *dev = egalax.input;
    u8 *data = egalax.data;
    u16 x, y;
    u8 shift;
    u8 mask;
    shift = 3 - ((data[0] & EGALAX_FORMAT_RESOLUTION_MASK) >> 1);
    mask = 0xff >> (shift + 1);
    x = (((u16)(data[1] & mask) << 7) | (data[2] & 0x7f)) << shift;
    y = (((u16)(data[3] & mask) << 7) | (data[4] & 0x7f)) << shift;
    input_report_key(dev, BTN_TOUCH, data[0] & EGALAX_FORMAT_TOUCH_BIT);
    input_report_abs(dev, ABS_X, x);
    input_report_abs(dev, ABS_Y, y);
    input_sync(dev);
    }
    static irqreturn_t egalax_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct egalax *egalax = serio_get_drvdata(serio);
    int pkt_len;
    egalax.data[egalax.idx++] = data;
    if (likely(egalax.data[0] & EGALAX_FORMAT_START_BIT)) {
    pkt_len = egalax.data[0] & EGALAX_FORMAT_PRESSURE_BIT ? 6 : 5;
    if (pkt_len == egalax.idx) {
    egalax_process_data(egalax);
    egalax.idx = 0;
    }
    } else {
    dev_dbg(&serio.dev, "unknown/unsynchronized data: %x\n",
    egalax.data[0]);
    egalax.idx = 0;
    }
    return IRQ_HANDLED;
    }
//
// egalax_connect() is the routine that is called when someone adds a
// new serio device that supports egalax protocol and registers it as
// an input device. This is usually accomplished using inputattach.
//
#[no_mangle]
unsafe extern "C" fn egalax_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int egalax_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct egalax *egalax;
    struct input_dev *input_dev;
    int error;
    egalax = kzalloc_obj(*egalax);
    input_dev = input_allocate_device();
    if (!egalax || !input_dev) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    egalax.serio = serio;
    egalax.input = input_dev;
    scnprintf(egalax.phys, sizeof(egalax.phys), "%s/input0", serio.phys);
    input_dev.name = "EETI eGalaxTouch Serial TouchScreen";
    input_dev.phys = egalax.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_EGALAX;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0001;
    input_dev.dev.parent = &serio.dev;
    input_set_capability(input_dev, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input_dev, ABS_X,
    EGALAX_MIN_XC, EGALAX_MAX_XC, 0, 0);
    input_set_abs_params(input_dev, ABS_Y,
    EGALAX_MIN_YC, EGALAX_MAX_YC, 0, 0);
    serio_set_drvdata(serio, egalax);
    error = serio_open(serio, drv);
    if (error)
    goto err_reset_drvdata;
    error = input_register_device(input_dev);
    if (error)
    goto err_close_serio;
    return 0;
    err_close_serio:
    serio_close(serio);
    err_reset_drvdata:
    serio_set_drvdata(serio, core::ptr::null_mut());
    err_free_mem:
    input_free_device(input_dev);
    kfree(egalax);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn egalax_disconnect(serio: *mut serio) {
    static void egalax_disconnect(struct serio *serio)
    {
    struct egalax *egalax = serio_get_drvdata(serio);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_unregister_device(egalax.input);
    kfree(egalax);
    }
//
// The serio driver structure.
//
    static const struct serio_device_id egalax_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_EGALAX,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, egalax_serio_ids);
    static struct serio_driver egalax_drv = {
    .driver		= {
    .name	= "egalax",
    },
    .description	= DRIVER_DESC,
    .id_table	= egalax_serio_ids,
    .interrupt	= egalax_interrupt,
    .connect	= egalax_connect,
    .disconnect	= egalax_disconnect,
    };
    module_serio_driver(egalax_drv);
    MODULE_AUTHOR("Zoltán Böszörményi <zboszor@pr.hu>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
