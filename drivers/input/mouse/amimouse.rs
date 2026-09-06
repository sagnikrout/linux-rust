//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/amimouse.c
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
// Amiga mouse driver for Linux/m68k
//
// Copyright (c) 2000-2002 Vojtech Pavlik
//
// Based on the work of:
// Michael Rausch		James Banks
// Matther Dillon		David Giller
// Nathan Laredo		Linus Torvalds
// Johan Myreen		Jes Sorensen
// Russell King
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Amiga mouse driver");
    MODULE_LICENSE("GPL");
    static int amimouse_lastx, amimouse_lasty;
#[no_mangle]
unsafe extern "C" fn amimouse_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t amimouse_interrupt(int irq, void *data)
    {
    struct input_dev *dev = data;
    unsigned short joy0dat, potgor;
    int nx, ny, dx, dy;
    joy0dat = amiga_custom.joy0dat;
    nx = joy0dat & 0xff;
    ny = joy0dat >> 8;
    dx = nx - amimouse_lastx;
    dy = ny - amimouse_lasty;
    if (dx < -127) dx = (256 + nx) - amimouse_lastx;
    if (dx >  127) dx = (nx - 256) - amimouse_lastx;
    if (dy < -127) dy = (256 + ny) - amimouse_lasty;
    if (dy >  127) dy = (ny - 256) - amimouse_lasty;
    amimouse_lastx = nx;
    amimouse_lasty = ny;
    potgor = amiga_custom.potgor;
    input_report_rel(dev, REL_X, dx);
    input_report_rel(dev, REL_Y, dy);
    input_report_key(dev, BTN_LEFT,   ciaa.pra & 0x40);
    input_report_key(dev, BTN_MIDDLE, potgor & 0x0100);
    input_report_key(dev, BTN_RIGHT,  potgor & 0x0400);
    input_sync(dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn amimouse_open(dev: *mut input_dev) -> c_int {
    static int amimouse_open(struct input_dev *dev)
    {
    unsigned short joy0dat;
    int error;
    joy0dat = amiga_custom.joy0dat;
    amimouse_lastx = joy0dat & 0xff;
    amimouse_lasty = joy0dat >> 8;
    error = request_irq(IRQ_AMIGA_VERTB, amimouse_interrupt, 0, "amimouse",
    dev);
    if (error)
    dev_err(&dev.dev, "Can't allocate irq %d\n", IRQ_AMIGA_VERTB);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn amimouse_close(dev: *mut input_dev) {
    static void amimouse_close(struct input_dev *dev)
    {
    free_irq(IRQ_AMIGA_VERTB, dev);
    }
#[no_mangle]
unsafe extern "C" fn amimouse_probe(pdev: *mut platform_device) -> int __init {
    static int __init amimouse_probe(struct platform_device *pdev)
    {
    int err;
    struct input_dev *dev;
    dev = input_allocate_device();
    if (!dev)
    return -ENOMEM;
    dev.name = pdev.name;
    dev.phys = "amimouse/input0";
    dev.id.bustype = BUS_AMIGA;
    dev.id.vendor = 0x0001;
    dev.id.product = 0x0002;
    dev.id.version = 0x0100;
    dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REL);
    dev.relbit[0] = BIT_MASK(REL_X) | BIT_MASK(REL_Y);
    dev.keybit[BIT_WORD(BTN_LEFT)] = BIT_MASK(BTN_LEFT) |
    BIT_MASK(BTN_MIDDLE) | BIT_MASK(BTN_RIGHT);
    dev.open = amimouse_open;
    dev.close = amimouse_close;
    dev.dev.parent = &pdev.dev;
    err = input_register_device(dev);
    if (err) {
    input_free_device(dev);
    return err;
    }
    platform_set_drvdata(pdev, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amimouse_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit amimouse_remove(struct platform_device *pdev)
    {
    struct input_dev *dev = platform_get_drvdata(pdev);
    input_unregister_device(dev);
    }
//
// amimouse_remove() lives in .exit.text. For drivers registered via
// module_platform_driver_probe() this is ok because they cannot get unbound at
// runtime. So mark the driver struct with __refdata to prevent modpost
// triggering a section mismatch warning.
//
    static struct platform_driver amimouse_driver __refdata = {
    .remove = __exit_p(amimouse_remove),
    .driver   = {
    .name	= "amiga-mouse",
    },
    };
    module_platform_driver_probe(amimouse_driver, amimouse_probe);
    MODULE_ALIAS("platform:amiga-mouse");
