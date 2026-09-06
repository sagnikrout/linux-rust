//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/sparx5-temp.c
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
// Sparx5 SoC temperature sensor driver
//
// Copyright (C) 2020 Lars Povlsen <lars.povlsen@microchip.com>
//

pub const TEMP_CTRL: c_int = 0;
pub const TEMP_CFG: c_int = 4;

pub const TEMP_STAT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5_hwmon {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn s5_temp_enable(hwmon: *mut s5_hwmon) {
    static void s5_temp_enable(struct s5_hwmon *hwmon)
    {
    let mut val: u32 = readl(hwmon.base + TEMP_CFG);
    let mut clk: u32 = clk_get_rate(hwmon.clk) / USEC_PER_SEC;
    val &= ~TEMP_CFG_CYCLES;
    val |= FIELD_PREP(TEMP_CFG_CYCLES, clk);
    val |= TEMP_CFG_ENA;
    writel(val, hwmon.base + TEMP_CFG);
    }
    static int s5_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    struct s5_hwmon *hwmon = dev_get_drvdata(dev);
    let mut rc: c_int = 0, value;
    u32 stat;
    switch (attr) {
    case hwmon_temp_input:
    stat = readl_relaxed(hwmon.base + TEMP_STAT);
    if (!(stat & TEMP_STAT_VALID))
    return -EAGAIN;
    value = stat & TEMP_STAT_TEMP;
//
// From register documentation:
// Temp(C) = TEMP_SENSOR_STAT.TEMP / 4096 * 352.2 - 109.4
//
    value = DIV_ROUND_CLOSEST(value * 3522, 4096) - 1094;
//
// Scale down by 10 from above and multiply by 1000 to
// have millidegrees as specified by the hwmon sysfs
// interface.
//
    value *= 100;
// temp = value;
    break;
    default:
    rc = -EOPNOTSUPP;
    break;
    }
    return rc;
    }
    static umode_t s5_is_visible(const void *_data, enum hwmon_sensor_types type,
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
    static const struct hwmon_channel_info * const s5_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops s5_hwmon_ops = {
    .is_visible = s5_is_visible,
    .read = s5_read,
    };
    static const struct hwmon_chip_info s5_chip_info = {
    .ops = &s5_hwmon_ops,
    .info = s5_info,
    };
#[no_mangle]
unsafe extern "C" fn s5_temp_probe(pdev: *mut platform_device) -> c_int {
    static int s5_temp_probe(struct platform_device *pdev)
    {
    struct device *hwmon_dev;
    struct s5_hwmon *hwmon;
    hwmon = devm_kzalloc(&pdev.dev, sizeof(*hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
    hwmon.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hwmon.base))
    return PTR_ERR(hwmon.base);
    hwmon.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(hwmon.clk))
    return PTR_ERR(hwmon.clk);
    s5_temp_enable(hwmon);
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev,
    "s5_temp",
    hwmon,
    &s5_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct of_device_id s5_temp_match[] = {
    { .compatible = "microchip,sparx5-temp" },
    {},
    };
    MODULE_DEVICE_TABLE(of, s5_temp_match);
    static struct platform_driver s5_temp_driver = {
    .probe = s5_temp_probe,
    .driver = {
    .name = "sparx5-temp",
    .of_match_table = s5_temp_match,
    },
    };
    module_platform_driver(s5_temp_driver);
    MODULE_AUTHOR("Lars Povlsen <lars.povlsen@microchip.com>");
    MODULE_DESCRIPTION("Sparx5 SoC temperature sensor driver");
    MODULE_LICENSE("GPL");
