//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/max77650-onkey.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 BayLibre SAS
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//
// ONKEY driver for MAXIM 77650/77651 charger/power-supply.

pub const MAX77650_ONKEY_MODE_PUSH: c_uint = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77650_onkey {
    pub input: *mut input_dev,
    pub code: c_uint,
}

#[no_mangle]
unsafe extern "C" fn max77650_onkey_falling(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max77650_onkey_falling(int irq, void *data)
    {
    struct max77650_onkey *onkey = data;
    input_report_key(onkey.input, onkey.code, 0);
    input_sync(onkey.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max77650_onkey_rising(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max77650_onkey_rising(int irq, void *data)
    {
    struct max77650_onkey *onkey = data;
    input_report_key(onkey.input, onkey.code, 1);
    input_sync(onkey.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max77650_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int max77650_onkey_probe(struct platform_device *pdev)
    {
    int irq_r, irq_f, error, mode;
    struct max77650_onkey *onkey;
    struct device *dev, *parent;
    struct regmap *map;
    unsigned int type;
    dev = &pdev.dev;
    parent = dev.parent;
    map = dev_get_regmap(parent, core::ptr::null_mut());
    if (!map)
    return -ENODEV;
    onkey = devm_kzalloc(dev, sizeof(*onkey), GFP_KERNEL);
    if (!onkey)
    return -ENOMEM;
    error = device_property_read_u32(dev, "linux,code", &onkey.code);
    if (error)
    onkey.code = KEY_POWER;
    if (device_property_read_bool(dev, "maxim,onkey-slide")) {
    mode = MAX77650_ONKEY_MODE_SLIDE;
    type = EV_SW;
    } else {
    mode = MAX77650_ONKEY_MODE_PUSH;
    type = EV_KEY;
    }
    error = regmap_update_bits(map, MAX77650_REG_CNFG_GLBL,
    MAX77650_ONKEY_MODE_MASK, mode);
    if (error)
    return error;
    irq_f = platform_get_irq_byname(pdev, "nEN_F");
    if (irq_f < 0)
    return irq_f;
    irq_r = platform_get_irq_byname(pdev, "nEN_R");
    if (irq_r < 0)
    return irq_r;
    onkey.input = devm_input_allocate_device(dev);
    if (!onkey.input)
    return -ENOMEM;
    onkey.input.name = "max77650_onkey";
    onkey.input.phys = "max77650_onkey/input0";
    onkey.input.id.bustype = BUS_I2C;
    input_set_capability(onkey.input, type, onkey.code);
    error = devm_request_any_context_irq(dev, irq_f, max77650_onkey_falling,
    IRQF_ONESHOT, "onkey-down", onkey);
    if (error < 0)
    return error;
    error = devm_request_any_context_irq(dev, irq_r, max77650_onkey_rising,
    IRQF_ONESHOT, "onkey-up", onkey);
    if (error < 0)
    return error;
    return input_register_device(onkey.input);
    }
    static const struct of_device_id max77650_onkey_of_match[] = {
    { .compatible = "maxim,max77650-onkey" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max77650_onkey_of_match);
    static struct platform_driver max77650_onkey_driver = {
    .driver = {
    .name = "max77650-onkey",
    .of_match_table = max77650_onkey_of_match,
    },
    .probe = max77650_onkey_probe,
    };
    module_platform_driver(max77650_onkey_driver);
    MODULE_DESCRIPTION("MAXIM 77650/77651 ONKEY driver");
    MODULE_AUTHOR("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:max77650-onkey");
