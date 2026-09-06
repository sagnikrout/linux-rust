//! Automatically rewritten from C to Rust
//! Source: drivers/phy/socionext/phy-uniphier-pcie.c
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
// phy-uniphier-pcie.c - PHY driver for UniPhier PCIe controller
// Copyright 2018, Socionext Inc.
// Author: Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

// PHY
pub const PCL_PHY_CLKCTRL: c_uint = 0x0000;

pub const PCL_PHY_TEST_I: c_uint = 0x2000;

pub const TESTIO_PHY_SHIFT: c_int = 16;
pub const PCL_PHY_TEST_O: c_uint = 0x2004;

pub const PCL_PHY_RESET: c_uint = 0x200c;

// SG
pub const SG_USBPCIESEL: c_uint = 0x590;

// SC
pub const SC_US3SRCSEL: c_uint = 0x2244;

pub const PCL_PHY_R00: c_int = 0;

pub const PCL_PHY_R06: c_int = 6;

pub const RX_EQ_ADJ_VAL: c_int = 0;
pub const PCL_PHY_R26: c_int = 26;

pub const VCO_CTRL_INIT_VAL: c_int = 5;
pub const PCL_PHY_R28: c_int = 28;

pub const VCOPLL_CLMP_VAL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pciephy_priv {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub clk_gio: *mut *mut clk clk,,
    pub rst_gio: *mut *mut reset_control rst,,
    pub data: *const uniphier_pciephy_soc_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pciephy_soc_data {
    pub is_legacy: bool,
    pub is_dual_phy: bool,
    pub regmap): *mut *mut void (set_phymode)(struct regmap,
}

    static void uniphier_pciephy_testio_write(struct uniphier_pciephy_priv *priv,
    int id, u32 data)
    {
    if (id)
    data <<= TESTIO_PHY_SHIFT;
// need to read TESTO twice after accessing TESTI
    writel(data, priv.base + PCL_PHY_TEST_I);
    readl(priv.base + PCL_PHY_TEST_O);
    readl(priv.base + PCL_PHY_TEST_O);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_testio_read(priv: *mut uniphier_pciephy_priv, id: c_int) -> u32 {
    static u32 uniphier_pciephy_testio_read(struct uniphier_pciephy_priv *priv, int id)
    {
    let mut val: u32 = readl(priv.base + PCL_PHY_TEST_O);
    if (id)
    val >>= TESTIO_PHY_SHIFT;
    return val & TESTO_DAT_MASK;
    }
    static void uniphier_pciephy_set_param(struct uniphier_pciephy_priv *priv,
    int id, u32 reg, u32 mask, u32 param)
    {
    u32 val;
// read previous data
    val  = FIELD_PREP(TESTI_DAT_MASK, 1);
    val |= FIELD_PREP(TESTI_ADR_MASK, reg);
    uniphier_pciephy_testio_write(priv, id, val);
    val = uniphier_pciephy_testio_read(priv, id);
// update value
    val &= ~mask;
    val |= mask & param;
    val = FIELD_PREP(TESTI_DAT_MASK, val);
    val |= FIELD_PREP(TESTI_ADR_MASK, reg);
    uniphier_pciephy_testio_write(priv, id, val);
    uniphier_pciephy_testio_write(priv, id, val | TESTI_WR_EN);
    uniphier_pciephy_testio_write(priv, id, val);
// read current data as dummy
    val  = FIELD_PREP(TESTI_DAT_MASK, 1);
    val |= FIELD_PREP(TESTI_ADR_MASK, reg);
    uniphier_pciephy_testio_write(priv, id, val);
    uniphier_pciephy_testio_read(priv, id);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_assert(priv: *mut uniphier_pciephy_priv) {
    static void uniphier_pciephy_assert(struct uniphier_pciephy_priv *priv)
    {
    u32 val;
    val = readl(priv.base + PCL_PHY_RESET);
    val &= ~PCL_PHY_RESET_N;
    val |= PCL_PHY_RESET_N_MNMODE;
    writel(val, priv.base + PCL_PHY_RESET);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_deassert(priv: *mut uniphier_pciephy_priv) {
    static void uniphier_pciephy_deassert(struct uniphier_pciephy_priv *priv)
    {
    u32 val;
    val = readl(priv.base + PCL_PHY_RESET);
    val |= PCL_PHY_RESET_N_MNMODE | PCL_PHY_RESET_N;
    writel(val, priv.base + PCL_PHY_RESET);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_init(phy: *mut phy) -> c_int {
    static int uniphier_pciephy_init(struct phy *phy)
    {
    struct uniphier_pciephy_priv *priv = phy_get_drvdata(phy);
    u32 val;
    int ret, id;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clk_gio);
    if (ret)
    goto out_clk_disable;
    ret = reset_control_deassert(priv.rst);
    if (ret)
    goto out_clk_gio_disable;
    ret = reset_control_deassert(priv.rst_gio);
    if (ret)
    goto out_rst_assert;
// support only 1 port
    val = readl(priv.base + PCL_PHY_CLKCTRL);
    val &= ~PORT_SEL_MASK;
    val |= PORT_SEL_1;
    writel(val, priv.base + PCL_PHY_CLKCTRL);
// legacy controller doesn't have phy_reset and parameters
    if (priv.data.is_legacy)
    return 0;
    for (id = 0; id < (priv.data.is_dual_phy ? 2 : 1); id++) {
    uniphier_pciephy_set_param(priv, id, PCL_PHY_R00,
    RX_EQ_ADJ_EN, RX_EQ_ADJ_EN);
    uniphier_pciephy_set_param(priv, id, PCL_PHY_R06, RX_EQ_ADJ,
    FIELD_PREP(RX_EQ_ADJ, RX_EQ_ADJ_VAL));
    uniphier_pciephy_set_param(priv, id, PCL_PHY_R26, VCO_CTRL,
    FIELD_PREP(VCO_CTRL, VCO_CTRL_INIT_VAL));
    uniphier_pciephy_set_param(priv, id, PCL_PHY_R28, VCOPLL_CLMP,
    FIELD_PREP(VCOPLL_CLMP, VCOPLL_CLMP_VAL));
    }
    usleep_range(1, 10);
    uniphier_pciephy_deassert(priv);
    usleep_range(1, 10);
    return 0;
    out_rst_assert:
    reset_control_assert(priv.rst);
    out_clk_gio_disable:
    clk_disable_unprepare(priv.clk_gio);
    out_clk_disable:
    clk_disable_unprepare(priv.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_exit(phy: *mut phy) -> c_int {
    static int uniphier_pciephy_exit(struct phy *phy)
    {
    struct uniphier_pciephy_priv *priv = phy_get_drvdata(phy);
    if (!priv.data.is_legacy)
    uniphier_pciephy_assert(priv);
    reset_control_assert(priv.rst_gio);
    reset_control_assert(priv.rst);
    clk_disable_unprepare(priv.clk_gio);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
    static const struct phy_ops uniphier_pciephy_ops = {
    .init  = uniphier_pciephy_init,
    .exit  = uniphier_pciephy_exit,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_pciephy_probe(struct platform_device *pdev)
    {
    struct uniphier_pciephy_priv *priv;
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    struct phy *phy;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.data = of_device_get_match_data(dev);
    if (WARN_ON(!priv.data))
    return -EINVAL;
    priv.dev = dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    if (priv.data.is_legacy) {
    priv.clk_gio = devm_clk_get(dev, "gio");
    if (IS_ERR(priv.clk_gio))
    return PTR_ERR(priv.clk_gio);
    priv.rst_gio =
    devm_reset_control_get_shared(dev, "gio");
    if (IS_ERR(priv.rst_gio))
    return PTR_ERR(priv.rst_gio);
    priv.clk = devm_clk_get(dev, "link");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.rst = devm_reset_control_get_shared(dev, "link");
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    } else {
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.rst = devm_reset_control_get_shared(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    }
    phy = devm_phy_create(dev, dev.of_node, &uniphier_pciephy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    regmap = syscon_regmap_lookup_by_phandle(dev.of_node,
    "socionext,syscon");
    if (!IS_ERR(regmap) && priv.data.set_phymode)
    priv.data.set_phymode(regmap);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_ld20_setmode(regmap: *mut regmap) {
    static void uniphier_pciephy_ld20_setmode(struct regmap *regmap)
    {
    regmap_update_bits(regmap, SG_USBPCIESEL,
    SG_USBPCIESEL_PCIE, SG_USBPCIESEL_PCIE);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pciephy_nx1_setmode(regmap: *mut regmap) {
    static void uniphier_pciephy_nx1_setmode(struct regmap *regmap)
    {
    regmap_update_bits(regmap, SC_US3SRCSEL,
    SC_US3SRCSEL_2LANE, SC_US3SRCSEL_2LANE);
    }
    static const struct uniphier_pciephy_soc_data uniphier_pro5_data = {
    .is_legacy = true,
    };
    static const struct uniphier_pciephy_soc_data uniphier_ld20_data = {
    .is_legacy = false,
    .is_dual_phy = false,
    .set_phymode = uniphier_pciephy_ld20_setmode,
    };
    static const struct uniphier_pciephy_soc_data uniphier_pxs3_data = {
    .is_legacy = false,
    .is_dual_phy = false,
    };
    static const struct uniphier_pciephy_soc_data uniphier_nx1_data = {
    .is_legacy = false,
    .is_dual_phy = true,
    .set_phymode = uniphier_pciephy_nx1_setmode,
    };
    static const struct of_device_id uniphier_pciephy_match[] = {
    {
    .compatible = "socionext,uniphier-pro5-pcie-phy",
    .data = &uniphier_pro5_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-pcie-phy",
    .data = &uniphier_ld20_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-pcie-phy",
    .data = &uniphier_pxs3_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-pcie-phy",
    .data = &uniphier_nx1_data,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, uniphier_pciephy_match);
    static struct platform_driver uniphier_pciephy_driver = {
    .probe = uniphier_pciephy_probe,
    .driver = {
    .name = "uniphier-pcie-phy",
    .of_match_table = uniphier_pciephy_match,
    },
    };
    module_platform_driver(uniphier_pciephy_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier PHY driver for PCIe controller");
    MODULE_LICENSE("GPL v2");
