//! Automatically rewritten from C to Rust
//! Source: drivers/phy/lantiq/phy-lantiq-rcu-usb2.c
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
// Lantiq XWAY SoC RCU module based USB 1.1/2.0 PHY driver
//
// Copyright (C) 2016 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
// Copyright (C) 2017 Hauke Mehrtens <hauke@hauke-m.de>
//

// Transmitter HS Pre-Emphasis Enable

// Disconnect Threshold
pub const RCU_CFG1_DIS_THR_MASK: c_uint = 0x00038000;
pub const RCU_CFG1_DIS_THR_SHIFT: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_rcu_usb2_bits {
    pub hostmode: u8,
    pub slave_endianness: u8,
    pub host_endianness: u8,
    pub have_ana_cfg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_rcu_usb2_priv {
    pub regmap: *mut regmap,
    pub phy_reg_offset: c_uint,
    pub ana_cfg1_reg_offset: c_uint,
    pub reg_bits: *const ltq_rcu_usb2_bits,
    pub dev: *mut device,
    pub phy: *mut phy,
    pub phy_gate_clk: *mut clk,
    pub ctrl_reset: *mut reset_control,
    pub phy_reset: *mut reset_control,
}

    static const struct ltq_rcu_usb2_bits xway_rcu_usb2_reg_bits = {
    .hostmode = 11,
    .slave_endianness = 9,
    .host_endianness = 10,
    .have_ana_cfg = false,
    };
    static const struct ltq_rcu_usb2_bits xrx100_rcu_usb2_reg_bits = {
    .hostmode = 11,
    .slave_endianness = 17,
    .host_endianness = 10,
    .have_ana_cfg = false,
    };
    static const struct ltq_rcu_usb2_bits xrx200_rcu_usb2_reg_bits = {
    .hostmode = 11,
    .slave_endianness = 9,
    .host_endianness = 10,
    .have_ana_cfg = true,
    };
    static const struct of_device_id ltq_rcu_usb2_phy_of_match[] = {
    {
    .compatible = "lantiq,ase-usb2-phy",
    .data = &xway_rcu_usb2_reg_bits,
    },
    {
    .compatible = "lantiq,danube-usb2-phy",
    .data = &xway_rcu_usb2_reg_bits,
    },
    {
    .compatible = "lantiq,xrx100-usb2-phy",
    .data = &xrx100_rcu_usb2_reg_bits,
    },
    {
    .compatible = "lantiq,xrx200-usb2-phy",
    .data = &xrx200_rcu_usb2_reg_bits,
    },
    {
    .compatible = "lantiq,xrx300-usb2-phy",
    .data = &xrx200_rcu_usb2_reg_bits,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ltq_rcu_usb2_phy_of_match);
#[no_mangle]
unsafe extern "C" fn ltq_rcu_usb2_phy_init(phy: *mut phy) -> c_int {
    static int ltq_rcu_usb2_phy_init(struct phy *phy)
    {
    struct ltq_rcu_usb2_priv *priv = phy_get_drvdata(phy);
    if (priv.reg_bits.have_ana_cfg) {
    regmap_update_bits(priv.regmap, priv.ana_cfg1_reg_offset,
    RCU_CFG1_TX_PEE, RCU_CFG1_TX_PEE);
    regmap_update_bits(priv.regmap, priv.ana_cfg1_reg_offset,
    RCU_CFG1_DIS_THR_MASK, 7 << RCU_CFG1_DIS_THR_SHIFT);
    }
// Configure core to host mode
    regmap_update_bits(priv.regmap, priv.phy_reg_offset,
    BIT(priv.reg_bits.hostmode), 0);
// Select DMA endianness (Host-endian: big-endian)
    regmap_update_bits(priv.regmap, priv.phy_reg_offset,
    BIT(priv.reg_bits.slave_endianness), 0);
    regmap_update_bits(priv.regmap, priv.phy_reg_offset,
    BIT(priv.reg_bits.host_endianness),
    BIT(priv.reg_bits.host_endianness));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_rcu_usb2_phy_power_on(phy: *mut phy) -> c_int {
    static int ltq_rcu_usb2_phy_power_on(struct phy *phy)
    {
    struct ltq_rcu_usb2_priv *priv = phy_get_drvdata(phy);
    struct device *dev = priv.dev;
    int ret;
    reset_control_deassert(priv.phy_reset);
    ret = clk_prepare_enable(priv.phy_gate_clk);
    if (ret) {
    dev_err(dev, "failed to enable PHY gate\n");
    return ret;
    }
//
// at least the xrx200 usb2 phy requires some extra time to be
// operational after enabling the clock
//
    usleep_range(100, 200);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltq_rcu_usb2_phy_power_off(phy: *mut phy) -> c_int {
    static int ltq_rcu_usb2_phy_power_off(struct phy *phy)
    {
    struct ltq_rcu_usb2_priv *priv = phy_get_drvdata(phy);
    reset_control_assert(priv.phy_reset);
    clk_disable_unprepare(priv.phy_gate_clk);
    return 0;
    }
    static const struct phy_ops ltq_rcu_usb2_phy_ops = {
    .init		= ltq_rcu_usb2_phy_init,
    .power_on	= ltq_rcu_usb2_phy_power_on,
    .power_off	= ltq_rcu_usb2_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static int ltq_rcu_usb2_of_parse(struct ltq_rcu_usb2_priv *priv,
    struct platform_device *pdev)
    {
    struct device *dev = priv.dev;
    const __be32 *offset;
    priv.reg_bits = of_device_get_match_data(dev);
    priv.regmap = syscon_node_to_regmap(dev.of_node.parent);
    if (IS_ERR(priv.regmap)) {
    dev_err(dev, "Failed to lookup RCU regmap\n");
    return PTR_ERR(priv.regmap);
    }
    offset = of_get_address(dev.of_node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!offset) {
    dev_err(dev, "Failed to get RCU PHY reg offset\n");
    return -ENOENT;
    }
    priv.phy_reg_offset = __be32_to_cpu(*offset);
    if (priv.reg_bits.have_ana_cfg) {
    offset = of_get_address(dev.of_node, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if (!offset) {
    dev_err(dev, "Failed to get RCU ANA CFG1 reg offset\n");
    return -ENOENT;
    }
    priv.ana_cfg1_reg_offset = __be32_to_cpu(*offset);
    }
    priv.phy_gate_clk = devm_clk_get(dev, "phy");
    if (IS_ERR(priv.phy_gate_clk)) {
    dev_err(dev, "Unable to get USB phy gate clk\n");
    return PTR_ERR(priv.phy_gate_clk);
    }
    priv.ctrl_reset = devm_reset_control_get_shared(dev, "ctrl");
    if (IS_ERR(priv.ctrl_reset)) {
    if (PTR_ERR(priv.ctrl_reset) != -EPROBE_DEFER)
    dev_err(dev, "failed to get 'ctrl' reset\n");
    return PTR_ERR(priv.ctrl_reset);
    }
    priv.phy_reset = devm_reset_control_get_optional(dev, "phy");
    return PTR_ERR_OR_ZERO(priv.phy_reset);
    }
#[no_mangle]
unsafe extern "C" fn ltq_rcu_usb2_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_rcu_usb2_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ltq_rcu_usb2_priv *priv;
    struct phy_provider *provider;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    ret = ltq_rcu_usb2_of_parse(priv, pdev);
    if (ret)
    return ret;
// Reset USB core through reset controller
    reset_control_deassert(priv.ctrl_reset);
    reset_control_assert(priv.phy_reset);
    priv.phy = devm_phy_create(dev, dev.of_node, &ltq_rcu_usb2_phy_ops);
    if (IS_ERR(priv.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(priv.phy);
    }
    phy_set_drvdata(priv.phy, priv);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider))
    return PTR_ERR(provider);
    dev_set_drvdata(priv.dev, priv);
    return 0;
    }
    static struct platform_driver ltq_rcu_usb2_phy_driver = {
    .probe	= ltq_rcu_usb2_phy_probe,
    .driver = {
    .name	= "lantiq-rcu-usb2-phy",
    .of_match_table	= ltq_rcu_usb2_phy_of_match,
    }
    };
    module_platform_driver(ltq_rcu_usb2_phy_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Lantiq XWAY USB2 PHY driver");
    MODULE_LICENSE("GPL v2");
