//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/m68kspkr.c
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
// m68k beeper driver for Linux
//
// Copyright (c) 2002 Richard Zidlicky
// Copyright (c) 2002 Vojtech Pavlik
// Copyright (c) 1992 Orest Zborowski
//

    MODULE_AUTHOR("Richard Zidlicky <rz@linux-m68k.org>");
    MODULE_DESCRIPTION("m68k beeper driver");
    MODULE_LICENSE("GPL");
    static struct platform_device *m68kspkr_platform_device;
#[no_mangle]
unsafe extern "C" fn m68kspkr_event(dev: *mut input_dev, type: c_uint, code: c_uint, value: c_int) -> c_int {
    static int m68kspkr_event(struct input_dev *dev, unsigned int type, unsigned int code, int value)
    {
    let mut count: c_uint = 0;
    if (type != EV_SND)
    return -1;
    switch (code) {
    case SND_BELL: if (value) value = 1000;
    case SND_TONE: break;
    default: return -1;
    }
    if (value > 20 && value < 32767)
    count = 1193182 / value;
    mach_beep(count, -1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m68kspkr_probe(dev: *mut platform_device) -> c_int {
    static int m68kspkr_probe(struct platform_device *dev)
    {
    struct input_dev *input_dev;
    int err;
    input_dev = input_allocate_device();
    if (!input_dev)
    return -ENOMEM;
    input_dev.name = "m68k beeper";
    input_dev.phys = "m68k/generic";
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor  = 0x001f;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &dev.dev;
    input_dev.evbit[0] = BIT_MASK(EV_SND);
    input_dev.sndbit[0] = BIT_MASK(SND_BELL) | BIT_MASK(SND_TONE);
    input_dev.event = m68kspkr_event;
    err = input_register_device(input_dev);
    if (err) {
    input_free_device(input_dev);
    return err;
    }
    platform_set_drvdata(dev, input_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m68kspkr_remove(dev: *mut platform_device) {
    static void m68kspkr_remove(struct platform_device *dev)
    {
    struct input_dev *input_dev = platform_get_drvdata(dev);
    input_unregister_device(input_dev);
// turn off the speaker
    m68kspkr_event(core::ptr::null_mut(), EV_SND, SND_BELL, 0);
    }
#[no_mangle]
unsafe extern "C" fn m68kspkr_shutdown(dev: *mut platform_device) {
    static void m68kspkr_shutdown(struct platform_device *dev)
    {
// turn off the speaker
    m68kspkr_event(core::ptr::null_mut(), EV_SND, SND_BELL, 0);
    }
    static struct platform_driver m68kspkr_platform_driver = {
    .driver		= {
    .name	= "m68kspkr",
    },
    .probe		= m68kspkr_probe,
    .remove		= m68kspkr_remove,
    .shutdown	= m68kspkr_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn m68kspkr_init() -> int __init {
    static int __init m68kspkr_init(void)
    {
    int err;
    if (!mach_beep) {
    printk(KERN_INFO "m68kspkr: no lowlevel beep support\n");
    return -ENODEV;
    }
    err = platform_driver_register(&m68kspkr_platform_driver);
    if (err)
    return err;
    m68kspkr_platform_device = platform_device_alloc("m68kspkr", -1);
    if (!m68kspkr_platform_device) {
    err = -ENOMEM;
    goto err_unregister_driver;
    }
    err = platform_device_add(m68kspkr_platform_device);
    if (err)
    goto err_free_device;
    return 0;
    err_free_device:
    platform_device_put(m68kspkr_platform_device);
    err_unregister_driver:
    platform_driver_unregister(&m68kspkr_platform_driver);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn m68kspkr_exit() -> void __exit {
    static void __exit m68kspkr_exit(void)
    {
    platform_device_unregister(m68kspkr_platform_device);
    platform_driver_unregister(&m68kspkr_platform_driver);
    }
    module_init(m68kspkr_init);
    module_exit(m68kspkr_exit);
