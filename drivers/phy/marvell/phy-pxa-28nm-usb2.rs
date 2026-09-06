//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-pxa-28nm-usb2.c
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
// Copyright (C) 2015 Linaro, Ltd.
// Rob Herring <robh@kernel.org>
//
// Based on vendor driver:
// Copyright (C) 2013 Marvell Inc.
// Author: Chao Xie <xiechao.mail@gmail.com>
//

// USB PXA1928 PHY mapping
pub const PHY_28NM_PLL_REG0: c_uint = 0x0;
pub const PHY_28NM_PLL_REG1: c_uint = 0x4;
pub const PHY_28NM_CAL_REG: c_uint = 0x8;
pub const PHY_28NM_TX_REG0: c_uint = 0x0c;
pub const PHY_28NM_TX_REG1: c_uint = 0x10;
pub const PHY_28NM_RX_REG0: c_uint = 0x14;
pub const PHY_28NM_RX_REG1: c_uint = 0x18;
pub const PHY_28NM_DIG_REG0: c_uint = 0x1c;
pub const PHY_28NM_DIG_REG1: c_uint = 0x20;
pub const PHY_28NM_TEST_REG0: c_uint = 0x24;
pub const PHY_28NM_TEST_REG1: c_uint = 0x28;
pub const PHY_28NM_MOC_REG: c_uint = 0x2c;
pub const PHY_28NM_PHY_RESERVE: c_uint = 0x30;
pub const PHY_28NM_OTG_REG: c_uint = 0x34;
pub const PHY_28NM_CHRG_DET: c_uint = 0x38;
pub const PHY_28NM_CTRL_REG0: c_uint = 0xc4;
pub const PHY_28NM_CTRL_REG1: c_uint = 0xc8;
pub const PHY_28NM_CTRL_REG2: c_uint = 0xd4;
pub const PHY_28NM_CTRL_REG3: c_uint = 0xdc;
// PHY_28NM_PLL_REG0

pub const PHY_28NM_PLL_SELLPFR_SHIFT: c_int = 28;

pub const PHY_28NM_PLL_FBDIV_SHIFT: c_int = 16;

pub const PHY_28NM_PLL_ICP_SHIFT: c_int = 8;

pub const PHY_28NM_PLL_REFDIV_SHIFT: c_int = 0;
pub const PHY_28NM_PLL_REFDIV_MASK: c_uint = 0x7f;
// PHY_28NM_PLL_REG1

// PHY_28NM_CAL_REG

pub const PHY_28NM_PLL_KVCO_SHIFT: c_int = 16;

pub const PHY_28NM_PLL_CAL12_SHIFT: c_int = 20;

pub const PHY_28NM_IMPCAL_VTH_SHIFT: c_int = 8;

pub const PHY_28NM_PLLCAL_START_SHIFT: c_int = 22;
pub const PHY_28NM_IMPCAL_START_SHIFT: c_int = 13;
// PHY_28NM_TX_REG0

pub const PHY_28NM_TX_AMP_SHIFT: c_int = 20;

// PHY_28NM_RX_REG0
pub const PHY_28NM_RX_SQ_THRESH_SHIFT: c_int = 0;

// PHY_28NM_RX_REG1

// PHY_28NM_DIG_REG0

pub const PHY_28NM_DIG_SQ_FILT_SHIFT: c_int = 16;

pub const PHY_28NM_DIG_SQ_BLK_SHIFT: c_int = 12;

pub const PHY_28NM_DIG_SYNC_NUM_SHIFT: c_int = 0;

// PHY_28NM_OTG_REG

