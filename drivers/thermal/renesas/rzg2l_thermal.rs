//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/renesas/rzg2l_thermal.c
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
// Renesas RZ/G2L TSU Thermal Sensor Driver
//
// Copyright (C) 2021 Renesas Electronics Corporation
//

pub const CTEMP_MASK: c_uint = 0xFFF;
// default calibration values, if FUSE values are missing
pub const SW_CALIB0_VAL: c_int = 3148;
pub const SW_CALIB1_VAL: c_int = 503;
// Register offsets
pub const TSU_SM: c_uint = 0x00;
pub const TSU_ST: c_uint = 0x04;
pub const TSU_SAD: c_uint = 0x0C;
pub const TSU_SS: c_uint = 0x10;

// Sensor Mode Register(TSU_SM)

// TSU_ST bits

pub const RZG2L_TSU_SS_TIMEOUT_US: c_int = 1000;
pub const CURVATURE_CORRECTION_CONST: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_thermal_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub zone: *mut thermal_zone_device,
    pub rstc: *mut reset_control,
    pub calib1: u32 calib0,,
}

#[no_mangle]
pub unsafe extern "C" fn rzg2l_thermal_read(priv: *mut rzg2l_thermal_priv, reg: u32) -> u32 {
    static inline u32 rzg2l_thermal_read(struct rzg2l_thermal_priv *priv, u32 reg)
    {
    return ioread32(priv.base + reg);
    }
    static inline void rzg2l_thermal_write(struct rzg2l_thermal_priv *priv, u32 reg,
    u32 data)
    {
    iowrite32(data, priv.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int rzg2l_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct rzg2l_thermal_priv *priv = thermal_zone_device_priv(tz);
    let mut result: u32 = 0, dsensor, ts_code_ave;
    int val, i;
    for (i = 0; i < TS_CODE_CAP_TIMES ; i++) {
//
// TSU repeats measurement at 20 microseconds intervals and
// automatically updates the results of measurement. As per
// the HW manual for measuring temperature we need to read 8
// values consecutively and then take the average.
// ts_code_ave = (ts_code[0] + ⋯ + ts_code[7]) / 8
//
    result += rzg2l_thermal_read(priv, TSU_SAD) & CTEMP_MASK;
    usleep_range(20, 30);
    }
    ts_code_ave = result / TS_CODE_CAP_TIMES;
//
// Calculate actual sensor value by applying curvature correction formula
// dsensor = ts_code_ave / (1 + ts_code_ave * 0.000013). Here we are doing
// integer calculation by scaling all the values by 1000000.
//
    dsensor = TS_CODE_AVE_SCALE(ts_code_ave) /
    (TS_CODE_AVE_SCALE(1) + (ts_code_ave * CURVATURE_CORRECTION_CONST));
//
// The temperature Tj is calculated by the formula
// Tj = (dsensor − calib1) * 165/ (calib0 − calib1) − 40
// where calib0 and calib1 are the calibration values.
//
    val = ((dsensor - priv.calib1) * (MCELSIUS(165) /
    (priv.calib0 - priv.calib1))) - MCELSIUS(40);
// temp = roundup(val, RZG2L_THERMAL_GRAN);
    return 0;
    }
    static const struct thermal_zone_device_ops rzg2l_tz_of_ops = {
    .get_temp = rzg2l_thermal_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn rzg2l_thermal_init(priv: *mut rzg2l_thermal_priv) -> c_int {
    static int rzg2l_thermal_init(struct rzg2l_thermal_priv *priv)
    {
    u32 reg_val;
    rzg2l_thermal_write(priv, TSU_SM, TSU_SM_NORMAL_MODE);
    rzg2l_thermal_write(priv, TSU_ST, 0);
//
// Before setting the START bit, TSU should be in normal operating
// mode. As per the HW manual, it will take 60 µs to place the TSU
// into normal operating mode.
//
    usleep_range(60, 80);
    reg_val = rzg2l_thermal_read(priv, TSU_ST);
    reg_val |= TSU_ST_START;
    rzg2l_thermal_write(priv, TSU_ST, reg_val);
    return readl_poll_timeout(priv.base + TSU_SS, reg_val,
    reg_val == TSU_SS_CONV_RUNNING, 50,
    RZG2L_TSU_SS_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_thermal_reset_assert_pm_disable_put(pdev: *mut platform_device) {
    static void rzg2l_thermal_reset_assert_pm_disable_put(struct platform_device *pdev)
    {
    struct rzg2l_thermal_priv *priv = dev_get_drvdata(&pdev.dev);
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(priv.rstc);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_thermal_remove(pdev: *mut platform_device) {
    static void rzg2l_thermal_remove(struct platform_device *pdev)
    {
    struct rzg2l_thermal_priv *priv = dev_get_drvdata(&pdev.dev);
    thermal_remove_hwmon_sysfs(priv.zone);
    rzg2l_thermal_reset_assert_pm_disable_put(pdev);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int rzg2l_thermal_probe(struct platform_device *pdev)
    {
    struct thermal_zone_device *zone;
    struct rzg2l_thermal_priv *priv;
    struct device *dev = &pdev.dev;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.dev = dev;
    priv.rstc = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rstc))
    return dev_err_probe(dev, PTR_ERR(priv.rstc),
    "failed to get cpg reset");
    ret = reset_control_deassert(priv.rstc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to deassert");
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    priv.calib0 = rzg2l_thermal_read(priv, OTPTSUTRIM_REG(0));
    if (priv.calib0 & OTPTSUTRIM_EN_MASK)
    priv.calib0 &= OTPTSUTRIM_MASK;
    else
    priv.calib0 = SW_CALIB0_VAL;
    priv.calib1 = rzg2l_thermal_read(priv, OTPTSUTRIM_REG(1));
    if (priv.calib1 & OTPTSUTRIM_EN_MASK)
    priv.calib1 &= OTPTSUTRIM_MASK;
    else
    priv.calib1 = SW_CALIB1_VAL;
    platform_set_drvdata(pdev, priv);
    ret = rzg2l_thermal_init(priv);
    if (ret) {
    dev_err(dev, "Failed to start TSU");
    goto err;
    }
    zone = devm_thermal_of_zone_register(dev, 0, priv,
    &rzg2l_tz_of_ops);
    if (IS_ERR(zone)) {
    dev_err(dev, "Can't register thermal zone");
    ret = PTR_ERR(zone);
    goto err;
    }
    priv.zone = zone;
    ret = thermal_add_hwmon_sysfs(priv.zone);
    if (ret)
    goto err;
    dev_dbg(dev, "TSU probed with %s calibration values",
    rzg2l_thermal_read(priv, OTPTSUTRIM_REG(0)) ?  "hw" : "sw");
    return 0;
    err:
    rzg2l_thermal_reset_assert_pm_disable_put(pdev);
    return ret;
    }
    static const struct of_device_id rzg2l_thermal_dt_ids[] = {
    { .compatible = "renesas,rzg2l-tsu", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzg2l_thermal_dt_ids);
    static struct platform_driver rzg2l_thermal_driver = {
    .driver = {
    .name = "rzg2l_thermal",
    .of_match_table = rzg2l_thermal_dt_ids,
    },
    .probe = rzg2l_thermal_probe,
    .remove = rzg2l_thermal_remove,
    };
    module_platform_driver(rzg2l_thermal_driver);
    MODULE_DESCRIPTION("Renesas RZ/G2L TSU Thermal Sensor Driver");
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_LICENSE("GPL v2");
