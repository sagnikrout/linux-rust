//! Automatically rewritten from C to Rust
//! Source: drivers/phy/hisilicon/phy-hisi-inno-usb2.c
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
// HiSilicon INNO USB2 PHY Driver.
//
// Copyright (c) 2016-2017 HiSilicon Technologies Co., Ltd.
//

pub const INNO_PHY_PORT_NUM: c_int = 2;

pub const PHY_TYPE_0: c_int = 0;
pub const PHY_TYPE_1: c_int = 1;

pub const PHY_TEST_ADDR_OFFSET: c_int = 8;

pub const PHY0_TEST_PORT_OFFSET: c_int = 16;

pub const PHY1_TEST_PORT_OFFSET: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_inno_phy_port {
    pub utmi_rst: *mut reset_control,
    pub priv: *mut hisi_inno_phy_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_inno_phy_priv {
    pub mmio: *mut void __iomem,
    pub ref_clk: *mut clk,
    pub por_rst: *mut reset_control,
    pub type: c_uint,
    pub ports: [hisi_inno_phy_port; INNO_PHY_PORT_NUM],
}

    static void hisi_inno_phy_write_reg(struct hisi_inno_phy_priv *priv,
    u8 port, u32 addr, u32 data)
    {
    void __iomem *reg = priv.mmio;
    u32 val;
    u32 value;
    if (priv.type == PHY_TYPE_0)
    val = (data & PHY_TEST_DATA) |
    ((addr << PHY_TEST_ADDR_OFFSET) & PHY0_TEST_ADDR) |
    ((port << PHY0_TEST_PORT_OFFSET) & PHY0_TEST_PORT) |
    PHY0_TEST_WREN | PHY0_TEST_RST;
    else
    val = (data & PHY_TEST_DATA) |
    ((addr << PHY_TEST_ADDR_OFFSET) & PHY1_TEST_ADDR) |
    ((port << PHY1_TEST_PORT_OFFSET) & PHY1_TEST_PORT) |
    PHY1_TEST_WREN | PHY1_TEST_RST;
    writel(val, reg);
    value = val;
    if (priv.type == PHY_TYPE_0)
    value |= PHY0_TEST_CLK;
    else
    value |= PHY1_TEST_CLK;
    writel(value, reg);
    writel(val, reg);
    }
#[no_mangle]
unsafe extern "C" fn hisi_inno_phy_setup(priv: *mut hisi_inno_phy_priv) {
    static void hisi_inno_phy_setup(struct hisi_inno_phy_priv *priv)
    {
// The phy clk is controlled by the port0 register 0x06.
    hisi_inno_phy_write_reg(priv, 0, 0x06, PHY_CLK_ENABLE);
    msleep(PHY_CLK_STABLE_TIME);
    }
#[no_mangle]
unsafe extern "C" fn hisi_inno_phy_init(phy: *mut phy) -> c_int {
    static int hisi_inno_phy_init(struct phy *phy)
    {
    struct hisi_inno_phy_port *port = phy_get_drvdata(phy);
    struct hisi_inno_phy_priv *priv = port.priv;
    int ret;
    ret = clk_prepare_enable(priv.ref_clk);
    if (ret)
    return ret;
    udelay(REF_CLK_STABLE_TIME);
    reset_control_deassert(priv.por_rst);
    udelay(POR_RST_COMPLETE_TIME);
// Set up phy registers
    hisi_inno_phy_setup(priv);
    reset_control_deassert(port.utmi_rst);
    udelay(UTMI_RST_COMPLETE_TIME);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_inno_phy_exit(phy: *mut phy) -> c_int {
    static int hisi_inno_phy_exit(struct phy *phy)
    {
    struct hisi_inno_phy_port *port = phy_get_drvdata(phy);
    struct hisi_inno_phy_priv *priv = port.priv;
    reset_control_assert(port.utmi_rst);
    reset_control_assert(priv.por_rst);
    clk_disable_unprepare(priv.ref_clk);
    return 0;
    }
    static const struct phy_ops hisi_inno_phy_ops = {
    .init = hisi_inno_phy_init,
    .exit = hisi_inno_phy_exit,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn hisi_inno_phy_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_inno_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct hisi_inno_phy_priv *priv;
    struct phy_provider *provider;
    let mut i: c_int = 0;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.mmio)) {
    ret = PTR_ERR(priv.mmio);
    return ret;
    }
    priv.ref_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.ref_clk))
    return PTR_ERR(priv.ref_clk);
    priv.por_rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.por_rst))
    return PTR_ERR(priv.por_rst);
    priv.type = (uintptr_t) of_device_get_match_data(dev);
    for_each_child_of_node_scoped(np, child) {
    struct reset_control *rst;
    struct phy *phy;
    rst = of_reset_control_get_exclusive(child, core::ptr::null_mut());
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    priv.ports[i].utmi_rst = rst;
    priv.ports[i].priv = priv;
    phy = devm_phy_create(dev, child, &hisi_inno_phy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_bus_width(phy, 8);
    phy_set_drvdata(phy, &priv.ports[i]);
    i++;
    if (i >= INNO_PHY_PORT_NUM) {
    dev_warn(dev, "Support %d ports in maximum\n", i);
    break;
    }
    }
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static const struct of_device_id hisi_inno_phy_of_match[] = {
    { .compatible = "hisilicon,inno-usb2-phy",
    .data = (void *) PHY_TYPE_0 },
    { .compatible = "hisilicon,hi3798cv200-usb2-phy",
    .data = (void *) PHY_TYPE_0 },
    { .compatible = "hisilicon,hi3798mv100-usb2-phy",
    .data = (void *) PHY_TYPE_1 },
    { },
    };
    MODULE_DEVICE_TABLE(of, hisi_inno_phy_of_match);
    static struct platform_driver hisi_inno_phy_driver = {
    .probe	= hisi_inno_phy_probe,
    .driver = {
    .name	= "hisi-inno-phy",
    .of_match_table	= hisi_inno_phy_of_match,
    }
    };
    module_platform_driver(hisi_inno_phy_driver);
    MODULE_DESCRIPTION("HiSilicon INNO USB2 PHY Driver");
    MODULE_LICENSE("GPL v2");
