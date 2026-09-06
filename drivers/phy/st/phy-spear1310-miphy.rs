//! Automatically rewritten from C to Rust
//! Source: drivers/phy/st/phy-spear1310-miphy.c
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
// ST SPEAr1310-miphy driver
//
// Copyright (C) 2014 ST Microelectronics
// Pratyush Anand <pratyush.anand@gmail.com>
// Mohit Kumar <mohit.kumar.dhaka@gmail.com>
//

// SPEAr1310 Registers
pub const SPEAR1310_PCIE_SATA_CFG: c_uint = 0x3A4;

    BIT((x + 29)))

    (SPEAR1310_PCIE_SATA##x##_SEL_PCIE | \
    SPEAR1310_PCIE##x##_CFG_AUX_CLK_EN | \
    SPEAR1310_PCIE##x##_CFG_CORE_CLK_EN | \
    SPEAR1310_PCIE##x##_CFG_POWERUP_RESET | \
    SPEAR1310_PCIE##x##_CFG_DEVICE_PRESENT)

    (SPEAR1310_PCIE_SATA##x##_SEL_SATA | \
    SPEAR1310_SATA##x##_CFG_PM_CLK_EN | \
    SPEAR1310_SATA##x##_CFG_POWERUP_RESET | \
    SPEAR1310_SATA##x##_CFG_RX_CLK_EN | \
    SPEAR1310_SATA##x##_CFG_TX_CLK_EN)
pub const SPEAR1310_PCIE_MIPHY_CFG_1: c_uint = 0x3A8;

    (SPEAR1310_MIPHY_DUAL_OSC_BYPASS_EXT | \
    SPEAR1310_MIPHY_DUAL_CLK_REF_DIV2 | \
    SPEAR1310_MIPHY_DUAL_PLL_RATIO_TOP(60) | \
    SPEAR1310_MIPHY_SINGLE_OSC_BYPASS_EXT | \
    SPEAR1310_MIPHY_SINGLE_CLK_REF_DIV2 | \
    SPEAR1310_MIPHY_SINGLE_PLL_RATIO_TOP(60))

    (SPEAR1310_MIPHY_SINGLE_PLL_RATIO_TOP(120))

    (SPEAR1310_MIPHY_DUAL_OSC_BYPASS_EXT | \
    SPEAR1310_MIPHY_DUAL_PLL_RATIO_TOP(25) | \
    SPEAR1310_MIPHY_SINGLE_OSC_BYPASS_EXT | \
    SPEAR1310_MIPHY_SINGLE_PLL_RATIO_TOP(25))
pub const SPEAR1310_PCIE_MIPHY_CFG_2: c_uint = 0x3AC;
    enum spear1310_miphy_mode {
    SATA,
    PCIE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear1310_miphy_priv {
// instance id of this phy
    pub id: u32,
// phy mode: 0 for SATA 1 for PCIe
    pub mode: enum spear1310_miphy_mode,
// regmap for any soc specific misc registers
    pub misc: *mut regmap,
// phy struct pointer
    pub phy: *mut phy,
}

#[no_mangle]
unsafe extern "C" fn spear1310_miphy_pcie_init(priv: *mut spear1310_miphy_priv) -> c_int {
    static int spear1310_miphy_pcie_init(struct spear1310_miphy_priv *priv)
    {
    u32 val;
    regmap_update_bits(priv.misc, SPEAR1310_PCIE_MIPHY_CFG_1,
    SPEAR1310_PCIE_SATA_MIPHY_CFG_PCIE_MASK,
    SPEAR1310_PCIE_SATA_MIPHY_CFG_PCIE);
    switch (priv.id) {
    case 0:
    val = SPEAR1310_PCIE_CFG_VAL(0);
    break;
    case 1:
    val = SPEAR1310_PCIE_CFG_VAL(1);
    break;
    case 2:
    val = SPEAR1310_PCIE_CFG_VAL(2);
    break;
    default:
    return -EINVAL;
    }
    regmap_update_bits(priv.misc, SPEAR1310_PCIE_SATA_CFG,
    SPEAR1310_PCIE_CFG_MASK(priv.id), val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1310_miphy_pcie_exit(priv: *mut spear1310_miphy_priv) -> c_int {
    static int spear1310_miphy_pcie_exit(struct spear1310_miphy_priv *priv)
    {
    regmap_update_bits(priv.misc, SPEAR1310_PCIE_SATA_CFG,
    SPEAR1310_PCIE_CFG_MASK(priv.id), 0);
    regmap_update_bits(priv.misc, SPEAR1310_PCIE_MIPHY_CFG_1,
    SPEAR1310_PCIE_SATA_MIPHY_CFG_PCIE_MASK, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1310_miphy_init(phy: *mut phy) -> c_int {
    static int spear1310_miphy_init(struct phy *phy)
    {
    struct spear1310_miphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.mode == PCIE)
    ret = spear1310_miphy_pcie_init(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spear1310_miphy_exit(phy: *mut phy) -> c_int {
    static int spear1310_miphy_exit(struct phy *phy)
    {
    struct spear1310_miphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.mode == PCIE)
    ret = spear1310_miphy_pcie_exit(priv);
    return ret;
    }
    static const struct of_device_id spear1310_miphy_of_match[] = {
    { .compatible = "st,spear1310-miphy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, spear1310_miphy_of_match);
    static const struct phy_ops spear1310_miphy_ops = {
    .init = spear1310_miphy_init,
    .exit = spear1310_miphy_exit,
    .owner = THIS_MODULE,
    };
    static struct phy *spear1310_miphy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct spear1310_miphy_priv *priv = dev_get_drvdata(dev);
    if (args.args_count < 1) {
    dev_err(dev, "DT did not pass correct no of args\n");
    return ERR_PTR(-ENODEV);
    }
    priv.mode = args.args[0];
    if (priv.mode != SATA && priv.mode != PCIE) {
    dev_err(dev, "DT did not pass correct phy mode\n");
    return ERR_PTR(-ENODEV);
    }
    return priv.phy;
    }
#[no_mangle]
unsafe extern "C" fn spear1310_miphy_probe(pdev: *mut platform_device) -> c_int {
    static int spear1310_miphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spear1310_miphy_priv *priv;
    struct phy_provider *phy_provider;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.misc =
    syscon_regmap_lookup_by_phandle(dev.of_node, "misc");
    if (IS_ERR(priv.misc)) {
    dev_err(dev, "failed to find misc regmap\n");
    return PTR_ERR(priv.misc);
    }
    if (of_property_read_u32(dev.of_node, "phy-id", &priv.id)) {
    dev_err(dev, "failed to find phy id\n");
    return -EINVAL;
    }
    priv.phy = devm_phy_create(dev, core::ptr::null_mut(), &spear1310_miphy_ops);
    if (IS_ERR(priv.phy)) {
    dev_err(dev, "failed to create SATA PCIe PHY\n");
    return PTR_ERR(priv.phy);
    }
    dev_set_drvdata(dev, priv);
    phy_set_drvdata(priv.phy, priv);
    phy_provider =
    devm_of_phy_provider_register(dev, spear1310_miphy_xlate);
    if (IS_ERR(phy_provider)) {
    dev_err(dev, "failed to register phy provider\n");
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
    static struct platform_driver spear1310_miphy_driver = {
    .probe		= spear1310_miphy_probe,
    .driver = {
    .name = "spear1310-miphy",
    .of_match_table = spear1310_miphy_of_match,
    },
    };
    module_platform_driver(spear1310_miphy_driver);
    MODULE_DESCRIPTION("ST SPEAR1310-MIPHY driver");
    MODULE_AUTHOR("Pratyush Anand <pratyush.anand@gmail.com>");
    MODULE_LICENSE("GPL v2");
