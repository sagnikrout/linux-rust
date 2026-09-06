//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltq-cputemp.c
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
// Lantiq cpu temperature sensor driver
//
// Copyright (C) 2017 Florian Eckert <fe@dev.tdt.de>
//

// gphy1 configuration register contains cpu temperature
pub const CGU_GPHY1_CR: c_uint = 0x0040;

#[no_mangle]
unsafe extern "C" fn ltq_cputemp_enable() {
    static void ltq_cputemp_enable(void)
    {
    ltq_cgu_w32(ltq_cgu_r32(CGU_GPHY1_CR) | CGU_TEMP_PD, CGU_GPHY1_CR);
    }
#[no_mangle]
unsafe extern "C" fn ltq_cputemp_disable(data: *mut c_void) {
    static void ltq_cputemp_disable(void *data)
    {
    ltq_cgu_w32(ltq_cgu_r32(CGU_GPHY1_CR) & ~CGU_TEMP_PD, CGU_GPHY1_CR);
    }
    static int ltq_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    int value;
    switch (attr) {
    case hwmon_temp_input:
// get the temperature including one decimal place
    value = (ltq_cgu_r32(CGU_GPHY1_CR) >> 9) & 0x01FF;
    value = value * 5;
// range -38 to +154 °C, register value zero is -38.0 °C
    value -= 380;
// scale temp to millidegree
    value = value * 100;
    break;
    default:
    return -EOPNOTSUPP;
    }
// temp = value;
    return 0;
    }
    static umode_t ltq_is_visible(const void *_data, enum hwmon_sensor_types type,
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
    static const struct hwmon_channel_info * const ltq_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ltq_hwmon_ops = {
    .is_visible = ltq_is_visible,
    .read = ltq_read,
    };
    static const struct hwmon_chip_info ltq_chip_info = {
    .ops = &ltq_hwmon_ops,
    .info = ltq_info,
    };
#[no_mangle]
unsafe extern "C" fn ltq_cputemp_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_cputemp_probe(struct platform_device *pdev)
    {
    struct device *hwmon_dev;
    let mut err: c_int = 0;
// available on vr9 v1.2 SoCs only
    if (ltq_soc_type() != SOC_TYPE_VR9_2)
    return -ENODEV;
    err = devm_add_action(&pdev.dev, ltq_cputemp_disable, core::ptr::null_mut());
    if (err)
    return err;
    ltq_cputemp_enable();
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev,
    "ltq_cputemp",
    core::ptr::null_mut(),
    &ltq_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev)) {
    dev_err(&pdev.dev, "Failed to register as hwmon device");
    return PTR_ERR(hwmon_dev);
    }
    return 0;
    }
    const struct of_device_id ltq_cputemp_match[] = {
    { .compatible = "lantiq,cputemp" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltq_cputemp_match);
    static struct platform_driver ltq_cputemp_driver = {
    .probe = ltq_cputemp_probe,
    .driver = {
    .name = "ltq-cputemp",
    .of_match_table = ltq_cputemp_match,
    },
    };
    module_platform_driver(ltq_cputemp_driver);
    MODULE_AUTHOR("Florian Eckert <fe@dev.tdt.de>");
    MODULE_DESCRIPTION("Lantiq cpu temperature sensor driver");
    MODULE_LICENSE("GPL");
