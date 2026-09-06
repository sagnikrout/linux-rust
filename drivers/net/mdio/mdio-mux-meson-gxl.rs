//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux-meson-gxl.c
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
// Copyright (c) 2022 Baylibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

pub const ETH_REG2: c_uint = 0x0;

pub const EPHY_GXL_ID: c_uint = 0x110181;

pub const ETH_REG3: c_uint = 0x4;

pub const ETH_REG4: c_uint = 0x8;

pub const MESON_GXL_MDIO_EXTERNAL_ID: c_int = 0;
pub const MESON_GXL_MDIO_INTERNAL_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxl_mdio_mux {
    pub regs: *mut void __iomem,
    pub mux_handle: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn gxl_enable_internal_mdio(priv: *mut gxl_mdio_mux) {
    static void gxl_enable_internal_mdio(struct gxl_mdio_mux *priv)
    {
    u32 val;
// Setup the internal phy
    val = (REG3_ENH |
    FIELD_PREP(REG3_CFGMODE, 0x7) |
    REG3_AUTOMDIX |
    FIELD_PREP(REG3_PHYADDR, 8) |
    REG3_LEDPOL |
    REG3_PHYMDI |
    REG3_CLKINEN |
    REG3_PHYIP);
    writel(REG4_PWRUPRSTSIG, priv.regs + ETH_REG4);
    writel(val, priv.regs + ETH_REG3);
    mdelay(10);
// NOTE: The HW kept the phy id configurable at runtime.
// The id below is arbitrary. It is the one used in the vendor code.
// The only constraint is that it must match the one in
// drivers/net/phy/meson-gxl.c to properly match the PHY.
//
    writel(REG2_REVERSED | FIELD_PREP(REG2_PHYID, EPHY_GXL_ID),
    priv.regs + ETH_REG2);
// Enable the internal phy
    val |= REG3_PHYEN;
    writel(val, priv.regs + ETH_REG3);
    writel(0, priv.regs + ETH_REG4);
// The phy needs a bit of time to power up
    mdelay(10);
    }
#[no_mangle]
unsafe extern "C" fn gxl_enable_external_mdio(priv: *mut gxl_mdio_mux) {
    static void gxl_enable_external_mdio(struct gxl_mdio_mux *priv)
    {
// Reset the mdio bus mux to the external phy
    writel(0, priv.regs + ETH_REG3);
    }
    static int gxl_mdio_switch_fn(int current_child, int desired_child,
    void *data)
    {
    struct gxl_mdio_mux *priv = dev_get_drvdata(data);
    if (current_child == desired_child)
    return 0;
    switch (desired_child) {
    case MESON_GXL_MDIO_EXTERNAL_ID:
    gxl_enable_external_mdio(priv);
    break;
    case MESON_GXL_MDIO_INTERNAL_ID:
    gxl_enable_internal_mdio(priv);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct of_device_id gxl_mdio_mux_match[] = {
    { .compatible = "amlogic,gxl-mdio-mux", },
    {},
    };
    MODULE_DEVICE_TABLE(of, gxl_mdio_mux_match);
#[no_mangle]
unsafe extern "C" fn gxl_mdio_mux_probe(pdev: *mut platform_device) -> c_int {
    static int gxl_mdio_mux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gxl_mdio_mux *priv;
    struct clk *rclk;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    rclk = devm_clk_get_enabled(dev, "ref");
    if (IS_ERR(rclk))
    return dev_err_probe(dev, PTR_ERR(rclk),
    "failed to get reference clock\n");
    ret = mdio_mux_init(dev, dev.of_node, gxl_mdio_switch_fn,
    &priv.mux_handle, dev, core::ptr::null_mut());
    if (ret)
    dev_err_probe(dev, ret, "mdio multiplexer init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gxl_mdio_mux_remove(pdev: *mut platform_device) {
    static void gxl_mdio_mux_remove(struct platform_device *pdev)
    {
    struct gxl_mdio_mux *priv = platform_get_drvdata(pdev);
    mdio_mux_uninit(priv.mux_handle);
    }
    static struct platform_driver gxl_mdio_mux_driver = {
    .probe		= gxl_mdio_mux_probe,
    .remove		= gxl_mdio_mux_remove,
    .driver		= {
    .name	= "gxl-mdio-mux",
    .of_match_table = gxl_mdio_mux_match,
    },
    };
    module_platform_driver(gxl_mdio_mux_driver);
    MODULE_DESCRIPTION("Amlogic GXL MDIO multiplexer driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL");
