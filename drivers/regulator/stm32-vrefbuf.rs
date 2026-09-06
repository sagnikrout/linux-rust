//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/stm32-vrefbuf.c
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
// Copyright (C) STMicroelectronics 2017
//
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>
//

// STM32 VREFBUF registers
pub const STM32_VREFBUF_CSR: c_uint = 0x00;
// STM32 VREFBUF CSR bitfields

pub const STM32_VREFBUF_AUTO_SUSPEND_DELAY_MS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_vrefbuf {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
}

    static const unsigned int stm32_vrefbuf_voltages[] = {
// Matches resp. VRS = 000b, 001b, 010b, 011b
    2500000, 2048000, 1800000, 1500000,
    };
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_enable(rdev: *mut regulator_dev) -> c_int {
    static int stm32_vrefbuf_enable(struct regulator_dev *rdev)
    {
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    u32 val;
    int ret;
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return ret;
    val = readl_relaxed(priv.base + STM32_VREFBUF_CSR);
    val = (val & ~STM32_HIZ) | STM32_ENVR;
    writel_relaxed(val, priv.base + STM32_VREFBUF_CSR);
//
// Vrefbuf startup time depends on external capacitor: wait here for
// VRR to be set. That means output has reached expected value.
// ~650us sleep should be enough for caps up to 1.5uF. Use 10ms as
// arbitrary timeout.
//
    ret = readl_poll_timeout(priv.base + STM32_VREFBUF_CSR, val,
    val & STM32_VRR, 650, 10000);
    if (ret) {
    dev_err(&rdev.dev, "stm32 vrefbuf timed out!\n");
    val = readl_relaxed(priv.base + STM32_VREFBUF_CSR);
    val = (val & ~STM32_ENVR) | STM32_HIZ;
    writel_relaxed(val, priv.base + STM32_VREFBUF_CSR);
    }
    pm_runtime_put_autosuspend(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_disable(rdev: *mut regulator_dev) -> c_int {
    static int stm32_vrefbuf_disable(struct regulator_dev *rdev)
    {
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    u32 val;
    int ret;
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return ret;
    val = readl_relaxed(priv.base + STM32_VREFBUF_CSR);
    val &= ~STM32_ENVR;
    writel_relaxed(val, priv.base + STM32_VREFBUF_CSR);
    pm_runtime_put_autosuspend(priv.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int stm32_vrefbuf_is_enabled(struct regulator_dev *rdev)
    {
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    int ret;
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return ret;
    ret = readl_relaxed(priv.base + STM32_VREFBUF_CSR) & STM32_ENVR;
    pm_runtime_put_autosuspend(priv.dev);
    return ret;
    }
    static int stm32_vrefbuf_set_voltage_sel(struct regulator_dev *rdev,
    unsigned sel)
    {
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    u32 val;
    int ret;
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return ret;
    val = readl_relaxed(priv.base + STM32_VREFBUF_CSR);
    val = (val & ~STM32_VRS) | FIELD_PREP(STM32_VRS, sel);
    writel_relaxed(val, priv.base + STM32_VREFBUF_CSR);
    pm_runtime_put_autosuspend(priv.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int stm32_vrefbuf_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    u32 val;
    int ret;
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return ret;
    val = readl_relaxed(priv.base + STM32_VREFBUF_CSR);
    ret = FIELD_GET(STM32_VRS, val);
    pm_runtime_put_autosuspend(priv.dev);
    return ret;
    }
    static const struct regulator_ops stm32_vrefbuf_volt_ops = {
    .enable		= stm32_vrefbuf_enable,
    .disable	= stm32_vrefbuf_disable,
    .is_enabled	= stm32_vrefbuf_is_enabled,
    .get_voltage_sel = stm32_vrefbuf_get_voltage_sel,
    .set_voltage_sel = stm32_vrefbuf_set_voltage_sel,
    .list_voltage	= regulator_list_voltage_table,
    };
    static const struct regulator_desc stm32_vrefbuf_regu = {
    .name = "vref",
    .supply_name = "vdda",
    .volt_table = stm32_vrefbuf_voltages,
    .n_voltages = ARRAY_SIZE(stm32_vrefbuf_voltages),
    .ops = &stm32_vrefbuf_volt_ops,
    .off_on_delay = 1000,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_vrefbuf_probe(struct platform_device *pdev)
    {
    struct stm32_vrefbuf *priv;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev,
    STM32_VREFBUF_AUTO_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(&pdev.dev, "clk prepare failed with error %d\n", ret);
    goto err_pm_stop;
    }
    config.dev = &pdev.dev;
    config.driver_data = priv;
    config.of_node = pdev.dev.of_node;
    config.init_data = of_get_regulator_init_data(&pdev.dev,
    pdev.dev.of_node,
    &stm32_vrefbuf_regu);
    rdev = regulator_register(&pdev.dev, &stm32_vrefbuf_regu, &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(&pdev.dev, "register failed with error %d\n", ret);
    goto err_clk_dis;
    }
    platform_set_drvdata(pdev, rdev);
    pm_runtime_put_autosuspend(&pdev.dev);
    return 0;
    err_clk_dis:
    clk_disable_unprepare(priv.clk);
    err_pm_stop:
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_remove(pdev: *mut platform_device) {
    static void stm32_vrefbuf_remove(struct platform_device *pdev)
    {
    struct regulator_dev *rdev = platform_get_drvdata(pdev);
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    pm_runtime_get_sync(&pdev.dev);
    regulator_unregister(rdev);
    clk_disable_unprepare(priv.clk);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    };
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_vrefbuf_runtime_suspend(struct device *dev)
    {
    struct regulator_dev *rdev = dev_get_drvdata(dev);
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_vrefbuf_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_vrefbuf_runtime_resume(struct device *dev)
    {
    struct regulator_dev *rdev = dev_get_drvdata(dev);
    struct stm32_vrefbuf *priv = rdev_get_drvdata(rdev);
    return clk_prepare_enable(priv.clk);
    }
    static const struct dev_pm_ops stm32_vrefbuf_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    SET_RUNTIME_PM_OPS(stm32_vrefbuf_runtime_suspend,
    stm32_vrefbuf_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id __maybe_unused stm32_vrefbuf_of_match[] = {
    { .compatible = "st,stm32-vrefbuf", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_vrefbuf_of_match);
    static struct platform_driver stm32_vrefbuf_driver = {
    .probe = stm32_vrefbuf_probe,
    .remove = stm32_vrefbuf_remove,
    .driver = {
    .name  = "stm32-vrefbuf",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(stm32_vrefbuf_of_match),
    .pm = &stm32_vrefbuf_pm_ops,
    },
    };
    module_platform_driver(stm32_vrefbuf_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Fabrice Gasnier <fabrice.gasnier@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 VREFBUF driver");
    MODULE_ALIAS("platform:stm32-vrefbuf");
