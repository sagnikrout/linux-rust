//! Automatically rewritten from C to Rust
//! Source: drivers/phy/tegra/phy-tegra194-p2u.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// P2U (PIPE to UPHY) driver for Tegra T194 SoC
//
// Copyright (C) 2019-2022 NVIDIA Corporation.
//
// Author: Vidya Sagar <vidyas@nvidia.com>
//

pub const P2U_CONTROL_CMN: c_uint = 0x74;

pub const P2U_PERIODIC_EQ_CTRL_GEN3: c_uint = 0xc0;

pub const P2U_PERIODIC_EQ_CTRL_GEN4: c_uint = 0xc4;

pub const P2U_RX_DEBOUNCE_TIME: c_uint = 0xa4;
pub const P2U_RX_DEBOUNCE_TIME_DEBOUNCE_TIMER_MASK: c_uint = 0xffff;
pub const P2U_RX_DEBOUNCE_TIME_DEBOUNCE_TIMER_VAL: c_int = 160;
pub const P2U_DIR_SEARCH_CTRL: c_uint = 0xd4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_p2u_of_data {
    pub one_dir_search: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_p2u {
    pub base: *mut void __iomem,
    pub /: *mut *mut bool skip_sz_protection_en; / Needed to support two retimers,
    pub of_data: *mut tegra_p2u_of_data,
}

    static inline void p2u_writel(struct tegra_p2u *phy, const u32 value,
    const u32 reg)
    {
    writel_relaxed(value, phy.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn p2u_readl(phy: *mut tegra_p2u, reg: u32) -> u32 {
    static inline u32 p2u_readl(struct tegra_p2u *phy, const u32 reg)
    {
    return readl_relaxed(phy.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn tegra_p2u_power_on(x: *mut phy) -> c_int {
    static int tegra_p2u_power_on(struct phy *x)
    {
    struct tegra_p2u *phy = phy_get_drvdata(x);
    u32 val;
    if (phy.skip_sz_protection_en) {
    val = p2u_readl(phy, P2U_CONTROL_CMN);
    val |= P2U_CONTROL_CMN_SKP_SIZE_PROTECTION_EN;
    p2u_writel(phy, val, P2U_CONTROL_CMN);
    }
    val = p2u_readl(phy, P2U_PERIODIC_EQ_CTRL_GEN3);
    val &= ~P2U_PERIODIC_EQ_CTRL_GEN3_PERIODIC_EQ_EN;
    val |= P2U_PERIODIC_EQ_CTRL_GEN3_INIT_PRESET_EQ_TRAIN_EN;
    p2u_writel(phy, val, P2U_PERIODIC_EQ_CTRL_GEN3);
    val = p2u_readl(phy, P2U_PERIODIC_EQ_CTRL_GEN4);
    val |= P2U_PERIODIC_EQ_CTRL_GEN4_INIT_PRESET_EQ_TRAIN_EN;
    p2u_writel(phy, val, P2U_PERIODIC_EQ_CTRL_GEN4);
    val = p2u_readl(phy, P2U_RX_DEBOUNCE_TIME);
    val &= ~P2U_RX_DEBOUNCE_TIME_DEBOUNCE_TIMER_MASK;
    val |= P2U_RX_DEBOUNCE_TIME_DEBOUNCE_TIMER_VAL;
    p2u_writel(phy, val, P2U_RX_DEBOUNCE_TIME);
    if (phy.of_data.one_dir_search) {
    val = p2u_readl(phy, P2U_DIR_SEARCH_CTRL);
    val &= ~P2U_DIR_SEARCH_CTRL_GEN4_FINE_GRAIN_SEARCH_TWICE;
    p2u_writel(phy, val, P2U_DIR_SEARCH_CTRL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_p2u_calibrate(x: *mut phy) -> c_int {
    static int tegra_p2u_calibrate(struct phy *x)
    {
    struct tegra_p2u *phy = phy_get_drvdata(x);
    u32 val;
    val = p2u_readl(phy, P2U_CONTROL_CMN);
    val |= P2U_CONTROL_CMN_ENABLE_L2_EXIT_RATE_CHANGE;
    p2u_writel(phy, val, P2U_CONTROL_CMN);
    return 0;
    }
    static const struct phy_ops ops = {
    .power_on = tegra_p2u_power_on,
    .calibrate = tegra_p2u_calibrate,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn tegra_p2u_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_p2u_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct phy *generic_phy;
    struct tegra_p2u *phy;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.of_data =
    (struct tegra_p2u_of_data *)of_device_get_match_data(dev);
    if (!phy.of_data)
    return -EINVAL;
    phy.base = devm_platform_ioremap_resource_byname(pdev, "ctl");
    if (IS_ERR(phy.base))
    return PTR_ERR(phy.base);
    phy.skip_sz_protection_en =
    of_property_read_bool(dev.of_node,
    "nvidia,skip-sz-protect-en");
    platform_set_drvdata(pdev, phy);
    generic_phy = devm_phy_create(dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(generic_phy))
    return PTR_ERR(generic_phy);
    phy_set_drvdata(generic_phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
    static const struct tegra_p2u_of_data tegra194_p2u_of_data = {
    .one_dir_search = false,
    };
    static const struct tegra_p2u_of_data tegra234_p2u_of_data = {
    .one_dir_search = true,
    };
    static const struct of_device_id tegra_p2u_id_table[] = {
    {
    .compatible = "nvidia,tegra194-p2u",
    .data = &tegra194_p2u_of_data,
    },
    {
    .compatible = "nvidia,tegra234-p2u",
    .data = &tegra234_p2u_of_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra_p2u_id_table);
    static struct platform_driver tegra_p2u_driver = {
    .probe = tegra_p2u_probe,
    .driver = {
    .name = "tegra194-p2u",
    .of_match_table = tegra_p2u_id_table,
    },
    };
    module_platform_driver(tegra_p2u_driver);
    MODULE_AUTHOR("Vidya Sagar <vidyas@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra194 PIPE2UPHY PHY driver");
    MODULE_LICENSE("GPL v2");
