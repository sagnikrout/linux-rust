//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/khadas_mcu_fan.c
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
// Khadas MCU Controlled FAN driver
//
// Copyright (C) 2020 BayLibre SAS
// Author(s): Neil Armstrong <narmstrong@baylibre.com>
//

pub const MAX_LEVEL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct khadas_mcu_fan_ctx {
    pub mcu: *mut khadas_mcu,
    pub level: c_uint,
    pub cdev: *mut thermal_cooling_device,
}

    static int khadas_mcu_fan_set_level(struct khadas_mcu_fan_ctx *ctx,
    unsigned int level)
    {
    int ret;
    ret = regmap_write(ctx.mcu.regmap, KHADAS_MCU_CMD_FAN_STATUS_CTRL_REG,
    level);
    if (ret)
    return ret;
    ctx.level = level;
    return 0;
    }
    static int khadas_mcu_fan_get_max_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
// state = MAX_LEVEL;
    return 0;
    }
    static int khadas_mcu_fan_get_cur_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
    struct khadas_mcu_fan_ctx *ctx = cdev.devdata;
// state = ctx->level;
    return 0;
    }
    static int
    khadas_mcu_fan_set_cur_state(struct thermal_cooling_device *cdev,
    unsigned long state)
    {
    struct khadas_mcu_fan_ctx *ctx = cdev.devdata;
    if (state > MAX_LEVEL)
    return -EINVAL;
    if (state == ctx.level)
    return 0;
    return khadas_mcu_fan_set_level(ctx, state);
    }
    static const struct thermal_cooling_device_ops khadas_mcu_fan_cooling_ops = {
    .get_max_state = khadas_mcu_fan_get_max_state,
    .get_cur_state = khadas_mcu_fan_get_cur_state,
    .set_cur_state = khadas_mcu_fan_set_cur_state,
    };
#[no_mangle]
unsafe extern "C" fn khadas_mcu_fan_probe(pdev: *mut platform_device) -> c_int {
    static int khadas_mcu_fan_probe(struct platform_device *pdev)
    {
    struct khadas_mcu *mcu = dev_get_drvdata(pdev.dev.parent);
    struct thermal_cooling_device *cdev;
    struct device *dev = &pdev.dev;
    struct khadas_mcu_fan_ctx *ctx;
    int ret;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.mcu = mcu;
    platform_set_drvdata(pdev, ctx);
    cdev = devm_thermal_of_child_cooling_device_register(dev.parent,
    dev.parent.of_node,
    "khadas-mcu-fan", ctx,
    &khadas_mcu_fan_cooling_ops);
    if (IS_ERR(cdev)) {
    ret = PTR_ERR(cdev);
    dev_err(dev, "Failed to register khadas-mcu-fan as cooling device: %d\n",
    ret);
    return ret;
    }
    ctx.cdev = cdev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn khadas_mcu_fan_shutdown(pdev: *mut platform_device) {
    static void khadas_mcu_fan_shutdown(struct platform_device *pdev)
    {
    struct khadas_mcu_fan_ctx *ctx = platform_get_drvdata(pdev);
    khadas_mcu_fan_set_level(ctx, 0);
    }

#[no_mangle]
unsafe extern "C" fn khadas_mcu_fan_suspend(dev: *mut device) -> c_int {
    static int khadas_mcu_fan_suspend(struct device *dev)
    {
    struct khadas_mcu_fan_ctx *ctx = dev_get_drvdata(dev);
    let mut level_save: c_uint = ctx.level;
    int ret;
    ret = khadas_mcu_fan_set_level(ctx, 0);
    if (ret)
    return ret;
    ctx.level = level_save;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn khadas_mcu_fan_resume(dev: *mut device) -> c_int {
    static int khadas_mcu_fan_resume(struct device *dev)
    {
    struct khadas_mcu_fan_ctx *ctx = dev_get_drvdata(dev);
    return khadas_mcu_fan_set_level(ctx, ctx.level);
    }

    static SIMPLE_DEV_PM_OPS(khadas_mcu_fan_pm, khadas_mcu_fan_suspend,
    khadas_mcu_fan_resume);
    static const struct platform_device_id khadas_mcu_fan_id_table[] = {
    { .name = "khadas-mcu-fan-ctrl", },
    {},
    };
    MODULE_DEVICE_TABLE(platform, khadas_mcu_fan_id_table);
    static struct platform_driver khadas_mcu_fan_driver = {
    .probe		= khadas_mcu_fan_probe,
    .shutdown	= khadas_mcu_fan_shutdown,
    .driver	= {
    .name		= "khadas-mcu-fan-ctrl",
    .pm		= &khadas_mcu_fan_pm,
    },
    .id_table	= khadas_mcu_fan_id_table,
    };
    module_platform_driver(khadas_mcu_fan_driver);
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_DESCRIPTION("Khadas MCU FAN driver");
    MODULE_LICENSE("GPL");
