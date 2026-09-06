//! Automatically rewritten from C to Rust
//! Source: drivers/phy/amlogic/phy-meson-g12a-mipi-dphy-analog.c
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
// Meson G12A MIPI DSI Analog PHY
//
// Copyright (C) 2018 Amlogic, Inc. All rights reserved
// Copyright (C) 2022 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

pub const HHI_MIPI_CNTL0: c_uint = 0x00;

pub const HHI_MIPI_CNTL1: c_uint = 0x04;

pub const HHI_MIPI_CNTL2: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_g12a_mipi_dphy_analog_priv {
    pub phy: *mut phy,
    pub regmap: *mut regmap,
    pub config: phy_configure_opts_mipi_dphy,
}

    static int phy_g12a_mipi_dphy_analog_configure(struct phy *phy,
    union phy_configure_opts *opts)
    {
    struct phy_g12a_mipi_dphy_analog_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = phy_mipi_dphy_config_validate(&opts.mipi_dphy);
    if (ret)
    return ret;
    memcpy(&priv.config, opts, sizeof(priv.config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_g12a_mipi_dphy_analog_power_on(phy: *mut phy) -> c_int {
    static int phy_g12a_mipi_dphy_analog_power_on(struct phy *phy)
    {
    struct phy_g12a_mipi_dphy_analog_priv *priv = phy_get_drvdata(phy);
    unsigned int reg;
    regmap_write(priv.regmap, HHI_MIPI_CNTL0,
    FIELD_PREP(HHI_MIPI_CNTL0_DIF_REF_CTL0, 0x8) |
    FIELD_PREP(HHI_MIPI_CNTL0_DIF_REF_CTL1, 0xa487));
    regmap_write(priv.regmap, HHI_MIPI_CNTL1,
    FIELD_PREP(HHI_MIPI_CNTL2_DIF_REF_CTL2, 0x2e) |
    HHI_MIPI_CNTL1_BANDGAP);
    regmap_write(priv.regmap, HHI_MIPI_CNTL2,
    FIELD_PREP(HHI_MIPI_CNTL2_DIF_TX_CTL0, 0x45a) |
    FIELD_PREP(HHI_MIPI_CNTL2_DIF_TX_CTL1, 0x2680));
    reg = DSI_LANE_CLK;
    switch (priv.config.lanes) {
    case 4:
    reg |= DSI_LANE_3;
    fallthrough;
    case 3:
    reg |= DSI_LANE_2;
    fallthrough;
    case 2:
    reg |= DSI_LANE_1;
    fallthrough;
    case 1:
    reg |= DSI_LANE_0;
    break;
    default:
    reg = 0;
    }
    regmap_update_bits(priv.regmap, HHI_MIPI_CNTL2,
    HHI_MIPI_CNTL2_CH_EN,
    FIELD_PREP(HHI_MIPI_CNTL2_CH_EN, reg));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_g12a_mipi_dphy_analog_power_off(phy: *mut phy) -> c_int {
    static int phy_g12a_mipi_dphy_analog_power_off(struct phy *phy)
    {
    struct phy_g12a_mipi_dphy_analog_priv *priv = phy_get_drvdata(phy);
    regmap_write(priv.regmap, HHI_MIPI_CNTL0, 0);
    regmap_write(priv.regmap, HHI_MIPI_CNTL1, 0);
    regmap_write(priv.regmap, HHI_MIPI_CNTL2, 0);
    return 0;
    }
    static const struct phy_ops phy_g12a_mipi_dphy_analog_ops = {
    .configure = phy_g12a_mipi_dphy_analog_configure,
    .power_on = phy_g12a_mipi_dphy_analog_power_on,
    .power_off = phy_g12a_mipi_dphy_analog_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn phy_g12a_mipi_dphy_analog_probe(pdev: *mut platform_device) -> c_int {
    static int phy_g12a_mipi_dphy_analog_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy;
    struct device *dev = &pdev.dev;
    struct phy_g12a_mipi_dphy_analog_priv *priv;
    struct device_node *np = dev.of_node, *parent_np;
    struct regmap *map;
    priv = devm_kmalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Get the hhi system controller node
    parent_np = of_get_parent(np);
    map = syscon_node_to_regmap(parent_np);
    of_node_put(parent_np);
    if (IS_ERR(map))
    return dev_err_probe(dev, PTR_ERR(map), "failed to get HHI regmap\n");
    priv.regmap = map;
    priv.phy = devm_phy_create(dev, np, &phy_g12a_mipi_dphy_analog_ops);
    if (IS_ERR(priv.phy))
    return dev_err_probe(dev, PTR_ERR(priv.phy), "failed to create PHY\n");
    phy_set_drvdata(priv.phy, priv);
    dev_set_drvdata(dev, priv);
    phy = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy);
    }
    static const struct of_device_id phy_g12a_mipi_dphy_analog_of_match[] = {
    {
    .compatible = "amlogic,g12a-mipi-dphy-analog",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, phy_g12a_mipi_dphy_analog_of_match);
    static struct platform_driver phy_g12a_mipi_dphy_analog_driver = {
    .probe = phy_g12a_mipi_dphy_analog_probe,
    .driver = {
    .name = "phy-meson-g12a-mipi-dphy-analog",
    .of_match_table = phy_g12a_mipi_dphy_analog_of_match,
    },
    };
    module_platform_driver(phy_g12a_mipi_dphy_analog_driver);
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_DESCRIPTION("Meson G12A MIPI Analog D-PHY driver");
    MODULE_LICENSE("GPL v2");
