//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/wm831x-hwmon.c
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
// drivers/hwmon/wm831x-hwmon.c - Wolfson Microelectronics WM831x PMIC
// hardware monitoring features.
//
// Copyright (C) 2009 Wolfson Microelectronics plc
//

    static const char * const input_names[] = {
    [WM831X_AUX_SYSVDD]    = "SYSVDD",
    [WM831X_AUX_USB]       = "USB",
    [WM831X_AUX_BKUP_BATT] = "Backup battery",
    [WM831X_AUX_BATT]      = "Battery",
    [WM831X_AUX_WALL]      = "WALL",
    [WM831X_AUX_CHIP_TEMP] = "PMIC",
    [WM831X_AUX_BATT_TEMP] = "Battery",
    };
    static ssize_t show_voltage(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct wm831x *wm831x = dev_get_drvdata(dev);
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    int ret;
    ret = wm831x_auxadc_read_uv(wm831x, channel);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%d\n", DIV_ROUND_CLOSEST(ret, 1000));
    }
    static ssize_t show_chip_temp(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct wm831x *wm831x = dev_get_drvdata(dev);
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    int ret;
    ret = wm831x_auxadc_read(wm831x, channel);
    if (ret < 0)
    return ret;
// Degrees celsius = (512.18-ret) / 1.0983
    ret = 512180 - (ret * 1000);
    ret = DIV_ROUND_CLOSEST(ret * 10000, 10983);
    return sprintf(buf, "%d\n", ret);
    }
    static ssize_t show_label(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    return sprintf(buf, "%s\n", input_names[channel]);
    }

    static SENSOR_DEVICE_ATTR(in##id##_input, S_IRUGO, show_voltage, \
    core::ptr::null_mut(), name)

    WM831X_VOLTAGE(id, name); \
    static SENSOR_DEVICE_ATTR(in##id##_label, S_IRUGO, show_label,	\
    core::ptr::null_mut(), name)
    WM831X_VOLTAGE(0, WM831X_AUX_AUX1);
    WM831X_VOLTAGE(1, WM831X_AUX_AUX2);
    WM831X_VOLTAGE(2, WM831X_AUX_AUX3);
    WM831X_VOLTAGE(3, WM831X_AUX_AUX4);
    WM831X_NAMED_VOLTAGE(4, WM831X_AUX_SYSVDD);
    WM831X_NAMED_VOLTAGE(5, WM831X_AUX_USB);
    WM831X_NAMED_VOLTAGE(6, WM831X_AUX_BATT);
    WM831X_NAMED_VOLTAGE(7, WM831X_AUX_WALL);
    WM831X_NAMED_VOLTAGE(8, WM831X_AUX_BKUP_BATT);
    static SENSOR_DEVICE_ATTR(temp1_input, S_IRUGO, show_chip_temp, core::ptr::null_mut(),
    WM831X_AUX_CHIP_TEMP);
    static SENSOR_DEVICE_ATTR(temp1_label, S_IRUGO, show_label, core::ptr::null_mut(),
    WM831X_AUX_CHIP_TEMP);
//
// Report as a voltage since conversion depends on external components
// and that's what the ABI wants.
//
    static SENSOR_DEVICE_ATTR(temp2_input, S_IRUGO, show_voltage, core::ptr::null_mut(),
    WM831X_AUX_BATT_TEMP);
    static SENSOR_DEVICE_ATTR(temp2_label, S_IRUGO, show_label, core::ptr::null_mut(),
    WM831X_AUX_BATT_TEMP);
    static struct attribute *wm831x_attrs[] = {
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in4_label.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in5_label.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in6_label.dev_attr.attr,
    &sensor_dev_attr_in7_input.dev_attr.attr,
    &sensor_dev_attr_in7_label.dev_attr.attr,
    &sensor_dev_attr_in8_input.dev_attr.attr,
    &sensor_dev_attr_in8_label.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_label.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(wm831x);
#[no_mangle]
unsafe extern "C" fn wm831x_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int wm831x_hwmon_probe(struct platform_device *pdev)
    {
    struct wm831x *wm831x = dev_get_drvdata(pdev.dev.parent);
    struct device *hwmon_dev;
    hwmon_dev = devm_hwmon_device_register_with_groups(&pdev.dev, "wm831x",
    wm831x,
    wm831x_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver wm831x_hwmon_driver = {
    .probe = wm831x_hwmon_probe,
    .driver = {
    .name = "wm831x-hwmon",
    },
    };
    module_platform_driver(wm831x_hwmon_driver);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("WM831x Hardware Monitoring");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm831x-hwmon");
