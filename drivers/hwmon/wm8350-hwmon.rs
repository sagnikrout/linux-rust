//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/wm8350-hwmon.c
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
// drivers/hwmon/wm8350-hwmon.c - Wolfson Microelectronics WM8350 PMIC
// hardware monitoring features.
//
// Copyright (C) 2009 Wolfson Microelectronics plc
//

    static const char * const input_names[] = {
    [WM8350_AUXADC_USB]  = "USB",
    [WM8350_AUXADC_LINE] = "Line",
    [WM8350_AUXADC_BATT] = "Battery",
    };
    static ssize_t show_voltage(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct wm8350 *wm8350 = dev_get_drvdata(dev);
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    int val;
    val = wm8350_read_auxadc(wm8350, channel, 0, 0) * WM8350_AUX_COEFF;
    val = DIV_ROUND_CLOSEST(val, 1000);
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t show_label(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    return sprintf(buf, "%s\n", input_names[channel]);
    }

    static SENSOR_DEVICE_ATTR(in##id##_input, S_IRUGO, show_voltage,\
    core::ptr::null_mut(), name);		\
    static SENSOR_DEVICE_ATTR(in##id##_label, S_IRUGO, show_label,	\
    core::ptr::null_mut(), name)
    WM8350_NAMED_VOLTAGE(0, WM8350_AUXADC_USB);
    WM8350_NAMED_VOLTAGE(1, WM8350_AUXADC_BATT);
    WM8350_NAMED_VOLTAGE(2, WM8350_AUXADC_LINE);
    static struct attribute *wm8350_attrs[] = {
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in0_label.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_label.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_label.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(wm8350);
#[no_mangle]
unsafe extern "C" fn wm8350_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int wm8350_hwmon_probe(struct platform_device *pdev)
    {
    struct wm8350 *wm8350 = platform_get_drvdata(pdev);
    struct device *hwmon_dev;
    hwmon_dev = devm_hwmon_device_register_with_groups(&pdev.dev, "wm8350",
    wm8350,
    wm8350_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver wm8350_hwmon_driver = {
    .probe = wm8350_hwmon_probe,
    .driver = {
    .name = "wm8350-hwmon",
    },
    };
    module_platform_driver(wm8350_hwmon_driver);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("WM8350 Hardware Monitoring");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm8350-hwmon");
