//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/broadcom/ns-thermal.c
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
// Copyright (C) 2017 Rafał Miłecki <rafal@milecki.pl>
//

pub const PVTMON_CONTROL0: c_uint = 0x00;
pub const PVTMON_CONTROL0_SEL_MASK: c_uint = 0x0000000e;
pub const PVTMON_CONTROL0_SEL_TEMP_MONITOR: c_uint = 0x00000000;
pub const PVTMON_CONTROL0_SEL_TEST_MODE: c_uint = 0x0000000e;
pub const PVTMON_STATUS: c_uint = 0x08;
#[no_mangle]
unsafe extern "C" fn ns_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int ns_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    void __iomem *pvtmon = thermal_zone_device_priv(tz);
    let mut offset: c_int = thermal_zone_get_offset(tz);
    let mut slope: c_int = thermal_zone_get_slope(tz);
    u32 val;
    val = readl(pvtmon + PVTMON_CONTROL0);
    if ((val & PVTMON_CONTROL0_SEL_MASK) != PVTMON_CONTROL0_SEL_TEMP_MONITOR) {
// Clear current mode selection
    val &= ~PVTMON_CONTROL0_SEL_MASK;
// Set temp monitor mode (it's the default actually)
    val |= PVTMON_CONTROL0_SEL_TEMP_MONITOR;
    writel(val, pvtmon + PVTMON_CONTROL0);
    }
    val = readl(pvtmon + PVTMON_STATUS);
// temp = slope * val + offset;
    return 0;
    }
    static const struct thermal_zone_device_ops ns_thermal_ops = {
    .get_temp = ns_thermal_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn ns_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int ns_thermal_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct thermal_zone_device *tz;
    void __iomem *pvtmon;
    pvtmon = of_iomap(dev_of_node(dev), 0);
    if (WARN_ON(!pvtmon))
    return -ENOENT;
    tz = devm_thermal_of_zone_register(dev, 0,
    pvtmon,
    &ns_thermal_ops);
    if (IS_ERR(tz)) {
    iounmap(pvtmon);
    return PTR_ERR(tz);
    }
    platform_set_drvdata(pdev, pvtmon);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns_thermal_remove(pdev: *mut platform_device) {
    static void ns_thermal_remove(struct platform_device *pdev)
    {
    void __iomem *pvtmon = platform_get_drvdata(pdev);
    iounmap(pvtmon);
    }
    static const struct of_device_id ns_thermal_of_match[] = {
    { .compatible = "brcm,ns-thermal", },
    {},
    };
    MODULE_DEVICE_TABLE(of, ns_thermal_of_match);
    static struct platform_driver ns_thermal_driver = {
    .probe		= ns_thermal_probe,
    .remove		= ns_thermal_remove,
    .driver = {
    .name = "ns-thermal",
    .of_match_table = ns_thermal_of_match,
    },
    };
    module_platform_driver(ns_thermal_driver);
    MODULE_AUTHOR("Rafał Miłecki <rafal@milecki.pl>");
    MODULE_DESCRIPTION("Northstar thermal driver");
    MODULE_LICENSE("GPL v2");
