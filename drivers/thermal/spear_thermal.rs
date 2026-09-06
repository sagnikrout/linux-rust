//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/spear_thermal.c
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
// SPEAr thermal driver.
//
// Copyright (C) 2011-2012 ST Microelectronics
// Author: Vincenzo Frascino <vincenzo.frascino@st.com>
//

pub const MD_FACTOR: c_int = 1000;
// SPEAr Thermal Sensor Dev Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_thermal_dev {
// pointer to base address of the thermal sensor
    pub thermal_base: *mut void __iomem,
// clk structure
    pub clk: *mut clk,
// pointer to thermal flags
    pub flags: c_uint,
}

    static inline int thermal_get_temp(struct thermal_zone_device *thermal,
    int *temp)
    {
    struct spear_thermal_dev *stdev = thermal_zone_device_priv(thermal);
//
// Data are ready to be read after 628 usec from POWERDOWN signal
// (PDN) = 1
//
// temp = (readl_relaxed(stdev->thermal_base) & 0x7F) * MD_FACTOR;
    return 0;
    }
    static const struct thermal_zone_device_ops ops = {
    .get_temp = thermal_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn spear_thermal_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused spear_thermal_suspend(struct device *dev)
    {
    struct thermal_zone_device *spear_thermal = dev_get_drvdata(dev);
    struct spear_thermal_dev *stdev = thermal_zone_device_priv(spear_thermal);
    let mut actual_mask: c_uint = 0;
// Disable SPEAr Thermal Sensor
    actual_mask = readl_relaxed(stdev.thermal_base);
    writel_relaxed(actual_mask & ~stdev.flags, stdev.thermal_base);
    clk_disable(stdev.clk);
    dev_info(dev, "Suspended.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_thermal_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused spear_thermal_resume(struct device *dev)
    {
    struct thermal_zone_device *spear_thermal = dev_get_drvdata(dev);
    struct spear_thermal_dev *stdev = thermal_zone_device_priv(spear_thermal);
    let mut actual_mask: c_uint = 0;
    let mut ret: c_int = 0;
    ret = clk_enable(stdev.clk);
    if (ret) {
    dev_err(dev, "Can't enable clock\n");
    return ret;
    }
// Enable SPEAr Thermal Sensor
    actual_mask = readl_relaxed(stdev.thermal_base);
    writel_relaxed(actual_mask | stdev.flags, stdev.thermal_base);
    dev_info(dev, "Resumed.\n");
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(spear_thermal_pm_ops, spear_thermal_suspend,
    spear_thermal_resume);
#[no_mangle]
unsafe extern "C" fn spear_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int spear_thermal_probe(struct platform_device *pdev)
    {
    struct thermal_zone_device *spear_thermal = core::ptr::null_mut();
    struct spear_thermal_dev *stdev;
    struct device_node *np = pdev.dev.of_node;
    let mut ret: c_int = 0, val;
    if (!np || of_property_read_u32(np, "st,thermal-flags", &val)) {
    dev_err(&pdev.dev, "Failed: DT Pdata not passed\n");
    return -EINVAL;
    }
    stdev = devm_kzalloc(&pdev.dev, sizeof(*stdev), GFP_KERNEL);
    if (!stdev)
    return -ENOMEM;
// Enable thermal sensor
    stdev.thermal_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(stdev.thermal_base))
    return PTR_ERR(stdev.thermal_base);
    stdev.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(stdev.clk)) {
    dev_err(&pdev.dev, "Can't get clock\n");
    return PTR_ERR(stdev.clk);
    }
    ret = clk_enable(stdev.clk);
    if (ret) {
    dev_err(&pdev.dev, "Can't enable clock\n");
    return ret;
    }
    stdev.flags = val;
    writel_relaxed(stdev.flags, stdev.thermal_base);
    spear_thermal = thermal_tripless_zone_device_register("spear_thermal",
    stdev, &ops, core::ptr::null_mut());
    if (IS_ERR(spear_thermal)) {
    dev_err(&pdev.dev, "thermal zone device is core::ptr::null_mut()\n");
    ret = PTR_ERR(spear_thermal);
    goto disable_clk;
    }
    ret = thermal_zone_device_enable(spear_thermal);
    if (ret) {
    dev_err(&pdev.dev, "Cannot enable thermal zone\n");
    goto unregister_tzd;
    }
    platform_set_drvdata(pdev, spear_thermal);
    dev_info(&pdev.dev, "Thermal Sensor Loaded at: 0x%p.\n",
    stdev.thermal_base);
    return 0;
    unregister_tzd:
    thermal_zone_device_unregister(spear_thermal);
    disable_clk:
    clk_disable(stdev.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spear_thermal_exit(pdev: *mut platform_device) {
    static void spear_thermal_exit(struct platform_device *pdev)
    {
    let mut actual_mask: c_uint = 0;
    struct thermal_zone_device *spear_thermal = platform_get_drvdata(pdev);
    struct spear_thermal_dev *stdev = thermal_zone_device_priv(spear_thermal);
    thermal_zone_device_unregister(spear_thermal);
// Disable SPEAr Thermal Sensor
    actual_mask = readl_relaxed(stdev.thermal_base);
    writel_relaxed(actual_mask & ~stdev.flags, stdev.thermal_base);
    clk_disable(stdev.clk);
    }
    static const struct of_device_id spear_thermal_id_table[] = {
    { .compatible = "st,thermal-spear1340" },
    {}
    };
    MODULE_DEVICE_TABLE(of, spear_thermal_id_table);
    static struct platform_driver spear_thermal_driver = {
    .probe = spear_thermal_probe,
    .remove = spear_thermal_exit,
    .driver = {
    .name = "spear_thermal",
    .pm = &spear_thermal_pm_ops,
    .of_match_table = spear_thermal_id_table,
    },
    };
    module_platform_driver(spear_thermal_driver);
    MODULE_AUTHOR("Vincenzo Frascino <vincenzo.frascino@st.com>");
    MODULE_DESCRIPTION("SPEAr thermal driver");
    MODULE_LICENSE("GPL");
