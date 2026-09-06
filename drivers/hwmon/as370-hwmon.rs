//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/as370-hwmon.c
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
// Synaptics AS370 SoC Hardware Monitoring Driver
//
// Copyright (C) 2018 Synaptics Incorporated
// Author: Jisheng Zhang <jszhang@kernel.org>
//

pub const CTRL: c_uint = 0x0;

pub const STS: c_uint = 0x4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as370_hwmon {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn init_pvt(hwmon: *mut as370_hwmon) {
    static void init_pvt(struct as370_hwmon *hwmon)
    {
    u32 val;
    void __iomem *addr = hwmon.base + CTRL;
    val = PD;
    writel_relaxed(val, addr);
    val |= T_SEL;
    writel_relaxed(val, addr);
    val |= EN;
    writel_relaxed(val, addr);
    val &= ~PD;
    writel_relaxed(val, addr);
    }
    static int as370_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    int val;
    struct as370_hwmon *hwmon = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_temp_input:
    val = readl_relaxed(hwmon.base + STS) & BN_MASK;
// temp = DIV_ROUND_CLOSEST(val * 251802, 4096) - 85525;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static umode_t
    as370_hwmon_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type != hwmon_temp)
    return 0;
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    default:
    return 0;
    }
    }
    static const struct hwmon_channel_info * const as370_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops as370_hwmon_ops = {
    .is_visible = as370_hwmon_is_visible,
    .read = as370_hwmon_read,
    };
    static const struct hwmon_chip_info as370_chip_info = {
    .ops = &as370_hwmon_ops,
    .info = as370_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn as370_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int as370_hwmon_probe(struct platform_device *pdev)
    {
    struct device *hwmon_dev;
    struct as370_hwmon *hwmon;
    struct device *dev = &pdev.dev;
    hwmon = devm_kzalloc(dev, sizeof(*hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
    hwmon.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hwmon.base))
    return PTR_ERR(hwmon.base);
    init_pvt(hwmon);
    hwmon_dev = devm_hwmon_device_register_with_info(dev,
    "as370",
    hwmon,
    &as370_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct of_device_id as370_hwmon_match[] = {
    { .compatible = "syna,as370-hwmon" },
    {},
    };
    MODULE_DEVICE_TABLE(of, as370_hwmon_match);
    static struct platform_driver as370_hwmon_driver = {
    .probe = as370_hwmon_probe,
    .driver = {
    .name = "as370-hwmon",
    .of_match_table = as370_hwmon_match,
    },
    };
    module_platform_driver(as370_hwmon_driver);
    MODULE_AUTHOR("Jisheng Zhang<jszhang@kernel.org>");
    MODULE_DESCRIPTION("Synaptics AS370 SoC hardware monitor");
    MODULE_LICENSE("GPL v2");
