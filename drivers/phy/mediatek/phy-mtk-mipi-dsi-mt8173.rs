//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-mipi-dsi-mt8173.c
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

pub const MIPITX_DSI_CON: c_uint = 0x00;

pub const MIPITX_DSI_CLOCK_LANE: c_uint = 0x04;
pub const MIPITX_DSI_DATA_LANE0: c_uint = 0x08;
pub const MIPITX_DSI_DATA_LANE1: c_uint = 0x0c;
pub const MIPITX_DSI_DATA_LANE2: c_uint = 0x10;
pub const MIPITX_DSI_DATA_LANE3: c_uint = 0x14;

pub const MIPITX_DSI_TOP_CON: c_uint = 0x40;

pub const MIPITX_DSI_BG_CON: c_uint = 0x44;

    (RG_DSI_V12_SEL | RG_DSI_V10_SEL | RG_DSI_V072_SEL | \
    RG_DSI_V04_SEL | RG_DSI_V032_SEL | RG_DSI_V02_SEL)

pub const MIPITX_DSI_PLL_CON0: c_uint = 0x50;

    (RG_DSI_MPPLL_PREDIV | RG_DSI_MPPLL_TXDIV0 | \
    RG_DSI_MPPLL_TXDIV1 | RG_DSI_MPPLL_POSDIV)

pub const MIPITX_DSI_PLL_CON1: c_uint = 0x54;

pub const MIPITX_DSI_PLL_CON2: c_uint = 0x58;
pub const MIPITX_DSI_PLL_TOP: c_uint = 0x64;

pub const MIPITX_DSI_PLL_PWR: c_uint = 0x68;

pub const MIPITX_DSI_SW_CTRL: c_uint = 0x80;

pub const MIPITX_DSI_SW_CTRL_CON0: c_uint = 0x84;

