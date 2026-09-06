//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/gxp-fan-ctrl.c
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
// Copyright (C) 2023 Hewlett-Packard Enterprise Development Company, L.P.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_fan_ctrl_drvdata {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn gxp_pwm_write(dev: *mut device, attr: u32, channel: c_int, val: c_long) -> c_int {
    static int gxp_pwm_write(struct device *dev, u32 attr, int channel, long val)
    {
    struct gxp_fan_ctrl_drvdata *drvdata = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_pwm_input:
    if (val > 255 || val < 0)
    return -EINVAL;
    writeb(val, drvdata.base + channel);
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int gxp_fan_ctrl_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    switch (type) {
    case hwmon_pwm:
    return gxp_pwm_write(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn gxp_pwm_read(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int gxp_pwm_read(struct device *dev, u32 attr, int channel, long *val)
    {
    struct gxp_fan_ctrl_drvdata *drvdata = dev_get_drvdata(dev);
// val = readb(drvdata->base + channel);
    return 0;
    }
    static int gxp_fan_ctrl_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_pwm:
    return gxp_pwm_read(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t gxp_fan_ctrl_is_visible(const void *_data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    let mut mode: umode_t = 0;
    switch (type) {
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    mode = 0644;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return mode;
    }
    static const struct hwmon_ops gxp_fan_ctrl_ops = {
    .is_visible = gxp_fan_ctrl_is_visible,
    .read = gxp_fan_ctrl_read,
    .write = gxp_fan_ctrl_write,
    };
    static const struct hwmon_channel_info * const gxp_fan_ctrl_info[] = {
    HWMON_CHANNEL_INFO(pwm,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT,
    HWMON_PWM_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info gxp_fan_ctrl_chip_info = {
    .ops = &gxp_fan_ctrl_ops,
    .info = gxp_fan_ctrl_info,
    };
#[no_mangle]
unsafe extern "C" fn gxp_fan_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int gxp_fan_ctrl_probe(struct platform_device *pdev)
    {
    struct gxp_fan_ctrl_drvdata *drvdata;
    struct device *dev = &pdev.dev;
    struct device *hwmon_dev;
    drvdata = devm_kzalloc(dev, sizeof(struct gxp_fan_ctrl_drvdata),
    GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(drvdata.base))
    return dev_err_probe(dev, PTR_ERR(drvdata.base),
    "failed to map base\n");
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev,
    "hpe_gxp_fan_ctrl",
    drvdata,
    &gxp_fan_ctrl_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct of_device_id gxp_fan_ctrl_of_match[] = {
    { .compatible = "hpe,gxp-fan-ctrl", },
    {},
    };
    MODULE_DEVICE_TABLE(of, gxp_fan_ctrl_of_match);
    static struct platform_driver gxp_fan_ctrl_driver = {
    .probe		= gxp_fan_ctrl_probe,
    .driver = {
    .name	= "gxp-fan-ctrl",
    .of_match_table = gxp_fan_ctrl_of_match,
    },
    };
    module_platform_driver(gxp_fan_ctrl_driver);
    MODULE_AUTHOR("Nick Hawkins <nick.hawkins@hpe.com>");
    MODULE_DESCRIPTION("HPE GXP fan controller");
    MODULE_LICENSE("GPL");
