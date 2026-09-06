//! Automatically rewritten from C to Rust
//! Source: drivers/phy/eswin/phy-eic7700-sata.c
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
// ESWIN SATA PHY driver
//
// Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
// All rights reserved.
//
// Authors: Yulin Lu <luyulin@eswincomputing.com>
//

pub const SATA_AXI_LP_CTRL: c_uint = 0x08;
pub const SATA_MPLL_CTRL: c_uint = 0x20;
pub const SATA_P0_PHY_STAT: c_uint = 0x24;
pub const SATA_PHY_CTRL0: c_uint = 0x28;
pub const SATA_PHY_CTRL1: c_uint = 0x2c;
pub const SATA_REF_CTRL: c_uint = 0x34;
pub const SATA_REF_CTRL1: c_uint = 0x38;
pub const SATA_LOS_IDEN: c_uint = 0x3c;

pub const SATA_P0_PHY_TX_AMPLITUDE_GEN1_DEFAULT: c_uint = 0x42;

pub const SATA_P0_PHY_TX_AMPLITUDE_GEN2_DEFAULT: c_uint = 0x46;

pub const SATA_P0_PHY_TX_AMPLITUDE_GEN3_DEFAULT: c_uint = 0x73;

pub const SATA_P0_PHY_TX_PREEMPH_GEN1_DEFAULT: c_uint = 0x5;

pub const SATA_P0_PHY_TX_PREEMPH_GEN2_DEFAULT: c_uint = 0x5;

pub const SATA_P0_PHY_TX_PREEMPH_GEN3_DEFAULT: c_uint = 0x23;

