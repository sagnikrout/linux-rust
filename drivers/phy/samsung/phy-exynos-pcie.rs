//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos-pcie.c
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
// Samsung Exynos SoC series PCIe PHY driver
//
// Phy provider for PCIe controller on Exynos SoC series
//
// Copyright (C) 2017-2020 Samsung Electronics Co., Ltd.
// Jaehoon Chung <jh80.chung@samsung.com>
//

// Sysreg FSYS register offsets and bits for Exynos5433
pub const PCIE_EXYNOS5433_PHY_MAC_RESET: c_uint = 0x0208;
pub const PCIE_MAC_RESET_MASK: c_uint = 0xFF;

pub const PCIE_EXYNOS5433_PHY_L1SUB_CM_CON: c_uint = 0x1010;

pub const PCIE_EXYNOS5433_PHY_COMMON_RESET: c_uint = 0x1020;

pub const PCIE_EXYNOS5433_PHY_GLOBAL_RESET: c_uint = 0x1040;

pub const PCIE_REFCLK_MASK: c_uint = 0x16;

// PMU PCIE PHY isolation control
pub const EXYNOS5433_PMU_PCIE_PHY_OFFSET: c_uint = 0x730;
// For Exynos pcie phy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pcie_phy {
    pub base: *mut void __iomem,
    pub pmureg: *mut regmap,
    pub fsysreg: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn exynos_pcie_phy_writel(base: *mut void __iomem, val: u32, offset: u32) {
    static void exynos_pcie_phy_writel(void __iomem *base, u32 val, u32 offset)
    {
    writel(val, base + offset);
    }
// Exynos5433 specific functions
#[no_mangle]
unsafe extern "C" fn exynos5433_pcie_phy_init(phy: *mut phy) -> c_int {
    static int exynos5433_pcie_phy_init(struct phy *phy)
    {
    struct exynos_pcie_phy *ep = phy_get_drvdata(phy);
    regmap_update_bits(ep.pmureg, EXYNOS5433_PMU_PCIE_PHY_OFFSET,
    BIT(0), 1);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_GLOBAL_RESET,
    PCIE_APP_REQ_EXIT_L1_MODE, 0);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_L1SUB_CM_CON,
    PCIE_REFCLK_GATING_EN, 0);
    regmap_update_bits(ep.fsysreg,	PCIE_EXYNOS5433_PHY_COMMON_RESET,
    PCIE_PHY_RESET, 1);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_MAC_RESET,
    PCIE_MAC_RESET, 0);
// PHY refclk 24MHz
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_GLOBAL_RESET,
    PCIE_REFCLK_MASK, PCIE_REFCLK);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_GLOBAL_RESET,
    PCIE_GLOBAL_RESET, 0);
    exynos_pcie_phy_writel(ep.base, 0x11, PCIE_PHY_OFFSET(0x3));
// band gap reference on
    exynos_pcie_phy_writel(ep.base, 0, PCIE_PHY_OFFSET(0x20));
    exynos_pcie_phy_writel(ep.base, 0, PCIE_PHY_OFFSET(0x4b));
// jitter tuning
    exynos_pcie_phy_writel(ep.base, 0x34, PCIE_PHY_OFFSET(0x4));
    exynos_pcie_phy_writel(ep.base, 0x02, PCIE_PHY_OFFSET(0x7));
    exynos_pcie_phy_writel(ep.base, 0x41, PCIE_PHY_OFFSET(0x21));
    exynos_pcie_phy_writel(ep.base, 0x7F, PCIE_PHY_OFFSET(0x14));
    exynos_pcie_phy_writel(ep.base, 0xC0, PCIE_PHY_OFFSET(0x15));
    exynos_pcie_phy_writel(ep.base, 0x61, PCIE_PHY_OFFSET(0x36));
// D0 uninit..
    exynos_pcie_phy_writel(ep.base, 0x44, PCIE_PHY_OFFSET(0x3D));
