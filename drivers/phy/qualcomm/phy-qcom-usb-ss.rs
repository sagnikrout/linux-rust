//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-usb-ss.c
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
// Copyright (c) 2012-2014,2017 The Linux Foundation. All rights reserved.
// Copyright (c) 2018-2020, Linaro Limited
//

pub const PHY_CTRL0: c_uint = 0x6C;
pub const PHY_CTRL1: c_uint = 0x70;
pub const PHY_CTRL2: c_uint = 0x74;
pub const PHY_CTRL4: c_uint = 0x7C;
// PHY_CTRL bits

pub const NUM_BULK_CLKS: c_int = 3;
pub const NUM_BULK_REGS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssphy_priv {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub reset_com: *mut reset_control,
    pub reset_phy: *mut reset_control,
    pub regs: [regulator_bulk_data; NUM_BULK_REGS],
    pub clks: [clk_bulk_data; NUM_BULK_CLKS],
    pub mode: enum phy_mode,
}

#[no_mangle]
pub unsafe extern "C" fn qcom_ssphy_updatel(addr: *mut void __iomem, mask: u32, val: u32) {
    static inline void qcom_ssphy_updatel(void __iomem *addr, u32 mask, u32 val)
    {
    writel((readl(addr) & ~mask) | val, addr);
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_do_reset(priv: *mut ssphy_priv) -> c_int {
    static int qcom_ssphy_do_reset(struct ssphy_priv *priv)
    {
    int ret;
    if (!priv.reset_com) {
    qcom_ssphy_updatel(priv.base + PHY_CTRL1, PHY_RESET,
    PHY_RESET);
    usleep_range(10, 20);
    qcom_ssphy_updatel(priv.base + PHY_CTRL1, PHY_RESET, 0);
    } else {
    ret = reset_control_assert(priv.reset_com);
    if (ret) {
    dev_err(priv.dev, "Failed to assert reset com\n");
    return ret;
    }
    ret = reset_control_assert(priv.reset_phy);
    if (ret) {
    dev_err(priv.dev, "Failed to assert reset phy\n");
    return ret;
    }
    usleep_range(10, 20);
    ret = reset_control_deassert(priv.reset_com);
    if (ret) {
    dev_err(priv.dev, "Failed to deassert reset com\n");
    return ret;
    }
    ret = reset_control_deassert(priv.reset_phy);
    if (ret) {
    dev_err(priv.dev, "Failed to deassert reset phy\n");
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_power_on(phy: *mut phy) -> c_int {
    static int qcom_ssphy_power_on(struct phy *phy)
    {
    struct ssphy_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = regulator_bulk_enable(NUM_BULK_REGS, priv.regs);
    if (ret)
    return ret;
    ret = clk_bulk_prepare_enable(NUM_BULK_CLKS, priv.clks);
    if (ret)
    goto err_disable_regulator;
    ret = qcom_ssphy_do_reset(priv);
    if (ret)
    goto err_disable_clock;
    writeb(SWI_PCS_CLK_SEL, priv.base + PHY_CTRL0);
    qcom_ssphy_updatel(priv.base + PHY_CTRL4, LANE0_PWR_ON, LANE0_PWR_ON);
    qcom_ssphy_updatel(priv.base + PHY_CTRL2, REF_PHY_EN, REF_PHY_EN);
    qcom_ssphy_updatel(priv.base + PHY_CTRL4, TST_PWR_DOWN, 0);
    return 0;
    err_disable_clock:
    clk_bulk_disable_unprepare(NUM_BULK_CLKS, priv.clks);
    err_disable_regulator:
    regulator_bulk_disable(NUM_BULK_REGS, priv.regs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_power_off(phy: *mut phy) -> c_int {
    static int qcom_ssphy_power_off(struct phy *phy)
    {
    struct ssphy_priv *priv = phy_get_drvdata(phy);
    qcom_ssphy_updatel(priv.base + PHY_CTRL4, LANE0_PWR_ON, 0);
    qcom_ssphy_updatel(priv.base + PHY_CTRL2, REF_PHY_EN, 0);
    qcom_ssphy_updatel(priv.base + PHY_CTRL4, TST_PWR_DOWN, TST_PWR_DOWN);
    clk_bulk_disable_unprepare(NUM_BULK_CLKS, priv.clks);
    regulator_bulk_disable(NUM_BULK_REGS, priv.regs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_init_clock(priv: *mut ssphy_priv) -> c_int {
    static int qcom_ssphy_init_clock(struct ssphy_priv *priv)
    {
    priv.clks[0].id = "ref";
    priv.clks[1].id = "ahb";
    priv.clks[2].id = "pipe";
    return devm_clk_bulk_get(priv.dev, NUM_BULK_CLKS, priv.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_init_regulator(priv: *mut ssphy_priv) -> c_int {
    static int qcom_ssphy_init_regulator(struct ssphy_priv *priv)
    {
    int ret;
    priv.regs[0].supply = "vdd";
    priv.regs[1].supply = "vdda1p8";
    ret = devm_regulator_bulk_get(priv.dev, NUM_BULK_REGS, priv.regs);
    if (ret) {
    if (ret != -EPROBE_DEFER)
    dev_err(priv.dev, "Failed to get regulators\n");
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_init_reset(priv: *mut ssphy_priv) -> c_int {
    static int qcom_ssphy_init_reset(struct ssphy_priv *priv)
    {
    priv.reset_com = devm_reset_control_get_optional_exclusive(priv.dev, "com");
    if (IS_ERR(priv.reset_com)) {
    dev_err(priv.dev, "Failed to get reset control com\n");
    return PTR_ERR(priv.reset_com);
    }
    if (priv.reset_com) {
// if reset_com is present, reset_phy is no longer optional
    priv.reset_phy = devm_reset_control_get_exclusive(priv.dev, "phy");
    if (IS_ERR(priv.reset_phy)) {
    dev_err(priv.dev, "Failed to get reset control phy\n");
    return PTR_ERR(priv.reset_phy);
    }
    }
    return 0;
    }
    static const struct phy_ops qcom_ssphy_ops = {
    .power_off = qcom_ssphy_power_off,
    .power_on = qcom_ssphy_power_on,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn qcom_ssphy_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_ssphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *provider;
    struct ssphy_priv *priv;
    struct phy *phy;
    int ret;
    priv = devm_kzalloc(dev, sizeof(struct ssphy_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.mode = PHY_MODE_INVALID;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    ret = qcom_ssphy_init_clock(priv);
    if (ret)
    return ret;
    ret = qcom_ssphy_init_reset(priv);
    if (ret)
    return ret;
    ret = qcom_ssphy_init_regulator(priv);
    if (ret)
    return ret;
    phy = devm_phy_create(dev, dev.of_node, &qcom_ssphy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "Failed to create the SS phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, priv);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static const struct of_device_id qcom_ssphy_match[] = {
    { .compatible = "qcom,usb-ss-28nm-phy", },
    { },
    };
    MODULE_DEVICE_TABLE(of, qcom_ssphy_match);
    static struct platform_driver qcom_ssphy_driver = {
    .probe		= qcom_ssphy_probe,
    .driver = {
    .name	= "qcom-usb-ssphy",
    .of_match_table = qcom_ssphy_match,
    },
    };
    module_platform_driver(qcom_ssphy_driver);
    MODULE_DESCRIPTION("Qualcomm SuperSpeed USB PHY driver");
    MODULE_LICENSE("GPL v2");
