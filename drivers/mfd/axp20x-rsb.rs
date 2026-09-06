//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/axp20x-rsb.c
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
// RSB driver for the X-Powers' Power Management ICs
//
// AXP20x typically comprises an adaptive USB-Compatible PWM charger, BUCK DC-DC
// converters, LDOs, multiple 12-bit ADCs of voltage, current and temperature
// as well as configurable GPIOs.
//
// This driver supports the RSB variants.
//
// Copyright (C) 2015 Chen-Yu Tsai
//
// Author: Chen-Yu Tsai <wens@csie.org>
//

#[no_mangle]
unsafe extern "C" fn axp20x_rsb_probe(rdev: *mut sunxi_rsb_device) -> c_int {
    static int axp20x_rsb_probe(struct sunxi_rsb_device *rdev)
    {
    struct axp20x_dev *axp20x;
    int ret;
    axp20x = devm_kzalloc(&rdev.dev, sizeof(*axp20x), GFP_KERNEL);
    if (!axp20x)
    return -ENOMEM;
    axp20x.dev = &rdev.dev;
    axp20x.irq = rdev.irq;
    dev_set_drvdata(&rdev.dev, axp20x);
    ret = axp20x_match_device(axp20x);
    if (ret)
    return ret;
    axp20x.regmap = devm_regmap_init_sunxi_rsb(rdev, axp20x.regmap_cfg);
    if (IS_ERR(axp20x.regmap)) {
    ret = PTR_ERR(axp20x.regmap);
    dev_err(&rdev.dev, "regmap init failed: %d\n", ret);
    return ret;
    }
    return axp20x_device_probe(axp20x);
    }
#[no_mangle]
unsafe extern "C" fn axp20x_rsb_remove(rdev: *mut sunxi_rsb_device) {
    static void axp20x_rsb_remove(struct sunxi_rsb_device *rdev)
    {
    struct axp20x_dev *axp20x = sunxi_rsb_device_get_drvdata(rdev);
    axp20x_device_remove(axp20x);
    }
    static const struct of_device_id axp20x_rsb_of_match[] = {
    { .compatible = "x-powers,axp223", .data = (void *)AXP223_ID },
    { .compatible = "x-powers,axp717", .data = (void *)AXP717_ID },
    { .compatible = "x-powers,axp803", .data = (void *)AXP803_ID },
    { .compatible = "x-powers,axp806", .data = (void *)AXP806_ID },
    { .compatible = "x-powers,axp809", .data = (void *)AXP809_ID },
    { .compatible = "x-powers,axp813", .data = (void *)AXP813_ID },
    { },
    };
    MODULE_DEVICE_TABLE(of, axp20x_rsb_of_match);
    static struct sunxi_rsb_driver axp20x_rsb_driver = {
    .driver = {
    .name	= "axp20x-rsb",
    .of_match_table	= of_match_ptr(axp20x_rsb_of_match),
    },
    .probe	= axp20x_rsb_probe,
    .remove	= axp20x_rsb_remove,
    };
    module_sunxi_rsb_driver(axp20x_rsb_driver);
    MODULE_DESCRIPTION("PMIC MFD sunXi RSB driver for AXP20X");
    MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
    MODULE_LICENSE("GPL v2");
