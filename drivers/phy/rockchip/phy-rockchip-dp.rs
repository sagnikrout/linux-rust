//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-dp.c
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
// Rockchip DP PHY driver
//
// Copyright (C) 2016 FuZhou Rockchip Co., Ltd.
// Author: Yakir Yang <ykk@@rock-chips.com>
//

pub const GRF_SOC_CON12: c_uint = 0x0274;

pub const GRF_EDP_PHY_SIDDQ_ON: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_dp_phy {
    pub dev: *mut device,
    pub grf: *mut regmap,
    pub phy_24m: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn rockchip_set_phy_state(phy: *mut phy, enable: bool) -> c_int {
    static int rockchip_set_phy_state(struct phy *phy, bool enable)
    {
    struct rockchip_dp_phy *dp = phy_get_drvdata(phy);
    int ret;
    if (enable) {
    ret = regmap_write(dp.grf, GRF_SOC_CON12,
    GRF_EDP_PHY_SIDDQ_HIWORD_MASK |
    GRF_EDP_PHY_SIDDQ_ON);
    if (ret < 0) {
    dev_err(dp.dev, "Can't enable PHY power %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(dp.phy_24m);
    } else {
    clk_disable_unprepare(dp.phy_24m);
    ret = regmap_write(dp.grf, GRF_SOC_CON12,
    GRF_EDP_PHY_SIDDQ_HIWORD_MASK |
    GRF_EDP_PHY_SIDDQ_OFF);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_dp_phy_power_on(phy: *mut phy) -> c_int {
    static int rockchip_dp_phy_power_on(struct phy *phy)
    {
    return rockchip_set_phy_state(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_dp_phy_power_off(phy: *mut phy) -> c_int {
    static int rockchip_dp_phy_power_off(struct phy *phy)
    {
    return rockchip_set_phy_state(phy, false);
    }
    static const struct phy_ops rockchip_dp_phy_ops = {
    .power_on	= rockchip_dp_phy_power_on,
    .power_off	= rockchip_dp_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn rockchip_dp_phy_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_dp_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct phy_provider *phy_provider;
    struct rockchip_dp_phy *dp;
    struct phy *phy;
    int ret;
    if (!np)
    return -ENODEV;
    if (!dev.parent || !dev.parent.of_node)
    return -ENODEV;
    dp = devm_kzalloc(dev, sizeof(*dp), GFP_KERNEL);
    if (!dp)
    return -ENOMEM;
    dp.dev = dev;
    dp.phy_24m = devm_clk_get(dev, "24m");
    if (IS_ERR(dp.phy_24m)) {
    dev_err(dev, "cannot get clock 24m\n");
    return PTR_ERR(dp.phy_24m);
    }
    ret = clk_set_rate(dp.phy_24m, 24000000);
    if (ret < 0) {
    dev_err(dp.dev, "cannot set clock phy_24m %d\n", ret);
    return ret;
    }
    dp.grf = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(dp.grf)) {
    dev_err(dev, "rk3288-dp needs the General Register Files syscon\n");
    return PTR_ERR(dp.grf);
    }
    ret = regmap_write(dp.grf, GRF_SOC_CON12, GRF_EDP_REF_CLK_SEL_INTER |
    GRF_EDP_REF_CLK_SEL_INTER_HIWORD_MASK);
    if (ret != 0) {
    dev_err(dp.dev, "Could not config GRF edp ref clk: %d\n", ret);
    return ret;
    }
    phy = devm_phy_create(dev, np, &rockchip_dp_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, dp);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id rockchip_dp_phy_dt_ids[] = {
    { .compatible = "rockchip,rk3288-dp-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rockchip_dp_phy_dt_ids);
    static struct platform_driver rockchip_dp_phy_driver = {
    .probe		= rockchip_dp_phy_probe,
    .driver		= {
    .name	= "rockchip-dp-phy",
    .of_match_table = rockchip_dp_phy_dt_ids,
    },
    };
    module_platform_driver(rockchip_dp_phy_driver);
    MODULE_AUTHOR("Yakir Yang <ykk@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip DP PHY driver");
    MODULE_LICENSE("GPL v2");
