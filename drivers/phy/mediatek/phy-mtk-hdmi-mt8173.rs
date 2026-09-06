//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-hdmi-mt8173.c
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Jie Qiu <jie.qiu@mediatek.com>
//

pub const HDMI_CON0: c_uint = 0x00;

pub const HDMI_CON1: c_uint = 0x04;

pub const HDMI_CON2: c_uint = 0x08;

pub const HDMI_CON3: c_uint = 0x0c;

pub const HDMI_CON4: c_uint = 0x10;

pub const HDMI_CON5: c_uint = 0x14;

pub const HDMI_CON6: c_uint = 0x18;

pub const HDMI_CON7: c_uint = 0x1c;

pub const HDMI_CON8: c_uint = 0x20;

#[no_mangle]
unsafe extern "C" fn mtk_hdmi_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int mtk_hdmi_pll_prepare(struct clk_hw *hw)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_set_bits(base + HDMI_CON1, RG_HDMITX_PLL_AUTOK_EN);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_PLL_POSDIV);
    mtk_phy_clear_bits(base + HDMI_CON3, RG_HDMITX_MHLCK_EN);
    mtk_phy_set_bits(base + HDMI_CON1, RG_HDMITX_PLL_BIAS_EN);
    usleep_range(100, 150);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_PLL_EN);
    usleep_range(100, 150);
    mtk_phy_set_bits(base + HDMI_CON1, RG_HDMITX_PLL_BIAS_LPF_EN);
    mtk_phy_set_bits(base + HDMI_CON1, RG_HDMITX_PLL_TXDIV_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_pll_unprepare(hw: *mut clk_hw) {
    static void mtk_hdmi_pll_unprepare(struct clk_hw *hw)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_clear_bits(base + HDMI_CON1, RG_HDMITX_PLL_TXDIV_EN);
    mtk_phy_clear_bits(base + HDMI_CON1, RG_HDMITX_PLL_BIAS_LPF_EN);
    usleep_range(100, 150);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_PLL_EN);
    usleep_range(100, 150);
    mtk_phy_clear_bits(base + HDMI_CON1, RG_HDMITX_PLL_BIAS_EN);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_PLL_POSDIV);
    mtk_phy_clear_bits(base + HDMI_CON1, RG_HDMITX_PLL_AUTOK_EN);
    usleep_range(100, 150);
    }
    static int mtk_hdmi_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    hdmi_phy.pll_rate = req.rate;
    if (req.rate <= 74250000)
    req.best_parent_rate = req.rate;
    else
    req.best_parent_rate = req.rate / 2;
    return 0;
    }
    static int mtk_hdmi_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    unsigned int pre_div;
    unsigned int div;
    unsigned int pre_ibias;
    unsigned int hdmi_ibias;
    unsigned int imp_en;
    dev_dbg(hdmi_phy.dev, "%s: %lu Hz, parent: %lu Hz\n", __func__,
    rate, parent_rate);
    if (rate <= 27000000) {
    pre_div = 0;
    div = 3;
    } else if (rate <= 74250000) {
    pre_div = 1;
    div = 2;
    } else {
    pre_div = 1;
    div = 1;
    }
    mtk_phy_update_field(base + HDMI_CON0, RG_HDMITX_PLL_PREDIV, pre_div);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_PLL_POSDIV);
    mtk_phy_update_bits(base + HDMI_CON0,
    RG_HDMITX_PLL_IC | RG_HDMITX_PLL_IR,
    FIELD_PREP(RG_HDMITX_PLL_IC, 0x1) |
    FIELD_PREP(RG_HDMITX_PLL_IR, 0x1));
    mtk_phy_update_field(base + HDMI_CON1, RG_HDMITX_PLL_TXDIV, div);
    mtk_phy_update_bits(base + HDMI_CON0,
    RG_HDMITX_PLL_FBKSEL | RG_HDMITX_PLL_FBKDIV,
    FIELD_PREP(RG_HDMITX_PLL_FBKSEL, 0x1) |
    FIELD_PREP(RG_HDMITX_PLL_FBKDIV, 19));
    mtk_phy_update_field(base + HDMI_CON1, RG_HDMITX_PLL_DIVEN, 0x2);
    mtk_phy_update_bits(base + HDMI_CON0,
    RG_HDMITX_PLL_BP | RG_HDMITX_PLL_BC |
    RG_HDMITX_PLL_BR,
    FIELD_PREP(RG_HDMITX_PLL_BP, 0xc) |
    FIELD_PREP(RG_HDMITX_PLL_BC, 0x2) |
    FIELD_PREP(RG_HDMITX_PLL_BR, 0x1));
    if (rate < 165000000) {
    mtk_phy_clear_bits(base + HDMI_CON3, RG_HDMITX_PRD_IMP_EN);
    pre_ibias = 0x3;
    imp_en = 0x0;
    hdmi_ibias = hdmi_phy.ibias;
    } else {
    mtk_phy_set_bits(base + HDMI_CON3, RG_HDMITX_PRD_IMP_EN);
    pre_ibias = 0x6;
    imp_en = 0xf;
    hdmi_ibias = hdmi_phy.ibias_up;
    }
    mtk_phy_update_bits(base + HDMI_CON4,
    RG_HDMITX_PRD_IBIAS_CLK | RG_HDMITX_PRD_IBIAS_D2 |
    RG_HDMITX_PRD_IBIAS_D1 | RG_HDMITX_PRD_IBIAS_D0,
    FIELD_PREP(RG_HDMITX_PRD_IBIAS_CLK, pre_ibias) |
    FIELD_PREP(RG_HDMITX_PRD_IBIAS_D2, pre_ibias) |
    FIELD_PREP(RG_HDMITX_PRD_IBIAS_D1, pre_ibias) |
    FIELD_PREP(RG_HDMITX_PRD_IBIAS_D0, pre_ibias));
    mtk_phy_update_field(base + HDMI_CON3, RG_HDMITX_DRV_IMP_EN, imp_en);
    mtk_phy_update_bits(base + HDMI_CON6,
    RG_HDMITX_DRV_IMP_CLK | RG_HDMITX_DRV_IMP_D2 |
    RG_HDMITX_DRV_IMP_D1 | RG_HDMITX_DRV_IMP_D0,
    FIELD_PREP(RG_HDMITX_DRV_IMP_CLK, hdmi_phy.drv_imp_clk) |
    FIELD_PREP(RG_HDMITX_DRV_IMP_D2, hdmi_phy.drv_imp_d2) |
    FIELD_PREP(RG_HDMITX_DRV_IMP_D1, hdmi_phy.drv_imp_d1) |
    FIELD_PREP(RG_HDMITX_DRV_IMP_D0, hdmi_phy.drv_imp_d0));
    mtk_phy_update_bits(base + HDMI_CON5,
    RG_HDMITX_DRV_IBIAS_CLK | RG_HDMITX_DRV_IBIAS_D2 |
    RG_HDMITX_DRV_IBIAS_D1 | RG_HDMITX_DRV_IBIAS_D0,
    FIELD_PREP(RG_HDMITX_DRV_IBIAS_CLK, hdmi_ibias) |
    FIELD_PREP(RG_HDMITX_DRV_IBIAS_D2, hdmi_ibias) |
    FIELD_PREP(RG_HDMITX_DRV_IBIAS_D1, hdmi_ibias) |
    FIELD_PREP(RG_HDMITX_DRV_IBIAS_D0, hdmi_ibias));
    return 0;
    }
    static unsigned long mtk_hdmi_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    return hdmi_phy.pll_rate;
    }
    static const struct clk_ops mtk_hdmi_phy_pll_ops = {
    .prepare = mtk_hdmi_pll_prepare,
    .unprepare = mtk_hdmi_pll_unprepare,
    .set_rate = mtk_hdmi_pll_set_rate,
    .determine_rate = mtk_hdmi_pll_determine_rate,
    .recalc_rate = mtk_hdmi_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_phy_enable_tmds(hdmi_phy: *mut mtk_hdmi_phy) {
    static void mtk_hdmi_phy_enable_tmds(struct mtk_hdmi_phy *hdmi_phy)
    {
    mtk_phy_set_bits(hdmi_phy.regs + HDMI_CON3,
    RG_HDMITX_SER_EN | RG_HDMITX_PRD_EN |
    RG_HDMITX_DRV_EN);
    usleep_range(100, 150);
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_phy_disable_tmds(hdmi_phy: *mut mtk_hdmi_phy) {
    static void mtk_hdmi_phy_disable_tmds(struct mtk_hdmi_phy *hdmi_phy)
    {
    mtk_phy_clear_bits(hdmi_phy.regs + HDMI_CON3,
    RG_HDMITX_DRV_EN | RG_HDMITX_PRD_EN |
    RG_HDMITX_SER_EN);
    }
    struct mtk_hdmi_phy_conf mtk_hdmi_phy_8173_conf = {
    .flags = CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE,
    .hdmi_phy_clk_ops = &mtk_hdmi_phy_pll_ops,
    .hdmi_phy_enable_tmds = mtk_hdmi_phy_enable_tmds,
    .hdmi_phy_disable_tmds = mtk_hdmi_phy_disable_tmds,
    };
    MODULE_AUTHOR("Jie Qiu <jie.qiu@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek MT8173 HDMI PHY Driver");
    MODULE_LICENSE("GPL v2");
