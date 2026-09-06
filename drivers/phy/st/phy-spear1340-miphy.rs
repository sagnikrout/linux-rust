//! Automatically rewritten from C to Rust
//! Source: drivers/phy/st/phy-spear1340-miphy.c
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
// ST spear1340-miphy driver
//
// Copyright (C) 2014 ST Microelectronics
// Pratyush Anand <pratyush.anand@gmail.com>
// Mohit Kumar <mohit.kumar.dhaka@gmail.com>
//

// SPEAr1340 Registers
// Power Management Registers
pub const SPEAR1340_PCM_CFG: c_uint = 0x100;

pub const SPEAR1340_PCM_WKUP_CFG: c_uint = 0x104;
pub const SPEAR1340_SWITCH_CTR: c_uint = 0x108;
pub const SPEAR1340_PERIP1_SW_RST: c_uint = 0x318;

pub const SPEAR1340_PERIP2_SW_RST: c_uint = 0x31C;
pub const SPEAR1340_PERIP3_SW_RST: c_uint = 0x320;
// PCIE - SATA configuration registers
pub const SPEAR1340_PCIE_SATA_CFG: c_uint = 0x424;
// PCIE CFG MASks

pub const SPEAR1340_PCIE_SATA_CFG_MASK: c_uint = 0xF1F;

    SPEAR1340_PCIE_CFG_AUX_CLK_EN | \
    SPEAR1340_PCIE_CFG_CORE_CLK_EN | \
    SPEAR1340_PCIE_CFG_POWERUP_RESET | \
    SPEAR1340_PCIE_CFG_DEVICE_PRESENT)

    SPEAR1340_SATA_CFG_PM_CLK_EN | \
    SPEAR1340_SATA_CFG_POWERUP_RESET | \
    SPEAR1340_SATA_CFG_RX_CLK_EN | \
    SPEAR1340_SATA_CFG_TX_CLK_EN)
pub const SPEAR1340_PCIE_MIPHY_CFG: c_uint = 0x428;

