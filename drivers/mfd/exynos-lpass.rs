//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/exynos-lpass.c
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
// Copyright (C) 2015 - 2016 Samsung Electronics Co., Ltd.
//
// Authors: Inha Song <ideal.song@samsung.com>
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//
// Samsung Exynos SoC series Low Power Audio Subsystem driver.
//
// This module provides regmap for the Top SFR region and instantiates
// devices for IP blocks like DMAC, I2S, UART.
//

// LPASS Top register definitions
pub const SFR_LPASS_CORE_SW_RESET: c_uint = 0x08;

pub const SFR_LPASS_INTR_CA5_MASK: c_uint = 0x48;
pub const SFR_LPASS_INTR_CPU_MASK: c_uint = 0x58;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_lpass {
// pointer to the LPASS TOP regmap
    pub top: *mut regmap,
    pub sfr0_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn exynos_lpass_core_sw_reset(lpass: *mut exynos_lpass, mask: c_int) {
    static void exynos_lpass_core_sw_reset(struct exynos_lpass *lpass, int mask)
    {
    let mut val: c_uint = 0;
    regmap_read(lpass.top, SFR_LPASS_CORE_SW_RESET, &val);
    val &= ~mask;
    regmap_write(lpass.top, SFR_LPASS_CORE_SW_RESET, val);
    usleep_range(100, 150);
    val |= mask;
    regmap_write(lpass.top, SFR_LPASS_CORE_SW_RESET, val);
    }
#[no_mangle]
unsafe extern "C" fn exynos_lpass_enable(lpass: *mut exynos_lpass) {
    static void exynos_lpass_enable(struct exynos_lpass *lpass)
    {
    clk_prepare_enable(lpass.sfr0_clk);
// Unmask SFR, DMA and I2S interrupt
    regmap_write(lpass.top, SFR_LPASS_INTR_CA5_MASK,
    LPASS_INTR_SFR | LPASS_INTR_DMA | LPASS_INTR_I2S);
    regmap_write(lpass.top, SFR_LPASS_INTR_CPU_MASK,
    LPASS_INTR_SFR | LPASS_INTR_DMA | LPASS_INTR_I2S |
    LPASS_INTR_UART);
    exynos_lpass_core_sw_reset(lpass, LPASS_I2S_SW_RESET);
    exynos_lpass_core_sw_reset(lpass, LPASS_DMA_SW_RESET);
    exynos_lpass_core_sw_reset(lpass, LPASS_MEM_SW_RESET);
    exynos_lpass_core_sw_reset(lpass, LPASS_UART_SW_RESET);
    }
#[no_mangle]
unsafe extern "C" fn exynos_lpass_disable(lpass: *mut exynos_lpass) {
    static void exynos_lpass_disable(struct exynos_lpass *lpass)
    {
// Mask any unmasked IP interrupt sources
    regmap_write(lpass.top, SFR_LPASS_INTR_CPU_MASK, 0);
    regmap_write(lpass.top, SFR_LPASS_INTR_CA5_MASK, 0);
    clk_disable_unprepare(lpass.sfr0_clk);
    }
    static const struct regmap_config exynos_lpass_reg_conf = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .max_register	= 0xfc,
    };
#[no_mangle]
unsafe extern "C" fn exynos_lpass_disable_lpass(data: *mut c_void) {
    static void exynos_lpass_disable_lpass(void *data)
    {
    struct platform_device *pdev = data;
    struct exynos_lpass *lpass = platform_get_drvdata(pdev);
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    exynos_lpass_disable(lpass);
    }
#[no_mangle]
unsafe extern "C" fn exynos_lpass_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_lpass_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct exynos_lpass *lpass;
    void __iomem *base_top;
    int ret;
    lpass = devm_kzalloc(dev, sizeof(*lpass), GFP_KERNEL);
    if (!lpass)
    return -ENOMEM;
    base_top = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base_top))
    return PTR_ERR(base_top);
    lpass.sfr0_clk = devm_clk_get(dev, "sfr0_ctrl");
    if (IS_ERR(lpass.sfr0_clk))
    return PTR_ERR(lpass.sfr0_clk);
    lpass.top = devm_regmap_init_mmio(dev, base_top,
    &exynos_lpass_reg_conf);
    if (IS_ERR(lpass.top)) {
    dev_err(dev, "LPASS top regmap initialization failed\n");
    return PTR_ERR(lpass.top);
    }
    platform_set_drvdata(pdev, lpass);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    exynos_lpass_enable(lpass);
    ret = devm_add_action_or_reset(dev, exynos_lpass_disable_lpass, pdev);
    if (ret)
    return ret;
    return devm_of_platform_populate(dev);
    }
#[no_mangle]
unsafe extern "C" fn exynos_lpass_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_lpass_suspend(struct device *dev)
    {
    struct exynos_lpass *lpass = dev_get_drvdata(dev);
    exynos_lpass_disable(lpass);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_lpass_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_lpass_resume(struct device *dev)
    {
    struct exynos_lpass *lpass = dev_get_drvdata(dev);
    exynos_lpass_enable(lpass);
    return 0;
    }
    static const struct dev_pm_ops lpass_pm_ops = {
    SET_RUNTIME_PM_OPS(exynos_lpass_suspend, exynos_lpass_resume, core::ptr::null_mut())
    SET_LATE_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id exynos_lpass_of_match[] = {
    { .compatible = "samsung,exynos5433-lpass" },
    { },
    };
    MODULE_DEVICE_TABLE(of, exynos_lpass_of_match);
    static struct platform_driver exynos_lpass_driver = {
    .driver	= {
    .name		= "exynos-lpass",
    .pm		= &lpass_pm_ops,
    .of_match_table	= exynos_lpass_of_match,
    },
    .probe	= exynos_lpass_probe,
    };
    module_platform_driver(exynos_lpass_driver);
    MODULE_DESCRIPTION("Samsung Low Power Audio Subsystem driver");
    MODULE_LICENSE("GPL v2");
