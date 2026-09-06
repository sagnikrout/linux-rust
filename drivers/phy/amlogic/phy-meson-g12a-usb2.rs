//! Automatically rewritten from C to Rust
//! Source: drivers/phy/amlogic/phy-meson-g12a-usb2.c
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
// Meson G12A USB2 PHY driver
//
// Copyright (C) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
// Copyright (C) 2017 Amlogic, Inc. All rights reserved
// Copyright (C) 2019 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

pub const PHY_CTRL_R0: c_uint = 0x0;
pub const PHY_CTRL_R1: c_uint = 0x4;
pub const PHY_CTRL_R2: c_uint = 0x8;
pub const PHY_CTRL_R3: c_uint = 0xc;

pub const PHY_CTRL_R4: c_uint = 0x10;

pub const PHY_CTRL_R5: c_uint = 0x14;
pub const PHY_CTRL_R6: c_uint = 0x18;
pub const PHY_CTRL_R7: c_uint = 0x1c;
pub const PHY_CTRL_R8: c_uint = 0x20;
pub const PHY_CTRL_R9: c_uint = 0x24;
pub const PHY_CTRL_R10: c_uint = 0x28;
pub const PHY_CTRL_R11: c_uint = 0x2c;
pub const PHY_CTRL_R12: c_uint = 0x30;
pub const PHY_CTRL_R13: c_uint = 0x34;

pub const PHY_CTRL_R14: c_uint = 0x38;

pub const PHY_CTRL_R15: c_uint = 0x3c;
pub const PHY_CTRL_R16: c_uint = 0x40;

pub const PHY_CTRL_R17: c_uint = 0x44;

pub const PHY_CTRL_R18: c_uint = 0x48;

pub const PHY_CTRL_R19: c_uint = 0x4c;
pub const PHY_CTRL_R20: c_uint = 0x50;

pub const PHY_CTRL_R21: c_uint = 0x54;

pub const PHY_CTRL_R22: c_uint = 0x58;
pub const PHY_CTRL_R23: c_uint = 0x5c;
pub const RESET_COMPLETE_TIME: c_int = 1000;
pub const PLL_RESET_COMPLETE_TIME: c_int = 100;
    enum meson_soc_id {
    MESON_SOC_G12A  = 0,
    MESON_SOC_A1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_meson_g12a_usb2_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub soc_id: c_int,
}

    static const struct regmap_config phy_meson_g12a_usb2_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = PHY_CTRL_R23,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson_g12a_usb2_init(phy: *mut phy) -> c_int {
    static int phy_meson_g12a_usb2_init(struct phy *phy)
    {
    struct phy_meson_g12a_usb2_priv *priv = phy_get_drvdata(phy);
    int ret;
    unsigned int value;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    ret = reset_control_reset(priv.reset);
    if (ret) {
    clk_disable_unprepare(priv.clk);
    return ret;
    }
    udelay(RESET_COMPLETE_TIME);
// usb2_otg_aca_en == 0
    regmap_update_bits(priv.regmap, PHY_CTRL_R21,
    PHY_CTRL_R21_USB2_OTG_ACA_EN, 0);
// PLL Setup : 24MHz * 20 / 1 = 480MHz
    regmap_write(priv.regmap, PHY_CTRL_R16,
    FIELD_PREP(PHY_CTRL_R16_MPLL_M, 20) |
    FIELD_PREP(PHY_CTRL_R16_MPLL_N, 1) |
    PHY_CTRL_R16_MPLL_LOAD |
    FIELD_PREP(PHY_CTRL_R16_MPLL_LOCK_LONG, 1) |
    PHY_CTRL_R16_MPLL_FAST_LOCK |
    PHY_CTRL_R16_MPLL_EN |
    PHY_CTRL_R16_MPLL_RESET);
    regmap_write(priv.regmap, PHY_CTRL_R17,
    FIELD_PREP(PHY_CTRL_R17_MPLL_FRAC_IN, 0) |
    FIELD_PREP(PHY_CTRL_R17_MPLL_LAMBDA1, 7) |
    FIELD_PREP(PHY_CTRL_R17_MPLL_LAMBDA0, 7) |
    FIELD_PREP(PHY_CTRL_R17_MPLL_FILTER_PVT2, 2) |
    FIELD_PREP(PHY_CTRL_R17_MPLL_FILTER_PVT1, 9));
    value = FIELD_PREP(PHY_CTRL_R18_MPLL_LKW_SEL, 1) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_LK_W, 9) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_LK_S, 0x27) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_PFD_GAIN, 1) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_ROU, 7) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_DATA_SEL, 3) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_BIAS_ADJ, 1) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_BB_MODE, 0) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_ALPHA, 3) |
    FIELD_PREP(PHY_CTRL_R18_MPLL_ADJ_LDO, 1) |
    PHY_CTRL_R18_MPLL_ACG_RANGE;
    if (priv.soc_id == MESON_SOC_A1)
    value |= PHY_CTRL_R18_MPLL_DCO_CLK_SEL;
    regmap_write(priv.regmap, PHY_CTRL_R18, value);
    udelay(PLL_RESET_COMPLETE_TIME);
// UnReset PLL
    regmap_write(priv.regmap, PHY_CTRL_R16,
    FIELD_PREP(PHY_CTRL_R16_MPLL_M, 20) |
    FIELD_PREP(PHY_CTRL_R16_MPLL_N, 1) |
    PHY_CTRL_R16_MPLL_LOAD |
    FIELD_PREP(PHY_CTRL_R16_MPLL_LOCK_LONG, 1) |
    PHY_CTRL_R16_MPLL_FAST_LOCK |
    PHY_CTRL_R16_MPLL_EN);
