//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-armada38x-comphy.c
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
//
// Copyright (C) 2018 Russell King, Deep Blue Solutions Ltd.
//
// Partly derived from CP110 comphy driver by Antoine Tenart
// <antoine.tenart@bootlin.com>
//

pub const MAX_A38X_COMPHY: c_int = 6;
pub const MAX_A38X_PORTS: c_int = 3;
pub const COMPHY_CFG1: c_uint = 0x00;

pub const GEN_SGMII_1_25GBPS: c_int = 6;
pub const GEN_SGMII_3_125GBPS: c_int = 8;
pub const COMPHY_STAT1: c_uint = 0x18;

pub const COMPHY_SELECTOR: c_uint = 0xfc;
    struct a38x_comphy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a38x_comphy_lane {
    pub base: *mut void __iomem,
    pub priv: *mut a38x_comphy,
    pub n: c_uint,
    pub port: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a38x_comphy {
    pub base: *mut void __iomem,
    pub conf: *mut void __iomem,
    pub dev: *mut device,
    pub lane: [a38x_comphy_lane; MAX_A38X_COMPHY],
}

//
// Map serdes lanes and gbe ports to serdes mux configuration values:
// row index = serdes lane,
// column index = gbe port number.
//
    static const u8 gbe_mux[MAX_A38X_COMPHY][MAX_A38X_PORTS] = {
    { 3, 0, 0 },
    { 4, 5, 0 },
    { 0, 4, 0 },
    { 0, 0, 4 },
    { 0, 3, 0 },
    { 0, 0, 3 },
    };
#[no_mangle]
unsafe extern "C" fn a38x_set_conf(lane: *mut a38x_comphy_lane, enable: bool) {
    static void a38x_set_conf(struct a38x_comphy_lane *lane, bool enable)
    {
    struct a38x_comphy *priv = lane.priv;
    u32 conf;
    if (priv.conf) {
    conf = readl_relaxed(priv.conf);
    if (enable)
    conf |= BIT(lane.port);
    else
    conf &= ~BIT(lane.port);
    writel(conf, priv.conf);
    }
    }
    static void a38x_comphy_set_reg(struct a38x_comphy_lane *lane,
    unsigned int offset, u32 mask, u32 value)
    {
    u32 val;
    val = readl_relaxed(lane.base + offset) & ~mask;
    writel(val | value, lane.base + offset);
    }
    static void a38x_comphy_set_speed(struct a38x_comphy_lane *lane,
    unsigned int gen_tx, unsigned int gen_rx)
    {
    a38x_comphy_set_reg(lane, COMPHY_CFG1,
    COMPHY_CFG1_GEN_TX_MSK | COMPHY_CFG1_GEN_RX_MSK,
    COMPHY_CFG1_GEN_TX(gen_tx) |
    COMPHY_CFG1_GEN_RX(gen_rx));
    }
    static int a38x_comphy_poll(struct a38x_comphy_lane *lane,
    unsigned int offset, u32 mask, u32 value)
    {
    u32 val;
    int ret;
    ret = readl_relaxed_poll_timeout_atomic(lane.base + offset, val,
    (val & mask) == value,
    1000, 150000);
    if (ret)
    dev_err(lane.priv.dev,
    "comphy%u: timed out waiting for status\n", lane.n);
    return ret;
    }
//
// We only support changing the speed for comphys configured for GBE.
// Since that is all we do, we only poll for PLL ready status.
//
#[no_mangle]
unsafe extern "C" fn a38x_comphy_set_mode(phy: *mut phy, mode: enum phy_mode, sub: c_int) -> c_int {
    static int a38x_comphy_set_mode(struct phy *phy, enum phy_mode mode, int sub)
    {
    struct a38x_comphy_lane *lane = phy_get_drvdata(phy);
    unsigned int gen;
    int ret;
    if (mode != PHY_MODE_ETHERNET)
    return -EINVAL;
    switch (sub) {
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_1000BASEX:
    gen = GEN_SGMII_1_25GBPS;
    break;
    case PHY_INTERFACE_MODE_2500BASEX:
    gen = GEN_SGMII_3_125GBPS;
    break;
    default:
    return -EINVAL;
    }
    a38x_set_conf(lane, false);
    a38x_comphy_set_speed(lane, gen, gen);
    ret = a38x_comphy_poll(lane, COMPHY_STAT1,
    COMPHY_STAT1_PLL_RDY_TX |
    COMPHY_STAT1_PLL_RDY_RX,
    COMPHY_STAT1_PLL_RDY_TX |
    COMPHY_STAT1_PLL_RDY_RX);
    if (ret == 0)
    a38x_set_conf(lane, true);
    return ret;
    }
    static const struct phy_ops a38x_comphy_ops = {
    .set_mode	= a38x_comphy_set_mode,
    .owner		= THIS_MODULE,
    };
    static struct phy *a38x_comphy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct a38x_comphy_lane *lane;
    struct phy *phy;
    u32 val;
    if (WARN_ON(args.args[0] >= MAX_A38X_PORTS))
    return ERR_PTR(-EINVAL);
    phy = of_phy_simple_xlate(dev, args);
    if (IS_ERR(phy))
    return phy;
    lane = phy_get_drvdata(phy);
    if (lane.port >= 0)
    return ERR_PTR(-EBUSY);
    lane.port = args.args[0];
    val = readl_relaxed(lane.priv.base + COMPHY_SELECTOR);
    val = (val >> (4 * lane.n)) & 0xf;
    if (!gbe_mux[lane.n][lane.port] ||
    val != gbe_mux[lane.n][lane.port]) {
    dev_warn(lane.priv.dev,
    "comphy%u: not configured for GBE\n", lane.n);
    phy = ERR_PTR(-EINVAL);
    }
    return phy;
    }
#[no_mangle]
unsafe extern "C" fn a38x_comphy_probe(pdev: *mut platform_device) -> c_int {
    static int a38x_comphy_probe(struct platform_device *pdev)
    {
    struct phy_provider *provider;
    struct device_node *child;
    struct a38x_comphy *priv;
    struct resource *res;
    void __iomem *base;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.dev = &pdev.dev;
    priv.base = base;
// Optional
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "conf");
    if (res) {
    priv.conf = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(priv.conf))
    return PTR_ERR(priv.conf);
    }
    for_each_available_child_of_node(pdev.dev.of_node, child) {
    struct phy *phy;
    int ret;
    u32 val;
    ret = of_property_read_u32(child, "reg", &val);
    if (ret < 0) {
    dev_err(&pdev.dev, "missing 'reg' property (%d)\n",
    ret);
    continue;
    }
    if (val >= MAX_A38X_COMPHY || priv.lane[val].base) {
    dev_err(&pdev.dev, "invalid 'reg' property\n");
    continue;
    }
    phy = devm_phy_create(&pdev.dev, child, &a38x_comphy_ops);
    if (IS_ERR(phy)) {
    of_node_put(child);
    return PTR_ERR(phy);
    }
    priv.lane[val].base = base + 0x28 * val;
    priv.lane[val].priv = priv;
    priv.lane[val].n = val;
    priv.lane[val].port = -1;
    phy_set_drvdata(phy, &priv.lane[val]);
    }
    dev_set_drvdata(&pdev.dev, priv);
    provider = devm_of_phy_provider_register(&pdev.dev, a38x_comphy_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static const struct of_device_id a38x_comphy_of_match_table[] = {
    { .compatible = "marvell,armada-380-comphy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, a38x_comphy_of_match_table);
    static struct platform_driver a38x_comphy_driver = {
    .probe	= a38x_comphy_probe,
    .driver	= {
    .name = "armada-38x-comphy",
    .of_match_table = a38x_comphy_of_match_table,
    },
    };
    module_platform_driver(a38x_comphy_driver);
    MODULE_AUTHOR("Russell King <rmk+kernel@armlinux.org.uk>");
    MODULE_DESCRIPTION("Common PHY driver for Armada 38x SoCs");
    MODULE_LICENSE("GPL v2");
