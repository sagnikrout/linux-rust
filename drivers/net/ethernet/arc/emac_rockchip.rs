//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/arc/emac_rockchip.c
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
// emac-rockchip.c - Rockchip EMAC specific glue layer
//
// Copyright (C) 2014 Romain Perier <romain.perier@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_rockchip_soc_data {
    pub grf_offset: c_uint,
    pub grf_mode_offset: c_uint,
    pub grf_speed_offset: c_uint,
    pub need_div_macclk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_priv_data {
    pub emac: arc_emac_priv,
    pub grf: *mut regmap,
    pub soc_data: *const emac_rockchip_soc_data,
    pub regulator: *mut regulator,
    pub refclk: *mut clk,
    pub macclk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn emac_rockchip_set_mac_speed(priv: *mut c_void, speed: c_uint) {
    static void emac_rockchip_set_mac_speed(void *priv, unsigned int speed)
    {
    struct rockchip_priv_data *emac = priv;
    let mut speed_offset: u32 = emac.soc_data.grf_speed_offset;
    u32 data;
    let mut err: c_int = 0;
    switch (speed) {
    case 10:
    data = (1 << (speed_offset + 16)) | (0 << speed_offset);
    break;
    case 100:
    data = (1 << (speed_offset + 16)) | (1 << speed_offset);
    break;
    default:
    pr_err("speed %u not supported\n", speed);
    return;
    }
    err = regmap_write(emac.grf, emac.soc_data.grf_offset, data);
    if (err)
    pr_err("unable to apply speed %u to grf (%d)\n", speed, err);
    }
    static const struct emac_rockchip_soc_data emac_rk3036_emac_data = {
    .grf_offset = 0x140,   .grf_mode_offset = 8,
    .grf_speed_offset = 9, .need_div_macclk = 1,
    };
    static const struct emac_rockchip_soc_data emac_rk3066_emac_data = {
    .grf_offset = 0x154,   .grf_mode_offset = 0,
    .grf_speed_offset = 1, .need_div_macclk = 0,
    };
    static const struct emac_rockchip_soc_data emac_rk3188_emac_data = {
    .grf_offset = 0x0a4,   .grf_mode_offset = 0,
    .grf_speed_offset = 1, .need_div_macclk = 0,
    };
    static const struct of_device_id emac_rockchip_dt_ids[] = {
    {
    .compatible = "rockchip,rk3036-emac",
    .data = &emac_rk3036_emac_data,
    },
    {
    .compatible = "rockchip,rk3066-emac",
    .data = &emac_rk3066_emac_data,
    },
    {
    .compatible = "rockchip,rk3188-emac",
    .data = &emac_rk3188_emac_data,
    },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, emac_rockchip_dt_ids);
#[no_mangle]
unsafe extern "C" fn emac_rockchip_probe(pdev: *mut platform_device) -> c_int {
    static int emac_rockchip_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct net_device *ndev;
    struct rockchip_priv_data *priv;
    const struct of_device_id *match;
    phy_interface_t interface;
    u32 data;
    int err;
    if (!pdev.dev.of_node)
    return -ENODEV;
    ndev = alloc_etherdev(sizeof(struct rockchip_priv_data));
    if (!ndev)
    return -ENOMEM;
    platform_set_drvdata(pdev, ndev);
    SET_NETDEV_DEV(ndev, dev);
    priv = netdev_priv(ndev);
    priv.emac.drv_name = DRV_NAME;
    priv.emac.set_mac_speed = emac_rockchip_set_mac_speed;
    err = of_get_phy_mode(dev.of_node, &interface);
    if (err)
    goto out_netdev;
// RK3036/RK3066/RK3188 SoCs only support RMII
    if (interface != PHY_INTERFACE_MODE_RMII) {
    dev_err(dev, "unsupported phy interface mode %d\n", interface);
    err = -ENOTSUPP;
    goto out_netdev;
    }
    priv.grf = syscon_regmap_lookup_by_phandle(dev.of_node,
    "rockchip,grf");
    if (IS_ERR(priv.grf)) {
    dev_err(dev, "failed to retrieve global register file (%ld)\n",
    PTR_ERR(priv.grf));
    err = PTR_ERR(priv.grf);
    goto out_netdev;
    }
    match = of_match_node(emac_rockchip_dt_ids, dev.of_node);
    priv.soc_data = match.data;
    priv.emac.clk = devm_clk_get(dev, "hclk");
    if (IS_ERR(priv.emac.clk)) {
    dev_err(dev, "failed to retrieve host clock (%ld)\n",
    PTR_ERR(priv.emac.clk));
    err = PTR_ERR(priv.emac.clk);
    goto out_netdev;
    }
    priv.refclk = devm_clk_get(dev, "macref");
    if (IS_ERR(priv.refclk)) {
    dev_err(dev, "failed to retrieve reference clock (%ld)\n",
    PTR_ERR(priv.refclk));
    err = PTR_ERR(priv.refclk);
    goto out_netdev;
    }
    err = clk_prepare_enable(priv.refclk);
    if (err) {
    dev_err(dev, "failed to enable reference clock (%d)\n", err);
    goto out_netdev;
    }
// Optional regulator for PHY
    priv.regulator = devm_regulator_get_optional(dev, "phy");
    if (IS_ERR(priv.regulator)) {
    if (PTR_ERR(priv.regulator) == -EPROBE_DEFER) {
    err = -EPROBE_DEFER;
    goto out_clk_disable;
    }
    dev_err(dev, "no regulator found\n");
    priv.regulator = core::ptr::null_mut();
    }
    if (priv.regulator) {
    err = regulator_enable(priv.regulator);
    if (err) {
    dev_err(dev, "failed to enable phy-supply (%d)\n", err);
    goto out_clk_disable;
    }
    }
// Set speed 100M
    data = (1 << (priv.soc_data.grf_speed_offset + 16)) |
    (1 << priv.soc_data.grf_speed_offset);
// Set RMII mode
    data |= (1 << (priv.soc_data.grf_mode_offset + 16)) |
    (0 << priv.soc_data.grf_mode_offset);
    err = regmap_write(priv.grf, priv.soc_data.grf_offset, data);
    if (err) {
    dev_err(dev, "unable to apply initial settings to grf (%d)\n",
    err);
    goto out_regulator_disable;
    }
// RMII interface needs always a rate of 50MHz
    err = clk_set_rate(priv.refclk, 50000000);
    if (err) {
    dev_err(dev,
    "failed to change reference clock rate (%d)\n", err);
    goto out_regulator_disable;
    }
    if (priv.soc_data.need_div_macclk) {
    priv.macclk = devm_clk_get(dev, "macclk");
    if (IS_ERR(priv.macclk)) {
    dev_err(dev, "failed to retrieve mac clock (%ld)\n",
    PTR_ERR(priv.macclk));
    err = PTR_ERR(priv.macclk);
    goto out_regulator_disable;
    }
    err = clk_prepare_enable(priv.macclk);
    if (err) {
    dev_err(dev, "failed to enable mac clock (%d)\n", err);
    goto out_regulator_disable;
    }
// RMII TX/RX needs always a rate of 25MHz
    err = clk_set_rate(priv.macclk, 25000000);
    if (err) {
    dev_err(dev,
    "failed to change mac clock rate (%d)\n", err);
    goto out_clk_disable_macclk;
    }
    }
    err = arc_emac_probe(ndev, interface);
    if (err) {
    dev_err(dev, "failed to probe arc emac (%d)\n", err);
    goto out_clk_disable_macclk;
    }
    return 0;
    out_clk_disable_macclk:
    if (priv.soc_data.need_div_macclk)
    clk_disable_unprepare(priv.macclk);
    out_regulator_disable:
    if (priv.regulator)
    regulator_disable(priv.regulator);
    out_clk_disable:
    clk_disable_unprepare(priv.refclk);
    out_netdev:
    free_netdev(ndev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn emac_rockchip_remove(pdev: *mut platform_device) {
    static void emac_rockchip_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct rockchip_priv_data *priv = netdev_priv(ndev);
    arc_emac_remove(ndev);
    clk_disable_unprepare(priv.refclk);
    if (priv.regulator)
    regulator_disable(priv.regulator);
    if (priv.soc_data.need_div_macclk)
    clk_disable_unprepare(priv.macclk);
    free_netdev(ndev);
    }
    static struct platform_driver emac_rockchip_driver = {
    .probe = emac_rockchip_probe,
    .remove = emac_rockchip_remove,
    .driver = {
    .name = DRV_NAME,
    .of_match_table  = emac_rockchip_dt_ids,
    },
    };
    module_platform_driver(emac_rockchip_driver);
    MODULE_AUTHOR("Romain Perier <romain.perier@gmail.com>");
    MODULE_DESCRIPTION("Rockchip EMAC platform driver");
    MODULE_LICENSE("GPL");
