//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/broadcom/bcm2711_thermal.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Broadcom AVS RO thermal sensor driver
//
// based on brcmstb_thermal
//
// Copyright (C) 2020 Stefan Wahren
//

pub const AVS_RO_TEMP_STATUS: c_uint = 0x200;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2711_thermal_priv {
    pub regmap: *mut regmap,
    pub thermal: *mut thermal_zone_device,
}

#[no_mangle]
unsafe extern "C" fn bcm2711_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int bcm2711_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct bcm2711_thermal_priv *priv = thermal_zone_device_priv(tz);
    let mut slope: c_int = thermal_zone_get_slope(tz);
    let mut offset: c_int = thermal_zone_get_offset(tz);
    u32 val;
    int ret;
    ret = regmap_read(priv.regmap, AVS_RO_TEMP_STATUS, &val);
    if (ret)
    return ret;
    if (!(val & AVS_RO_TEMP_STATUS_VALID_MSK))
    return -EIO;
    val &= AVS_RO_TEMP_STATUS_DATA_MSK;
// Convert a HW code to a temperature reading (millidegree celsius)
// temp = slope * val + offset;
    return 0;
    }
    static const struct thermal_zone_device_ops bcm2711_thermal_of_ops = {
    .get_temp	= bcm2711_get_temp,
    };
    static const struct of_device_id bcm2711_thermal_id_table[] = {
    { .compatible = "brcm,bcm2711-thermal" },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2711_thermal_id_table);
#[no_mangle]
unsafe extern "C" fn bcm2711_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2711_thermal_probe(struct platform_device *pdev)
    {
    struct thermal_zone_device *thermal;
    struct bcm2711_thermal_priv *priv;
    struct device *dev = &pdev.dev;
    struct device_node *parent;
    struct regmap *regmap;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// get regmap from syscon node
    parent = of_get_parent(dev.of_node); /* parent should be syscon node */
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(dev, "failed to get regmap: %d\n", ret);
    return ret;
    }
    priv.regmap = regmap;
    thermal = devm_thermal_of_zone_register(dev, 0, priv,
    &bcm2711_thermal_of_ops);
    if (IS_ERR(thermal)) {
    ret = PTR_ERR(thermal);
    dev_err(dev, "could not register sensor: %d\n", ret);
    return ret;
    }
    priv.thermal = thermal;
    return thermal_add_hwmon_sysfs(thermal);
    }
    static struct platform_driver bcm2711_thermal_driver = {
    .probe = bcm2711_thermal_probe,
    .driver = {
    .name = "bcm2711_thermal",
    .of_match_table = bcm2711_thermal_id_table,
    },
    };
    module_platform_driver(bcm2711_thermal_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Stefan Wahren");
    MODULE_DESCRIPTION("Broadcom AVS RO thermal sensor driver");
