//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/dove_thermal.c
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
// Dove thermal sensor driver
//
// Copyright (C) 2013 Andrew Lunn <andrew@lunn.ch>
//

pub const DOVE_THERMAL_TEMP_OFFSET: c_int = 1;
pub const DOVE_THERMAL_TEMP_MASK: c_uint = 0x1FF;
// Dove Thermal Manager Control and Status Register
pub const PMU_TM_DISABLE_OFFS: c_int = 0;

// Dove Theraml Diode Control 0 Register

pub const PMU_TDC0_SEL_VCAL_OFFS: c_int = 5;

pub const PMU_TDC0_REF_CAL_CNT_OFFS: c_int = 11;

pub const PMU_TDC0_AVG_NUM_OFFS: c_int = 25;

// Dove Thermal Diode Control 1 Register
pub const PMU_TEMP_DIOD_CTRL1_REG: c_uint = 0x04;

// Dove Thermal Sensor Dev Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dove_thermal_priv {
    pub sensor: *mut void __iomem,
    pub control: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn dove_init_sensor(priv: *const dove_thermal_priv) -> c_int {
    static int dove_init_sensor(const struct dove_thermal_priv *priv)
    {
    u32 reg;
    u32 i;
// Configure the Diode Control Register #0
    reg = readl_relaxed(priv.control);
// Use average of 2
    reg &= ~PMU_TDC0_AVG_NUM_MASK;
    reg |= (0x1 << PMU_TDC0_AVG_NUM_OFFS);
// Reference calibration value
    reg &= ~PMU_TDC0_REF_CAL_CNT_MASK;
    reg |= (0x0F1 << PMU_TDC0_REF_CAL_CNT_OFFS);
// Set the high level reference for calibration
    reg &= ~PMU_TDC0_SEL_VCAL_MASK;
    reg |= (0x2 << PMU_TDC0_SEL_VCAL_OFFS);
    writel(reg, priv.control);
// Reset the sensor
    reg = readl_relaxed(priv.control);
    writel((reg | PMU_TDC0_SW_RST_MASK), priv.control);
    writel(reg, priv.control);
// Enable the sensor
    reg = readl_relaxed(priv.sensor);
    reg &= ~PMU_TM_DISABLE_MASK;
    writel(reg, priv.sensor);
// Poll the sensor for the first reading
    for (i = 0; i < 1000000; i++) {
    reg = readl_relaxed(priv.sensor);
    if (reg & DOVE_THERMAL_TEMP_MASK)
    break;
    }
    if (i == 1000000)
    return -EIO;
    return 0;
    }
    static int dove_get_temp(struct thermal_zone_device *thermal,
    int *temp)
    {
    unsigned long reg;
    struct dove_thermal_priv *priv = thermal_zone_device_priv(thermal);
// Valid check
    reg = readl_relaxed(priv.control + PMU_TEMP_DIOD_CTRL1_REG);
    if ((reg & PMU_TDC1_TEMP_VALID_MASK) == 0x0)
    return -EIO;
//
// Calculate temperature. According to Marvell internal
// documentation the formula for this is:
// Celsius = (322-reg)/1.3625
//
    reg = readl_relaxed(priv.sensor);
    reg = (reg >> DOVE_THERMAL_TEMP_OFFSET) & DOVE_THERMAL_TEMP_MASK;
// temp = ((3220000000UL - (10000000UL * reg)) / 13625);
    return 0;
    }
    static const struct thermal_zone_device_ops ops = {
    .get_temp = dove_get_temp,
    };
    static const struct of_device_id dove_thermal_id_table[] = {
    { .compatible = "marvell,dove-thermal" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn dove_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int dove_thermal_probe(struct platform_device *pdev)
    {
    struct thermal_zone_device *thermal = core::ptr::null_mut();
    struct dove_thermal_priv *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.sensor = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(priv.sensor))
    return PTR_ERR(priv.sensor);
    priv.control = devm_platform_get_and_ioremap_resource(pdev, 1, core::ptr::null_mut());
    if (IS_ERR(priv.control))
    return PTR_ERR(priv.control);
    ret = dove_init_sensor(priv);
    if (ret) {
    dev_err(&pdev.dev, "Failed to initialize sensor\n");
    return ret;
    }
    thermal = thermal_tripless_zone_device_register("dove_thermal", priv,
    &ops, core::ptr::null_mut());
    if (IS_ERR(thermal)) {
    dev_err(&pdev.dev,
    "Failed to register thermal zone device\n");
    return PTR_ERR(thermal);
    }
    ret = thermal_zone_device_enable(thermal);
    if (ret) {
    thermal_zone_device_unregister(thermal);
    return ret;
    }
    platform_set_drvdata(pdev, thermal);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dove_thermal_exit(pdev: *mut platform_device) {
    static void dove_thermal_exit(struct platform_device *pdev)
    {
    struct thermal_zone_device *dove_thermal =
    platform_get_drvdata(pdev);
    thermal_zone_device_unregister(dove_thermal);
    }
    MODULE_DEVICE_TABLE(of, dove_thermal_id_table);
    static struct platform_driver dove_thermal_driver = {
    .probe = dove_thermal_probe,
    .remove = dove_thermal_exit,
    .driver = {
    .name = "dove_thermal",
    .of_match_table = dove_thermal_id_table,
    },
    };
    module_platform_driver(dove_thermal_driver);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_DESCRIPTION("Dove thermal driver");
    MODULE_LICENSE("GPL");
