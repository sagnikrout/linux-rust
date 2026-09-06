//! Automatically rewritten from C to Rust
//! Source: drivers/phy/axiado/phy-axiado-emmc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Axiado eMMC PHY driver
//
// Copyright (C) 2017 Arasan Chip Systems Inc.
// Copyright (C) 2022-2026 Axiado Corporation (or its affiliates).
//
// Based on Arasan Driver (sdhci-pci-arasan.c)
// sdhci-pci-arasan.c - Driver for Arasan PCI Controller with integrated phy.
//

// Arasan eMMC 5.1 - PHY configuration registers
pub const CAP_REG_IN_S1_MSB: c_uint = 0x04;
pub const PHY_CTRL_1: c_uint = 0x38;
pub const PHY_CTRL_2: c_uint = 0x3c;
pub const PHY_CTRL_3: c_uint = 0x40;
pub const STATUS: c_uint = 0x50;

// Pull-UP Enable on CMD Line

// Selection value for the optimum delay from 1-32 output tap lines
pub const OTAP_DLY: c_uint = 0x02;
// DLL charge pump current trim default [1000]
pub const DLL_TRM_ICP: c_uint = 0x08;
// Select the frequency range of DLL Operation
pub const FRQ_SEL: c_uint = 0x01;

pub const CALDONE_MASK: c_uint = 0x40;
pub const DLL_RDY_MASK: c_uint = 0x1;

pub const CLK_MULTIPLIER: c_uint = 0xc008e;
pub const POLL_TIMEOUT_MS: c_int = 3000;
pub const POLL_DELAY_US: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axiado_emmc_phy {
    pub reg_base: *mut void __iomem,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn axiado_emmc_phy_init(phy: *mut phy) -> c_int {
    static int axiado_emmc_phy_init(struct phy *phy)
    {
    struct axiado_emmc_phy *ax_phy = phy_get_drvdata(phy);
    struct device *dev = ax_phy.dev;
    u32 val;
    int ret;
    val = readl(ax_phy.reg_base + PHY_CTRL_1);
    writel(val | RETB_ENBL | RTRIM_EN, ax_phy.reg_base + PHY_CTRL_1);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    writel(val | PDB_ENBL, ax_phy.reg_base + PHY_CTRL_3);
    ret = readl_poll_timeout(ax_phy.reg_base + STATUS, val,
    val & CALDONE_MASK, POLL_DELAY_US,
    POLL_TIMEOUT_MS * 1000);
    if (ret) {
    dev_err(dev, "PHY calibration timeout\n");
    return ret;
    }
    val = readl(ax_phy.reg_base + PHY_CTRL_1);
    writel(val | REN_CMD_EN | PU_CMD_EN, ax_phy.reg_base + PHY_CTRL_1);
    val = readl(ax_phy.reg_base + PHY_CTRL_2);
    writel(val | REN_STRB, ax_phy.reg_base + PHY_CTRL_2);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    writel(val | MAX_CLK_BUF0 | MAX_CLK_BUF1 | MAX_CLK_BUF2,
    ax_phy.reg_base + PHY_CTRL_3);
    writel(CLK_MULTIPLIER, ax_phy.reg_base + CAP_REG_IN_S1_MSB);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    writel(val | SEL_DLY_RXCLK | SEL_DLY_TXCLK,
    ax_phy.reg_base + PHY_CTRL_3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axiado_emmc_phy_power_on(phy: *mut phy) -> c_int {
    static int axiado_emmc_phy_power_on(struct phy *phy)
    {
    struct axiado_emmc_phy *ax_phy = phy_get_drvdata(phy);
    struct device *dev = ax_phy.dev;
    u32 val;
    int ret;
    val = readl(ax_phy.reg_base + PHY_CTRL_1);
    writel(val | RETB_ENBL, ax_phy.reg_base + PHY_CTRL_1);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    writel(val | PDB_ENBL, ax_phy.reg_base + PHY_CTRL_3);
    val = readl(ax_phy.reg_base + PHY_CTRL_2);
    writel(val | OTAP_SEL(OTAP_DLY), ax_phy.reg_base + PHY_CTRL_2);
    val = readl(ax_phy.reg_base + PHY_CTRL_1);
    writel(val | DLL_TRM(DLL_TRM_ICP), ax_phy.reg_base + PHY_CTRL_1);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    writel(val | DLL_FRQSEL(FRQ_SEL), ax_phy.reg_base + PHY_CTRL_3);
    ret = read_poll_timeout(readl, val, val & DLL_RDY_MASK, POLL_DELAY_US,
    POLL_TIMEOUT_MS * 1000, false,
    ax_phy.reg_base + STATUS);
    if (ret) {
    dev_err(dev, "DLL ready timeout\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axiado_emmc_phy_power_off(phy: *mut phy) -> c_int {
    static int axiado_emmc_phy_power_off(struct phy *phy)
    {
    struct axiado_emmc_phy *ax_phy = phy_get_drvdata(phy);
    u32 val;
    val = readl(ax_phy.reg_base + PHY_CTRL_1);
    val &= ~(DLL_TRM_MASK | DLL_ENBL);
    writel(val, ax_phy.reg_base + PHY_CTRL_1);
    val = readl(ax_phy.reg_base + PHY_CTRL_3);
    val &= ~(DLL_FRQSEL_MASK | PDB_ENBL);
    writel(val, ax_phy.reg_base + PHY_CTRL_3);
    return 0;
    }
    static const struct phy_ops axiado_emmc_phy_ops = {
    .init = axiado_emmc_phy_init,
    .power_on = axiado_emmc_phy_power_on,
    .power_off = axiado_emmc_phy_power_off,
    .owner = THIS_MODULE,
    };
    static const struct of_device_id axiado_emmc_phy_of_match[] = {
    { .compatible = "axiado,ax3000-emmc-phy" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, axiado_emmc_phy_of_match);
#[no_mangle]
unsafe extern "C" fn axiado_emmc_phy_probe(pdev: *mut platform_device) -> c_int {
    static int axiado_emmc_phy_probe(struct platform_device *pdev)
    {
    struct axiado_emmc_phy *ax_phy;
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct phy *generic_phy;
    if (!dev.of_node)
    return -ENODEV;
    ax_phy = devm_kzalloc(dev, sizeof(*ax_phy), GFP_KERNEL);
    if (!ax_phy)
    return -ENOMEM;
    ax_phy.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ax_phy.reg_base))
    return PTR_ERR(ax_phy.reg_base);
    ax_phy.dev = dev;
    generic_phy = devm_phy_create(dev, dev.of_node, &axiado_emmc_phy_ops);
    if (IS_ERR(generic_phy))
    return dev_err_probe(dev, PTR_ERR(generic_phy),
    "failed to create PHY\n");
    phy_set_drvdata(generic_phy, ax_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver axiado_emmc_phy_driver = {
    .probe = axiado_emmc_phy_probe,
    .driver = {
    .name = "axiado-emmc-phy",
    .of_match_table = axiado_emmc_phy_of_match,
    },
    };
    module_platform_driver(axiado_emmc_phy_driver);
    MODULE_DESCRIPTION("AX3000 eMMC PHY Driver");
    MODULE_AUTHOR("Axiado Corporation");
    MODULE_LICENSE("GPL");
