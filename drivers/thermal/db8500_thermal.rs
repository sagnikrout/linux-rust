//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/db8500_thermal.c
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
// db8500_thermal.c - DB8500 Thermal Management Implementation
//
// Copyright (C) 2012 ST-Ericsson
// Copyright (C) 2012-2019 Linaro Ltd.
//
// Authors: Hongbo Zhang, Linus Walleij
//

pub const PRCMU_DEFAULT_MEASURE_TIME: c_uint = 0xFFF;
pub const PRCMU_DEFAULT_LOW_TEMP: c_int = 0;
//
// db8500_thermal_points - the interpolation points that trigger
// interrupts
//
    static const unsigned long db8500_thermal_points[] = {
    15000,
    20000,
    25000,
    30000,
    35000,
    40000,
    45000,
    50000,
    55000,
    60000,
    65000,
    70000,
    75000,
    80000,
//
// This is where things start to get really bad for the
// SoC and the thermal zones should be set up to trigger
// critical temperature at 85000 mC so we don't get above
// this point.
//
    85000,
    90000,
    95000,
    100000,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db8500_thermal_zone {
    pub tz: *mut thermal_zone_device,
    pub dev: *mut device,
    pub interpolated_temp: c_ulong,
    pub cur_index: c_uint,
}

// Callback to get current temperature
#[no_mangle]
unsafe extern "C" fn db8500_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int db8500_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct db8500_thermal_zone *th = thermal_zone_device_priv(tz);
//
// TODO: There is no PRCMU interface to get temperature data currently,
// so a pseudo temperature is returned , it works for thermal framework
// and this will be fixed when the PRCMU interface is available.
//
// temp = th->interpolated_temp;
    return 0;
    }
    static const struct thermal_zone_device_ops thdev_ops = {
    .get_temp = db8500_thermal_get_temp,
    };
    static void db8500_thermal_update_config(struct db8500_thermal_zone *th,
    unsigned int idx,
    unsigned long next_low,
    unsigned long next_high)
    {
    db8500_prcmu_stop_temp_sense();
    th.cur_index = idx;
    th.interpolated_temp = (next_low + next_high)/2;
//
// The PRCMU accept absolute temperatures in celsius so divide
// down the millicelsius with 1000
//
    db8500_prcmu_config_hotmon((u8)(next_low / 1000), (u8)(next_high / 1000));
    db8500_prcmu_start_temp_sense(PRCMU_DEFAULT_MEASURE_TIME);
    }
#[no_mangle]
unsafe extern "C" fn prcmu_low_irq_handler(irq: c_int, irq_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t prcmu_low_irq_handler(int irq, void *irq_data)
    {
    struct db8500_thermal_zone *th = irq_data;
    let mut idx: c_uint = th.cur_index;
    unsigned long next_low, next_high;
    if (idx == 0)
// Meaningless for thermal management, ignoring it
    return IRQ_HANDLED;
    if (idx == 1) {
    next_high = db8500_thermal_points[0];
    next_low = PRCMU_DEFAULT_LOW_TEMP;
    } else {
    next_high = db8500_thermal_points[idx - 1];
    next_low = db8500_thermal_points[idx - 2];
    }
    idx -= 1;
    db8500_thermal_update_config(th, idx, next_low, next_high);
    dev_dbg(th.dev,
    "PRCMU set max %ld, min %ld\n", next_high, next_low);
    thermal_zone_device_update(th.tz, THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn prcmu_high_irq_handler(irq: c_int, irq_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t prcmu_high_irq_handler(int irq, void *irq_data)
    {
    struct db8500_thermal_zone *th = irq_data;
    let mut idx: c_uint = th.cur_index;
    unsigned long next_low, next_high;
    let mut num_points: c_int = ARRAY_SIZE(db8500_thermal_points);
    if (idx < num_points - 1) {
    next_high = db8500_thermal_points[idx+1];
    next_low = db8500_thermal_points[idx];
    idx += 1;
    db8500_thermal_update_config(th, idx, next_low, next_high);
    dev_dbg(th.dev,
    "PRCMU set max %ld, min %ld\n", next_high, next_low);
    } else if (idx == num_points - 1)
// So we roof out 1 degree over the max point
    th.interpolated_temp = db8500_thermal_points[idx] + 1;
    thermal_zone_device_update(th.tz, THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn db8500_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int db8500_thermal_probe(struct platform_device *pdev)
    {
    struct db8500_thermal_zone *th = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    int low_irq, high_irq, ret = 0;
    th = devm_kzalloc(dev, sizeof(*th), GFP_KERNEL);
    if (!th)
    return -ENOMEM;
    th.dev = dev;
    low_irq = platform_get_irq_byname(pdev, "IRQ_HOTMON_LOW");
    if (low_irq < 0)
    return low_irq;
    ret = devm_request_threaded_irq(dev, low_irq, core::ptr::null_mut(),
    prcmu_low_irq_handler, IRQF_NO_SUSPEND | IRQF_ONESHOT,
    "dbx500_temp_low", th);
    if (ret < 0)
    return ret;
    high_irq = platform_get_irq_byname(pdev, "IRQ_HOTMON_HIGH");
    if (high_irq < 0)
    return high_irq;
    ret = devm_request_threaded_irq(dev, high_irq, core::ptr::null_mut(),
    prcmu_high_irq_handler, IRQF_NO_SUSPEND | IRQF_ONESHOT,
    "dbx500_temp_high", th);
    if (ret < 0)
    return ret;
// register of thermal sensor and get info from DT
    th.tz = devm_thermal_of_zone_register(dev, 0, th, &thdev_ops);
    if (IS_ERR(th.tz)) {
    dev_err(dev, "register thermal zone sensor failed\n");
    return PTR_ERR(th.tz);
    }
    dev_info(dev, "thermal zone sensor registered\n");
// Start measuring at the lowest point
    db8500_thermal_update_config(th, 0, PRCMU_DEFAULT_LOW_TEMP,
    db8500_thermal_points[0]);
    platform_set_drvdata(pdev, th);
    return 0;
    }
    static int db8500_thermal_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    db8500_prcmu_stop_temp_sense();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn db8500_thermal_resume(pdev: *mut platform_device) -> c_int {
    static int db8500_thermal_resume(struct platform_device *pdev)
    {
    struct db8500_thermal_zone *th = platform_get_drvdata(pdev);
// Resume and start measuring at the lowest point
    db8500_thermal_update_config(th, 0, PRCMU_DEFAULT_LOW_TEMP,
    db8500_thermal_points[0]);
    return 0;
    }
    static const struct of_device_id db8500_thermal_match[] = {
    { .compatible = "stericsson,db8500-thermal" },
    {},
    };
    MODULE_DEVICE_TABLE(of, db8500_thermal_match);
    static struct platform_driver db8500_thermal_driver = {
    .driver = {
    .name = "db8500-thermal",
    .of_match_table = db8500_thermal_match,
    },
    .probe = db8500_thermal_probe,
    .suspend = db8500_thermal_suspend,
    .resume = db8500_thermal_resume,
    };
    module_platform_driver(db8500_thermal_driver);
    MODULE_AUTHOR("Hongbo Zhang <hongbo.zhang@stericsson.com>");
    MODULE_DESCRIPTION("DB8500 thermal driver");
    MODULE_LICENSE("GPL");
