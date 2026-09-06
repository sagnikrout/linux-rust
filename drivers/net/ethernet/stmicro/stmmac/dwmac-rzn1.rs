//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-rzn1.c
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
// Copyright (C) 2024 Schneider-Electric
//
// Clément Léger <clement.leger@bootlin.com>
//

#[no_mangle]
unsafe extern "C" fn rzn1_dwmac_pcs_init(priv: *mut stmmac_priv) -> c_int {
    static int rzn1_dwmac_pcs_init(struct stmmac_priv *priv)
    {
    struct device_node *np = priv.device.of_node;
    struct device_node *pcs_node;
    struct phylink_pcs *pcs;
    pcs_node = of_parse_phandle(np, "pcs-handle", 0);
    if (pcs_node) {
    pcs = miic_create(priv.device, pcs_node);
    of_node_put(pcs_node);
    if (IS_ERR(pcs))
    return PTR_ERR(pcs);
    priv.hw.phylink_pcs = pcs;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzn1_dwmac_pcs_exit(priv: *mut stmmac_priv) {
    static void rzn1_dwmac_pcs_exit(struct stmmac_priv *priv)
    {
    if (priv.hw.phylink_pcs)
    miic_destroy(priv.hw.phylink_pcs);
    }
    static struct phylink_pcs *rzn1_dwmac_select_pcs(struct stmmac_priv *priv,
    phy_interface_t interface)
    {
    return priv.hw.phylink_pcs;
    }
#[no_mangle]
unsafe extern "C" fn rzn1_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int rzn1_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct device *dev = &pdev.dev;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    plat_dat.bsp_priv = plat_dat;
    plat_dat.pcs_init = rzn1_dwmac_pcs_init;
    plat_dat.pcs_exit = rzn1_dwmac_pcs_exit;
    plat_dat.select_pcs = rzn1_dwmac_select_pcs;
    ret = stmmac_dvr_probe(dev, plat_dat, &stmmac_res);
    if (ret)
    return ret;
    return 0;
    }
    static const struct of_device_id rzn1_dwmac_match[] = {
    { .compatible = "renesas,rzn1-gmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rzn1_dwmac_match);
    static struct platform_driver rzn1_dwmac_driver = {
    .probe  = rzn1_dwmac_probe,
    .remove = stmmac_pltfr_remove,
    .driver = {
    .name           = "rzn1-dwmac",
    .of_match_table = rzn1_dwmac_match,
    },
    };
    module_platform_driver(rzn1_dwmac_driver);
    MODULE_AUTHOR("Clément Léger <clement.leger@bootlin.com>");
    MODULE_DESCRIPTION("Renesas RZN1 DWMAC specific glue layer");
    MODULE_LICENSE("GPL");
