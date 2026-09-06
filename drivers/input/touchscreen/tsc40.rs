//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/tsc40.c
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
// TSC-40 serial touchscreen driver. It should be compatible with
// TSC-10 and 25.
//
// Author: Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

pub const PACKET_LENGTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsc_ser {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: u32,
    pub data: [c_uchar; PACKET_LENGTH],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn tsc_process_data(ptsc: *mut tsc_ser) {
    static void tsc_process_data(struct tsc_ser *ptsc)
    {
    struct input_dev *dev = ptsc.dev;
    u8 *data = ptsc.data;
    u32 x;
    u32 y;
    x = ((data[1] & 0x03) << 8) | data[2];
    y = ((data[3] & 0x03) << 8) | data[4];
    input_report_abs(dev, ABS_X, x);
    input_report_abs(dev, ABS_Y, y);
    input_report_key(dev, BTN_TOUCH, 1);
    input_sync(dev);
    }
    static irqreturn_t tsc_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct tsc_ser *ptsc = serio_get_drvdata(serio);
    struct input_dev *dev = ptsc.dev;
    ptsc.data[ptsc.idx] = data;
    switch (ptsc.idx++) {
    case 0:
    if (unlikely((data & 0x3e) != 0x10)) {
    dev_dbg(&serio.dev,
    "unsynchronized packet start (0x%02x)\n", data);
    ptsc.idx = 0;
    } else if (!(data & 0x01)) {
    input_report_key(dev, BTN_TOUCH, 0);
    input_sync(dev);
    ptsc.idx = 0;
    }
    break;
    case 1:
    case 3:
    if (unlikely(data & 0xfc)) {
    dev_dbg(&serio.dev,
    "unsynchronized data 0x%02x at offset %d\n",
    data, ptsc.idx - 1);
    ptsc.idx = 0;
    }
    break;
    case 4:
    tsc_process_data(ptsc);
    ptsc.idx = 0;
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tsc_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int tsc_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct tsc_ser *ptsc;
    struct input_dev *input_dev;
    int error;
    ptsc = kzalloc_obj(*ptsc);
    input_dev = input_allocate_device();
    if (!ptsc || !input_dev) {
    error = -ENOMEM;
    goto fail1;
    }
    ptsc.serio = serio;
    ptsc.dev = input_dev;
    scnprintf(ptsc.phys, sizeof(ptsc.phys), "%s/input0", serio.phys);
    input_dev.name = "TSC-10/25/40 Serial TouchScreen";
    input_dev.phys = ptsc.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_TSC40;
    input_dev.id.product = 40;
    input_dev.id.version = 0x0001;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    input_set_abs_params(ptsc.dev, ABS_X, 0, 0x3ff, 0, 0);
    input_set_abs_params(ptsc.dev, ABS_Y, 0, 0x3ff, 0, 0);
    serio_set_drvdata(serio, ptsc);
    error = serio_open(serio, drv);
    if (error)
    goto fail2;
    error = input_register_device(ptsc.dev);
    if (error)
    goto fail3;
    return 0;
    fail3:
    serio_close(serio);
    fail2:
    serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:
    input_free_device(input_dev);
    kfree(ptsc);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn tsc_disconnect(serio: *mut serio) {
    static void tsc_disconnect(struct serio *serio)
    {
    struct tsc_ser *ptsc = serio_get_drvdata(serio);
    serio_close(serio);
    input_unregister_device(ptsc.dev);
    kfree(ptsc);
    serio_set_drvdata(serio, core::ptr::null_mut());
    }
    static const struct serio_device_id tsc_serio_ids[] = {
    {
    .type   = SERIO_RS232,
    .proto  = SERIO_TSC40,
    .id     = SERIO_ANY,
    .extra  = SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, tsc_serio_ids);

    static struct serio_driver tsc_drv = {
    .driver	= {
    .name   = "tsc40",
    },
    .description    = DRIVER_DESC,
    .id_table	= tsc_serio_ids,
    .interrupt      = tsc_interrupt,
    .connect	= tsc_connect,
    .disconnect     = tsc_disconnect,
    };
    module_serio_driver(tsc_drv);
    MODULE_AUTHOR("Sebastian Andrzej Siewior <bigeasy@linutronix.de>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
