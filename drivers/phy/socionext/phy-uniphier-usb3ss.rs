//! Automatically rewritten from C to Rust
//! Source: drivers/phy/socionext/phy-uniphier-usb3ss.c
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
// phy-uniphier-usb3ss.c - SS-PHY driver for Socionext UniPhier USB3 controller
// Copyright 2015-2018 Socionext Inc.
// Author:
// Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
// Contributors:
// Motoya Tanigawa <tanigawa.motoya@socionext.com>
// Masami Hiramatsu <masami.hiramatsu@linaro.org>
//

pub const SSPHY_TESTI: c_uint = 0x0;

pub const SSPHY_TESTO: c_uint = 0x4;

pub const MAX_PHY_PARAMS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u3ssphy_param {
    struct {
    pub reg_no: c_int,
    pub msb: c_int,
    pub lsb: c_int,
    pub field: },
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u3ssphy_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clk_parent_gio: *mut *mut *mut *mut clk clk, clk_ext, clk_parent,,
    pub rst_parent_gio: *mut *mut *mut reset_control rst, rst_parent,,
    pub vbus: *mut regulator,
    pub data: *const uniphier_u3ssphy_soc_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u3ssphy_soc_data {
    pub is_legacy: bool,
    pub nparams: c_int,
    pub param: [uniphier_u3ssphy_param; MAX_PHY_PARAMS],
}

    static void uniphier_u3ssphy_testio_write(struct uniphier_u3ssphy_priv *priv,
    u32 data)
    {
// need to read TESTO twice after accessing TESTI
    writel(data, priv.base + SSPHY_TESTI);
    readl(priv.base + SSPHY_TESTO);
    readl(priv.base + SSPHY_TESTO);
    }
    static void uniphier_u3ssphy_set_param(struct uniphier_u3ssphy_priv *priv,
    const struct uniphier_u3ssphy_param *p)
    {
    u32 val;
    let mut field_mask: u8 = GENMASK(p.field.msb, p.field.lsb);
    u8 data;
// read previous data
    val  = FIELD_PREP(TESTI_DAT_MASK, 1);
    val |= FIELD_PREP(TESTI_ADR_MASK, p.field.reg_no);
    uniphier_u3ssphy_testio_write(priv, val);
    val = readl(priv.base + SSPHY_TESTO) & TESTO_DAT_MASK;
// update value
    val &= ~field_mask;
    data = field_mask & (p.value << p.field.lsb);
    val  = FIELD_PREP(TESTI_DAT_MASK, data | val);
    val |= FIELD_PREP(TESTI_ADR_MASK, p.field.reg_no);
    uniphier_u3ssphy_testio_write(priv, val);
    uniphier_u3ssphy_testio_write(priv, val | TESTI_WR_EN);
    uniphier_u3ssphy_testio_write(priv, val);
// read current data as dummy
    val  = FIELD_PREP(TESTI_DAT_MASK, 1);
    val |= FIELD_PREP(TESTI_ADR_MASK, p.field.reg_no);
    uniphier_u3ssphy_testio_write(priv, val);
    readl(priv.base + SSPHY_TESTO);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u3ssphy_power_on(phy: *mut phy) -> c_int {
    static int uniphier_u3ssphy_power_on(struct phy *phy)
    {
    struct uniphier_u3ssphy_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = clk_prepare_enable(priv.clk_ext);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    goto out_clk_ext_disable;
    ret = reset_control_deassert(priv.rst);
    if (ret)
    goto out_clk_disable;
    if (priv.vbus) {
    ret = regulator_enable(priv.vbus);
    if (ret)
    goto out_rst_assert;
    }
    return 0;
    out_rst_assert:
    reset_control_assert(priv.rst);
    out_clk_disable:
    clk_disable_unprepare(priv.clk);
    out_clk_ext_disable:
    clk_disable_unprepare(priv.clk_ext);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u3ssphy_power_off(phy: *mut phy) -> c_int {
    static int uniphier_u3ssphy_power_off(struct phy *phy)
    {
    struct uniphier_u3ssphy_priv *priv = phy_get_drvdata(phy);
    if (priv.vbus)
    regulator_disable(priv.vbus);
    reset_control_assert(priv.rst);
    clk_disable_unprepare(priv.clk);
    clk_disable_unprepare(priv.clk_ext);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u3ssphy_init(phy: *mut phy) -> c_int {
    static int uniphier_u3ssphy_init(struct phy *phy)
    {
    struct uniphier_u3ssphy_priv *priv = phy_get_drvdata(phy);
    int i, ret;
    ret = clk_prepare_enable(priv.clk_parent);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clk_parent_gio);
    if (ret)
    goto out_clk_disable;
    ret = reset_control_deassert(priv.rst_parent);
    if (ret)
    goto out_clk_gio_disable;
    ret = reset_control_deassert(priv.rst_parent_gio);
    if (ret)
    goto out_rst_assert;
    if (priv.data.is_legacy)
    return 0;
    for (i = 0; i < priv.data.nparams; i++)
    uniphier_u3ssphy_set_param(priv, &priv.data.param[i]);
    return 0;
    out_rst_assert:
    reset_control_assert(priv.rst_parent);
    out_clk_gio_disable:
    clk_disable_unprepare(priv.clk_parent_gio);
    out_clk_disable:
    clk_disable_unprepare(priv.clk_parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u3ssphy_exit(phy: *mut phy) -> c_int {
    static int uniphier_u3ssphy_exit(struct phy *phy)
    {
    struct uniphier_u3ssphy_priv *priv = phy_get_drvdata(phy);
    reset_control_assert(priv.rst_parent_gio);
    reset_control_assert(priv.rst_parent);
    clk_disable_unprepare(priv.clk_parent_gio);
    clk_disable_unprepare(priv.clk_parent);
    return 0;
    }
    static const struct phy_ops uniphier_u3ssphy_ops = {
    .init           = uniphier_u3ssphy_init,
    .exit           = uniphier_u3ssphy_exit,
    .power_on       = uniphier_u3ssphy_power_on,
    .power_off      = uniphier_u3ssphy_power_off,
    .owner          = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_u3ssphy_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_u3ssphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_u3ssphy_priv *priv;
    struct phy_provider *phy_provider;
    struct phy *phy;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.data = of_device_get_match_data(dev);
    if (WARN_ON(!priv.data ||
    priv.data.nparams > MAX_PHY_PARAMS))
    return -EINVAL;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    if (!priv.data.is_legacy) {
    priv.clk = devm_clk_get(dev, "phy");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.clk_ext = devm_clk_get_optional(dev, "phy-ext");
    if (IS_ERR(priv.clk_ext))
    return PTR_ERR(priv.clk_ext);
    priv.rst = devm_reset_control_get_shared(dev, "phy");
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    } else {
    priv.clk_parent_gio = devm_clk_get(dev, "gio");
    if (IS_ERR(priv.clk_parent_gio))
    return PTR_ERR(priv.clk_parent_gio);
    priv.rst_parent_gio =
    devm_reset_control_get_shared(dev, "gio");
    if (IS_ERR(priv.rst_parent_gio))
    return PTR_ERR(priv.rst_parent_gio);
    }
    priv.clk_parent = devm_clk_get(dev, "link");
    if (IS_ERR(priv.clk_parent))
    return PTR_ERR(priv.clk_parent);
    priv.rst_parent = devm_reset_control_get_shared(dev, "link");
    if (IS_ERR(priv.rst_parent))
    return PTR_ERR(priv.rst_parent);
    priv.vbus = devm_regulator_get_optional(dev, "vbus");
    if (IS_ERR(priv.vbus)) {
    if (PTR_ERR(priv.vbus) == -EPROBE_DEFER)
    return PTR_ERR(priv.vbus);
    priv.vbus = core::ptr::null_mut();
    }
    phy = devm_phy_create(dev, dev.of_node, &uniphier_u3ssphy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct uniphier_u3ssphy_soc_data uniphier_pro4_data = {
    .is_legacy = true,
    };
    static const struct uniphier_u3ssphy_soc_data uniphier_pxs2_data = {
    .is_legacy = false,
    .nparams = 7,
    .param = {
    { CDR_CPD_TRIM, 10 },
    { CDR_CPF_TRIM, 3 },
    { TX_PLL_TRIM, 5 },
    { BGAP_TRIM, 9 },
    { CDR_TRIM, 2 },
    { VCOPLL_CTRL, 7 },
    { VCOPLL_CM, 1 },
    },
    };
    static const struct uniphier_u3ssphy_soc_data uniphier_ld20_data = {
    .is_legacy = false,
    .nparams = 3,
    .param = {
    { CDR_CPD_TRIM, 6 },
    { CDR_TRIM, 2 },
    { VCO_CTRL, 5 },
    },
    };
    static const struct of_device_id uniphier_u3ssphy_match[] = {
    {
    .compatible = "socionext,uniphier-pro4-usb3-ssphy",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-usb3-ssphy",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-usb3-ssphy",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-usb3-ssphy",
    .data = &uniphier_ld20_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-usb3-ssphy",
    .data = &uniphier_ld20_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-usb3-ssphy",
    .data = &uniphier_ld20_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_u3ssphy_match);
    static struct platform_driver uniphier_u3ssphy_driver = {
    .probe = uniphier_u3ssphy_probe,
    .driver	= {
    .name = "uniphier-usb3-ssphy",
    .of_match_table	= uniphier_u3ssphy_match,
    },
    };
    module_platform_driver(uniphier_u3ssphy_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier SS-PHY driver for USB3 controller");
    MODULE_LICENSE("GPL v2");
