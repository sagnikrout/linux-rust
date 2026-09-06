//! Automatically rewritten from C to Rust
//! Source: drivers/phy/amlogic/phy-meson-axg-pcie.c
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
// Amlogic AXG PCIE PHY driver
//
// Copyright (C) 2020 Remi Pommarel <repk@triplefau.lt>
//

pub const MESON_PCIE_REG0: c_uint = 0x00;

    MESON_PCIE_COMMON_REF_CLK)
pub const MESON_PCIE_RESET_DELAY: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_axg_pcie_priv {
    pub phy: *mut phy,
    pub analog: *mut phy,
    pub regmap: *mut regmap,
    pub reset: *mut reset_control,
}

    static const struct regmap_config phy_axg_pcie_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = MESON_PCIE_REG0,
    };
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_power_on(phy: *mut phy) -> c_int {
    static int phy_axg_pcie_power_on(struct phy *phy)
    {
    struct phy_axg_pcie_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = phy_power_on(priv.analog);
    if (ret != 0)
    return ret;
    regmap_update_bits(priv.regmap, MESON_PCIE_REG0,
    MESON_PCIE_POWERDOWN, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_power_off(phy: *mut phy) -> c_int {
    static int phy_axg_pcie_power_off(struct phy *phy)
    {
    struct phy_axg_pcie_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = phy_power_off(priv.analog);
    if (ret != 0)
    return ret;
    regmap_update_bits(priv.regmap, MESON_PCIE_REG0,
    MESON_PCIE_POWERDOWN, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_init(phy: *mut phy) -> c_int {
    static int phy_axg_pcie_init(struct phy *phy)
    {
    struct phy_axg_pcie_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = phy_init(priv.analog);
    if (ret != 0)
    return ret;
    regmap_write(priv.regmap, MESON_PCIE_REG0, MESON_PCIE_PHY_INIT);
    return reset_control_reset(priv.reset);
    }
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_exit(phy: *mut phy) -> c_int {
    static int phy_axg_pcie_exit(struct phy *phy)
    {
    struct phy_axg_pcie_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = phy_exit(priv.analog);
    if (ret != 0)
    return ret;
    return reset_control_reset(priv.reset);
    }
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_reset(phy: *mut phy) -> c_int {
    static int phy_axg_pcie_reset(struct phy *phy)
    {
    struct phy_axg_pcie_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    ret = phy_reset(priv.analog);
    if (ret != 0)
    goto out;
    ret = reset_control_assert(priv.reset);
    if (ret != 0)
    goto out;
    udelay(MESON_PCIE_RESET_DELAY);
    ret = reset_control_deassert(priv.reset);
    if (ret != 0)
    goto out;
    udelay(MESON_PCIE_RESET_DELAY);
    out:
    return ret;
    }
    static const struct phy_ops phy_axg_pcie_ops = {
    .init = phy_axg_pcie_init,
    .exit = phy_axg_pcie_exit,
    .power_on = phy_axg_pcie_power_on,
    .power_off = phy_axg_pcie_power_off,
    .reset = phy_axg_pcie_reset,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn phy_axg_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int phy_axg_pcie_probe(struct platform_device *pdev)
    {
    struct phy_provider *pphy;
    struct device *dev = &pdev.dev;
    struct phy_axg_pcie_priv *priv;
    struct device_node *np = dev.of_node;
    void __iomem *base;
    priv = devm_kmalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.regmap = devm_regmap_init_mmio(dev, base,
    &phy_axg_pcie_regmap_conf);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.reset = devm_reset_control_array_get_exclusive(dev);
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    priv.analog = devm_phy_get(dev, "analog");
    if (IS_ERR(priv.analog))
    return PTR_ERR(priv.analog);
    priv.phy = devm_phy_create(dev, np, &phy_axg_pcie_ops);
    if (IS_ERR(priv.phy))
    return dev_err_probe(dev, PTR_ERR(priv.phy),
    "failed to create PHY\n");
    phy_set_drvdata(priv.phy, priv);
    dev_set_drvdata(dev, priv);
    pphy = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(pphy);
    }
    static const struct of_device_id phy_axg_pcie_of_match[] = {
    {
    .compatible = "amlogic,axg-pcie-phy",
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, phy_axg_pcie_of_match);
    static struct platform_driver phy_axg_pcie_driver = {
    .probe = phy_axg_pcie_probe,
    .driver = {
    .name = "phy-axg-pcie",
    .of_match_table = phy_axg_pcie_of_match,
    },
    };
    module_platform_driver(phy_axg_pcie_driver);
    MODULE_AUTHOR("Remi Pommarel <repk@triplefau.lt>");
    MODULE_DESCRIPTION("Amlogic AXG PCIE PHY driver");
    MODULE_LICENSE("GPL v2");
