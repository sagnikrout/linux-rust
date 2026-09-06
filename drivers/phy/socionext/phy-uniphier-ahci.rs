//! Automatically rewritten from C to Rust
//! Source: drivers/phy/socionext/phy-uniphier-ahci.c
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
// phy-uniphier-ahci.c - PHY driver for UniPhier AHCI controller
// Copyright 2016-2020, Socionext Inc.
// Author: Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_ahciphy_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clk_parent_gio: *mut *mut *mut clk clk, clk_parent,,
    pub rst_parent_gio: *mut *mut *mut reset_control rst, rst_parent,,
    pub rst_rx: *mut *mut *mut reset_control rst_pm, rst_tx,,
    pub data: *const uniphier_ahciphy_soc_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_ahciphy_soc_data {
    pub priv): *mut *mut int (init)(struct uniphier_ahciphy_priv,
    pub priv): *mut *mut int (power_on)(struct uniphier_ahciphy_priv,
    pub priv): *mut *mut int (power_off)(struct uniphier_ahciphy_priv,
    pub is_legacy: bool,
    pub is_ready_high: bool,
    pub is_phy_clk: bool,
}

// for Pro4
pub const CKCTRL0: c_uint = 0x0;

pub const CKCTRL1: c_uint = 0x4;

pub const RXTXCTRL: c_uint = 0x8;

pub const RSTPWR: c_uint = 0x30;

// for PXs2/PXs3
pub const CKCTRL: c_uint = 0x0;

pub const TXCTRL0: c_uint = 0x4;

pub const TXCTRL1: c_uint = 0x8;

pub const RXCTRL: c_uint = 0xc;