pub const PHY_28NM_CHGDTC_ENABLE_SWITCH_DM_SHIFT_28: c_int = 13;
pub const PHY_28NM_CHGDTC_ENABLE_SWITCH_DP_SHIFT_28: c_int = 12;
pub const PHY_28NM_CHGDTC_VSRC_CHARGE_SHIFT_28: c_int = 10;
pub const PHY_28NM_CHGDTC_VDAT_CHARGE_SHIFT_28: c_int = 8;
pub const PHY_28NM_CHGDTC_CDP_DM_AUTO_SWITCH_SHIFT_28: c_int = 7;
pub const PHY_28NM_CHGDTC_DP_DM_SWAP_SHIFT_28: c_int = 6;
pub const PHY_28NM_CHGDTC_PU_CHRG_DTC_SHIFT_28: c_int = 5;
pub const PHY_28NM_CHGDTC_PD_EN_SHIFT_28: c_int = 4;
pub const PHY_28NM_CHGDTC_DCP_EN_SHIFT_28: c_int = 3;
pub const PHY_28NM_CHGDTC_CDP_EN_SHIFT_28: c_int = 2;
pub const PHY_28NM_CHGDTC_TESTMON_CHRGDTC_SHIFT_28: c_int = 0;
pub const PHY_28NM_CTRL1_CHRG_DTC_OUT_SHIFT_28: c_int = 4;
pub const PHY_28NM_CTRL1_VBUSDTC_OUT_SHIFT_28: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_usb2_phy {
    pub phy: *mut phy,
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn wait_for_reg(reg: *mut void __iomem, mask: u32, ms: u32) -> c_int {
    static int wait_for_reg(void __iomem *reg, u32 mask, u32 ms)
    {
    u32 val;
    return readl_poll_timeout(reg, val, ((val & mask) == mask),
    1000, 1000 * ms);
    }
#[no_mangle]
unsafe extern "C" fn mv_usb2_phy_28nm_init(phy: *mut phy) -> c_int {
    static int mv_usb2_phy_28nm_init(struct phy *phy)
    {
    struct mv_usb2_phy *mv_phy = phy_get_drvdata(phy);
    struct platform_device *pdev = mv_phy.pdev;
    void __iomem *base = mv_phy.base;
    u32 reg;
    int ret;
    clk_prepare_enable(mv_phy.clk);
// PHY_28NM_PLL_REG0
    reg = readl(base + PHY_28NM_PLL_REG0) &
    ~(PHY_28NM_PLL_SELLPFR_MASK | PHY_28NM_PLL_FBDIV_MASK
    | PHY_28NM_PLL_ICP_MASK	| PHY_28NM_PLL_REFDIV_MASK);
    writel(reg | (0x1 << PHY_28NM_PLL_SELLPFR_SHIFT
    | 0xf0 << PHY_28NM_PLL_FBDIV_SHIFT
    | 0x3 << PHY_28NM_PLL_ICP_SHIFT
    | 0xd << PHY_28NM_PLL_REFDIV_SHIFT),
    base + PHY_28NM_PLL_REG0);
// PHY_28NM_PLL_REG1
    reg = readl(base + PHY_28NM_PLL_REG1);
    writel(reg | PHY_28NM_PLL_PU_PLL | PHY_28NM_PLL_PU_BY_REG,
    base + PHY_28NM_PLL_REG1);
// PHY_28NM_TX_REG0
    reg = readl(base + PHY_28NM_TX_REG0) & ~PHY_28NM_TX_AMP_MASK;
    writel(reg | PHY_28NM_TX_PU_BY_REG | 0x3 << PHY_28NM_TX_AMP_SHIFT |
    PHY_28NM_TX_PU_ANA,
    base + PHY_28NM_TX_REG0);
// PHY_28NM_RX_REG0
    reg = readl(base + PHY_28NM_RX_REG0) & ~PHY_28NM_RX_SQ_THRESH_MASK;
    writel(reg | 0xa << PHY_28NM_RX_SQ_THRESH_SHIFT,
    base + PHY_28NM_RX_REG0);
// PHY_28NM_DIG_REG0
    reg = readl(base + PHY_28NM_DIG_REG0) &
    ~(PHY_28NM_DIG_BITSTAFFING_ERR | PHY_28NM_DIG_SYNC_ERR |
    PHY_28NM_DIG_SQ_FILT_MASK | PHY_28NM_DIG_SQ_BLK_MASK |
    PHY_28NM_DIG_SYNC_NUM_MASK);
    writel(reg | (0x1 << PHY_28NM_DIG_SYNC_NUM_SHIFT |
    PHY_28NM_PLL_LOCK_BYPASS),
    base + PHY_28NM_DIG_REG0);
// PHY_28NM_OTG_REG
    reg = readl(base + PHY_28NM_OTG_REG) | PHY_28NM_OTG_PU_OTG;
    writel(reg & ~PHY_28NM_OTG_CONTROL_BY_PIN, base + PHY_28NM_OTG_REG);
//
// Calibration Timing
// ____________________________
// CAL START   ___|
// ____________________
// CAL_DONE    ___________|
// | 400us |
//
// Make sure PHY Calibration is ready
    ret = wait_for_reg(base + PHY_28NM_CAL_REG,
    PHY_28NM_PLL_PLLCAL_DONE | PHY_28NM_PLL_IMPCAL_DONE,
    100);
    if (ret) {
    dev_warn(&pdev.dev, "USB PHY PLL calibrate not done after 100mS.");
    goto err_clk;
    }
    ret = wait_for_reg(base + PHY_28NM_RX_REG1,
    PHY_28NM_RX_SQCAL_DONE, 100);
    if (ret) {
    dev_warn(&pdev.dev, "USB PHY RX SQ calibrate not done after 100mS.");
    goto err_clk;
    }
// Make sure PHY PLL is ready
    ret = wait_for_reg(base + PHY_28NM_PLL_REG0, PHY_28NM_PLL_READY, 100);
    if (ret) {
    dev_warn(&pdev.dev, "PLL_READY not set after 100mS.");
    goto err_clk;
    }
    return 0;
    err_clk:
    clk_disable_unprepare(mv_phy.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mv_usb2_phy_28nm_power_on(phy: *mut phy) -> c_int {
    static int mv_usb2_phy_28nm_power_on(struct phy *phy)
    {
    struct mv_usb2_phy *mv_phy = phy_get_drvdata(phy);
    void __iomem *base = mv_phy.base;
    writel(readl(base + PHY_28NM_CTRL_REG3) |
    (PHY_28NM_CTRL3_OVERWRITE | PHY_28NM_CTRL3_VBUS_VALID |
    PHY_28NM_CTRL3_AVALID | PHY_28NM_CTRL3_BVALID),
    base + PHY_28NM_CTRL_REG3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_usb2_phy_28nm_power_off(phy: *mut phy) -> c_int {
    static int mv_usb2_phy_28nm_power_off(struct phy *phy)
    {
    struct mv_usb2_phy *mv_phy = phy_get_drvdata(phy);
    void __iomem *base = mv_phy.base;
    writel(readl(base + PHY_28NM_CTRL_REG3) |
    ~(PHY_28NM_CTRL3_OVERWRITE | PHY_28NM_CTRL3_VBUS_VALID
    | PHY_28NM_CTRL3_AVALID	| PHY_28NM_CTRL3_BVALID),
    base + PHY_28NM_CTRL_REG3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_usb2_phy_28nm_exit(phy: *mut phy) -> c_int {
    static int mv_usb2_phy_28nm_exit(struct phy *phy)
    {
    struct mv_usb2_phy *mv_phy = phy_get_drvdata(phy);
    void __iomem *base = mv_phy.base;
    unsigned int val;
    val = readw(base + PHY_28NM_PLL_REG1);
    val &= ~PHY_28NM_PLL_PU_PLL;
    writew(val, base + PHY_28NM_PLL_REG1);
// power down PHY Analog part
    val = readw(base + PHY_28NM_TX_REG0);
    val &= ~PHY_28NM_TX_PU_ANA;
    writew(val, base + PHY_28NM_TX_REG0);
// power down PHY OTG part
    val = readw(base + PHY_28NM_OTG_REG);
    val &= ~PHY_28NM_OTG_PU_OTG;
    writew(val, base + PHY_28NM_OTG_REG);
    clk_disable_unprepare(mv_phy.clk);
    return 0;
    }
    static const struct phy_ops usb_ops = {
    .init		= mv_usb2_phy_28nm_init,
    .power_on	= mv_usb2_phy_28nm_power_on,
    .power_off	= mv_usb2_phy_28nm_power_off,
    .exit		= mv_usb2_phy_28nm_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mv_usb2_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mv_usb2_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct mv_usb2_phy *mv_phy;
    mv_phy = devm_kzalloc(&pdev.dev, sizeof(*mv_phy), GFP_KERNEL);
    if (!mv_phy)
    return -ENOMEM;
    mv_phy.pdev = pdev;
    mv_phy.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mv_phy.clk)) {
    dev_err(&pdev.dev, "failed to get clock.\n");
    return PTR_ERR(mv_phy.clk);
    }
    mv_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mv_phy.base))
    return PTR_ERR(mv_phy.base);
    mv_phy.phy = devm_phy_create(&pdev.dev, pdev.dev.of_node, &usb_ops);
    if (IS_ERR(mv_phy.phy))
    return PTR_ERR(mv_phy.phy);
    phy_set_drvdata(mv_phy.phy, mv_phy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id mv_usbphy_dt_match[] = {
    { .compatible = "marvell,pxa1928-usb-phy", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mv_usbphy_dt_match);
    static struct platform_driver mv_usb2_phy_driver = {
    .probe	= mv_usb2_phy_probe,
    .driver = {
    .name   = "mv-usb2-phy",
    .of_match_table = mv_usbphy_dt_match,
    },
    };
    module_platform_driver(mv_usb2_phy_driver);
    MODULE_AUTHOR("Rob Herring <robh@kernel.org>");
    MODULE_DESCRIPTION("Marvell USB2 phy driver");
    MODULE_LICENSE("GPL v2");