// 24MHz
    exynos_pcie_phy_writel(ep.base, 0x94, PCIE_PHY_OFFSET(0x8));
    exynos_pcie_phy_writel(ep.base, 0xA7, PCIE_PHY_OFFSET(0x9));
    exynos_pcie_phy_writel(ep.base, 0x93, PCIE_PHY_OFFSET(0xA));
    exynos_pcie_phy_writel(ep.base, 0x6B, PCIE_PHY_OFFSET(0xC));
    exynos_pcie_phy_writel(ep.base, 0xA5, PCIE_PHY_OFFSET(0xF));
    exynos_pcie_phy_writel(ep.base, 0x34, PCIE_PHY_OFFSET(0x16));
    exynos_pcie_phy_writel(ep.base, 0xA3, PCIE_PHY_OFFSET(0x17));
    exynos_pcie_phy_writel(ep.base, 0xA7, PCIE_PHY_OFFSET(0x1A));
    exynos_pcie_phy_writel(ep.base, 0x71, PCIE_PHY_OFFSET(0x23));
    exynos_pcie_phy_writel(ep.base, 0x4C, PCIE_PHY_OFFSET(0x24));
    exynos_pcie_phy_writel(ep.base, 0x0E, PCIE_PHY_OFFSET(0x26));
    exynos_pcie_phy_writel(ep.base, 0x14, PCIE_PHY_OFFSET(0x7));
    exynos_pcie_phy_writel(ep.base, 0x48, PCIE_PHY_OFFSET(0x43));
    exynos_pcie_phy_writel(ep.base, 0x44, PCIE_PHY_OFFSET(0x44));
    exynos_pcie_phy_writel(ep.base, 0x03, PCIE_PHY_OFFSET(0x45));
    exynos_pcie_phy_writel(ep.base, 0xA7, PCIE_PHY_OFFSET(0x48));
    exynos_pcie_phy_writel(ep.base, 0x13, PCIE_PHY_OFFSET(0x54));
    exynos_pcie_phy_writel(ep.base, 0x04, PCIE_PHY_OFFSET(0x31));
    exynos_pcie_phy_writel(ep.base, 0, PCIE_PHY_OFFSET(0x32));
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_COMMON_RESET,
    PCIE_PHY_RESET, 0);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_MAC_RESET,
    PCIE_MAC_RESET_MASK, PCIE_MAC_RESET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5433_pcie_phy_exit(phy: *mut phy) -> c_int {
    static int exynos5433_pcie_phy_exit(struct phy *phy)
    {
    struct exynos_pcie_phy *ep = phy_get_drvdata(phy);
    regmap_update_bits(ep.fsysreg, PCIE_EXYNOS5433_PHY_L1SUB_CM_CON,
    PCIE_REFCLK_GATING_EN, PCIE_REFCLK_GATING_EN);
    regmap_update_bits(ep.pmureg, EXYNOS5433_PMU_PCIE_PHY_OFFSET,
    BIT(0), 0);
    return 0;
    }
    static const struct phy_ops exynos5433_phy_ops = {
    .init		= exynos5433_pcie_phy_init,
    .exit		= exynos5433_pcie_phy_exit,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id exynos_pcie_phy_match[] = {
    {
    .compatible = "samsung,exynos5433-pcie-phy",
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn exynos_pcie_phy_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_pcie_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct exynos_pcie_phy *exynos_phy;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    exynos_phy = devm_kzalloc(dev, sizeof(*exynos_phy), GFP_KERNEL);
    if (!exynos_phy)
    return -ENOMEM;
    exynos_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(exynos_phy.base))
    return PTR_ERR(exynos_phy.base);
    exynos_phy.pmureg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "samsung,pmu-syscon");
    if (IS_ERR(exynos_phy.pmureg)) {
    dev_err(&pdev.dev, "PMU regmap lookup failed.\n");
    return PTR_ERR(exynos_phy.pmureg);
    }
    exynos_phy.fsysreg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "samsung,fsys-sysreg");
    if (IS_ERR(exynos_phy.fsysreg)) {
    dev_err(&pdev.dev, "FSYS sysreg regmap lookup failed.\n");
    return PTR_ERR(exynos_phy.fsysreg);
    }
    generic_phy = devm_phy_create(dev, dev.of_node, &exynos5433_phy_ops);
    if (IS_ERR(generic_phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(generic_phy);
    }
    phy_set_drvdata(generic_phy, exynos_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver exynos_pcie_phy_driver = {
    .probe	= exynos_pcie_phy_probe,
    .driver = {
    .of_match_table	= exynos_pcie_phy_match,
    .name		= "exynos_pcie_phy",
    .suppress_bind_attrs = true,
    }
    };
    builtin_platform_driver(exynos_pcie_phy_driver);
