//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/stm32-pwr.c
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
// Copyright (C) STMicroelectronics 2019
// Authors: Gabriel Fernandez <gabriel.fernandez@st.com>
// Pascal Paillet <p.paillet@st.com>.

//
// Registers description
//
pub const REG_PWR_CR3: c_uint = 0x0C;

// list of supported regulators
    enum {
    PWR_REG11,
    PWR_REG18,
    PWR_USB33,
    STM32PWR_REG_NUM_REGS
    };
    static u32 ready_mask_table[STM32PWR_REG_NUM_REGS] = {
    [PWR_REG11] = REG_1_1_RDY,
    [PWR_REG18] = REG_1_8_RDY,
    [PWR_USB33] = USB_3_3_RDY,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_pwr_reg {
    pub base: *mut void __iomem,
    pub ready_mask: u32,
}

#[no_mangle]
unsafe extern "C" fn stm32_pwr_reg_is_ready(rdev: *mut regulator_dev) -> c_int {
    static int stm32_pwr_reg_is_ready(struct regulator_dev *rdev)
    {
    struct stm32_pwr_reg *priv = rdev_get_drvdata(rdev);
    u32 val;
    val = readl_relaxed(priv.base + REG_PWR_CR3);
    return (val & priv.ready_mask);
    }
#[no_mangle]
unsafe extern "C" fn stm32_pwr_reg_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int stm32_pwr_reg_is_enabled(struct regulator_dev *rdev)
    {
    struct stm32_pwr_reg *priv = rdev_get_drvdata(rdev);
    u32 val;
    val = readl_relaxed(priv.base + REG_PWR_CR3);
    return (val & rdev.desc.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn stm32_pwr_reg_enable(rdev: *mut regulator_dev) -> c_int {
    static int stm32_pwr_reg_enable(struct regulator_dev *rdev)
    {
    struct stm32_pwr_reg *priv = rdev_get_drvdata(rdev);
    int ret;
    u32 val;
    val = readl_relaxed(priv.base + REG_PWR_CR3);
    val |= rdev.desc.enable_mask;
    writel_relaxed(val, priv.base + REG_PWR_CR3);
// use an arbitrary timeout of 20ms
    ret = readx_poll_timeout(stm32_pwr_reg_is_ready, rdev, val, val,
    100, 20 * 1000);
    if (ret)
    dev_err(&rdev.dev, "regulator enable timed out!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_pwr_reg_disable(rdev: *mut regulator_dev) -> c_int {
    static int stm32_pwr_reg_disable(struct regulator_dev *rdev)
    {
    struct stm32_pwr_reg *priv = rdev_get_drvdata(rdev);
    int ret;
    u32 val;
    val = readl_relaxed(priv.base + REG_PWR_CR3);
    val &= ~rdev.desc.enable_mask;
    writel_relaxed(val, priv.base + REG_PWR_CR3);
// use an arbitrary timeout of 20ms
    ret = readx_poll_timeout(stm32_pwr_reg_is_enabled, rdev, val, !val,
    100, 20 * 1000);
    if (ret)
    dev_err(&rdev.dev, "regulator disable timed out!\n");
    return ret;
    }
    static const struct regulator_ops stm32_pwr_reg_ops = {
    .enable		= stm32_pwr_reg_enable,
    .disable	= stm32_pwr_reg_disable,
    .is_enabled	= stm32_pwr_reg_is_enabled,
    };

    [_id] = { \
    .id = _id, \
    .name = _name, \
    .of_match = of_match_ptr(_name), \
    .n_voltages = 1, \
    .type = REGULATOR_VOLTAGE, \
    .fixed_uV = _volt, \
    .ops = &stm32_pwr_reg_ops, \
    .enable_mask = _en, \
    .owner = THIS_MODULE, \
    .supply_name = _supply, \
    } \
    static const struct regulator_desc stm32_pwr_desc[] = {
    PWR_REG(PWR_REG11, "reg11", 1100000, REG_1_1_EN, "vdd"),
    PWR_REG(PWR_REG18, "reg18", 1800000, REG_1_8_EN, "vdd"),
    PWR_REG(PWR_USB33, "usb33", 3300000, USB_3_3_EN, "vdd_3v3_usbfs"),
    };
#[no_mangle]
unsafe extern "C" fn stm32_pwr_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_pwr_regulator_probe(struct platform_device *pdev)
    {
    struct stm32_pwr_reg *priv;
    void __iomem *base;
    struct regulator_dev *rdev;
    let mut config: regulator_config = { };
    int i, ret = 0;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base)) {
    dev_err(&pdev.dev, "Unable to map IO memory\n");
    return PTR_ERR(base);
    }
    config.dev = &pdev.dev;
    for (i = 0; i < STM32PWR_REG_NUM_REGS; i++) {
    priv = devm_kzalloc(&pdev.dev, sizeof(struct stm32_pwr_reg),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = base;
    priv.ready_mask = ready_mask_table[i];
    config.driver_data = priv;
    rdev = devm_regulator_register(&pdev.dev,
    &stm32_pwr_desc[i],
    &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(&pdev.dev,
    "Failed to register regulator: %d\n", ret);
    break;
    }
    }
    return ret;
    }
    static const struct of_device_id __maybe_unused stm32_pwr_of_match[] = {
    { .compatible = "st,stm32mp1,pwr-reg", },
    { .compatible = "st,stm32mp13-pwr-reg", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_pwr_of_match);
    static struct platform_driver stm32_pwr_driver = {
    .probe = stm32_pwr_regulator_probe,
    .driver = {
    .name  = "stm32-pwr-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(stm32_pwr_of_match),
    },
    };
    module_platform_driver(stm32_pwr_driver);
    MODULE_DESCRIPTION("STM32MP1 PWR voltage regulator driver");
    MODULE_AUTHOR("Pascal Paillet <p.paillet@st.com>");