#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pro4_init(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pro4_init(struct uniphier_ahciphy_priv *priv)
    {
    u32 val;
// set phy MPLL parameters
    val = readl(priv.base + CKCTRL0);
    val &= ~CKCTRL0_NCY_MASK;
    val |= FIELD_PREP(CKCTRL0_NCY_MASK, 0x6);
    val &= ~CKCTRL0_NCY5_MASK;
    val |= FIELD_PREP(CKCTRL0_NCY5_MASK, 0x2);
    val &= ~CKCTRL0_PRESCALE_MASK;
    val |= FIELD_PREP(CKCTRL0_PRESCALE_MASK, 0x1);
    writel(val, priv.base + CKCTRL0);
// setup phy control parameters
    val = readl(priv.base + CKCTRL1);
    val &= ~CKCTRL1_LOS_LVL_MASK;
    val |= FIELD_PREP(CKCTRL1_LOS_LVL_MASK, 0x10);
    val &= ~CKCTRL1_TX_LVL_MASK;
    val |= FIELD_PREP(CKCTRL1_TX_LVL_MASK, 0x06);
    writel(val, priv.base + CKCTRL1);
    val = readl(priv.base + RXTXCTRL);
    val &= ~RXTXCTRL_RX_EQ_VALL_MASK;
    val |= FIELD_PREP(RXTXCTRL_RX_EQ_VALL_MASK, 0x6);
    val &= ~RXTXCTRL_RX_DPLL_MODE_MASK;
    val |= FIELD_PREP(RXTXCTRL_RX_DPLL_MODE_MASK, 0x3);
    val &= ~RXTXCTRL_TX_ATTEN_MASK;
    val |= FIELD_PREP(RXTXCTRL_TX_ATTEN_MASK, 0x3);
    val &= ~RXTXCTRL_TX_BOOST_MASK;
    val |= FIELD_PREP(RXTXCTRL_TX_BOOST_MASK, 0x5);
    val &= ~RXTXCTRL_TX_EDGERATE_MASK;
    val |= FIELD_PREP(RXTXCTRL_TX_EDGERATE_MASK, 0x0);
    writel(val, priv.base + RXTXCTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pro4_power_on(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pro4_power_on(struct uniphier_ahciphy_priv *priv)
    {
    u32 val;
    int ret;
// enable reference clock for phy
    val = readl(priv.base + CKCTRL0);
    val &= ~CKCTRL0_CK_OFF;
    writel(val, priv.base + CKCTRL0);
// enable TX clock
    val = readl(priv.base + RXTXCTRL);
    val |= RXTXCTRL_TX_CKO_EN;
    writel(val, priv.base + RXTXCTRL);
// wait until RX is ready
    ret = readl_poll_timeout(priv.base + RSTPWR, val,
    !(val & RSTPWR_RX_EN_VAL), 200, 2000);
    if (ret) {
    dev_err(priv.dev, "Failed to check whether Rx is ready\n");
    goto out_disable_clock;
    }
// release all reset
    ret = reset_control_deassert(priv.rst_pm);
    if (ret) {
    dev_err(priv.dev, "Failed to release PM reset\n");
    goto out_disable_clock;
    }
    ret = reset_control_deassert(priv.rst_tx);
    if (ret) {
    dev_err(priv.dev, "Failed to release Tx reset\n");
    goto out_reset_pm_assert;
    }
    ret = reset_control_deassert(priv.rst_rx);
    if (ret) {
    dev_err(priv.dev, "Failed to release Rx reset\n");
    goto out_reset_tx_assert;
    }
    return 0;
    out_reset_tx_assert:
    reset_control_assert(priv.rst_tx);
    out_reset_pm_assert:
    reset_control_assert(priv.rst_pm);
    out_disable_clock:
// disable TX clock
    val = readl(priv.base + RXTXCTRL);
    val &= ~RXTXCTRL_TX_CKO_EN;
    writel(val, priv.base + RXTXCTRL);
// disable reference clock for phy
    val = readl(priv.base + CKCTRL0);
    val |= CKCTRL0_CK_OFF;
    writel(val, priv.base + CKCTRL0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pro4_power_off(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pro4_power_off(struct uniphier_ahciphy_priv *priv)
    {
    u32 val;
    reset_control_assert(priv.rst_rx);
    reset_control_assert(priv.rst_tx);
    reset_control_assert(priv.rst_pm);
// disable TX clock
    val = readl(priv.base + RXTXCTRL);
    val &= ~RXTXCTRL_TX_CKO_EN;
    writel(val, priv.base + RXTXCTRL);
// disable reference clock for phy
    val = readl(priv.base + CKCTRL0);
    val |= CKCTRL0_CK_OFF;
    writel(val, priv.base + CKCTRL0);
    return 0;
    }
    static void uniphier_ahciphy_pxs2_enable(struct uniphier_ahciphy_priv *priv,
    bool enable)
    {
    u32 val;
    val = readl(priv.base + CKCTRL);
    if (enable) {
    val |= CKCTRL_REF_SSP_EN;
    writel(val, priv.base + CKCTRL);
    val &= ~CKCTRL_P0_RESET;
    writel(val, priv.base + CKCTRL);
    } else {
    val |= CKCTRL_P0_RESET;
    writel(val, priv.base + CKCTRL);
    val &= ~CKCTRL_REF_SSP_EN;
    writel(val, priv.base + CKCTRL);
    }
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pxs2_power_on(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pxs2_power_on(struct uniphier_ahciphy_priv *priv)
    {
    int ret;
    u32 val;
    uniphier_ahciphy_pxs2_enable(priv, true);
// wait until PLL is ready
    if (priv.data.is_ready_high)
    ret = readl_poll_timeout(priv.base + CKCTRL, val,
    (val & CKCTRL_P0_READY), 200, 400);
    else
    ret = readl_poll_timeout(priv.base + CKCTRL, val,
    !(val & CKCTRL_P0_READY), 200, 400);
    if (ret) {
    dev_err(priv.dev, "Failed to check whether PHY PLL is ready\n");
    uniphier_ahciphy_pxs2_enable(priv, false);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pxs2_power_off(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pxs2_power_off(struct uniphier_ahciphy_priv *priv)
    {
    uniphier_ahciphy_pxs2_enable(priv, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_pxs3_init(priv: *mut uniphier_ahciphy_priv) -> c_int {
    static int uniphier_ahciphy_pxs3_init(struct uniphier_ahciphy_priv *priv)
    {
    int i;
    u32 val;
// setup port parameter
    val = readl(priv.base + TXCTRL0);
    val &= ~TXCTRL0_AMP_G3_MASK;
    val |= FIELD_PREP(TXCTRL0_AMP_G3_MASK, 0x73);
    val &= ~TXCTRL0_AMP_G2_MASK;
    val |= FIELD_PREP(TXCTRL0_AMP_G2_MASK, 0x46);
    val &= ~TXCTRL0_AMP_G1_MASK;
    val |= FIELD_PREP(TXCTRL0_AMP_G1_MASK, 0x42);
    writel(val, priv.base + TXCTRL0);
    val = readl(priv.base + TXCTRL1);
    val &= ~TXCTRL1_DEEMPH_G3_MASK;
    val |= FIELD_PREP(TXCTRL1_DEEMPH_G3_MASK, 0x23);
    val &= ~TXCTRL1_DEEMPH_G2_MASK;
    val |= FIELD_PREP(TXCTRL1_DEEMPH_G2_MASK, 0x05);
    val &= ~TXCTRL1_DEEMPH_G1_MASK;
    val |= FIELD_PREP(TXCTRL1_DEEMPH_G1_MASK, 0x05);
    val = readl(priv.base + RXCTRL);
    val &= ~RXCTRL_LOS_LVL_MASK;
    val |= FIELD_PREP(RXCTRL_LOS_LVL_MASK, 0x9);
    val &= ~RXCTRL_LOS_BIAS_MASK;
    val |= FIELD_PREP(RXCTRL_LOS_BIAS_MASK, 0x2);
    val &= ~RXCTRL_RX_EQ_MASK;
    val |= FIELD_PREP(RXCTRL_RX_EQ_MASK, 0x1);
// dummy read 25 times to make a wait time for the phy to stabilize
    for (i = 0; i < 25; i++)
    readl(priv.base + CKCTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_init(phy: *mut phy) -> c_int {
    static int uniphier_ahciphy_init(struct phy *phy)
    {
    struct uniphier_ahciphy_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = clk_prepare_enable(priv.clk_parent_gio);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clk_parent);
    if (ret)
    goto out_clk_gio_disable;
    ret = reset_control_deassert(priv.rst_parent_gio);
    if (ret)
    goto out_clk_disable;
    ret = reset_control_deassert(priv.rst_parent);
    if (ret)
    goto out_rst_gio_assert;
    if (priv.data.init) {
    ret = priv.data.init(priv);
    if (ret)
    goto out_rst_assert;
    }
    return 0;
    out_rst_assert:
    reset_control_assert(priv.rst_parent);
    out_rst_gio_assert:
    reset_control_assert(priv.rst_parent_gio);
    out_clk_disable:
    clk_disable_unprepare(priv.clk_parent);
    out_clk_gio_disable:
    clk_disable_unprepare(priv.clk_parent_gio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_exit(phy: *mut phy) -> c_int {
    static int uniphier_ahciphy_exit(struct phy *phy)
    {
    struct uniphier_ahciphy_priv *priv = phy_get_drvdata(phy);
    reset_control_assert(priv.rst_parent);
    reset_control_assert(priv.rst_parent_gio);
    clk_disable_unprepare(priv.clk_parent);
    clk_disable_unprepare(priv.clk_parent_gio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_power_on(phy: *mut phy) -> c_int {
    static int uniphier_ahciphy_power_on(struct phy *phy)
    {
    struct uniphier_ahciphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    ret = reset_control_deassert(priv.rst);
    if (ret)
    goto out_clk_disable;
    if (priv.data.power_on) {
    ret = priv.data.power_on(priv);
    if (ret)
    goto out_reset_assert;
    }
    return 0;
    out_reset_assert:
    reset_control_assert(priv.rst);
    out_clk_disable:
    clk_disable_unprepare(priv.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_power_off(phy: *mut phy) -> c_int {
    static int uniphier_ahciphy_power_off(struct phy *phy)
    {
    struct uniphier_ahciphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.data.power_off)
    ret = priv.data.power_off(priv);
    reset_control_assert(priv.rst);
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    static const struct phy_ops uniphier_ahciphy_ops = {
    .init  = uniphier_ahciphy_init,
    .exit  = uniphier_ahciphy_exit,
    .power_on  = uniphier_ahciphy_power_on,
    .power_off = uniphier_ahciphy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_ahciphy_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_ahciphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_ahciphy_priv *priv;
    struct phy *phy;
    struct phy_provider *phy_provider;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.data = of_device_get_match_data(dev);
    if (WARN_ON(!priv.data))
    return -EINVAL;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.clk_parent = devm_clk_get(dev, "link");
    if (IS_ERR(priv.clk_parent))
    return PTR_ERR(priv.clk_parent);
    if (priv.data.is_phy_clk) {
    priv.clk = devm_clk_get(dev, "phy");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    }
    priv.rst_parent = devm_reset_control_get_shared(dev, "link");
    if (IS_ERR(priv.rst_parent))
    return PTR_ERR(priv.rst_parent);
    priv.rst = devm_reset_control_get_shared(dev, "phy");
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    if (priv.data.is_legacy) {
    priv.clk_parent_gio = devm_clk_get(dev, "gio");
    if (IS_ERR(priv.clk_parent_gio))
    return PTR_ERR(priv.clk_parent_gio);
    priv.rst_parent_gio =
    devm_reset_control_get_shared(dev, "gio");
    if (IS_ERR(priv.rst_parent_gio))
    return PTR_ERR(priv.rst_parent_gio);
    priv.rst_pm = devm_reset_control_get_shared(dev, "pm");
    if (IS_ERR(priv.rst_pm))
    return PTR_ERR(priv.rst_pm);
    priv.rst_tx = devm_reset_control_get_shared(dev, "tx");
    if (IS_ERR(priv.rst_tx))
    return PTR_ERR(priv.rst_tx);
    priv.rst_rx = devm_reset_control_get_shared(dev, "rx");
    if (IS_ERR(priv.rst_rx))
    return PTR_ERR(priv.rst_rx);
    }
    phy = devm_phy_create(dev, dev.of_node, &uniphier_ahciphy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
    static const struct uniphier_ahciphy_soc_data uniphier_pro4_data = {
    .init = uniphier_ahciphy_pro4_init,
    .power_on  = uniphier_ahciphy_pro4_power_on,
    .power_off = uniphier_ahciphy_pro4_power_off,
    .is_legacy = true,
    .is_phy_clk = false,
    };
    static const struct uniphier_ahciphy_soc_data uniphier_pxs2_data = {
    .power_on  = uniphier_ahciphy_pxs2_power_on,
    .power_off = uniphier_ahciphy_pxs2_power_off,
    .is_legacy = false,
    .is_ready_high = false,
    .is_phy_clk = false,
    };
    static const struct uniphier_ahciphy_soc_data uniphier_pxs3_data = {
    .init      = uniphier_ahciphy_pxs3_init,
    .power_on  = uniphier_ahciphy_pxs2_power_on,
    .power_off = uniphier_ahciphy_pxs2_power_off,
    .is_legacy = false,
    .is_ready_high = true,
    .is_phy_clk = true,
    };
    static const struct of_device_id uniphier_ahciphy_match[] = {
    {
    .compatible = "socionext,uniphier-pro4-ahci-phy",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-ahci-phy",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-ahci-phy",
    .data = &uniphier_pxs3_data,
    },
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, uniphier_ahciphy_match);
    static struct platform_driver uniphier_ahciphy_driver = {
    .probe = uniphier_ahciphy_probe,
    .driver = {
    .name = "uniphier-ahci-phy",
    .of_match_table = uniphier_ahciphy_match,
    },
    };
    module_platform_driver(uniphier_ahciphy_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier PHY driver for AHCI controller");
    MODULE_LICENSE("GPL v2");
