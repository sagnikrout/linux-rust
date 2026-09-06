//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/rpcmouse.c
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
// Acorn RiscPC mouse driver for Linux/ARM
//
// Copyright (c) 2000-2002 Vojtech Pavlik
// Copyright (C) 1996-2002 Russell King
//
// This handles the Acorn RiscPCs mouse.  We basically have a couple of
// hardware registers that track the sensor count for the X-Y movement and
// another register holding the button state.  On every VSYNC interrupt we read
// the complete state and then work out if something has changed.
//

    MODULE_AUTHOR("Vojtech Pavlik, Russell King");
    MODULE_DESCRIPTION("Acorn RiscPC mouse driver");
    MODULE_LICENSE("GPL");
    static short rpcmouse_lastx, rpcmouse_lasty;
    static struct input_dev *rpcmouse_dev;
#[no_mangle]
unsafe extern "C" fn rpcmouse_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rpcmouse_irq(int irq, void *dev_id)
    {
    struct input_dev *dev = dev_id;
    short x, y, dx, dy, b;
    x = (short) iomd_readl(IOMD_MOUSEX);
    y = (short) iomd_readl(IOMD_MOUSEY);
    b = (short) (__raw_readl(IOMEM(0xe0310000)) ^ 0x70);
    dx = x - rpcmouse_lastx;
    dy = y - rpcmouse_lasty;
    rpcmouse_lastx = x;
    rpcmouse_lasty = y;
    input_report_rel(dev, REL_X, dx);
    input_report_rel(dev, REL_Y, -dy);
    input_report_key(dev, BTN_LEFT,   b & 0x40);
    input_report_key(dev, BTN_MIDDLE, b & 0x20);
    input_report_key(dev, BTN_RIGHT,  b & 0x10);
    input_sync(dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rpcmouse_init() -> int __init {
    static int __init rpcmouse_init(void)
    {
    int err;
    rpcmouse_dev = input_allocate_device();
    if (!rpcmouse_dev)
    return -ENOMEM;
    rpcmouse_dev.name = "Acorn RiscPC Mouse";
    rpcmouse_dev.phys = "rpcmouse/input0";
    rpcmouse_dev.id.bustype = BUS_HOST;
    rpcmouse_dev.id.vendor  = 0x0005;
    rpcmouse_dev.id.product = 0x0001;
    rpcmouse_dev.id.version = 0x0100;
    rpcmouse_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REL);
    rpcmouse_dev.keybit[BIT_WORD(BTN_LEFT)] = BIT_MASK(BTN_LEFT) |
    BIT_MASK(BTN_MIDDLE) | BIT_MASK(BTN_RIGHT);
    rpcmouse_dev.relbit[0]	= BIT_MASK(REL_X) | BIT_MASK(REL_Y);
    rpcmouse_lastx = (short) iomd_readl(IOMD_MOUSEX);
    rpcmouse_lasty = (short) iomd_readl(IOMD_MOUSEY);
    if (request_irq(IRQ_VSYNCPULSE, rpcmouse_irq, IRQF_SHARED, "rpcmouse", rpcmouse_dev)) {
    printk(KERN_ERR "rpcmouse: unable to allocate VSYNC interrupt\n");
    err = -EBUSY;
    goto err_free_dev;
    }
    err = input_register_device(rpcmouse_dev);
    if (err)
    goto err_free_irq;
    return 0;
    err_free_irq:
    free_irq(IRQ_VSYNCPULSE, rpcmouse_dev);
    err_free_dev:
    input_free_device(rpcmouse_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rpcmouse_exit() -> void __exit {
    static void __exit rpcmouse_exit(void)
    {
    free_irq(IRQ_VSYNCPULSE, rpcmouse_dev);
    input_unregister_device(rpcmouse_dev);
    }
    module_init(rpcmouse_init);
    module_exit(rpcmouse_exit);
