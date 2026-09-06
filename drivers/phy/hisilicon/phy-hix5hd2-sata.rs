//! Automatically rewritten from C to Rust
//! Source: drivers/phy/hisilicon/phy-hix5hd2-sata.c
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 HiSilicon Limited.
//

pub const SATA_PHY0_CTLL: c_uint = 0xa0;
pub const MPLL_MULTIPLIER_SHIFT: c_int = 1;
pub const MPLL_MULTIPLIER_MASK: c_uint = 0xfe;
pub const MPLL_MULTIPLIER_50M: c_uint = 0x3c;
pub const MPLL_MULTIPLIER_100M: c_uint = 0x1e;

pub const SATA_PORT_PHYCTL: c_uint = 0x174;
pub const SPEED_MODE_MASK: c_uint = 0x6f0000;
pub const HALF_RATE_SHIFT: c_int = 16;
pub const PHY_CONFIG_SHIFT: c_int = 18;
pub const GEN2_EN_SHIFT: c_int = 21;

pub const SATA_PORT_PHYCTL1: c_uint = 0x148;
pub const AMPLITUDE_MASK: c_uint = 0x3ffffe;
pub const AMPLITUDE_GEN3: c_uint = 0x68;
pub const AMPLITUDE_GEN3_SHIFT: c_int = 15;
pub const AMPLITUDE_GEN2: c_uint = 0x56;
pub const AMPLITUDE_GEN2_SHIFT: c_int = 8;
pub const AMPLITUDE_GEN1: c_uint = 0x56;
pub const AMPLITUDE_GEN1_SHIFT: c_int = 1;
pub const SATA_PORT_PHYCTL2: c_uint = 0x14c;
pub const PREEMPH_MASK: c_uint = 0x3ffff;
pub const PREEMPH_GEN3: c_uint = 0x20;
pub const PREEMPH_GEN3_SHIFT: c_int = 12;
pub const PREEMPH_GEN2: c_uint = 0x15;
pub const PREEMPH_GEN2_SHIFT: c_int = 6;
pub const PREEMPH_GEN1: c_uint = 0x5;
pub const PREEMPH_GEN1_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_priv {
    pub base: *mut void __iomem,
    pub peri_ctrl: *mut regmap,
}

    enum phy_speed_mode {
    SPEED_MODE_GEN1 = 0,
    SPEED_MODE_GEN2 = 1,
    SPEED_MODE_GEN3 = 2,
    };
#[no_mangle]
unsafe extern "C" fn hix5hd2_sata_phy_init(phy: *mut phy) -> c_int {
    static int hix5hd2_sata_phy_init(struct phy *phy)
    {
    struct hix5hd2_priv *priv = phy_get_drvdata(phy);
    u32 val, data[2];
    int ret;
    if (priv.peri_ctrl) {
    ret = of_property_read_u32_array(phy.dev.of_node,
    "hisilicon,power-reg",
    &data[0], 2);
    if (ret) {
    dev_err(&phy.dev, "Fail read hisilicon,power-reg\n");
    return ret;
    }
    regmap_update_bits(priv.peri_ctrl, data[0],
    BIT(data[1]), BIT(data[1]));
    }
// reset phy
    val = readl_relaxed(priv.base + SATA_PHY0_CTLL);
    val &= ~(MPLL_MULTIPLIER_MASK | REF_USE_PAD);
    val |= MPLL_MULTIPLIER_50M << MPLL_MULTIPLIER_SHIFT |
    REF_SSP_EN | PHY_RESET;
    writel_relaxed(val, priv.base + SATA_PHY0_CTLL);
    msleep(20);
    val &= ~PHY_RESET;
    writel_relaxed(val, priv.base + SATA_PHY0_CTLL);
    val = readl_relaxed(priv.base + SATA_PORT_PHYCTL1);
    val &= ~AMPLITUDE_MASK;
    val |= AMPLITUDE_GEN3 << AMPLITUDE_GEN3_SHIFT |
    AMPLITUDE_GEN2 << AMPLITUDE_GEN2_SHIFT |
    AMPLITUDE_GEN1 << AMPLITUDE_GEN1_SHIFT;
    writel_relaxed(val, priv.base + SATA_PORT_PHYCTL1);
    val = readl_relaxed(priv.base + SATA_PORT_PHYCTL2);
    val &= ~PREEMPH_MASK;
    val |= PREEMPH_GEN3 << PREEMPH_GEN3_SHIFT |
    PREEMPH_GEN2 << PREEMPH_GEN2_SHIFT |
    PREEMPH_GEN1 << PREEMPH_GEN1_SHIFT;
    writel_relaxed(val, priv.base + SATA_PORT_PHYCTL2);
// ensure PHYCTRL setting takes effect
    val = readl_relaxed(priv.base + SATA_PORT_PHYCTL);
    val &= ~SPEED_MODE_MASK;
    val |= SPEED_MODE_GEN1 << HALF_RATE_SHIFT |
    SPEED_MODE_GEN1 << PHY_CONFIG_SHIFT |
    SPEED_MODE_GEN1 << GEN2_EN_SHIFT | SPEED_CTRL;
    writel_relaxed(val, priv.base + SATA_PORT_PHYCTL);
    msleep(20);
    val &= ~SPEED_MODE_MASK;
    val |= SPEED_MODE_GEN3 << HALF_RATE_SHIFT |
    SPEED_MODE_GEN3 << PHY_CONFIG_SHIFT |
    SPEED_MODE_GEN3 << GEN2_EN_SHIFT | SPEED_CTRL;
    writel_relaxed(val, priv.base + SATA_PORT_PHYCTL);
    val &= ~(SPEED_MODE_MASK | SPEED_CTRL);
    val |= SPEED_MODE_GEN2 << HALF_RATE_SHIFT |
    SPEED_MODE_GEN2 << PHY_CONFIG_SHIFT |
    SPEED_MODE_GEN2 << GEN2_EN_SHIFT;
    writel_relaxed(val, priv.base + SATA_PORT_PHYCTL);
    return 0;
    }
    static const struct phy_ops hix5hd2_sata_phy_ops = {
    .init		= hix5hd2_sata_phy_init,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn hix5hd2_sata_phy_probe(pdev: *mut platform_device) -> c_int {
    static int hix5hd2_sata_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct resource *res;
    struct phy *phy;
    struct hix5hd2_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    priv.base = devm_ioremap(dev, res.start, resource_size(res));
    if (!priv.base)
    return -ENOMEM;
    priv.peri_ctrl = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,peripheral-syscon");
    if (IS_ERR(priv.peri_ctrl))
    priv.peri_ctrl = core::ptr::null_mut();
    phy = devm_phy_create(dev, core::ptr::null_mut(), &hix5hd2_sata_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id hix5hd2_sata_phy_of_match[] = {
    {.compatible = "hisilicon,hix5hd2-sata-phy",},
    { },
    };
    MODULE_DEVICE_TABLE(of, hix5hd2_sata_phy_of_match);
    static struct platform_driver hix5hd2_sata_phy_driver = {
    .probe	= hix5hd2_sata_phy_probe,
    .driver = {
    .name	= "hix5hd2-sata-phy",
    .of_match_table	= hix5hd2_sata_phy_of_match,
    }
    };
    module_platform_driver(hix5hd2_sata_phy_driver);
    MODULE_AUTHOR("Jiancheng Xue <xuejiancheng@huawei.com>");
    MODULE_DESCRIPTION("HISILICON HIX5HD2 SATA PHY driver");
    MODULE_ALIAS("platform:hix5hd2-sata-phy");
    MODULE_LICENSE("GPL v2");