pub const SPEAR1340_PCIE_MIPHY_CFG_MASK: c_uint = 0xF80000FF;

    (SPEAR1340_MIPHY_OSC_BYPASS_EXT | \
    SPEAR1340_MIPHY_CLK_REF_DIV2 | \
    SPEAR1340_MIPHY_PLL_RATIO_TOP(60))

    (SPEAR1340_MIPHY_PLL_RATIO_TOP(120))

    (SPEAR1340_MIPHY_OSC_BYPASS_EXT | \
    SPEAR1340_MIPHY_PLL_RATIO_TOP(25))
    enum spear1340_miphy_mode {
    SATA,
    PCIE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear1340_miphy_priv {
// phy mode: 0 for SATA 1 for PCIe
    pub mode: enum spear1340_miphy_mode,
// regmap for any soc specific misc registers
    pub misc: *mut regmap,
// phy struct pointer
    pub phy: *mut phy,
}

#[no_mangle]
unsafe extern "C" fn spear1340_miphy_sata_init(priv: *mut spear1340_miphy_priv) -> c_int {
    static int spear1340_miphy_sata_init(struct spear1340_miphy_priv *priv)
    {
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_SATA_CFG,
    SPEAR1340_PCIE_SATA_CFG_MASK,
    SPEAR1340_SATA_CFG_VAL);
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_MIPHY_CFG,
    SPEAR1340_PCIE_MIPHY_CFG_MASK,
    SPEAR1340_PCIE_SATA_MIPHY_CFG_SATA_25M_CRYSTAL_CLK);
// Switch on sata power domain
    regmap_update_bits(priv.misc, SPEAR1340_PCM_CFG,
    SPEAR1340_PCM_CFG_SATA_POWER_EN,
    SPEAR1340_PCM_CFG_SATA_POWER_EN);
// Wait for SATA power domain on
    msleep(20);
// Disable PCIE SATA Controller reset
    regmap_update_bits(priv.misc, SPEAR1340_PERIP1_SW_RST,
    SPEAR1340_PERIP1_SW_RSATA, 0);
// Wait for SATA reset de-assert completion
    msleep(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_sata_exit(priv: *mut spear1340_miphy_priv) -> c_int {
    static int spear1340_miphy_sata_exit(struct spear1340_miphy_priv *priv)
    {
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_SATA_CFG,
    SPEAR1340_PCIE_SATA_CFG_MASK, 0);
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_MIPHY_CFG,
    SPEAR1340_PCIE_MIPHY_CFG_MASK, 0);
// Enable PCIE SATA Controller reset
    regmap_update_bits(priv.misc, SPEAR1340_PERIP1_SW_RST,
    SPEAR1340_PERIP1_SW_RSATA,
    SPEAR1340_PERIP1_SW_RSATA);
// Wait for SATA power domain off
    msleep(20);
// Switch off sata power domain
    regmap_update_bits(priv.misc, SPEAR1340_PCM_CFG,
    SPEAR1340_PCM_CFG_SATA_POWER_EN, 0);
// Wait for SATA reset assert completion
    msleep(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_pcie_init(priv: *mut spear1340_miphy_priv) -> c_int {
    static int spear1340_miphy_pcie_init(struct spear1340_miphy_priv *priv)
    {
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_MIPHY_CFG,
    SPEAR1340_PCIE_MIPHY_CFG_MASK,
    SPEAR1340_PCIE_SATA_MIPHY_CFG_PCIE);
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_SATA_CFG,
    SPEAR1340_PCIE_SATA_CFG_MASK,
    SPEAR1340_PCIE_CFG_VAL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_pcie_exit(priv: *mut spear1340_miphy_priv) -> c_int {
    static int spear1340_miphy_pcie_exit(struct spear1340_miphy_priv *priv)
    {
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_MIPHY_CFG,
    SPEAR1340_PCIE_MIPHY_CFG_MASK, 0);
    regmap_update_bits(priv.misc, SPEAR1340_PCIE_SATA_CFG,
    SPEAR1340_PCIE_SATA_CFG_MASK, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_init(phy: *mut phy) -> c_int {
    static int spear1340_miphy_init(struct phy *phy)
    {
    struct spear1340_miphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.mode == SATA)
    ret = spear1340_miphy_sata_init(priv);
#[no_mangle]
pub unsafe extern "C" fn if(PCIE: priv->mode ==) -> else {
    else if (priv.mode == PCIE)
    ret = spear1340_miphy_pcie_init(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_exit(phy: *mut phy) -> c_int {
    static int spear1340_miphy_exit(struct phy *phy)
    {
    struct spear1340_miphy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.mode == SATA)
    ret = spear1340_miphy_sata_exit(priv);
#[no_mangle]
pub unsafe extern "C" fn if(PCIE: priv->mode ==) -> else {
    else if (priv.mode == PCIE)
    ret = spear1340_miphy_pcie_exit(priv);
    return ret;
    }
    static const struct of_device_id spear1340_miphy_of_match[] = {
    { .compatible = "st,spear1340-miphy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, spear1340_miphy_of_match);
    static const struct phy_ops spear1340_miphy_ops = {
    .init = spear1340_miphy_init,
    .exit = spear1340_miphy_exit,
    .owner = THIS_MODULE,
    };

#[no_mangle]
unsafe extern "C" fn spear1340_miphy_suspend(dev: *mut device) -> c_int {
    static int spear1340_miphy_suspend(struct device *dev)
    {
    struct spear1340_miphy_priv *priv = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (priv.mode == SATA)
    ret = spear1340_miphy_sata_exit(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn spear1340_miphy_resume(dev: *mut device) -> c_int {
    static int spear1340_miphy_resume(struct device *dev)
    {
    struct spear1340_miphy_priv *priv = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (priv.mode == SATA)
    ret = spear1340_miphy_sata_init(priv);
    return ret;
    }

    static SIMPLE_DEV_PM_OPS(spear1340_miphy_pm_ops, spear1340_miphy_suspend,
    spear1340_miphy_resume);
    static struct phy *spear1340_miphy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct spear1340_miphy_priv *priv = dev_get_drvdata(dev);
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
unsafe extern "C" fn spear1340_miphy_probe(pdev: *mut platform_device) -> c_int {
    static int spear1340_miphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spear1340_miphy_priv *priv;
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
    priv.phy = devm_phy_create(dev, core::ptr::null_mut(), &spear1340_miphy_ops);
    if (IS_ERR(priv.phy)) {
    dev_err(dev, "failed to create SATA PCIe PHY\n");
    return PTR_ERR(priv.phy);
    }
    dev_set_drvdata(dev, priv);
    phy_set_drvdata(priv.phy, priv);
    phy_provider =
    devm_of_phy_provider_register(dev, spear1340_miphy_xlate);
    if (IS_ERR(phy_provider)) {
    dev_err(dev, "failed to register phy provider\n");
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
    static struct platform_driver spear1340_miphy_driver = {
    .probe		= spear1340_miphy_probe,
    .driver = {
    .name = "spear1340-miphy",
    .pm = &spear1340_miphy_pm_ops,
    .of_match_table = spear1340_miphy_of_match,
    },
    };
    module_platform_driver(spear1340_miphy_driver);
    MODULE_DESCRIPTION("ST SPEAR1340-MIPHY driver");
    MODULE_AUTHOR("Pratyush Anand <pratyush.anand@gmail.com>");
    MODULE_LICENSE("GPL v2");
