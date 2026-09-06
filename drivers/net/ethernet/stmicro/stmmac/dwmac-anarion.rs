//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-anarion.c
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
// Adaptrum Anarion DWMAC glue layer
//
// Copyright (C) 2017, Adaptrum, Inc.
// (Written by Alexandru Gagniuc <alex.g at adaptrum.com> for Adaptrum, Inc.)
//

pub const GMAC_RESET_CONTROL_REG: c_int = 0;
pub const GMAC_SW_CONFIG_REG: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anarion_gmac {
    pub ctl_block: *mut void __iomem,
    pub phy_intf_sel: u32,
}

#[no_mangle]
unsafe extern "C" fn gmac_read_reg(gmac: *mut anarion_gmac, reg: u8) -> u32 {
    static uint32_t gmac_read_reg(struct anarion_gmac *gmac, uint8_t reg)
    {
    return readl(gmac.ctl_block + reg);
    };
#[no_mangle]
unsafe extern "C" fn gmac_write_reg(gmac: *mut anarion_gmac, reg: u8, val: u32) {
    static void gmac_write_reg(struct anarion_gmac *gmac, uint8_t reg, uint32_t val)
    {
    writel(val, gmac.ctl_block + reg);
    }
#[no_mangle]
unsafe extern "C" fn anarion_gmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int anarion_gmac_init(struct device *dev, void *priv)
    {
    uint32_t sw_config;
    struct anarion_gmac *gmac = priv;
// Reset logic, configure interface mode, then release reset. SIMPLE!
    gmac_write_reg(gmac, GMAC_RESET_CONTROL_REG, 1);
    sw_config = gmac_read_reg(gmac, GMAC_SW_CONFIG_REG);
    sw_config &= ~GMAC_CONFIG_INTF_SEL_MASK;
    sw_config |= (gmac.phy_intf_sel & GMAC_CONFIG_INTF_SEL_MASK);
    gmac_write_reg(gmac, GMAC_SW_CONFIG_REG, sw_config);
    gmac_write_reg(gmac, GMAC_RESET_CONTROL_REG, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anarion_gmac_exit(dev: *mut device, priv: *mut c_void) {
    static void anarion_gmac_exit(struct device *dev, void *priv)
    {
    struct anarion_gmac *gmac = priv;
    gmac_write_reg(gmac, GMAC_RESET_CONTROL_REG, 1);
    }
    static struct anarion_gmac *
    anarion_config_dt(struct platform_device *pdev,
    struct plat_stmmacenet_data *plat_dat)
    {
    struct anarion_gmac *gmac;
    void __iomem *ctl_block;
    ctl_block = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(ctl_block)) {
    dev_err(&pdev.dev, "Cannot get reset region (%pe)!\n",
    ctl_block);
    return ERR_CAST(ctl_block);
    }
    gmac = devm_kzalloc(&pdev.dev, sizeof(*gmac), GFP_KERNEL);
    if (!gmac)
    return ERR_PTR(-ENOMEM);
    gmac.ctl_block = ctl_block;
    if (phy_interface_mode_is_rgmii(plat_dat.phy_interface)) {
    gmac.phy_intf_sel = GMAC_CONFIG_INTF_RGMII;
    } else {
    dev_err(&pdev.dev, "Unsupported phy-mode (%s)\n",
    phy_modes(plat_dat.phy_interface));
    return ERR_PTR(-ENOTSUPP);
    }
    return gmac;
    }
#[no_mangle]
unsafe extern "C" fn anarion_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int anarion_dwmac_probe(struct platform_device *pdev)
    {
    int ret;
    struct anarion_gmac *gmac;
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    gmac = anarion_config_dt(pdev, plat_dat);
    if (IS_ERR(gmac))
    return PTR_ERR(gmac);
    plat_dat.init = anarion_gmac_init;
    plat_dat.exit = anarion_gmac_exit;
    plat_dat.bsp_priv = gmac;
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id anarion_dwmac_match[] = {
    { .compatible = "adaptrum,anarion-gmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, anarion_dwmac_match);
    static struct platform_driver anarion_dwmac_driver = {
    .probe  = anarion_dwmac_probe,
    .driver = {
    .name           = "anarion-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = anarion_dwmac_match,
    },
    };
    module_platform_driver(anarion_dwmac_driver);
    MODULE_DESCRIPTION("Adaptrum Anarion DWMAC specific glue layer");
    MODULE_AUTHOR("Alexandru Gagniuc <mr.nuke.me@gmail.com>");
    MODULE_LICENSE("GPL v2");