pub const PLL_LOCK_SLEEP_US: c_int = 10;
pub const PLL_LOCK_TIMEOUT_US: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_sata_phy {
    pub tx_amplitude_tuning_val: [u32; 3],
    pub tx_preemph_tuning_val: [u32; 3],
    pub rst: *mut reset_control,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub phy: *mut phy,
}

    static const struct regmap_config eic7700_sata_phy_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = SATA_LOS_IDEN,
    };
    static int wait_for_phy_ready(struct regmap *regmap, u32 reg, u32 checkbit,
    u32 status)
    {
    u32 val;
    int ret;
    ret = regmap_read_poll_timeout(regmap, reg, val,
    (val & checkbit) == status,
    PLL_LOCK_SLEEP_US, PLL_LOCK_TIMEOUT_US);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eic7700_sata_phy_init(phy: *mut phy) -> c_int {
    static int eic7700_sata_phy_init(struct phy *phy)
    {
    struct eic7700_sata_phy *sata_phy = phy_get_drvdata(phy);
    u32 val;
    int ret;
    ret = clk_prepare_enable(sata_phy.clk);
    if (ret)
    return ret;
    regmap_write(sata_phy.regmap, SATA_REF_CTRL1, SATA_CLK_RST_SOURCE_PHY);
    val = FIELD_PREP(SATA_P0_PHY_TX_AMPLITUDE_GEN1_MASK,
    sata_phy.tx_amplitude_tuning_val[0]) |
    FIELD_PREP(SATA_P0_PHY_TX_AMPLITUDE_GEN2_MASK,
    sata_phy.tx_amplitude_tuning_val[1]) |
    FIELD_PREP(SATA_P0_PHY_TX_AMPLITUDE_GEN3_MASK,
    sata_phy.tx_amplitude_tuning_val[2]);
    regmap_write(sata_phy.regmap, SATA_PHY_CTRL0, val);
    val = FIELD_PREP(SATA_P0_PHY_TX_PREEMPH_GEN1_MASK,
    sata_phy.tx_preemph_tuning_val[0]) |
    FIELD_PREP(SATA_P0_PHY_TX_PREEMPH_GEN2_MASK,
    sata_phy.tx_preemph_tuning_val[1]) |
    FIELD_PREP(SATA_P0_PHY_TX_PREEMPH_GEN3_MASK,
    sata_phy.tx_preemph_tuning_val[2]);
    regmap_write(sata_phy.regmap, SATA_PHY_CTRL1, val);
    val = FIELD_PREP(SATA_LOS_LEVEL_MASK, 0x9) |
    FIELD_PREP(SATA_LOS_BIAS_MASK, 0x2);
    regmap_write(sata_phy.regmap, SATA_LOS_IDEN, val);
    val = SATA_M_CSYSREQ | SATA_S_CSYSREQ;
    regmap_write(sata_phy.regmap, SATA_AXI_LP_CTRL, val);
    val = SATA_REF_REPEATCLK_EN | SATA_REF_USE_PAD;
    regmap_write(sata_phy.regmap, SATA_REF_CTRL, val);
    val = FIELD_PREP(SATA_MPLL_MULTIPLIER_MASK, 0x3c);
    regmap_write(sata_phy.regmap, SATA_MPLL_CTRL, val);
    usleep_range(15, 20);
    ret = reset_control_deassert(sata_phy.rst);
    if (ret)
    goto disable_clk;
    ret = wait_for_phy_ready(sata_phy.regmap, SATA_P0_PHY_STAT,
    SATA_P0_PHY_READY, 1);
    if (ret < 0) {
    dev_err(&sata_phy.phy.dev, "PHY READY check failed\n");
    goto disable_clk;
    }
    return 0;
    disable_clk:
    clk_disable_unprepare(sata_phy.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eic7700_sata_phy_exit(phy: *mut phy) -> c_int {
    static int eic7700_sata_phy_exit(struct phy *phy)
    {
    struct eic7700_sata_phy *sata_phy = phy_get_drvdata(phy);
    int ret;
    ret = reset_control_assert(sata_phy.rst);
    if (ret)
    return ret;
    clk_disable_unprepare(sata_phy.clk);
    return 0;
    }
    static const struct phy_ops eic7700_sata_phy_ops = {
    .init		= eic7700_sata_phy_init,
    .exit		= eic7700_sata_phy_exit,
    .owner		= THIS_MODULE,
    };
    static void eic7700_get_tuning_param(struct device_node *np,
    struct eic7700_sata_phy *sata_phy)
    {
    if (of_property_read_u32_array
    (np, "eswin,tx-amplitude-tuning",
    sata_phy.tx_amplitude_tuning_val,
    ARRAY_SIZE(sata_phy.tx_amplitude_tuning_val))) {
    sata_phy.tx_amplitude_tuning_val[0] =
    SATA_P0_PHY_TX_AMPLITUDE_GEN1_DEFAULT;
    sata_phy.tx_amplitude_tuning_val[1] =
    SATA_P0_PHY_TX_AMPLITUDE_GEN2_DEFAULT;
    sata_phy.tx_amplitude_tuning_val[2] =
    SATA_P0_PHY_TX_AMPLITUDE_GEN3_DEFAULT;
    }
    if (of_property_read_u32_array
    (np, "eswin,tx-preemph-tuning",
    sata_phy.tx_preemph_tuning_val,
    ARRAY_SIZE(sata_phy.tx_preemph_tuning_val))) {
    sata_phy.tx_preemph_tuning_val[0] =
    SATA_P0_PHY_TX_PREEMPH_GEN1_DEFAULT;
    sata_phy.tx_preemph_tuning_val[1] =
    SATA_P0_PHY_TX_PREEMPH_GEN2_DEFAULT;
    sata_phy.tx_preemph_tuning_val[2] =
    SATA_P0_PHY_TX_PREEMPH_GEN3_DEFAULT;
    }
    }
#[no_mangle]
unsafe extern "C" fn eic7700_sata_phy_probe(pdev: *mut platform_device) -> c_int {
    static int eic7700_sata_phy_probe(struct platform_device *pdev)
    {
    struct eic7700_sata_phy *sata_phy;
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct resource *res;
    void __iomem *regs;
    sata_phy = devm_kzalloc(dev, sizeof(*sata_phy), GFP_KERNEL);
    if (!sata_phy)
    return -ENOMEM;
//
// Map the I/O resource with platform_get_resource and devm_ioremap
// instead of the devm_platform_ioremap_resource API, because the
// address region of the SATA-PHY falls into the region of the HSP
// clock & reset that has already been obtained by the HSP
// clock-and-reset driver.
//
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOENT;
    regs = devm_ioremap(dev, res.start, resource_size(res));
    if (!regs)
    return -ENOMEM;
    sata_phy.regmap = devm_regmap_init_mmio
    (dev, regs, &eic7700_sata_phy_regmap_config);
    if (IS_ERR(sata_phy.regmap))
    return dev_err_probe(dev, PTR_ERR(sata_phy.regmap),
    "failed to init regmap\n");
    dev_set_drvdata(dev, sata_phy);
    eic7700_get_tuning_param(np, sata_phy);
    sata_phy.clk = devm_clk_get(dev, "phy");
    if (IS_ERR(sata_phy.clk))
    return PTR_ERR(sata_phy.clk);
    sata_phy.rst = devm_reset_control_array_get_exclusive(dev);
    if (IS_ERR(sata_phy.rst))
    return dev_err_probe(dev, PTR_ERR(sata_phy.rst),
    "failed to get reset control\n");
    sata_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &eic7700_sata_phy_ops);
    if (IS_ERR(sata_phy.phy))
    return dev_err_probe(dev, PTR_ERR(sata_phy.phy),
    "failed to create PHY\n");
    phy_set_drvdata(sata_phy.phy, sata_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return dev_err_probe(dev, PTR_ERR(phy_provider),
    "failed to register PHY provider\n");
    return 0;
    }
    static const struct of_device_id eic7700_sata_phy_of_match[] = {
    { .compatible = "eswin,eic7700-sata-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, eic7700_sata_phy_of_match);
    static struct platform_driver eic7700_sata_phy_driver = {
    .probe	= eic7700_sata_phy_probe,
    .driver = {
    .of_match_table	= eic7700_sata_phy_of_match,
    .name  = "eic7700-sata-phy",
    }
    };
    module_platform_driver(eic7700_sata_phy_driver);
    MODULE_DESCRIPTION("SATA PHY driver for the ESWIN EIC7700 SoC");
    MODULE_AUTHOR("Yulin Lu <luyulin@eswincomputing.com>");
    MODULE_LICENSE("GPL");
