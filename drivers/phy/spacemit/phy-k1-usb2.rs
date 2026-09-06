//! Automatically rewritten from C to Rust
//! Source: drivers/phy/spacemit/phy-k1-usb2.c
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
// SpacemiT K1 USB 2.0 PHY driver
//
// Copyright (C) 2025 SpacemiT (Hangzhou) Technology Co. Ltd
// Copyright (C) 2025 Ze Huang <huang.ze@linux.dev>
//

pub const PHY_RST_MODE_CTRL: c_uint = 0x04;

//
// hs line state sel (Bit 13):
// - 1 (Default): Internal HS line state is set to 01 when usb_hs_tx_en is valid.
// - 0: Internal HS line state is always driven by usb_hs_lstate.
//
// fs line state sel (Bit 14):
// - 1 (Default): FS line state is determined by the output data
// (usb_fs_datain/b).
// - 0: FS line state is always determined by the input data (dmo/dpo).
//

    PHY_CLK_MAC_EN)

    PHY_MAC_RSTN)
pub const PHY_TX_HOST_CTRL: c_uint = 0x10;

pub const PHY_HSTXP_HW_CTRL: c_uint = 0x34;

pub const PHY_K1_HS_HOST_DISC: c_uint = 0x40;

pub const PHY_K3_HS_HOST_DISC: c_uint = 0x20;

pub const PHY_PLL_DIV_CFG: c_uint = 0x98;

//
// freq_sel<1:0>
// if ref clk freq=24.0MHz-->freq_sel<2:0> == 3b'001, then internal divider value == 80
//

//
// pll divider value selection
// 1: divider value will choose internal default value ,dependent on freq_sel<1:0>
// 0: divider value will be over ride by fdiv_reg<21:0>
//

pub const PHY_SEL_FREQ_24MHZ: c_uint = 0x01;

    PHY_FDIV_FRACT_8_15)
pub const FDIV_REG_VAL: c_uint = 0x1ec4		/* 0x100 selects 24MHz, rest are default */;
pub const K1_USB2PHY_RESET_TIME_MS: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spacemit_usb2phy {
    pub phy: *mut phy,
    pub clk: *mut clk,
    pub regmap_base: *mut regmap,
}

    static const struct regmap_config phy_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 0x200,
    };