// PHY Tuning
    regmap_write(priv.regmap, PHY_CTRL_R20,
    FIELD_PREP(PHY_CTRL_R20_USB2_OTG_VBUS_TRIM_2_0, 4) |
    PHY_CTRL_R20_USB2_OTG_VBUSDET_EN |
    FIELD_PREP(PHY_CTRL_R20_USB2_DMON_SEL_3_0, 15) |
    PHY_CTRL_R20_USB2_EDGE_DRV_EN |
    FIELD_PREP(PHY_CTRL_R20_USB2_EDGE_DRV_TRIM_1_0, 3) |
    FIELD_PREP(PHY_CTRL_R20_USB2_BGR_ADJ_4_0, 0) |
    FIELD_PREP(PHY_CTRL_R20_USB2_BGR_VREF_4_0, 0) |
    FIELD_PREP(PHY_CTRL_R20_USB2_BGR_DBG_1_0, 0));
    if (priv.soc_id == MESON_SOC_G12A)
    regmap_write(priv.regmap, PHY_CTRL_R4,
    FIELD_PREP(PHY_CTRL_R4_CALIB_CODE_7_0, 0xf) |
    FIELD_PREP(PHY_CTRL_R4_CALIB_CODE_15_8, 0xf) |
    FIELD_PREP(PHY_CTRL_R4_CALIB_CODE_23_16, 0xf) |
    PHY_CTRL_R4_TEST_BYPASS_MODE_EN |
    FIELD_PREP(PHY_CTRL_R4_I_C2L_BIAS_TRIM_1_0, 0) |
    FIELD_PREP(PHY_CTRL_R4_I_C2L_BIAS_TRIM_3_2, 0));
#[no_mangle]
pub unsafe extern "C" fn if(MESON_SOC_A1: priv->soc_id ==) -> else {
    regmap_write(priv.regmap, PHY_CTRL_R21,
    PHY_CTRL_R21_USB2_CAL_ACK_EN |
    PHY_CTRL_R21_USB2_TX_STRG_PD |
    FIELD_PREP(PHY_CTRL_R21_USB2_OTG_ACA_TRIM_1_0, 2));
// Analog Settings
    regmap_write(priv.regmap, PHY_CTRL_R13,
    FIELD_PREP(PHY_CTRL_R13_MIN_COUNT_FOR_SYNC_DET, 7));
    }
// Tuning Disconnect Threshold
    regmap_write(priv.regmap, PHY_CTRL_R3,
    FIELD_PREP(PHY_CTRL_R3_SQUELCH_REF, 0) |
    FIELD_PREP(PHY_CTRL_R3_HSDIC_REF, 1) |
    FIELD_PREP(PHY_CTRL_R3_DISC_THRESH, 3));
    if (priv.soc_id == MESON_SOC_G12A) {
// Analog Settings
    regmap_write(priv.regmap, PHY_CTRL_R14, 0);
    regmap_write(priv.regmap, PHY_CTRL_R13,
    PHY_CTRL_R13_UPDATE_PMA_SIGNALS |
    FIELD_PREP(PHY_CTRL_R13_MIN_COUNT_FOR_SYNC_DET, 7));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson_g12a_usb2_exit(phy: *mut phy) -> c_int {
    static int phy_meson_g12a_usb2_exit(struct phy *phy)
    {
    struct phy_meson_g12a_usb2_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = reset_control_reset(priv.reset);
    if (!ret)
    clk_disable_unprepare(priv.clk);
    return ret;
    }
// set_mode is not needed, mode setting is handled via the UTMI bus
    static const struct phy_ops phy_meson_g12a_usb2_ops = {
    .init		= phy_meson_g12a_usb2_init,
    .exit		= phy_meson_g12a_usb2_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson_g12a_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int phy_meson_g12a_usb2_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct phy_meson_g12a_usb2_priv *priv;
    struct phy *phy;
    void __iomem *base;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    platform_set_drvdata(pdev, priv);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.soc_id = (uintptr_t)of_device_get_match_data(&pdev.dev);
    priv.regmap = devm_regmap_init_mmio(dev, base,
    &phy_meson_g12a_usb2_regmap_conf);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.clk = devm_clk_get(dev, "xtal");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.reset = devm_reset_control_get(dev, "phy");
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    ret = reset_control_deassert(priv.reset);
    if (ret)
    return ret;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &phy_meson_g12a_usb2_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy),
    "failed to create PHY\n");
    phy_set_bus_width(phy, 8);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id phy_meson_g12a_usb2_of_match[] = {
    {
    .compatible = "amlogic,g12a-usb2-phy",
    .data = (void *)MESON_SOC_G12A,
    },
    {
    .compatible = "amlogic,a1-usb2-phy",
    .data = (void *)MESON_SOC_A1,
    },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, phy_meson_g12a_usb2_of_match);
    static struct platform_driver phy_meson_g12a_usb2_driver = {
    .probe	= phy_meson_g12a_usb2_probe,
    .driver	= {
    .name		= "phy-meson-g12a-usb2",
    .of_match_table	= phy_meson_g12a_usb2_of_match,
    },
    };
    module_platform_driver(phy_meson_g12a_usb2_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_DESCRIPTION("Meson G12A USB2 PHY driver");
    MODULE_LICENSE("GPL v2");
