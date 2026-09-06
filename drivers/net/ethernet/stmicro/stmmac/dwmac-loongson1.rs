//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-loongson1.c
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
// Loongson-1 DWMAC glue layer
//
// Copyright (C) 2011-2023 Keguang Zhang <keguang.zhang@gmail.com>
//

// Loongson-1 SYSCON Registers

// Loongson-1B SYSCON Register Bits

// Loongson-1C SYSCON Register Bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls1x_dwmac {
    pub plat_dat: *mut plat_stmmacenet_data,
    pub regmap: *mut regmap,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls1x_data {
    int (*setup)(struct platform_device *pdev,
    pub plat_dat): *mut plat_stmmacenet_data,
    pub bsp_priv): *mut *mut *mut int (init)(struct device dev, void,
}

    static int ls1b_dwmac_setup(struct platform_device *pdev,
    struct plat_stmmacenet_data *plat_dat)
    {
    struct ls1x_dwmac *dwmac = plat_dat.bsp_priv;
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
// This shouldn't fail - stmmac_get_platform_resources()
// already mapped this resource.
//
    dev_err(&pdev.dev, "Could not get IO_MEM resources\n");
    return -EINVAL;
    }
    if (res.start == LS1B_GMAC0_BASE) {
    dwmac.id = 0;
    } else if (res.start == LS1B_GMAC1_BASE) {
    dwmac.id = 1;
    } else {
    dev_err(&pdev.dev, "Invalid Ethernet MAC base address %pR",
    res);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1b_dwmac_syscon_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int ls1b_dwmac_syscon_init(struct device *dev, void *priv)
    {
    struct ls1x_dwmac *dwmac = priv;
    struct plat_stmmacenet_data *plat = dwmac.plat_dat;
    struct regmap *regmap = dwmac.regmap;
    if (dwmac.id == 0) {
    switch (plat.phy_interface) {
    case PHY_INTERFACE_MODE_RGMII_ID:
    regmap_update_bits(regmap, LS1X_SYSCON0,
    GMAC0_USE_TXCLK | GMAC0_USE_PWM01,
    0);
    break;
    case PHY_INTERFACE_MODE_MII:
    regmap_update_bits(regmap, LS1X_SYSCON0,
    GMAC0_USE_TXCLK | GMAC0_USE_PWM01,
    GMAC0_USE_TXCLK | GMAC0_USE_PWM01);
    break;
    default:
    dev_err(dev, "Unsupported PHY mode %u\n",
    plat.phy_interface);
    return -EOPNOTSUPP;
    }
    regmap_update_bits(regmap, LS1X_SYSCON0, GMAC0_SHUT, 0);
    } else if (dwmac.id == 1) {
    regmap_update_bits(regmap, LS1X_SYSCON0,
    GMAC1_USE_UART1 | GMAC1_USE_UART0,
    GMAC1_USE_UART1 | GMAC1_USE_UART0);
    switch (plat.phy_interface) {
    case PHY_INTERFACE_MODE_RGMII_ID:
    regmap_update_bits(regmap, LS1X_SYSCON1,
    GMAC1_USE_TXCLK | GMAC1_USE_PWM23,
    0);
    break;
    case PHY_INTERFACE_MODE_MII:
    regmap_update_bits(regmap, LS1X_SYSCON1,
    GMAC1_USE_TXCLK | GMAC1_USE_PWM23,
    GMAC1_USE_TXCLK | GMAC1_USE_PWM23);
    break;
    default:
    dev_err(dev, "Unsupported PHY mode %u\n",
    plat.phy_interface);
    return -EOPNOTSUPP;
    }
    regmap_update_bits(regmap, LS1X_SYSCON1, GMAC1_SHUT, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1c_dwmac_syscon_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int ls1c_dwmac_syscon_init(struct device *dev, void *priv)
    {
    struct ls1x_dwmac *dwmac = priv;
    struct plat_stmmacenet_data *plat = dwmac.plat_dat;
    struct regmap *regmap = dwmac.regmap;
    int phy_intf_sel;
    phy_intf_sel = stmmac_get_phy_intf_sel(plat.phy_interface);
    if (phy_intf_sel != PHY_INTF_SEL_GMII_MII &&
    phy_intf_sel != PHY_INTF_SEL_RMII) {
    dev_err(dev, "Unsupported PHY-mode %u\n",
    plat.phy_interface);
    return -EOPNOTSUPP;
    }
    regmap_update_bits(regmap, LS1X_SYSCON1, PHY_INTF_SELI,
    FIELD_PREP(PHY_INTF_SELI, phy_intf_sel));
    regmap_update_bits(regmap, LS1X_SYSCON0, GMAC0_SHUT, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1x_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int ls1x_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    const struct ls1x_data *data;
    struct regmap *regmap;
    struct ls1x_dwmac *dwmac;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
// Probe syscon
    regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "loongson,ls1-syscon");
    if (IS_ERR(regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(regmap),
    "Unable to find syscon\n");
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "No of match data provided\n");
    return -EINVAL;
    }
    dwmac = devm_kzalloc(&pdev.dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return dev_err_probe(&pdev.dev, PTR_ERR(plat_dat),
    "dt configuration failed\n");
    plat_dat.bsp_priv = dwmac;
    plat_dat.init = data.init;
    dwmac.plat_dat = plat_dat;
    dwmac.regmap = regmap;
    if (data.setup) {
    ret = data.setup(pdev, plat_dat);
    if (ret)
    return ret;
    }
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct ls1x_data ls1b_dwmac_data = {
    .setup = ls1b_dwmac_setup,
    .init = ls1b_dwmac_syscon_init,
    };
    static const struct ls1x_data ls1c_dwmac_data = {
    .init = ls1c_dwmac_syscon_init,
    };
    static const struct of_device_id ls1x_dwmac_match[] = {
    {
    .compatible = "loongson,ls1b-gmac",
    .data = &ls1b_dwmac_data,
    },
    {
    .compatible = "loongson,ls1c-emac",
    .data = &ls1c_dwmac_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, ls1x_dwmac_match);
    static struct platform_driver ls1x_dwmac_driver = {
    .probe = ls1x_dwmac_probe,
    .driver = {
    .name = "loongson1-dwmac",
    .of_match_table = ls1x_dwmac_match,
    },
    };
    module_platform_driver(ls1x_dwmac_driver);
    MODULE_AUTHOR("Keguang Zhang <keguang.zhang@gmail.com>");
    MODULE_DESCRIPTION("Loongson-1 DWMAC glue layer");
    MODULE_LICENSE("GPL");
