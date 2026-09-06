//! Automatically rewritten from C to Rust
//! Source: drivers/phy/starfive/phy-jh7110-dphy-rx.c
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
// StarFive JH7110 DPHY RX driver
//
// Copyright (C) 2023 StarFive Technology Co., Ltd.
// Author: Jack Zhu <jack.zhu@starfivetech.com>
// Author: Changhuang Liang <changhuang.liang@starfivetech.com>
//

pub const STF_MAP_LANES_NUM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stf_dphy_info {
//
// @maps:
//
// Physical lanes and logic lanes mapping table.
//
// The default order is:
// [clk lane0, data lane 0, data lane 1, data lane 2, date lane 3, clk lane 1]
//
    pub maps: [u8; STF_MAP_LANES_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stf_dphy {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub cfg_clk: *mut clk,
    pub ref_clk: *mut clk,
    pub tx_clk: *mut clk,
    pub rstc: *mut reset_control,
    pub mipi_0p9: *mut regulator,
    pub phy: *mut phy,
    pub info: *const stf_dphy_info,
}

#[no_mangle]
unsafe extern "C" fn stf_dphy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int stf_dphy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct stf_dphy *dphy = phy_get_drvdata(phy);
    const struct stf_dphy_info *info = dphy.info;
    writel(FIELD_PREP(STF_DPHY_ENABLE_CLK, 1) |
    FIELD_PREP(STF_DPHY_ENABLE_CLK1, 1) |
    FIELD_PREP(STF_DPHY_ENABLE_LAN0, 1) |
    FIELD_PREP(STF_DPHY_ENABLE_LAN1, 1) |
    FIELD_PREP(STF_DPHY_ENABLE_LAN2, 1) |
    FIELD_PREP(STF_DPHY_ENABLE_LAN3, 1) |
    FIELD_PREP(STF_DPHY_LANE_SWAP_CLK, info.maps[0]) |
    FIELD_PREP(STF_DPHY_LANE_SWAP_CLK1, info.maps[5]) |
    FIELD_PREP(STF_DPHY_LANE_SWAP_LAN0, info.maps[1]) |
    FIELD_PREP(STF_DPHY_LANE_SWAP_LAN1, info.maps[2]),
    dphy.regs + STF_DPHY_APBCFGSAIF_SYSCFG(188));
    writel(FIELD_PREP(STF_DPHY_LANE_SWAP_LAN2, info.maps[3]) |
    FIELD_PREP(STF_DPHY_LANE_SWAP_LAN3, info.maps[4]) |
    FIELD_PREP(STF_DPHY_PRECOUNTER_IN_CLK, 8),
    dphy.regs + STF_DPHY_APBCFGSAIF_SYSCFG(192));
    writel(FIELD_PREP(STF_DPHY_PRECOUNTER_IN_CLK1, 8) |
    FIELD_PREP(STF_DPHY_PRECOUNTER_IN_LAN0, 7) |
    FIELD_PREP(STF_DPHY_PRECOUNTER_IN_LAN1, 7) |
    FIELD_PREP(STF_DPHY_PRECOUNTER_IN_LAN2, 7),
    dphy.regs + STF_DPHY_APBCFGSAIF_SYSCFG(196));
    writel(FIELD_PREP(STF_DPHY_PRECOUNTER_IN_LAN3, 7),
    dphy.regs + STF_DPHY_APBCFGSAIF_SYSCFG(200));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stf_dphy_power_on(phy: *mut phy) -> c_int {
    static int stf_dphy_power_on(struct phy *phy)
    {
    struct stf_dphy *dphy = phy_get_drvdata(phy);
    int ret;
    ret = pm_runtime_resume_and_get(dphy.dev);
    if (ret < 0)
    return ret;
    ret = regulator_enable(dphy.mipi_0p9);
    if (ret) {
    pm_runtime_put(dphy.dev);
    return ret;
    }
    clk_set_rate(dphy.cfg_clk, 99000000);
    clk_set_rate(dphy.ref_clk, 49500000);
    clk_set_rate(dphy.tx_clk, 19800000);
    reset_control_deassert(dphy.rstc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stf_dphy_power_off(phy: *mut phy) -> c_int {
    static int stf_dphy_power_off(struct phy *phy)
    {
    struct stf_dphy *dphy = phy_get_drvdata(phy);
    reset_control_assert(dphy.rstc);
    regulator_disable(dphy.mipi_0p9);
    pm_runtime_put_sync(dphy.dev);
    return 0;
    }
    static const struct phy_ops stf_dphy_ops = {
    .configure = stf_dphy_configure,
    .power_on  = stf_dphy_power_on,
    .power_off = stf_dphy_power_off,
    };
#[no_mangle]
unsafe extern "C" fn stf_dphy_probe(pdev: *mut platform_device) -> c_int {
    static int stf_dphy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct stf_dphy *dphy;
    int ret;
    dphy = devm_kzalloc(&pdev.dev, sizeof(*dphy), GFP_KERNEL);
    if (!dphy)
    return -ENOMEM;
    dphy.info = of_device_get_match_data(&pdev.dev);
    dev_set_drvdata(&pdev.dev, dphy);
    dphy.dev = &pdev.dev;
    dphy.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dphy.regs))
    return PTR_ERR(dphy.regs);
    dphy.cfg_clk = devm_clk_get(&pdev.dev, "cfg");
    if (IS_ERR(dphy.cfg_clk))
    return PTR_ERR(dphy.cfg_clk);
    dphy.ref_clk = devm_clk_get(&pdev.dev, "ref");
    if (IS_ERR(dphy.ref_clk))
    return PTR_ERR(dphy.ref_clk);
    dphy.tx_clk = devm_clk_get(&pdev.dev, "tx");
    if (IS_ERR(dphy.tx_clk))
    return PTR_ERR(dphy.tx_clk);
    dphy.rstc = devm_reset_control_array_get_exclusive(&pdev.dev);
    if (IS_ERR(dphy.rstc))
    return PTR_ERR(dphy.rstc);
    dphy.mipi_0p9 = devm_regulator_get(&pdev.dev, "mipi_0p9");
    if (IS_ERR(dphy.mipi_0p9))
    return PTR_ERR(dphy.mipi_0p9);
    dphy.phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &stf_dphy_ops);
    if (IS_ERR(dphy.phy)) {
    dev_err(&pdev.dev, "Failed to create PHY\n");
    return PTR_ERR(dphy.phy);
    }
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return ret;
    phy_set_drvdata(dphy.phy, dphy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev,
    of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct stf_dphy_info starfive_dphy_info = {
    .maps = {4, 0, 1, 2, 3, 5},
    };
    static const struct of_device_id stf_dphy_dt_ids[] = {
    {
    .compatible = "starfive,jh7110-dphy-rx",
    .data = &starfive_dphy_info,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, stf_dphy_dt_ids);
    static struct platform_driver stf_dphy_driver = {
    .probe = stf_dphy_probe,
    .driver = {
    .name	= "starfive-dphy-rx",
    .of_match_table = stf_dphy_dt_ids,
    },
    };
    module_platform_driver(stf_dphy_driver);
    MODULE_AUTHOR("Jack Zhu <jack.zhu@starfivetech.com>");
    MODULE_AUTHOR("Changhuang Liang <changhuang.liang@starfivetech.com>");
    MODULE_DESCRIPTION("StarFive JH7110 DPHY RX driver");
    MODULE_LICENSE("GPL");
