//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/rt5120-pwrkey.c
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
// Copyright (C) 2022 Richtek Technology Corp.
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const RT5120_REG_INTSTAT: c_uint = 0x1E;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5120_priv {
    pub regmap: *mut regmap,
    pub input: *mut input_dev,
}

#[no_mangle]
unsafe extern "C" fn rt5120_pwrkey_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t rt5120_pwrkey_handler(int irq, void *devid)
    {
    struct rt5120_priv *priv = devid;
    unsigned int stat;
    int error;
    error = regmap_read(priv.regmap, RT5120_REG_INTSTAT, &stat);
    if (error)
    return IRQ_NONE;
    input_report_key(priv.input, KEY_POWER,
    !(stat & RT5120_PWRKEYSTAT_MASK));
    input_sync(priv.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rt5120_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int rt5120_pwrkey_probe(struct platform_device *pdev)
    {
    struct rt5120_priv *priv;
    struct device *dev = &pdev.dev;
    int press_irq, release_irq;
    int error;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.regmap) {
    dev_err(dev, "Failed to init regmap\n");
    return -ENODEV;
    }
    press_irq = platform_get_irq_byname(pdev, "pwrkey-press");
    if (press_irq < 0)
    return press_irq;
    release_irq = platform_get_irq_byname(pdev, "pwrkey-release");
    if (release_irq < 0)
    return release_irq;
// Make input device be device resource managed
    priv.input = devm_input_allocate_device(dev);
    if (!priv.input)
    return -ENOMEM;
    priv.input.name = "rt5120_pwrkey";
    priv.input.phys = "rt5120_pwrkey/input0";
    priv.input.id.bustype = BUS_I2C;
    input_set_capability(priv.input, EV_KEY, KEY_POWER);
    error = input_register_device(priv.input);
    if (error) {
    dev_err(dev, "Failed to register input device: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(dev, press_irq,
    core::ptr::null_mut(), rt5120_pwrkey_handler,
    0, "pwrkey-press", priv);
    if (error) {
    dev_err(dev,
    "Failed to register pwrkey press irq: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(dev, release_irq,
    core::ptr::null_mut(), rt5120_pwrkey_handler,
    0, "pwrkey-release", priv);
    if (error) {
    dev_err(dev,
    "Failed to register pwrkey release irq: %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id r5120_pwrkey_match_table[] = {
    { .compatible = "richtek,rt5120-pwrkey" },
    {}
    };
    MODULE_DEVICE_TABLE(of, r5120_pwrkey_match_table);
    static struct platform_driver rt5120_pwrkey_driver = {
    .driver = {
    .name = "rt5120-pwrkey",
    .of_match_table = r5120_pwrkey_match_table,
    },
    .probe = rt5120_pwrkey_probe,
    };
    module_platform_driver(rt5120_pwrkey_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT5120 power key driver");
    MODULE_LICENSE("GPL");
