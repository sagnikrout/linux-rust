//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-mipi-dsi-mt8196.c
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
// Copyright (c) 2019 MediaTek Inc.
// Author: jitao.shi <jitao.shi@mediatek.com>
//
// Copyright (c) 2026 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

pub const MIPITX_LANE_CON: c_uint = 0x0004;

pub const MIPITX_VOLTAGE_SEL: c_uint = 0x0008;

pub const RG_DSI_PRD_REF_MINI: c_int = 0;
pub const RG_DSI_PRD_REF_DEF: c_int = 4;
pub const RG_DSI_PRD_REF_MAX: c_int = 7;
pub const MIPITX_PRESERVED: c_uint = 0x000c;
pub const MIPITX_PRESERVED_DEF: c_uint = 0xffff0040;
pub const MIPITX_PRESERVED_MINI: c_uint = 0xffff00f0;
pub const MIPITX_PLL_PWR: c_uint = 0x0028;

pub const MIPITX_PLL_CON0: c_uint = 0x002c;
pub const MIPITX_PLL_CON1: c_uint = 0x0030;

pub const MIPITX_PLL_CON2: c_uint = 0x0034;
pub const MIPITX_PLL_CON3: c_uint = 0x0038;
pub const MIPITX_PLL_CON4: c_uint = 0x003c;

pub const MIPITX_D2_SW_CTL_EN: c_uint = 0x015c;
pub const MIPITX_D0_SW_CTL_EN: c_uint = 0x025c;
pub const MIPITX_CK_CKMODE_EN: c_uint = 0x0320;

pub const MIPITX_CK_SW_CTL_EN: c_uint = 0x035c;
pub const MIPITX_D1_SW_CTL_EN: c_uint = 0x045c;
pub const MIPITX_D3_SW_CTL_EN: c_uint = 0x055c;

