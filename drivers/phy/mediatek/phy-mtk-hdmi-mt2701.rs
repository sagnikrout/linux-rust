//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-hdmi-mt2701.c
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
// Copyright (c) 2018 MediaTek Inc.
// Author: Chunhui Dai <chunhui.dai@mediatek.com>
//

pub const HDMI_CON0: c_uint = 0x00;

pub const HDMI_CON1: c_uint = 0x04;

pub const HDMI_CON2: c_uint = 0x08;

pub const HDMI_CON4: c_uint = 0x10;

pub const HDMI_CON6: c_uint = 0x18;

pub const HDMI_CON7: c_uint = 0x1c;

#[no_mangle]
unsafe extern "C" fn mtk_hdmi_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int mtk_hdmi_pll_prepare(struct clk_hw *hw)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_set_bits(base + HDMI_CON7, RG_HTPLL_AUTOK_EN);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_RLH_EN);
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_POSDIV_MASK);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_EN_MBIAS);
    usleep_range(80, 100);
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_EN);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_EN_TX_CKLDO);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_SLDO_MASK);
    usleep_range(80, 100);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_MBIAS_LPF_EN);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_SER_MASK);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_PRED_MASK);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_DRV_MASK);
    usleep_range(80, 100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_pll_unprepare(hw: *mut clk_hw) {
    static void mtk_hdmi_pll_unprepare(struct clk_hw *hw)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_DRV_MASK);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_PRED_MASK);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_SER_MASK);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_MBIAS_LPF_EN);
    usleep_range(80, 100);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_SLDO_MASK);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_EN_TX_CKLDO);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_EN);
    usleep_range(80, 100);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_EN_MBIAS);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_POSDIV_MASK);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_RLH_EN);
    mtk_phy_clear_bits(base + HDMI_CON7, RG_HTPLL_AUTOK_EN);
    usleep_range(80, 100);
    }
    static int mtk_hdmi_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    void __iomem *base = hdmi_phy.regs;
    u32 pos_div;
    if (rate <= 64000000)
    pos_div = 3;
#[no_mangle]
pub unsafe extern "C" fn if(128000000: rate <=) -> else {
    else if (rate <= 128000000)
    pos_div = 2;
    else
    pos_div = 1;
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_PREDIV_MASK);
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_POSDIV_MASK);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_EN_TX_POSDIV);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_IC_MASK, 0x1);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_IR_MASK, 0x1);
    mtk_phy_update_field(base + HDMI_CON2, RG_HDMITX_TX_POSDIV_MASK, pos_div);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_FBKSEL_MASK, 1);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_FBKDIV_MASK, 19);
    mtk_phy_update_field(base + HDMI_CON7, RG_HTPLL_DIVEN_MASK, 0x2);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_BP_MASK, 0xc);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_BC_MASK, 0x2);
    mtk_phy_update_field(base + HDMI_CON6, RG_HTPLL_BR_MASK, 0x1);
    mtk_phy_clear_bits(base + HDMI_CON1, RG_HDMITX_PRED_IMP);
    mtk_phy_update_field(base + HDMI_CON1, RG_HDMITX_PRED_IBIAS_MASK, 0x3);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_IMP_MASK);
    mtk_phy_update_field(base + HDMI_CON1, RG_HDMITX_DRV_IMP_MASK, 0x28);
    mtk_phy_update_field(base + HDMI_CON4, RG_HDMITX_RESERVE_MASK, 0x28);
    mtk_phy_update_field(base + HDMI_CON0, RG_HDMITX_DRV_IBIAS_MASK, 0xa);
    return 0;
    }
    static unsigned long mtk_hdmi_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct mtk_hdmi_phy *hdmi_phy = to_mtk_hdmi_phy(hw);
    unsigned long out_rate, val;
    u32 tmp;
    tmp = readl(hdmi_phy.regs + HDMI_CON6);
    val = FIELD_GET(RG_HTPLL_PREDIV_MASK, tmp);
    switch (val) {
    case 0x00:
    out_rate = parent_rate;
    break;
    case 0x01:
    out_rate = parent_rate / 2;
    break;
    default:
    out_rate = parent_rate / 4;
    break;
    }
    val = FIELD_GET(RG_HTPLL_FBKDIV_MASK, tmp);
    out_rate *= (val + 1) * 2;
    tmp = readl(hdmi_phy.regs + HDMI_CON2);
    val = FIELD_GET(RG_HDMITX_TX_POSDIV_MASK, tmp);
    out_rate >>= val;
    if (tmp & RG_HDMITX_EN_TX_POSDIV)
    out_rate /= 5;
    return out_rate;
    }
    static const struct clk_ops mtk_hdmi_phy_pll_ops = {
    .prepare = mtk_hdmi_pll_prepare,
    .unprepare = mtk_hdmi_pll_unprepare,
    .set_rate = mtk_hdmi_pll_set_rate,
    .determine_rate = clk_determine_rate_noop,
    .recalc_rate = mtk_hdmi_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_phy_enable_tmds(hdmi_phy: *mut mtk_hdmi_phy) {
    static void mtk_hdmi_phy_enable_tmds(struct mtk_hdmi_phy *hdmi_phy)
    {
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_set_bits(base + HDMI_CON7, RG_HTPLL_AUTOK_EN);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_RLH_EN);
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_POSDIV_MASK);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_EN_MBIAS);
    usleep_range(80, 100);
    mtk_phy_set_bits(base + HDMI_CON6, RG_HTPLL_EN);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_EN_TX_CKLDO);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_SLDO_MASK);
    usleep_range(80, 100);
    mtk_phy_set_bits(base + HDMI_CON2, RG_HDMITX_MBIAS_LPF_EN);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_SER_MASK);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_PRED_MASK);
    mtk_phy_set_bits(base + HDMI_CON0, RG_HDMITX_EN_DRV_MASK);
    usleep_range(80, 100);
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_phy_disable_tmds(hdmi_phy: *mut mtk_hdmi_phy) {
    static void mtk_hdmi_phy_disable_tmds(struct mtk_hdmi_phy *hdmi_phy)
    {
    void __iomem *base = hdmi_phy.regs;
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_DRV_MASK);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_PRED_MASK);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_SER_MASK);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_MBIAS_LPF_EN);
    usleep_range(80, 100);
    mtk_phy_clear_bits(base + HDMI_CON0, RG_HDMITX_EN_SLDO_MASK);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_EN_TX_CKLDO);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_EN);
    usleep_range(80, 100);
    mtk_phy_clear_bits(base + HDMI_CON2, RG_HDMITX_EN_MBIAS);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_POSDIV_MASK);
    mtk_phy_clear_bits(base + HDMI_CON6, RG_HTPLL_RLH_EN);
    mtk_phy_clear_bits(base + HDMI_CON7, RG_HTPLL_AUTOK_EN);
    usleep_range(80, 100);
    }
    struct mtk_hdmi_phy_conf mtk_hdmi_phy_2701_conf = {
    .flags = CLK_SET_RATE_GATE,
    .pll_default_off = true,
    .hdmi_phy_clk_ops = &mtk_hdmi_phy_pll_ops,
    .hdmi_phy_enable_tmds = mtk_hdmi_phy_enable_tmds,
    .hdmi_phy_disable_tmds = mtk_hdmi_phy_disable_tmds,
    };
    MODULE_AUTHOR("Chunhui Dai <chunhui.dai@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek HDMI PHY Driver");
    MODULE_LICENSE("GPL v2");
