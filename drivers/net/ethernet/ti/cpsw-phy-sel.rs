//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/ti/cpsw-phy-sel.c
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


// SPDX-License-Identifier: GPL-2.0
// Texas Instruments Ethernet Switch Driver
//
// Copyright (C) 2013 Texas Instruments
//
// Module Author: Mugunthan V N <mugunthanvnm@ti.com>
//

// AM33xx SoC specific definitions for the CONTROL port
pub const AM33XX_GMII_SEL_MODE_MII: c_int = 0;
pub const AM33XX_GMII_SEL_MODE_RMII: c_int = 1;
pub const AM33XX_GMII_SEL_MODE_RGMII: c_int = 2;

pub const GMII_SEL_MODE_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_phy_sel_priv {
    pub dev: *mut device,
    pub gmii_sel: *mut u32 __iomem,
    pub rmii_clock_external: bool,
    void (*cpsw_phy_sel)(struct cpsw_phy_sel_priv *priv,
    pub slave): phy_interface_t phy_mode, int,
}

    static void cpsw_gmii_sel_am3352(struct cpsw_phy_sel_priv *priv,
    phy_interface_t phy_mode, int slave)
    {
    u32 reg;
    u32 mask;
    let mut mode: u32 = 0;
    let mut rgmii_id: bool = false;
    reg = readl(priv.gmii_sel);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RMII:
    mode = AM33XX_GMII_SEL_MODE_RMII;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    mode = AM33XX_GMII_SEL_MODE_RGMII;
    break;
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    mode = AM33XX_GMII_SEL_MODE_RGMII;
    rgmii_id = true;
    break;
    default:
    dev_warn(priv.dev,
    "Unsupported PHY mode: \"%s\". Defaulting to MII.\n",
    phy_modes(phy_mode));
    fallthrough;
    case PHY_INTERFACE_MODE_MII:
    mode = AM33XX_GMII_SEL_MODE_MII;
    break;
    }
    mask = GMII_SEL_MODE_MASK << (slave * 2) | BIT(slave + 6);
    mask |= BIT(slave + 4);
    mode <<= slave * 2;
    if (priv.rmii_clock_external) {
    if (slave == 0)
    mode |= AM33XX_GMII_SEL_RMII1_IO_CLK_EN;
    else
    mode |= AM33XX_GMII_SEL_RMII2_IO_CLK_EN;
    }
    if (rgmii_id) {
    if (slave == 0)
    mode |= AM33XX_GMII_SEL_RGMII1_IDMODE;
    else
    mode |= AM33XX_GMII_SEL_RGMII2_IDMODE;
    }
    reg &= ~mask;
    reg |= mode;
    writel(reg, priv.gmii_sel);
    }
    static void cpsw_gmii_sel_dra7xx(struct cpsw_phy_sel_priv *priv,
    phy_interface_t phy_mode, int slave)
    {
    u32 reg;
    u32 mask;
    let mut mode: u32 = 0;
    reg = readl(priv.gmii_sel);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RMII:
    mode = AM33XX_GMII_SEL_MODE_RMII;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    mode = AM33XX_GMII_SEL_MODE_RGMII;
    break;
    default:
    dev_warn(priv.dev,
    "Unsupported PHY mode: \"%s\". Defaulting to MII.\n",
    phy_modes(phy_mode));
    fallthrough;
    case PHY_INTERFACE_MODE_MII:
    mode = AM33XX_GMII_SEL_MODE_MII;
    break;
    }
    switch (slave) {
    case 0:
    mask = GMII_SEL_MODE_MASK;
    break;
    case 1:
    mask = GMII_SEL_MODE_MASK << 4;
    mode <<= 4;
    break;
    default:
    dev_err(priv.dev, "invalid slave number...\n");
    return;
    }
    if (priv.rmii_clock_external)
    dev_err(priv.dev, "RMII External clock is not supported\n");
    reg &= ~mask;
    reg |= mode;
    writel(reg, priv.gmii_sel);
    }
    static struct platform_driver cpsw_phy_sel_driver;
#[no_mangle]
unsafe extern "C" fn match(dev: *mut device, data: *const c_void) -> c_int {
    static int match(struct device *dev, const void *data)
    {
    const struct device_node *node = (const struct device_node *)data;
    return dev.of_node == node &&
    dev.driver == &cpsw_phy_sel_driver.driver;
    }
#[no_mangle]
pub unsafe extern "C" fn cpsw_phy_sel(dev: *mut device, phy_mode: phy_interface_t, slave: c_int) {
    void cpsw_phy_sel(struct device *dev, phy_interface_t phy_mode, int slave)
    {
    struct device_node *node;
    struct cpsw_phy_sel_priv *priv;
    node = of_parse_phandle(dev.of_node, "cpsw-phy-sel", 0);
    if (!node) {
    node = of_get_child_by_name(dev.of_node, "cpsw-phy-sel");
    if (!node) {
    dev_err(dev, "Phy mode driver DT not found\n");
    return;
    }
    }
    dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), node, match);
    if (!dev) {
    dev_err(dev, "unable to find platform device for %pOF\n", node);
    goto out;
    }
    priv = dev_get_drvdata(dev);
    priv.cpsw_phy_sel(priv, phy_mode, slave);
    put_device(dev);
    out:
    of_node_put(node);
    }
    EXPORT_SYMBOL_GPL(cpsw_phy_sel);
    static const struct of_device_id cpsw_phy_sel_id_table[] = {
    {
    .compatible	= "ti,am3352-cpsw-phy-sel",
    .data		= &cpsw_gmii_sel_am3352,
    },
    {
    .compatible	= "ti,dra7xx-cpsw-phy-sel",
    .data		= &cpsw_gmii_sel_dra7xx,
    },
    {
    .compatible	= "ti,am43xx-cpsw-phy-sel",
    .data		= &cpsw_gmii_sel_am3352,
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn cpsw_phy_sel_probe(pdev: *mut platform_device) -> c_int {
    static int cpsw_phy_sel_probe(struct platform_device *pdev)
    {
    const struct of_device_id *of_id;
    struct cpsw_phy_sel_priv *priv;
    of_id = of_match_node(cpsw_phy_sel_id_table, pdev.dev.of_node);
    if (!of_id)
    return -EINVAL;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    dev_err(&pdev.dev, "unable to alloc memory for cpsw phy sel\n");
    return -ENOMEM;
    }
    priv.dev = &pdev.dev;
    priv.cpsw_phy_sel = of_id.data;
    priv.gmii_sel = devm_platform_ioremap_resource_byname(pdev, "gmii-sel");
    if (IS_ERR(priv.gmii_sel))
    return PTR_ERR(priv.gmii_sel);
    priv.rmii_clock_external = of_property_read_bool(pdev.dev.of_node, "rmii-clock-ext");
    dev_set_drvdata(&pdev.dev, priv);
    return 0;
    }
    static struct platform_driver cpsw_phy_sel_driver = {
    .probe		= cpsw_phy_sel_probe,
    .driver		= {
    .name	= "cpsw-phy-sel",
    .of_match_table = cpsw_phy_sel_id_table,
    },
    };
    builtin_platform_driver(cpsw_phy_sel_driver);