pub const DSI_PHY_XTAL_CLK_HZ: c_int = 26000000;
pub const DSI_PHY_PLL_MIN_RATE_HZ: c_int = 125000000;
pub const DSI_PHY_PLL_MAX_RATE_HZ: c_int = 2000000000;
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_pll_enable(hw: *mut clk_hw) -> c_int {
    static int mtk_mipi_tx_pll_enable(struct clk_hw *hw)
    {
    struct mtk_mipi_tx *mipi_tx = mtk_mipi_tx_from_clk_hw(hw);
    void __iomem *base = mipi_tx.regs;
    let mut voltage: u32 = RG_DSI_PRD_REF_MINI;
    let mut pres: u32 = MIPITX_PRESERVED_MINI;
    unsigned long long pcw_calc;
    unsigned int txdiv, txdiv0;
    u32 pcw;
    dev_dbg(mipi_tx.dev, "enable: %u bps\n", mipi_tx.data_rate);
    if (mipi_tx.data_rate >= DSI_PHY_PLL_MAX_RATE_HZ) {
// Select higher signaling voltage for fast data rates
    voltage = RG_DSI_PRD_REF_DEF;
    pres = MIPITX_PRESERVED_DEF;
    txdiv = 1;
    txdiv0 = 0;
    } else if (mipi_tx.data_rate >= 1000000000) {
    txdiv = 2;
    txdiv0 = 1;
    } else if (mipi_tx.data_rate >= 500000000) {
    txdiv = 4;
    txdiv0 = 2;
    } else if (mipi_tx.data_rate > 250000000) {
    txdiv = 8;
    txdiv0 = 3;
    } else if (mipi_tx.data_rate >= 125000000) {
    txdiv = 16;
    txdiv0 = 4;
    } else {
    return -EINVAL;
    }
    pcw_calc = ((u64)(mipi_tx.data_rate / 2) * txdiv) << 24;
    pcw_calc = div_u64(pcw_calc, DSI_PHY_XTAL_CLK_HZ);
    if (pcw_calc > U32_MAX) {
    dev_err(mipi_tx.dev, "Calculated PCW=%llu overflow!\n", pcw_calc);
    return -EINVAL;
    }
    pcw = (u32)pcw_calc;
    mtk_phy_update_field(base + MIPITX_VOLTAGE_SEL, RG_DSI_PRD_REF_SEL, voltage);
    writel(pres, base + MIPITX_PRESERVED);
// Enable the SDM and wait for power to stabilize
    mtk_phy_set_bits(base + MIPITX_PLL_PWR, AD_DSI_PLL_SDM_PWR_ON);
    mtk_phy_clear_bits(base + MIPITX_PLL_CON1, RG_DSI_PLL_EN);
    udelay(30);
// Disable isolation and program PLL's PCW and dividers
    mtk_phy_clear_bits(base + MIPITX_PLL_PWR, AD_DSI_PLL_SDM_ISO_EN);
    writel(pcw, base + MIPITX_PLL_CON0);
    mtk_phy_update_field(base + MIPITX_PLL_CON1, RG_DSI_PLL_POSDIV, txdiv0);
// Enable the PLL and wait for the clock output to stabilize
    mtk_phy_set_bits(base + MIPITX_PLL_CON1, RG_DSI_PLL_EN);
    udelay(30);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_pll_disable(hw: *mut clk_hw) {
    static void mtk_mipi_tx_pll_disable(struct clk_hw *hw)
    {
    struct mtk_mipi_tx *mipi_tx = mtk_mipi_tx_from_clk_hw(hw);
    void __iomem *base = mipi_tx.regs;
    mtk_phy_clear_bits(base + MIPITX_PLL_CON1, RG_DSI_PLL_EN);
    mtk_phy_set_bits(base + MIPITX_PLL_PWR, AD_DSI_PLL_SDM_ISO_EN);
    mtk_phy_clear_bits(base + MIPITX_PLL_PWR, AD_DSI_PLL_SDM_PWR_ON);
    }
    static int mtk_mipi_tx_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    req.rate = clamp_val(req.rate,
    DSI_PHY_PLL_MIN_RATE_HZ, DSI_PHY_PLL_MAX_RATE_HZ);
    return 0;
    }
    static const struct clk_ops mtk_mipi_tx_pll_ops = {
    .enable = mtk_mipi_tx_pll_enable,
    .disable = mtk_mipi_tx_pll_disable,
    .determine_rate = mtk_mipi_tx_pll_determine_rate,
    .set_rate = mtk_mipi_tx_pll_set_rate,
    .recalc_rate = mtk_mipi_tx_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_power_on_signal(phy: *mut phy) {
    static void mtk_mipi_tx_power_on_signal(struct phy *phy)
    {
    struct mtk_mipi_tx *mipi_tx = phy_get_drvdata(phy);
    void __iomem *base = mipi_tx.regs;
// BG_LPF_EN / BG_CORE_EN
    writel(RG_DSI_PAD_TIEL_SEL | RG_DSI_BG_CORE_EN, base + MIPITX_LANE_CON);
// Wait for MIPI core to enable
    usleep_range(30, 100);
    writel(RG_DSI_BG_CORE_EN | RG_DSI_BG_LPF_EN, base + MIPITX_LANE_CON);
// Switch OFF each Lane
    mtk_phy_clear_bits(base + MIPITX_D0_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_clear_bits(base + MIPITX_D1_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_clear_bits(base + MIPITX_D2_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_clear_bits(base + MIPITX_D3_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_clear_bits(base + MIPITX_CK_SW_CTL_EN, DSI_SW_CTL_EN);
//
// The MIPI TX drive strength is in the range of 3000 ~ 6000 microamps:
// RG_DSI_HSTX_LDO_REF_SEL expresses an offset from the minimum drive
// strength (3000uA) and can add a maximum offset of 3000uA, reaching a
// maximum drive strength of 3000+3000=6000uA.
//
    mtk_phy_update_field(base + MIPITX_VOLTAGE_SEL, RG_DSI_HSTX_LDO_REF_SEL,
    (mipi_tx.mipitx_drive - 3000) / 200);
    mtk_phy_set_bits(base + MIPITX_CK_CKMODE_EN, DSI_CK_CKMODE_EN);
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_power_off_signal(phy: *mut phy) {
    static void mtk_mipi_tx_power_off_signal(struct phy *phy)
    {
    struct mtk_mipi_tx *mipi_tx = phy_get_drvdata(phy);
    void __iomem *base = mipi_tx.regs;
// Switch ON each lane one by one
    mtk_phy_set_bits(base + MIPITX_D0_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_set_bits(base + MIPITX_D1_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_set_bits(base + MIPITX_D2_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_set_bits(base + MIPITX_D3_SW_CTL_EN, DSI_SW_CTL_EN);
    mtk_phy_set_bits(base + MIPITX_CK_SW_CTL_EN, DSI_SW_CTL_EN);
    writel(RG_DSI_PAD_TIEL_SEL | RG_DSI_BG_CORE_EN, base + MIPITX_LANE_CON);
    writel(RG_DSI_PAD_TIEL_SEL, base + MIPITX_LANE_CON);
    }
    const struct mtk_mipitx_data mt8196_mipitx_data = {
    .mipi_tx_clk_ops = &mtk_mipi_tx_pll_ops,
    .mipi_tx_enable_signal = mtk_mipi_tx_power_on_signal,
    .mipi_tx_disable_signal = mtk_mipi_tx_power_off_signal,
    };