#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int mtk_mipi_tx_pll_prepare(struct clk_hw *hw)
    {
    struct mtk_mipi_tx *mipi_tx = mtk_mipi_tx_from_clk_hw(hw);
    void __iomem *base = mipi_tx.regs;
    u8 txdiv, txdiv0, txdiv1;
    u64 pcw;
    dev_dbg(mipi_tx.dev, "prepare: %u Hz\n", mipi_tx.data_rate);
    if (mipi_tx.data_rate >= 500000000) {
    txdiv = 1;
    txdiv0 = 0;
    txdiv1 = 0;
    } else if (mipi_tx.data_rate >= 250000000) {
    txdiv = 2;
    txdiv0 = 1;
    txdiv1 = 0;
    } else if (mipi_tx.data_rate >= 125000000) {
    txdiv = 4;
    txdiv0 = 2;
    txdiv1 = 0;
    } else if (mipi_tx.data_rate > 62000000) {
    txdiv = 8;
    txdiv0 = 2;
    txdiv1 = 1;
    } else if (mipi_tx.data_rate >= 50000000) {
    txdiv = 16;
    txdiv0 = 2;
    txdiv1 = 2;
    } else {
    return -EINVAL;
    }
    mtk_phy_update_bits(base + MIPITX_DSI_BG_CON,
    RG_DSI_VOUT_MSK | RG_DSI_BG_CKEN |
    RG_DSI_BG_CORE_EN,
    FIELD_PREP(RG_DSI_V02_SEL, 4) |
    FIELD_PREP(RG_DSI_V032_SEL, 4) |
    FIELD_PREP(RG_DSI_V04_SEL, 4) |
    FIELD_PREP(RG_DSI_V072_SEL, 4) |
    FIELD_PREP(RG_DSI_V10_SEL, 4) |
    FIELD_PREP(RG_DSI_V12_SEL, 4) |
    RG_DSI_BG_CKEN | RG_DSI_BG_CORE_EN);
    usleep_range(30, 100);
    mtk_phy_update_bits(base + MIPITX_DSI_TOP_CON,
    RG_DSI_LNT_IMP_CAL_CODE | RG_DSI_LNT_HS_BIAS_EN,
    FIELD_PREP(RG_DSI_LNT_IMP_CAL_CODE, 8) |
    RG_DSI_LNT_HS_BIAS_EN);
    mtk_phy_set_bits(base + MIPITX_DSI_CON,
    RG_DSI_CKG_LDOOUT_EN | RG_DSI_LDOCORE_EN);
    mtk_phy_update_bits(base + MIPITX_DSI_PLL_PWR,
    RG_DSI_MPPLL_SDM_PWR_ON | RG_DSI_MPPLL_SDM_ISO_EN,
    RG_DSI_MPPLL_SDM_PWR_ON);
    mtk_phy_clear_bits(base + MIPITX_DSI_PLL_CON0, RG_DSI_MPPLL_PLL_EN);
    mtk_phy_update_bits(base + MIPITX_DSI_PLL_CON0,
    RG_DSI_MPPLL_TXDIV0 | RG_DSI_MPPLL_TXDIV1 |
    RG_DSI_MPPLL_PREDIV,
    FIELD_PREP(RG_DSI_MPPLL_TXDIV0, txdiv0) |
    FIELD_PREP(RG_DSI_MPPLL_TXDIV1, txdiv1));
//
// PLL PCW config
// PCW bit 24~30 = integer part of pcw
// PCW bit 0~23 = fractional part of pcw
// pcw = data_Rate*4*txdiv/(Ref_clk*2);
// Post DIV =4, so need data_Rate*4
// Ref_clk is 26MHz
//
    pcw = div_u64(((u64)mipi_tx.data_rate * 2 * txdiv) << 24, 26000000);
    writel(pcw, base + MIPITX_DSI_PLL_CON2);
    mtk_phy_set_bits(base + MIPITX_DSI_PLL_CON1, RG_DSI_MPPLL_SDM_FRA_EN);
    mtk_phy_set_bits(base + MIPITX_DSI_PLL_CON0, RG_DSI_MPPLL_PLL_EN);
    usleep_range(20, 100);
    mtk_phy_clear_bits(base + MIPITX_DSI_PLL_CON1, RG_DSI_MPPLL_SDM_SSC_EN);
    mtk_phy_update_field(base + MIPITX_DSI_PLL_TOP,
    RG_DSI_MPPLL_PRESERVE,
    mipi_tx.driver_data.mppll_preserve);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_pll_unprepare(hw: *mut clk_hw) {
    static void mtk_mipi_tx_pll_unprepare(struct clk_hw *hw)
    {
    struct mtk_mipi_tx *mipi_tx = mtk_mipi_tx_from_clk_hw(hw);
    void __iomem *base = mipi_tx.regs;
    dev_dbg(mipi_tx.dev, "unprepare\n");
    mtk_phy_clear_bits(base + MIPITX_DSI_PLL_CON0, RG_DSI_MPPLL_PLL_EN);
    mtk_phy_clear_bits(base + MIPITX_DSI_PLL_TOP, RG_DSI_MPPLL_PRESERVE);
    mtk_phy_update_bits(base + MIPITX_DSI_PLL_PWR,
    RG_DSI_MPPLL_SDM_ISO_EN | RG_DSI_MPPLL_SDM_PWR_ON,
    RG_DSI_MPPLL_SDM_ISO_EN);
    mtk_phy_clear_bits(base + MIPITX_DSI_TOP_CON, RG_DSI_LNT_HS_BIAS_EN);
    mtk_phy_clear_bits(base + MIPITX_DSI_CON,
    RG_DSI_CKG_LDOOUT_EN | RG_DSI_LDOCORE_EN);
    mtk_phy_clear_bits(base + MIPITX_DSI_BG_CON,
    RG_DSI_BG_CKEN | RG_DSI_BG_CORE_EN);
    mtk_phy_clear_bits(base + MIPITX_DSI_PLL_CON0, RG_DSI_MPPLL_DIV_MSK);
    }
    static int mtk_mipi_tx_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    req.rate = clamp_val(req.rate, 50000000, 1250000000);
    return 0;
    }
    static const struct clk_ops mtk_mipi_tx_pll_ops = {
    .prepare = mtk_mipi_tx_pll_prepare,
    .unprepare = mtk_mipi_tx_pll_unprepare,
    .determine_rate = mtk_mipi_tx_pll_determine_rate,
    .set_rate = mtk_mipi_tx_pll_set_rate,
    .recalc_rate = mtk_mipi_tx_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_power_on_signal(phy: *mut phy) {
    static void mtk_mipi_tx_power_on_signal(struct phy *phy)
    {
    struct mtk_mipi_tx *mipi_tx = phy_get_drvdata(phy);
    u32 reg;
    for (reg = MIPITX_DSI_CLOCK_LANE;
    reg <= MIPITX_DSI_DATA_LANE3; reg += 4)
    mtk_phy_set_bits(mipi_tx.regs + reg, RG_DSI_LNTx_LDOOUT_EN);
    mtk_phy_clear_bits(mipi_tx.regs + MIPITX_DSI_TOP_CON,
    RG_DSI_PAD_TIE_LOW_EN);
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_tx_power_off_signal(phy: *mut phy) {
    static void mtk_mipi_tx_power_off_signal(struct phy *phy)
    {
    struct mtk_mipi_tx *mipi_tx = phy_get_drvdata(phy);
    u32 reg;
    mtk_phy_set_bits(mipi_tx.regs + MIPITX_DSI_TOP_CON,
    RG_DSI_PAD_TIE_LOW_EN);
    for (reg = MIPITX_DSI_CLOCK_LANE;
    reg <= MIPITX_DSI_DATA_LANE3; reg += 4)
    mtk_phy_clear_bits(mipi_tx.regs + reg, RG_DSI_LNTx_LDOOUT_EN);
    }
    const struct mtk_mipitx_data mt2701_mipitx_data = {
    .mppll_preserve = 3,
    .mipi_tx_clk_ops = &mtk_mipi_tx_pll_ops,
    .mipi_tx_enable_signal = mtk_mipi_tx_power_on_signal,
    .mipi_tx_disable_signal = mtk_mipi_tx_power_off_signal,
    };
    const struct mtk_mipitx_data mt8173_mipitx_data = {
    .mppll_preserve = 0,
    .mipi_tx_clk_ops = &mtk_mipi_tx_pll_ops,
    .mipi_tx_enable_signal = mtk_mipi_tx_power_on_signal,
    .mipi_tx_disable_signal = mtk_mipi_tx_power_off_signal,
    };
