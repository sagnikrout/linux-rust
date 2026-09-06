//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/w1-gpio.c
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
// w1-gpio - GPIO w1 bus master driver
//
// Copyright (C) 2007 Ville Syrjala <syrjala@sci.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_gpio_ddata {
    pub gpiod: *mut gpio_desc,
    pub pullup_gpiod: *mut gpio_desc,
    pub pullup_duration: c_uint,
}

#[no_mangle]
unsafe extern "C" fn w1_gpio_set_pullup(data: *mut c_void, delay: c_int) -> u8 {
    static u8 w1_gpio_set_pullup(void *data, int delay)
    {
    struct w1_gpio_ddata *ddata = data;
    if (delay) {
    ddata.pullup_duration = delay;
    } else {
    if (ddata.pullup_duration) {
//
// This will OVERRIDE open drain emulation and force-pull
// the line high for some time.
//
    gpiod_set_raw_value(ddata.gpiod, 1);
    msleep(ddata.pullup_duration);
//
// This will simply set the line as input since we are doing
// open drain emulation in the GPIO library.
//
    gpiod_set_value(ddata.gpiod, 1);
    }
    ddata.pullup_duration = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_gpio_write_bit(data: *mut c_void, bit: u8) {
    static void w1_gpio_write_bit(void *data, u8 bit)
    {
    struct w1_gpio_ddata *ddata = data;
    gpiod_set_value(ddata.gpiod, bit);
    }
#[no_mangle]
unsafe extern "C" fn w1_gpio_read_bit(data: *mut c_void) -> u8 {
    static u8 w1_gpio_read_bit(void *data)
    {
    struct w1_gpio_ddata *ddata = data;
    return gpiod_get_value(ddata.gpiod) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int w1_gpio_probe(struct platform_device *pdev)
    {
    struct w1_bus_master *master;
    struct w1_gpio_ddata *ddata;
    struct device *dev = &pdev.dev;
// Enforce open drain mode by default
    let mut gflags: enum gpiod_flags = GPIOD_OUT_LOW_OPEN_DRAIN;
    int err;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
//
// This parameter means that something else than the gpiolib has
// already set the line into open drain mode, so we should just
// driver it high/low like we are in full control of the line and
// open drain will happen transparently.
//
    if (device_property_present(dev, "linux,open-drain"))
    gflags = GPIOD_OUT_LOW;
    master = devm_kzalloc(dev, sizeof(*master), GFP_KERNEL);
    if (!master)
    return -ENOMEM;
    ddata.gpiod = devm_gpiod_get_index(dev, core::ptr::null_mut(), 0, gflags);
    if (IS_ERR(ddata.gpiod))
    return dev_err_probe(dev, PTR_ERR(ddata.gpiod), "gpio_request (pin) failed\n");
    ddata.pullup_gpiod =
    devm_gpiod_get_index_optional(dev, core::ptr::null_mut(), 1, GPIOD_OUT_LOW);
    if (IS_ERR(ddata.pullup_gpiod))
    return dev_err_probe(dev, PTR_ERR(ddata.pullup_gpiod),
    "gpio_request (ext_pullup_enable_pin) failed\n");
    master.data = ddata;
    master.read_bit = w1_gpio_read_bit;
    gpiod_direction_output(ddata.gpiod, 1);
    master.write_bit = w1_gpio_write_bit;
//
// If we are using open drain emulation from the GPIO library,
// we need to use this pullup function that hammers the line
// high using a raw accessor to provide pull-up for the w1
// line.
//
    if (gflags == GPIOD_OUT_LOW_OPEN_DRAIN)
    master.set_pullup = w1_gpio_set_pullup;
    err = w1_add_master_device(master);
    if (err)
    return dev_err_probe(dev, err, "w1_add_master device failed\n");
    gpiod_set_value(ddata.pullup_gpiod, 1);
    platform_set_drvdata(pdev, master);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn w1_gpio_remove(pdev: *mut platform_device) {
    static void w1_gpio_remove(struct platform_device *pdev)
    {
    struct w1_bus_master *master = platform_get_drvdata(pdev);
    struct w1_gpio_ddata *ddata = master.data;
    gpiod_set_value(ddata.pullup_gpiod, 0);
    w1_remove_master_device(master);
    }
    static const struct of_device_id w1_gpio_dt_ids[] = {
    { .compatible = "w1-gpio" },
    {}
    };
    MODULE_DEVICE_TABLE(of, w1_gpio_dt_ids);
    static struct platform_driver w1_gpio_driver = {
    .driver = {
    .name	= "w1-gpio",
    .of_match_table = w1_gpio_dt_ids,
    },
    .probe = w1_gpio_probe,
    .remove = w1_gpio_remove,
    };
    module_platform_driver(w1_gpio_driver);
    MODULE_DESCRIPTION("GPIO w1 bus master driver");
    MODULE_AUTHOR("Ville Syrjala <syrjala@sci.fi>");
    MODULE_LICENSE("GPL");