#[no_mangle]
unsafe extern "C" fn spacemit_usb2phy_init(phy: *mut phy) -> c_int {
    static int spacemit_usb2phy_init(struct phy *phy)
    {
    struct spacemit_usb2phy *sphy = phy_get_drvdata(phy);
    struct regmap *map = sphy.regmap_base;
    u32 val;
    int ret;
    ret = clk_enable(sphy.clk);
    if (ret) {
    dev_err(&phy.dev, "failed to enable clock\n");
    return ret;
    }
//
// make sure the usb controller is not under reset process before
// any configuration
//
    usleep_range(150, 200);
// 24M ref clk
    val = FIELD_PREP(FDIV_REG_MASK, FDIV_REG_VAL) |
    FIELD_PREP(PHY_FDIV_FRACT_0_1, PHY_SEL_FREQ_24MHZ) |
    PHY_DIV_LOCAL_EN;
    regmap_write(map, PHY_PLL_DIV_CFG, val);
    ret = regmap_read_poll_timeout(map, PHY_RST_MODE_CTRL, val,
    (val & PHY_PLL_RDY),
    500, K1_USB2PHY_RESET_TIME_MS * 1000);
    if (ret) {
    dev_err(&phy.dev, "wait PLLREADY timeout\n");
    clk_disable(sphy.clk);
    return ret;
    }
// release usb2 phy internal reset and enable clock gating
    val = (PHY_INIT_MODE_BITS | PHY_CLK_ENABLE_BITS | PHY_DEASSERT_RST_BITS);
    regmap_write(map, PHY_RST_MODE_CTRL, val);
    val = (PHY_HSTXP_RSTN | PHY_CLK_HSTXP_EN | PHY_HSTXP_MODE);
    regmap_write(map, PHY_HSTXP_HW_CTRL, val);
// auto clear host disc
    regmap_update_bits(map, PHY_TX_HOST_CTRL, PHY_HST_DISC_AUTO_CLR,
    PHY_HST_DISC_AUTO_CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_usb2phy_exit(phy: *mut phy) -> c_int {
    static int spacemit_usb2phy_exit(struct phy *phy)
    {
    struct spacemit_usb2phy *sphy = phy_get_drvdata(phy);
    clk_disable(sphy.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_k1_usb2phy_disconnect(phy: *mut phy, port: c_int) -> c_int {
    static int spacemit_k1_usb2phy_disconnect(struct phy *phy, int port)
    {
    struct spacemit_usb2phy *sphy = phy_get_drvdata(phy);
    regmap_update_bits(sphy.regmap_base, PHY_K1_HS_HOST_DISC,
    PHY_K1_HS_HOST_DISC_CLR, PHY_K1_HS_HOST_DISC_CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_k3_usb2phy_disconnect(phy: *mut phy, port: c_int) -> c_int {
    static int spacemit_k3_usb2phy_disconnect(struct phy *phy, int port)
    {
    struct spacemit_usb2phy *sphy = phy_get_drvdata(phy);
    regmap_update_bits(sphy.regmap_base, PHY_K3_HS_HOST_DISC,
    PHY_K3_HS_HOST_DISC_CLR, PHY_K3_HS_HOST_DISC_CLR);
    return 0;
    }
    static const struct phy_ops spacemit_k1_usb2phy_ops = {
    .init = spacemit_usb2phy_init,
    .exit = spacemit_usb2phy_exit,
    .disconnect = spacemit_k1_usb2phy_disconnect,
    .owner = THIS_MODULE,
    };
    static const struct phy_ops spacemit_k3_usb2phy_ops = {
    .init = spacemit_usb2phy_init,
    .exit = spacemit_usb2phy_exit,
    .disconnect = spacemit_k3_usb2phy_disconnect,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn spacemit_usb2phy_probe(pdev: *mut platform_device) -> c_int {
    static int spacemit_usb2phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct spacemit_usb2phy *sphy;
    const struct phy_ops *ops;
    void __iomem *base;
    sphy = devm_kzalloc(dev, sizeof(*sphy), GFP_KERNEL);
    if (!sphy)
    return -ENOMEM;
    ops = device_get_match_data(dev);
    sphy.clk = devm_clk_get_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sphy.clk))
    return dev_err_probe(dev, PTR_ERR(sphy.clk), "Failed to get clock\n");
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    sphy.regmap_base = devm_regmap_init_mmio(dev, base, &phy_regmap_config);
    if (IS_ERR(sphy.regmap_base))
    return dev_err_probe(dev, PTR_ERR(sphy.regmap_base), "Failed to init regmap\n");
    sphy.phy = devm_phy_create(dev, core::ptr::null_mut(), ops);
    if (IS_ERR(sphy.phy))
    return dev_err_probe(dev, PTR_ERR(sphy.phy), "Failed to create phy\n");
    phy_set_drvdata(sphy.phy, sphy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id spacemit_usb2phy_dt_match[] = {
    { .compatible = "spacemit,k1-usb2-phy", .data = &spacemit_k1_usb2phy_ops },
    { .compatible = "spacemit,k3-usb2-phy", .data = &spacemit_k3_usb2phy_ops },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, spacemit_usb2phy_dt_match);
    static struct platform_driver spacemit_usb2_phy_driver = {
    .probe	= spacemit_usb2phy_probe,
    .driver = {
    .name   = "spacemit-usb2-phy",
    .of_match_table = spacemit_usb2phy_dt_match,
    },
    };
    module_platform_driver(spacemit_usb2_phy_driver);
    MODULE_DESCRIPTION("Spacemit USB 2.0 PHY driver");
    MODULE_LICENSE("GPL");
