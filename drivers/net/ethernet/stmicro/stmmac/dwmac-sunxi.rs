//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-sunxi.c
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
// dwmac-sunxi.c - Allwinner sunxi DWMAC specific glue layer
//
// Copyright (C) 2013 Chen-Yu Tsai
//
// Chen-Yu Tsai  <wens@csie.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_priv_data {
    pub interface: phy_interface_t,
    pub clk_enabled: c_int,
    pub tx_clk: *mut clk,
    pub regulator: *mut regulator,
}

pub const SUN7I_GMAC_GMII_RGMII_RATE: c_int = 125000000;
pub const SUN7I_GMAC_MII_RATE: c_int = 25000000;
#[no_mangle]
unsafe extern "C" fn sun7i_gmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int sun7i_gmac_init(struct device *dev, void *priv)
    {
    struct sunxi_priv_data *gmac = priv;
    let mut ret: c_int = 0;
    if (gmac.regulator) {
    ret = regulator_enable(gmac.regulator);
    if (ret)
    return ret;
    }
// Set GMAC interface port mode
//
// The GMAC TX clock lines are configured by setting the clock
// rate, which then uses the auto-reparenting feature of the
// clock driver, and enabling/disabling the clock.
//
    if (phy_interface_mode_is_rgmii(gmac.interface)) {
    clk_set_rate(gmac.tx_clk, SUN7I_GMAC_GMII_RGMII_RATE);
    clk_prepare_enable(gmac.tx_clk);
    gmac.clk_enabled = 1;
    } else {
    clk_set_rate(gmac.tx_clk, SUN7I_GMAC_MII_RATE);
    ret = clk_prepare(gmac.tx_clk);
    if (ret && gmac.regulator)
    regulator_disable(gmac.regulator);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun7i_gmac_exit(dev: *mut device, priv: *mut c_void) {
    static void sun7i_gmac_exit(struct device *dev, void *priv)
    {
    struct sunxi_priv_data *gmac = priv;
    if (gmac.clk_enabled) {
    clk_disable(gmac.tx_clk);
    gmac.clk_enabled = 0;
    }
    clk_unprepare(gmac.tx_clk);
    if (gmac.regulator)
    regulator_disable(gmac.regulator);
    }
    static int sun7i_set_clk_tx_rate(void *bsp_priv, struct clk *clk_tx_i,
    phy_interface_t interface, int speed)
    {
    struct sunxi_priv_data *gmac = bsp_priv;
    if (interface == PHY_INTERFACE_MODE_GMII) {
    if (gmac.clk_enabled) {
    clk_disable(gmac.tx_clk);
    gmac.clk_enabled = 0;
    }
    clk_unprepare(gmac.tx_clk);
    if (speed == 1000) {
    clk_set_rate(gmac.tx_clk, SUN7I_GMAC_GMII_RGMII_RATE);
    clk_prepare_enable(gmac.tx_clk);
    gmac.clk_enabled = 1;
    } else {
    clk_set_rate(gmac.tx_clk, SUN7I_GMAC_MII_RATE);
    clk_prepare(gmac.tx_clk);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun7i_gmac_probe(pdev: *mut platform_device) -> c_int {
    static int sun7i_gmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct sunxi_priv_data *gmac;
    struct device *dev = &pdev.dev;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    gmac = devm_kzalloc(dev, sizeof(*gmac), GFP_KERNEL);
    if (!gmac)
    return -ENOMEM;
    gmac.interface = plat_dat.phy_interface;
    gmac.tx_clk = devm_clk_get(dev, "allwinner_gmac_tx");
    if (IS_ERR(gmac.tx_clk)) {
    dev_err(dev, "could not get tx clock\n");
    return PTR_ERR(gmac.tx_clk);
    }
// Optional regulator for PHY
    gmac.regulator = devm_regulator_get_optional(dev, "phy");
    if (IS_ERR(gmac.regulator)) {
    if (PTR_ERR(gmac.regulator) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_info(dev, "no regulator found\n");
    gmac.regulator = core::ptr::null_mut();
    }
// platform data specifying hardware features and callbacks.
// hardware features were copied from Allwinner drivers.
    plat_dat.tx_coe = true;
    plat_dat.core_type = DWMAC_CORE_GMAC;
    plat_dat.bsp_priv = gmac;
    plat_dat.init = sun7i_gmac_init;
    plat_dat.exit = sun7i_gmac_exit;
    plat_dat.set_clk_tx_rate = sun7i_set_clk_tx_rate;
    plat_dat.tx_fifo_size = 4096;
    plat_dat.rx_fifo_size = 16384;
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id sun7i_dwmac_match[] = {
    { .compatible = "allwinner,sun7i-a20-gmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sun7i_dwmac_match);
    static struct platform_driver sun7i_dwmac_driver = {
    .probe  = sun7i_gmac_probe,
    .driver = {
    .name           = "sun7i-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = sun7i_dwmac_match,
    },
    };
    module_platform_driver(sun7i_dwmac_driver);
    MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
    MODULE_DESCRIPTION("Allwinner sunxi DWMAC specific glue layer");
    MODULE_LICENSE("GPL");
