//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-lpc18xx.c
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


//
// DWMAC glue for NXP LPC18xx/LPC43xx Ethernet
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// Register defines for CREG syscon
pub const LPC18XX_CREG_CREG6: c_uint = 0x12c;

#[no_mangle]
unsafe extern "C" fn lpc18xx_set_phy_intf_sel(bsp_priv: *mut c_void, phy_intf_sel: u8) -> c_int {
    static int lpc18xx_set_phy_intf_sel(void *bsp_priv, u8 phy_intf_sel)
    {
    struct regmap *reg = bsp_priv;
    if (phy_intf_sel != PHY_INTF_SEL_GMII_MII &&
    phy_intf_sel != PHY_INTF_SEL_RMII)
    return -EINVAL;
    regmap_update_bits(reg, LPC18XX_CREG_CREG6,
    LPC18XX_CREG_CREG6_ETHMODE_MASK,
    FIELD_PREP(LPC18XX_CREG_CREG6_ETHMODE_MASK,
    phy_intf_sel));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct regmap *regmap;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    plat_dat.core_type = DWMAC_CORE_GMAC;
    regmap = syscon_regmap_lookup_by_compatible("nxp,lpc1850-creg");
    if (IS_ERR(regmap)) {
    dev_err(&pdev.dev, "syscon lookup failed\n");
    return PTR_ERR(regmap);
    }
    plat_dat.bsp_priv = regmap;
    plat_dat.set_phy_intf_sel = lpc18xx_set_phy_intf_sel;
    return stmmac_dvr_probe(&pdev.dev, plat_dat, &stmmac_res);
    }
    static const struct of_device_id lpc18xx_dwmac_match[] = {
    { .compatible = "nxp,lpc1850-dwmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_dwmac_match);
    static struct platform_driver lpc18xx_dwmac_driver = {
    .probe  = lpc18xx_dwmac_probe,
    .remove = stmmac_pltfr_remove,
    .driver = {
    .name           = "lpc18xx-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = lpc18xx_dwmac_match,
    },
    };
    module_platform_driver(lpc18xx_dwmac_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("DWMAC glue for LPC18xx/43xx Ethernet");
    MODULE_LICENSE("GPL v2");
