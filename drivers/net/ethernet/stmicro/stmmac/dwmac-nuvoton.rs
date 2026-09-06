//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-nuvoton.c
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
// Nuvoton DWMAC specific glue layer
//
// Copyright (C) 2025 Nuvoton Technology Corp.
//
// Author: Joey Lu <a0987203069@gmail.com>
//

pub const NVT_REG_SYS_GMAC0MISCR: c_uint = 0x108;
pub const NVT_REG_SYS_GMAC1MISCR: c_uint = 0x10C;

// Two thousand picoseconds are evenly mapped to a 4-bit field,
// resulting in each step being 2000/15 picoseconds.
//
pub const NVT_PATH_DELAY_STEP: c_int = 134;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvt_priv_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub macid: u32,
}

#[no_mangle]
unsafe extern "C" fn nvt_gmac_get_delay(dev: *mut device, property: *const c_char) -> c_int {
    static int nvt_gmac_get_delay(struct device *dev, const char *property)
    {
    u32 arg;
    if (of_property_read_u32(dev.of_node, property, &arg))
    return 0;
    if (arg > 2000)
    return -EINVAL;
    if (arg == 2000)
    return 15;
    return arg / NVT_PATH_DELAY_STEP;
    }
#[no_mangle]
unsafe extern "C" fn nvt_set_phy_intf_sel(bsp_priv: *mut c_void, phy_intf_sel: u8) -> c_int {
    static int nvt_set_phy_intf_sel(void *bsp_priv, u8 phy_intf_sel)
    {
    struct nvt_priv_data *priv = bsp_priv;
    u32 reg, val;
    int ret;
    if (phy_intf_sel == PHY_INTF_SEL_RGMII) {
    ret = nvt_gmac_get_delay(priv.dev, "rx-internal-delay-ps");
    if (ret < 0)
    return ret;
    val = FIELD_PREP(NVT_RX_DELAY_MASK, ret);
    ret = nvt_gmac_get_delay(priv.dev, "tx-internal-delay-ps");
    if (ret < 0)
    return ret;
    val |= FIELD_PREP(NVT_TX_DELAY_MASK, ret);
    } else if (phy_intf_sel == PHY_INTF_SEL_RMII) {
    val = NVT_MISCR_RMII;
    } else {
    return -EINVAL;
    }
    reg = (priv.macid == 0) ? NVT_REG_SYS_GMAC0MISCR : NVT_REG_SYS_GMAC1MISCR;
    regmap_update_bits(priv.regmap, reg,
    NVT_RX_DELAY_MASK | NVT_TX_DELAY_MASK | NVT_MISCR_RMII, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvt_gmac_probe(pdev: *mut platform_device) -> c_int {
    static int nvt_gmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct device *dev = &pdev.dev;
    struct nvt_priv_data *priv;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get platform resources\n");
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return dev_err_probe(dev, PTR_ERR(plat_dat), "Failed to get platform data\n");
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return dev_err_probe(dev, -ENOMEM, "Failed to allocate private data\n");
    priv.dev = dev;
    priv.regmap = syscon_regmap_lookup_by_phandle_args(dev.of_node, "nuvoton,sys",
    1, &priv.macid);
    if (IS_ERR(priv.regmap))
    return dev_err_probe(dev, PTR_ERR(priv.regmap), "Failed to get sys register\n");
    if (priv.macid > 1)
    return dev_err_probe(dev, -EINVAL, "Invalid sys arguments\n");
    plat_dat.bsp_priv = priv;
    plat_dat.set_phy_intf_sel = nvt_set_phy_intf_sel;
    return stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id nvt_dwmac_match[] = {
    { .compatible = "nuvoton,ma35d1-dwmac"},
    { }
    };
    MODULE_DEVICE_TABLE(of, nvt_dwmac_match);
    static struct platform_driver nvt_dwmac_driver = {
    .probe  = nvt_gmac_probe,
    .remove = stmmac_pltfr_remove,
    .driver = {
    .name           = "nuvoton-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = nvt_dwmac_match,
    },
    };
    module_platform_driver(nvt_dwmac_driver);
    MODULE_AUTHOR("Joey Lu <a0987203069@gmail.com>");
    MODULE_DESCRIPTION("Nuvoton DWMAC specific glue layer");
    MODULE_LICENSE("GPL");
