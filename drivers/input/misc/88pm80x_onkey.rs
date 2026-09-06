//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/88pm80x_onkey.c
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
// Marvell 88PM80x ONKEY driver
//
// Copyright (C) 2012 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
// Qiao Zhou <zhouqiao@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_onkey_info {
    pub idev: *mut input_dev,
    pub pm80x: *mut pm80x_chip,
    pub map: *mut regmap,
    pub irq: c_int,
}

// 88PM80x gives us an interrupt when ONKEY is held
#[no_mangle]
unsafe extern "C" fn pm80x_onkey_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm80x_onkey_handler(int irq, void *data)
    {
    struct pm80x_onkey_info *info = data;
    let mut ret: c_int = 0;
    unsigned int val;
    ret = regmap_read(info.map, PM800_STATUS_1, &val);
    if (ret < 0) {
    dev_err(info.idev.dev.parent, "failed to read status: %d\n", ret);
    return IRQ_NONE;
    }
    val &= PM800_ONKEY_STS1;
    input_report_key(info.idev, KEY_POWER, val);
    input_sync(info.idev);
    return IRQ_HANDLED;
    }
    static SIMPLE_DEV_PM_OPS(pm80x_onkey_pm_ops, pm80x_dev_suspend,
    pm80x_dev_resume);
#[no_mangle]
unsafe extern "C" fn pm80x_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int pm80x_onkey_probe(struct platform_device *pdev)
    {
    struct pm80x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct pm80x_onkey_info *info;
    int err;
    info = kzalloc_obj(*info);
    if (!info)
    return -ENOMEM;
    info.pm80x = chip;
    info.irq = platform_get_irq(pdev, 0);
    if (info.irq < 0) {
    err = -EINVAL;
    goto out;
    }
    info.map = info.pm80x.regmap;
    if (!info.map) {
    dev_err(&pdev.dev, "no regmap!\n");
    err = -EINVAL;
    goto out;
    }
    info.idev = input_allocate_device();
    if (!info.idev) {
    dev_err(&pdev.dev, "Failed to allocate input dev\n");
    err = -ENOMEM;
    goto out;
    }
    info.idev.name = "88pm80x_on";
    info.idev.phys = "88pm80x_on/input0";
    info.idev.id.bustype = BUS_I2C;
    info.idev.dev.parent = &pdev.dev;
    info.idev.evbit[0] = BIT_MASK(EV_KEY);
    __set_bit(KEY_POWER, info.idev.keybit);
    err = pm80x_request_irq(info.pm80x, info.irq, pm80x_onkey_handler,
    IRQF_ONESHOT, "onkey", info);
    if (err < 0) {
    dev_err(&pdev.dev, "Failed to request IRQ: #%d: %d\n",
    info.irq, err);
    goto out_reg;
    }
    err = input_register_device(info.idev);
    if (err) {
    dev_err(&pdev.dev, "Can't register input device: %d\n", err);
    goto out_irq;
    }
    platform_set_drvdata(pdev, info);
// Enable long onkey detection
    regmap_update_bits(info.map, PM800_RTC_MISC4, PM800_LONG_ONKEY_EN,
    PM800_LONG_ONKEY_EN);
// Set 8-second interval
    regmap_update_bits(info.map, PM800_RTC_MISC3,
    PM800_LONKEY_PRESS_TIME_MASK,
    PM800_LONKEY_PRESS_TIME);
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    out_irq:
    pm80x_free_irq(info.pm80x, info.irq, info);
    out_reg:
    input_free_device(info.idev);
    out:
    kfree(info);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pm80x_onkey_remove(pdev: *mut platform_device) {
    static void pm80x_onkey_remove(struct platform_device *pdev)
    {
    struct pm80x_onkey_info *info = platform_get_drvdata(pdev);
    pm80x_free_irq(info.pm80x, info.irq, info);
    input_unregister_device(info.idev);
    kfree(info);
    }
    static struct platform_driver pm80x_onkey_driver = {
    .driver = {
    .name = "88pm80x-onkey",
    .pm = &pm80x_onkey_pm_ops,
    },
    .probe = pm80x_onkey_probe,
    .remove = pm80x_onkey_remove,
    };
    module_platform_driver(pm80x_onkey_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Marvell 88PM80x ONKEY driver");
    MODULE_AUTHOR("Qiao Zhou <zhouqiao@marvell.com>");
    MODULE_ALIAS("platform:88pm80x-onkey");
