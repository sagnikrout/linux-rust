//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/apcs-msm8996.c
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
// Qualcomm APCS clock controller driver
//
// Copyright (c) 2022, Linaro Limited
// Author: Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
//

pub const APCS_AUX_OFFSET: c_uint = 0x50;

pub const APCS_AUX_DIV_2: c_uint = 0x1;
#[no_mangle]
unsafe extern "C" fn qcom_apcs_msm8996_clk_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_apcs_msm8996_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *parent = dev.parent;
    struct regmap *regmap;
    struct clk_hw *hw;
    unsigned int val;
    let mut ret: c_int = -ENODEV;
    regmap = dev_get_regmap(parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(dev, "failed to get regmap: %d\n", ret);
    return ret;
    }
    regmap_read(regmap, APCS_AUX_OFFSET, &val);
    regmap_update_bits(regmap, APCS_AUX_OFFSET, APCS_AUX_DIV_MASK,
    FIELD_PREP(APCS_AUX_DIV_MASK, APCS_AUX_DIV_2));
//
// This clock is used during CPU cluster setup while setting up CPU PLLs.
// Add hardware mandated delay to make sure that the sys_apcs_aux clock
// is stable (after setting the divider) before continuing
// bootstrapping to keep CPUs from ending up in a weird state.
//
    udelay(5);
//
// As this clocks is a parent of the CPU cluster clocks and is actually
// used as a parent during CPU clocks setup, we want for it to register
// as early as possible, without letting fw_devlink to delay probing of
// either of the drivers.
//
// The sys_apcs_aux is a child (divider) of gpll0, but we register it
// as a fixed rate clock instead to ease bootstrapping procedure. By
// doing this we make sure that CPU cluster clocks are able to be setup
// early during the boot process (as it is recommended by Qualcomm).
//
    hw = devm_clk_hw_register_fixed_rate(dev, "sys_apcs_aux", core::ptr::null_mut(), 0, 300000000);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, hw);
    }
    static struct platform_driver qcom_apcs_msm8996_clk_driver = {
    .probe = qcom_apcs_msm8996_clk_probe,
    .driver = {
    .name = "qcom-apcs-msm8996-clk",
    },
    };
// Register early enough to fix the clock to be used for other cores
#[no_mangle]
unsafe extern "C" fn qcom_apcs_msm8996_clk_init() -> int __init {
    static int __init qcom_apcs_msm8996_clk_init(void)
    {
    return platform_driver_register(&qcom_apcs_msm8996_clk_driver);
    }
    postcore_initcall(qcom_apcs_msm8996_clk_init);
#[no_mangle]
unsafe extern "C" fn qcom_apcs_msm8996_clk_exit() -> void __exit {
    static void __exit qcom_apcs_msm8996_clk_exit(void)
    {
    platform_driver_unregister(&qcom_apcs_msm8996_clk_driver);
    }
    module_exit(qcom_apcs_msm8996_clk_exit);
    MODULE_AUTHOR("Dmitry Baryshkov <dmitry.baryshkov@linaro.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Qualcomm MSM8996 APCS clock driver");
