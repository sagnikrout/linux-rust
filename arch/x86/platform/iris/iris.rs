//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/iris/iris.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Eurobraille/Iris power off support.
//
// Eurobraille's Iris machine is a PC with no APM or ACPI support.
// It is shutdown by a special I/O sequence which this module provides.
//
// Copyright (C) Shérab <Sebastien.Hinderer@ens-lyon.org>
//

pub const IRIS_GIO_BASE: c_uint = 0x340;

pub const IRIS_GIO_PULSE: c_uint = 0x80 /* First byte to send */;
pub const IRIS_GIO_REST: c_uint = 0x00 /* Second byte to send */;
pub const IRIS_GIO_NODEV: c_uint = 0xff /* Likely not an Iris */;
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sébastien Hinderer <Sebastien.Hinderer@ens-lyon.org>");
    MODULE_DESCRIPTION("A power_off handler for Iris devices from EuroBraille");
    static bool force;
    module_param(force, bool, 0);
    MODULE_PARM_DESC(force, "Set to one to force poweroff handler installation.");
    static void (*old_pm_power_off)(void);
#[no_mangle]
unsafe extern "C" fn iris_power_off() {
    static void iris_power_off(void)
    {
    outb(IRIS_GIO_PULSE, IRIS_GIO_OUTPUT);
    msleep(850);
    outb(IRIS_GIO_REST, IRIS_GIO_OUTPUT);
    }
//
// Before installing the power_off handler, try to make sure the OS is
// running on an Iris.  Since Iris does not support DMI, this is done
// by reading its input port and seeing whether the read value is
// meaningful.
//
#[no_mangle]
unsafe extern "C" fn iris_probe(pdev: *mut platform_device) -> c_int {
    static int iris_probe(struct platform_device *pdev)
    {
    let mut status: c_uchar = inb(IRIS_GIO_INPUT);
    if (status == IRIS_GIO_NODEV) {
    printk(KERN_ERR "This machine does not seem to be an Iris. "
    "Power off handler not installed.\n");
    return -ENODEV;
    }
    old_pm_power_off = pm_power_off;
    pm_power_off = &iris_power_off;
    printk(KERN_INFO "Iris power_off handler installed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iris_remove(pdev: *mut platform_device) {
    static void iris_remove(struct platform_device *pdev)
    {
    pm_power_off = old_pm_power_off;
    printk(KERN_INFO "Iris power_off handler uninstalled.\n");
    }
    static struct platform_driver iris_driver = {
    .driver		= {
    .name   = "iris",
    },
    .probe          = iris_probe,
    .remove         = iris_remove,
    };
    static struct resource iris_resources[] = {
    {
    .start  = IRIS_GIO_BASE,
    .end    = IRIS_GIO_OUTPUT,
    .flags  = IORESOURCE_IO,
    .name   = "address"
    }
    };
    static struct platform_device *iris_device;
#[no_mangle]
unsafe extern "C" fn iris_init() -> c_int {
    static int iris_init(void)
    {
    int ret;
    if (force != 1) {
    printk(KERN_ERR "The force parameter has not been set to 1."
    " The Iris poweroff handler will not be installed.\n");
    return -ENODEV;
    }
    ret = platform_driver_register(&iris_driver);
    if (ret < 0) {
    printk(KERN_ERR "Failed to register iris platform driver: %d\n",
    ret);
    return ret;
    }
    iris_device = platform_device_register_simple("iris", (-1),
    iris_resources, ARRAY_SIZE(iris_resources));
    if (IS_ERR(iris_device)) {
    printk(KERN_ERR "Failed to register iris platform device\n");
    platform_driver_unregister(&iris_driver);
    return PTR_ERR(iris_device);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iris_exit() {
    static void iris_exit(void)
    {
    platform_device_unregister(iris_device);
    platform_driver_unregister(&iris_driver);
    }
    module_init(iris_init);
    module_exit(iris_exit);
