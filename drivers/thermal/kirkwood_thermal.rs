//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/kirkwood_thermal.c
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
// Kirkwood thermal sensor driver
//
// Copyright (C) 2012 Nobuhiro Iwamatsu <iwamatsu@nigauri.org>
//

pub const KIRKWOOD_THERMAL_VALID_OFFSET: c_int = 9;
pub const KIRKWOOD_THERMAL_VALID_MASK: c_uint = 0x1;
pub const KIRKWOOD_THERMAL_TEMP_OFFSET: c_int = 10;
pub const KIRKWOOD_THERMAL_TEMP_MASK: c_uint = 0x1FF;
// Kirkwood Thermal Sensor Dev Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirkwood_thermal_priv {
    pub sensor: *mut void __iomem,
}

    static int kirkwood_get_temp(struct thermal_zone_device *thermal,
    int *temp)
    {
    unsigned long reg;
    struct kirkwood_thermal_priv *priv = thermal_zone_device_priv(thermal);
    reg = readl_relaxed(priv.sensor);
// Valid check
    if (!((reg >> KIRKWOOD_THERMAL_VALID_OFFSET) &
    KIRKWOOD_THERMAL_VALID_MASK))
    return -EIO;
//
// Calculate temperature. According to Marvell internal
// documentation the formula for this is:
// Celsius = (322-reg)/1.3625
//
    reg = (reg >> KIRKWOOD_THERMAL_TEMP_OFFSET) &
    KIRKWOOD_THERMAL_TEMP_MASK;
// temp = ((3220000000UL - (10000000UL * reg)) / 13625);
    return 0;
    }
    static const struct thermal_zone_device_ops ops = {
    .get_temp = kirkwood_get_temp,
    };
    static const struct of_device_id kirkwood_thermal_id_table[] = {
    { .compatible = "marvell,kirkwood-thermal" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn kirkwood_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int kirkwood_thermal_probe(struct platform_device *pdev)
    {
    struct thermal_zone_device *thermal = core::ptr::null_mut();
    struct kirkwood_thermal_priv *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.sensor = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(priv.sensor))
    return PTR_ERR(priv.sensor);
    thermal = thermal_tripless_zone_device_register("kirkwood_thermal",
    priv, &ops, core::ptr::null_mut());
    if (IS_ERR(thermal)) {
    dev_err(&pdev.dev,
    "Failed to register thermal zone device\n");
    return PTR_ERR(thermal);
    }
    ret = thermal_zone_device_enable(thermal);
    if (ret) {
    thermal_zone_device_unregister(thermal);
    dev_err(&pdev.dev, "Failed to enable thermal zone device\n");
    return ret;
    }
    platform_set_drvdata(pdev, thermal);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kirkwood_thermal_exit(pdev: *mut platform_device) {
    static void kirkwood_thermal_exit(struct platform_device *pdev)
    {
    struct thermal_zone_device *kirkwood_thermal =
    platform_get_drvdata(pdev);
    thermal_zone_device_unregister(kirkwood_thermal);
    }
    MODULE_DEVICE_TABLE(of, kirkwood_thermal_id_table);
    static struct platform_driver kirkwood_thermal_driver = {
    .probe = kirkwood_thermal_probe,
    .remove = kirkwood_thermal_exit,
    .driver = {
    .name = "kirkwood_thermal",
    .of_match_table = kirkwood_thermal_id_table,
    },
    };
    module_platform_driver(kirkwood_thermal_driver);
    MODULE_AUTHOR("Nobuhiro Iwamatsu <iwamatsu@nigauri.org>");
    MODULE_DESCRIPTION("kirkwood thermal driver");
    MODULE_LICENSE("GPL");
