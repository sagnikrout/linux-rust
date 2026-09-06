//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-ipq806x-sata.c
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_ipq806x_sata_phy {
    pub mmio: *mut void __iomem,
    pub cfg_clk: *mut clk,
    pub dev: *mut device,
}

pub const SATA_PHY_P0_PARAM0: c_uint = 0x200;

pub const SATA_PHY_P0_PARAM1: c_uint = 0x204;

pub const SATA_PHY_P0_PARAM2: c_uint = 0x208;

pub const SATA_PHY_P0_PARAM3: c_uint = 0x20C;
pub const SATA_PHY_SSC_EN: c_uint = 0x8;
pub const SATA_PHY_P0_PARAM4: c_uint = 0x210;
pub const SATA_PHY_REF_SSP_EN: c_uint = 0x2;
pub const SATA_PHY_RESET: c_uint = 0x1;
#[no_mangle]
unsafe extern "C" fn qcom_ipq806x_sata_phy_init(generic_phy: *mut phy) -> c_int {
    static int qcom_ipq806x_sata_phy_init(struct phy *generic_phy)
    {
    struct qcom_ipq806x_sata_phy *phy = phy_get_drvdata(generic_phy);
    u32 reg;
// Setting SSC_EN to 1
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM3);
    reg = reg | SATA_PHY_SSC_EN;
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM3);
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM0) &
    ~(SATA_PHY_P0_PARAM0_P0_TX_PREEMPH_GEN3_MASK |
    SATA_PHY_P0_PARAM0_P0_TX_PREEMPH_GEN2_MASK |
    SATA_PHY_P0_PARAM0_P0_TX_PREEMPH_GEN1_MASK);
    reg |= SATA_PHY_P0_PARAM0_P0_TX_PREEMPH_GEN3(0xf);
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM0);
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM1) &
    ~(SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN3_MASK |
    SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN2_MASK |
    SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN1_MASK);
    reg |= SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN3(0x55) |
    SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN2(0x55) |
    SATA_PHY_P0_PARAM1_P0_TX_AMPLITUDE_GEN1(0x55);
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM1);
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM2) &
    ~SATA_PHY_P0_PARAM2_RX_EQ_MASK;
    reg |= SATA_PHY_P0_PARAM2_RX_EQ(0x3);
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM2);
// Setting PHY_RESET to 1
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM4);
    reg = reg | SATA_PHY_RESET;
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM4);
// Setting REF_SSP_EN to 1
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM4);
    reg = reg | SATA_PHY_REF_SSP_EN | SATA_PHY_RESET;
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM4);
// make sure all changes complete before we let the PHY out of reset
    mb();
// sleep for max. 50us more to combine processor wakeups
    usleep_range(20, 20 + 50);
// Clearing PHY_RESET to 0
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM4);
    reg = reg & ~SATA_PHY_RESET;
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM4);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq806x_sata_phy_exit(generic_phy: *mut phy) -> c_int {
    static int qcom_ipq806x_sata_phy_exit(struct phy *generic_phy)
    {
    struct qcom_ipq806x_sata_phy *phy = phy_get_drvdata(generic_phy);
    u32 reg;
// Setting PHY_RESET to 1
    reg = readl_relaxed(phy.mmio + SATA_PHY_P0_PARAM4);
    reg = reg | SATA_PHY_RESET;
    writel_relaxed(reg, phy.mmio + SATA_PHY_P0_PARAM4);
    return 0;
    }
    static const struct phy_ops qcom_ipq806x_sata_phy_ops = {
    .init		= qcom_ipq806x_sata_phy_init,
    .exit		= qcom_ipq806x_sata_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn qcom_ipq806x_sata_phy_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_ipq806x_sata_phy_probe(struct platform_device *pdev)
    {
    struct qcom_ipq806x_sata_phy *phy;
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct phy *generic_phy;
    int ret;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.mmio))
    return PTR_ERR(phy.mmio);
    generic_phy = devm_phy_create(dev, core::ptr::null_mut(), &qcom_ipq806x_sata_phy_ops);
    if (IS_ERR(generic_phy)) {
    dev_err(dev, "%s: failed to create phy\n", __func__);
    return PTR_ERR(generic_phy);
    }
    phy.dev = dev;
    phy_set_drvdata(generic_phy, phy);
    platform_set_drvdata(pdev, phy);
    phy.cfg_clk = devm_clk_get(dev, "cfg");
    if (IS_ERR(phy.cfg_clk)) {
    dev_err(dev, "Failed to get sata cfg clock\n");
    return PTR_ERR(phy.cfg_clk);
    }
    ret = clk_prepare_enable(phy.cfg_clk);
    if (ret)
    return ret;
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    clk_disable_unprepare(phy.cfg_clk);
    dev_err(dev, "%s: failed to register phy\n", __func__);
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq806x_sata_phy_remove(pdev: *mut platform_device) {
    static void qcom_ipq806x_sata_phy_remove(struct platform_device *pdev)
    {
    struct qcom_ipq806x_sata_phy *phy = platform_get_drvdata(pdev);
    clk_disable_unprepare(phy.cfg_clk);
    }
    static const struct of_device_id qcom_ipq806x_sata_phy_of_match[] = {
    { .compatible = "qcom,ipq806x-sata-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, qcom_ipq806x_sata_phy_of_match);
    static struct platform_driver qcom_ipq806x_sata_phy_driver = {
    .probe = qcom_ipq806x_sata_phy_probe,
    .remove = qcom_ipq806x_sata_phy_remove,
    .driver = {
    .name = "qcom-ipq806x-sata-phy",
    .of_match_table = qcom_ipq806x_sata_phy_of_match,
    }
    };
    module_platform_driver(qcom_ipq806x_sata_phy_driver);
    MODULE_DESCRIPTION("QCOM IPQ806x SATA PHY driver");
    MODULE_LICENSE("GPL v2");
