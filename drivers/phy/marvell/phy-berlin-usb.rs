//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-berlin-usb.c
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
// Copyright (C) 2014 Marvell Technology Group Ltd.
//
// Antoine Tenart <antoine.tenart@free-electrons.com>
// Jisheng Zhang <jszhang@marvell.com>
//

pub const USB_PHY_PLL: c_uint = 0x04;
pub const USB_PHY_PLL_CONTROL: c_uint = 0x08;
pub const USB_PHY_TX_CTRL0: c_uint = 0x10;
pub const USB_PHY_TX_CTRL1: c_uint = 0x14;
pub const USB_PHY_TX_CTRL2: c_uint = 0x18;
pub const USB_PHY_RX_CTRL: c_uint = 0x20;
pub const USB_PHY_ANALOG: c_uint = 0x34;
// USB_PHY_PLL

// USB_PHY_PLL_CONTROL

// USB_PHY_TX_CTRL0

// USB_PHY_TX_CTRL1

// USB_PHY_TX_CTRL2

// USB_PHY_RX_CTRL

// USB_PHY_ANALOG

    static const u32 phy_berlin_pll_dividers[] = {
// Berlin 2
    CLK_REF_DIV(0x6) | FEEDBACK_CLK_DIV(0x55),
// Berlin 2CD/Q
    CLK_REF_DIV(0xc) | FEEDBACK_CLK_DIV(0x54),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_berlin_usb_priv {
    pub base: *mut void __iomem,
    pub rst_ctrl: *mut reset_control,
    pub pll_divider: u32,
}

#[no_mangle]
unsafe extern "C" fn phy_berlin_usb_power_on(phy: *mut phy) -> c_int {
    static int phy_berlin_usb_power_on(struct phy *phy)
    {
    struct phy_berlin_usb_priv *priv = phy_get_drvdata(phy);
    reset_control_reset(priv.rst_ctrl);
    writel(priv.pll_divider,
    priv.base + USB_PHY_PLL);
    writel(CLK_STABLE | PLL_CTRL_REG | PHASE_OFF_TOL_250 | KVC0_REG_CTRL |
    CLK_BLK_EN, priv.base + USB_PHY_PLL_CONTROL);
    writel(V2I_VCO_RATIO(0x5) | R_ROTATE_0 | ANA_TEST_DC_CTRL(0x5),
    priv.base + USB_PHY_ANALOG);
    writel(PHASE_FREEZE_DLY_4_CL | ACK_LENGTH_16_CL | SQ_LENGTH_12 |
    DISCON_THRESHOLD_270 | SQ_THRESHOLD(0xa) | LPF_COEF(0x2) |
    INTPL_CUR_30, priv.base + USB_PHY_RX_CTRL);
    writel(TX_VDD12_13 | TX_OUT_AMP(0x3), priv.base + USB_PHY_TX_CTRL1);
    writel(EXT_HS_RCAL_EN | IMPCAL_VTH_DIV(0x3) | EXT_RS_RCAL_DIV(0x4),
    priv.base + USB_PHY_TX_CTRL0);
    writel(EXT_HS_RCAL_EN | IMPCAL_VTH_DIV(0x3) | EXT_RS_RCAL_DIV(0x4) |
    EXT_FS_RCAL_DIV(0x2), priv.base + USB_PHY_TX_CTRL0);
    writel(EXT_HS_RCAL_EN | IMPCAL_VTH_DIV(0x3) | EXT_RS_RCAL_DIV(0x4),
    priv.base + USB_PHY_TX_CTRL0);
    writel(TX_CHAN_CTRL_REG(0xf) | DRV_SLEWRATE(0x3) | IMP_CAL_FS_HS_DLY_3 |
    FS_DRV_EN_MASK(0xd), priv.base + USB_PHY_TX_CTRL2);
    return 0;
    }
    static const struct phy_ops phy_berlin_usb_ops = {
    .power_on	= phy_berlin_usb_power_on,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id phy_berlin_usb_of_match[] = {
    {
    .compatible = "marvell,berlin2-usb-phy",
    .data = &phy_berlin_pll_dividers[0],
    },
    {
    .compatible = "marvell,berlin2cd-usb-phy",
    .data = &phy_berlin_pll_dividers[1],
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, phy_berlin_usb_of_match);
#[no_mangle]
unsafe extern "C" fn phy_berlin_usb_probe(pdev: *mut platform_device) -> c_int {
    static int phy_berlin_usb_probe(struct platform_device *pdev)
    {
    struct phy_berlin_usb_priv *priv;
    struct phy *phy;
    struct phy_provider *phy_provider;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.rst_ctrl = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst_ctrl))
    return PTR_ERR(priv.rst_ctrl);
    priv.pll_divider = *((u32 *)device_get_match_data(&pdev.dev));
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &phy_berlin_usb_ops);
    if (IS_ERR(phy)) {
    dev_err(&pdev.dev, "failed to create PHY\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, priv);
    phy_provider =
    devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver phy_berlin_usb_driver = {
    .probe	= phy_berlin_usb_probe,
    .driver	= {
    .name		= "phy-berlin-usb",
    .of_match_table	= phy_berlin_usb_of_match,
    },
    };
    module_platform_driver(phy_berlin_usb_driver);
    MODULE_AUTHOR("Antoine Tenart <antoine.tenart@free-electrons.com>");
    MODULE_DESCRIPTION("Marvell Berlin PHY driver for USB");
    MODULE_LICENSE("GPL");
