//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/max77620_thermal.c
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
// Junction temperature thermal driver for Maxim Max77620.
//
// Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
// Mallikarjun Kasoju <mkasoju@nvidia.com>
//

pub const MAX77620_NORMAL_OPERATING_TEMP: c_int = 100000;
pub const MAX77620_TJALARM1_TEMP: c_int = 120000;
pub const MAX77620_TJALARM2_TEMP: c_int = 140000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77620_therm_info {
    pub dev: *mut device,
    pub rmap: *mut regmap,
    pub tz_device: *mut thermal_zone_device,
    pub irq_tjalarm1: c_int,
    pub irq_tjalarm2: c_int,
}

//
// max77620_thermal_read_temp: Read PMIC die temperatue.
// @data:	Device specific data.
// @temp:	Temperature in millidegrees Celsius
//
// The actual temperature of PMIC die is not available from PMIC.
// PMIC only tells the status if it has crossed or not the threshold level
// of 120degC or 140degC.
// If threshold has not been crossed then assume die temperature as 100degC
// else 120degC or 140deG based on the PMIC die temp threshold status.
//
// Return 0 on success otherwise error number to show reason of failure.
//
#[no_mangle]
unsafe extern "C" fn max77620_thermal_read_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int max77620_thermal_read_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct max77620_therm_info *mtherm = thermal_zone_device_priv(tz);
    unsigned int val;
    int ret;
    ret = regmap_read(mtherm.rmap, MAX77620_REG_STATLBT, &val);
    if (ret < 0)
    return ret;
    if (val & MAX77620_IRQ_TJALRM2_MASK)
// temp = MAX77620_TJALARM2_TEMP;
#[no_mangle]
pub unsafe extern "C" fn if(MAX77620_IRQ_TJALRM1_MASK: val &) -> else {
    else if (val & MAX77620_IRQ_TJALRM1_MASK)
// temp = MAX77620_TJALARM1_TEMP;
    else
// temp = MAX77620_NORMAL_OPERATING_TEMP;
    return 0;
    }
    static const struct thermal_zone_device_ops max77620_thermal_ops = {
    .get_temp = max77620_thermal_read_temp,
    };
#[no_mangle]
unsafe extern "C" fn max77620_thermal_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max77620_thermal_irq(int irq, void *data)
    {
    struct max77620_therm_info *mtherm = data;
    if (irq == mtherm.irq_tjalarm1)
    dev_warn(mtherm.dev, "Junction Temp Alarm1(120C) occurred\n");
#[no_mangle]
pub unsafe extern "C" fn if(mtherm->irq_tjalarm2: irq ==) -> else {
    else if (irq == mtherm.irq_tjalarm2)
    dev_crit(mtherm.dev, "Junction Temp Alarm2(140C) occurred\n");
    thermal_zone_device_update(mtherm.tz_device,
    THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max77620_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int max77620_thermal_probe(struct platform_device *pdev)
    {
    struct max77620_therm_info *mtherm;
    int ret;
    mtherm = devm_kzalloc(&pdev.dev, sizeof(*mtherm), GFP_KERNEL);
    if (!mtherm)
    return -ENOMEM;
    mtherm.irq_tjalarm1 = platform_get_irq(pdev, 0);
    mtherm.irq_tjalarm2 = platform_get_irq(pdev, 1);
    if ((mtherm.irq_tjalarm1 < 0) || (mtherm.irq_tjalarm2 < 0)) {
    dev_err(&pdev.dev, "Alarm irq number not available\n");
    return -EINVAL;
    }
    mtherm.dev = &pdev.dev;
    mtherm.rmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!mtherm.rmap) {
    dev_err(&pdev.dev, "Failed to get parent regmap\n");
    return -ENODEV;
    }
//
// The reference taken to the parent's node which will be balanced on
// reprobe or on platform-device release.
//
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    mtherm.tz_device = devm_thermal_of_zone_register(&pdev.dev, 0,
    mtherm, &max77620_thermal_ops);
    if (IS_ERR(mtherm.tz_device))
    return PTR_ERR(mtherm.tz_device);
    ret = devm_request_threaded_irq(&pdev.dev, mtherm.irq_tjalarm1, core::ptr::null_mut(),
    max77620_thermal_irq,
    IRQF_ONESHOT | IRQF_SHARED,
    dev_name(&pdev.dev), mtherm);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(&pdev.dev, mtherm.irq_tjalarm2, core::ptr::null_mut(),
    max77620_thermal_irq,
    IRQF_ONESHOT | IRQF_SHARED,
    dev_name(&pdev.dev), mtherm);
    if (ret < 0)
    return ret;
    return 0;
    }
    static struct platform_device_id max77620_thermal_devtype[] = {
    { .name = "max77620-thermal", },
    {},
    };
    MODULE_DEVICE_TABLE(platform, max77620_thermal_devtype);
    static struct platform_driver max77620_thermal_driver = {
    .driver = {
    .name = "max77620-thermal",
    },
    .probe = max77620_thermal_probe,
    .id_table = max77620_thermal_devtype,
    };
    module_platform_driver(max77620_thermal_driver);
    MODULE_DESCRIPTION("Max77620 Junction temperature Thermal driver");
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_AUTHOR("Mallikarjun Kasoju <mkasoju@nvidia.com>");
    MODULE_LICENSE("GPL v2");
