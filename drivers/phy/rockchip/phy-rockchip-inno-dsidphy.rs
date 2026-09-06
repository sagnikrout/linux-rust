//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-inno-dsidphy.c
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
// Copyright (c) 2018 Rockchip Electronics Co. Ltd.
//
// Author: Wyon Bi <bivvy.bi@rock-chips.com>
//

//
// The offset address[7:0] is distributed two parts, one from the bit7 to bit5
// is the first address, the other from the bit4 to bit0 is the second address.
// when you configure the registers, you must set both of them. The Clock Lane
// and Data Lane use the same registers with the same second address, but the
// first address is different.
//

    SECOND_ADDRESS(second))
// Analog Register Part: reg00

pub const BANDGAP_POWER_ON: c_int = 0;

// Analog Register Part: reg01

pub const REG_SYNCRST_NORMAL: c_int = 0;

pub const REG_LDOPD_POWER_ON: c_int = 0;

pub const REG_PLLPD_POWER_ON: c_int = 0;
// Analog Register Part: reg03

// Analog Register Part: reg04

// Analog Register Part: reg05

// Analog Register Part: reg06

// Analog Register Part: reg07

// Analog Register Part: reg08

pub const SAMPLE_CLOCK_DIRECTION_FORWARD: c_int = 0;

pub const PLL_OUTPUT_FREQUENCY_DIV_BY_1: c_int = 0;
pub const PLL_OUTPUT_FREQUENCY_DIV_BY_2: c_int = 1;
// Analog Register Part: reg0b

pub const VOD_MIN_RANGE: c_uint = 0x1;
pub const VOD_MID_RANGE: c_uint = 0x3;
pub const VOD_BIG_RANGE: c_uint = 0x7;
pub const VOD_MAX_RANGE: c_uint = 0xf;
// Analog Register Part: reg18

pub const LANE0_PRE_EMPHASIS_DISABLE: c_int = 0;

pub const LANE1_PRE_EMPHASIS_DISABLE: c_int = 0;
// Analog Register Part: reg19

// Analog Register Part: reg1E

pub const PLL_MODE_SEL_LVDS_MODE: c_int = 0;

// Analog Register Part: reg20

// Analog Register Part: reg21

pub const PRE_EMPHASIS_MIN_RANGE: c_uint = 0x0;
pub const PRE_EMPHASIS_MID_RANGE: c_uint = 0x1;
pub const PRE_EMPHASIS_MAX_RANGE: c_uint = 0x2;
pub const PRE_EMPHASIS_RESERVED_RANGE: c_uint = 0x3;
// Digital Register Part: reg00

pub const REG_DIG_RSTN_RESET: c_int = 0;
// Digital Register Part: reg01

pub const INVERT_TXCLKESC_DISABLE: c_int = 0;

pub const INVERT_TXBYTECLKHS_DISABLE: c_int = 0;
// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg05

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg06

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg07

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg08

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg09

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg0a

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg0c

pub const LPDT_TX_PPI_SYNC_DISABLE: c_int = 0;

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg0d

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg0e

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg10

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg11

// Clock/Data0/Data1/Data2/Data3 Lane Register Part: reg12

// LVDS Register Part: reg00

pub const LVDS_DIGITAL_INTERNAL_RESET_ENABLE: c_int = 0;
// LVDS Register Part: reg01

pub const LVDS_DIGITAL_INTERNAL_DISABLE: c_int = 0;
// LVDS Register Part: reg03

// LVDS Register Part: reg0b

pub const LVDS_PLL_POWER_ON: c_int = 0;

pub const LVDS_BANDGAP_POWER_ON: c_int = 0;
pub const DSI_PHY_RSTZ: c_uint = 0xa0;

pub const DSI_PHY_STATUS: c_uint = 0xb0;

    enum phy_max_rate {
    MAX_1GHZ,
    MAX_1_5GHZ,
    MAX_2_5GHZ,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_video_phy_plat_data {
    pub inno_mipi_dphy_timing_table: *const inno_mipi_dphy_timing,
    pub num_timings: c_uint,
    pub max_rate: enum phy_max_rate,
    pub max_lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_dsidphy {
    pub dev: *mut device,
    pub ref_clk: *mut clk,
    pub pclk_phy: *mut clk,
    pub pclk_host: *mut clk,
    pub pdata: *const inno_video_phy_plat_data,
    pub phy_base: *mut void __iomem,
    pub host_base: *mut void __iomem,
    pub rst: *mut reset_control,
    pub mode: enum phy_mode,
    pub dphy_cfg: phy_configure_opts_mipi_dphy,
    pub pll_clk: *mut clk,
    struct {
    pub hw: clk_hw,
    pub prediv: u8,
    pub fbdiv: u16,
    pub rate: c_ulong,
    pub pll: },
}

    enum {
    REGISTER_PART_ANALOG,
    REGISTER_PART_DIGITAL,
    REGISTER_PART_CLOCK_LANE,
    REGISTER_PART_DATA0_LANE,
    REGISTER_PART_DATA1_LANE,
    REGISTER_PART_DATA2_LANE,
    REGISTER_PART_DATA3_LANE,
    REGISTER_PART_LVDS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_mipi_dphy_timing {
    pub rate: c_ulong,
    pub lpx: u8,
    pub hs_prepare: u8,
    pub clk_lane_hs_zero: u8,
    pub data_lane_hs_zero: u8,
    pub hs_trail: u8,
}

    static const
    struct inno_mipi_dphy_timing inno_mipi_dphy_timing_table_max_1ghz[] = {
    { 110000000, 0x0, 0x20, 0x16, 0x02, 0x22},
    { 150000000, 0x0, 0x06, 0x16, 0x03, 0x45},
    { 200000000, 0x0, 0x18, 0x17, 0x04, 0x0b},
    { 250000000, 0x0, 0x05, 0x17, 0x05, 0x16},
    { 300000000, 0x0, 0x51, 0x18, 0x06, 0x2c},
    { 400000000, 0x0, 0x64, 0x19, 0x07, 0x33},
    { 500000000, 0x0, 0x20, 0x1b, 0x07, 0x4e},
    { 600000000, 0x0, 0x6a, 0x1d, 0x08, 0x3a},
    { 700000000, 0x0, 0x3e, 0x1e, 0x08, 0x6a},
    { 800000000, 0x0, 0x21, 0x1f, 0x09, 0x29},
    {1000000000, 0x0, 0x09, 0x20, 0x09, 0x27},
    };
    static const
    struct inno_mipi_dphy_timing inno_mipi_dphy_timing_table_max_1_5ghz[] = {
    { 110, 0x02, 0x7f, 0x16, 0x02, 0x02},
    { 150, 0x02, 0x7f, 0x16, 0x03, 0x02},
    { 200, 0x02, 0x7f, 0x17, 0x04, 0x02},
    { 250, 0x02, 0x7f, 0x17, 0x05, 0x04},
    { 300, 0x02, 0x7f, 0x18, 0x06, 0x04},
    { 400, 0x03, 0x7e, 0x19, 0x07, 0x04},
    { 500, 0x03, 0x7c, 0x1b, 0x07, 0x08},
    { 600, 0x03, 0x70, 0x1d, 0x08, 0x10},
    { 700, 0x05, 0x40, 0x1e, 0x08, 0x30},
    { 800, 0x05, 0x02, 0x1f, 0x09, 0x30},
    {1000, 0x05, 0x08, 0x20, 0x09, 0x30},
    {1200, 0x06, 0x03, 0x32, 0x14, 0x0f},
    {1400, 0x09, 0x03, 0x32, 0x14, 0x0f},
    {1500, 0x0d, 0x42, 0x36, 0x0e, 0x0f},
    };
    static const
    struct inno_mipi_dphy_timing inno_mipi_dphy_timing_table_max_2_5ghz[] = {
    { 110000000, 0x02, 0x7f, 0x16, 0x02, 0x02},
    { 150000000, 0x02, 0x7f, 0x16, 0x03, 0x02},
    { 200000000, 0x02, 0x7f, 0x17, 0x04, 0x02},
    { 250000000, 0x02, 0x7f, 0x17, 0x05, 0x04},
    { 300000000, 0x02, 0x7f, 0x18, 0x06, 0x04},
    { 400000000, 0x03, 0x7e, 0x19, 0x07, 0x04},
    { 500000000, 0x03, 0x7c, 0x1b, 0x07, 0x08},
    { 600000000, 0x03, 0x70, 0x1d, 0x08, 0x10},
    { 700000000, 0x05, 0x40, 0x1e, 0x08, 0x30},
    { 800000000, 0x05, 0x02, 0x1f, 0x09, 0x30},
    {1000000000, 0x05, 0x08, 0x20, 0x09, 0x30},
    {1200000000, 0x06, 0x03, 0x32, 0x14, 0x0f},
    {1400000000, 0x09, 0x03, 0x32, 0x14, 0x0f},
    {1600000000, 0x0d, 0x42, 0x36, 0x0e, 0x0f},
    {1800000000, 0x0e, 0x47, 0x7a, 0x0e, 0x0f},
    {2000000000, 0x11, 0x64, 0x7a, 0x0e, 0x0b},
    {2200000000, 0x13, 0x64, 0x7e, 0x15, 0x0b},
    {2400000000, 0x13, 0x33, 0x7f, 0x15, 0x6a},
    {2500000000, 0x15, 0x54, 0x7f, 0x15, 0x6a},
    };
    static void phy_update_bits(struct inno_dsidphy *inno,
    u8 first, u8 second, u8 mask, u8 val)
    {
    let mut reg: u32 = PHY_REG(first, second) << 2;
    unsigned int tmp, orig;
    orig = readl(inno.phy_base + reg);
    tmp = orig & ~mask;
    tmp |= val & mask;
    writel(tmp, inno.phy_base + reg);
    }
    static unsigned long inno_dsidphy_pll_calc_rate(struct inno_dsidphy *inno,
    unsigned long rate)
    {
    let mut prate: c_ulong = clk_get_rate(inno.ref_clk);
    let mut best_freq: c_ulong = 0;
    unsigned long fref, fout;
    u8 min_prediv, max_prediv;
    u8 _prediv, best_prediv = 1;
    u16 _fbdiv, best_fbdiv = 1;
    let mut min_delta: u32 = UINT_MAX;
//
// The PLL output frequency can be calculated using a simple formula:
// PLL_Output_Frequency = (FREF / PREDIV * FBDIV) / 2
// PLL_Output_Frequency: it is equal to DDR-Clock-Frequency * 2
//
    fref = prate / 2;
    if (rate > 1000000000UL)
    fout = 1000000000UL;
    else
    fout = rate;
// 5Mhz < Fref / prediv < 40MHz
    min_prediv = DIV_ROUND_UP(fref, 40000000);
    max_prediv = fref / 5000000;
    for (_prediv = min_prediv; _prediv <= max_prediv; _prediv++) {
    u64 tmp;
    u32 delta;
    tmp = (u64)fout * _prediv;
    do_div(tmp, fref);
    _fbdiv = tmp;
//
// The possible settings of feedback divider are
// 12, 13, 14, 16, ~ 511
//
    if (_fbdiv == 15)
    continue;
    if (_fbdiv < 12 || _fbdiv > 511)
    continue;
    tmp = (u64)_fbdiv * fref;
    do_div(tmp, _prediv);
    delta = abs(fout - tmp);
    if (!delta) {
    best_prediv = _prediv;
    best_fbdiv = _fbdiv;
    best_freq = tmp;
    break;
    } else if (delta < min_delta) {
    best_prediv = _prediv;
    best_fbdiv = _fbdiv;
    best_freq = tmp;
    min_delta = delta;
    }
    }
    if (best_freq) {
    inno.pll.prediv = best_prediv;
    inno.pll.fbdiv = best_fbdiv;
    inno.pll.rate = best_freq;
    }
    return best_freq;
    }
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_mipi_mode_enable(inno: *mut inno_dsidphy) {
    static void inno_dsidphy_mipi_mode_enable(struct inno_dsidphy *inno)
    {
    struct phy_configure_opts_mipi_dphy *cfg = &inno.dphy_cfg;
    const struct inno_mipi_dphy_timing *timings;
    u32 t_txbyteclkhs, t_txclkesc;
    u32 txbyteclkhs, txclkesc, esc_clk_div;
    u32 hs_exit, clk_post, clk_pre, wakeup, lpx, ta_go, ta_sure, ta_wait;
    u32 hs_prepare, hs_trail, hs_zero, clk_lane_hs_zero, data_lane_hs_zero;
    unsigned int i;
    u32 val;
    timings = inno.pdata.inno_mipi_dphy_timing_table;
    inno_dsidphy_pll_calc_rate(inno, cfg.hs_clk_rate);
// Select MIPI mode
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x03,
    MODE_ENABLE_MASK, MIPI_MODE_ENABLE);
// Configure PLL
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x03,
    REG_PREDIV_MASK, REG_PREDIV(inno.pll.prediv));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x03,
    REG_FBDIV_HI_MASK, REG_FBDIV_HI(inno.pll.fbdiv));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x04,
    REG_FBDIV_LO_MASK, REG_FBDIV_LO(inno.pll.fbdiv));
    if (inno.pdata.max_rate == MAX_2_5GHZ) {
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x08,
    PLL_POST_DIV_ENABLE_MASK, PLL_POST_DIV_ENABLE);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x0b,
    CLOCK_LANE_VOD_RANGE_SET_MASK,
    CLOCK_LANE_VOD_RANGE_SET(VOD_MAX_RANGE));
    } else if (inno.pdata.max_rate == MAX_1_5GHZ) {
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x18,
    LANE0_PRE_EMPHASIS_ENABLE_MASK, LANE0_PRE_EMPHASIS_ENABLE);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x18,
    LANE1_PRE_EMPHASIS_ENABLE_MASK, LANE1_PRE_EMPHASIS_ENABLE);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x19,
    PRE_EMPHASIS_RANGE_SET_MASK,
    PRE_EMPHASIS_RANGE_SET(PRE_EMPHASIS_MID_RANGE));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x1a,
    LANE0_PRE_EMPHASIS_RANGE_SET_MASK,
    LANE0_PRE_EMPHASIS_RANGE_SET(PRE_EMPHASIS_MID_RANGE));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x1b,
    LANE1_PRE_EMPHASIS_RANGE_SET_MASK,
    LANE1_PRE_EMPHASIS_RANGE_SET(PRE_EMPHASIS_MID_RANGE));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x0b,
    CLOCK_LANE_VOD_RANGE_SET_MASK,
    CLOCK_LANE_VOD_RANGE_SET(VOD_MAX_RANGE));
    }
// Enable PLL and LDO
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x01,
    REG_LDOPD_MASK | REG_PLLPD_MASK,
    REG_LDOPD_POWER_ON | REG_PLLPD_POWER_ON);
// Reset analog
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x01,
    REG_SYNCRST_MASK, REG_SYNCRST_RESET);
    udelay(1);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x01,
    REG_SYNCRST_MASK, REG_SYNCRST_NORMAL);
// Reset digital
    phy_update_bits(inno, REGISTER_PART_DIGITAL, 0x00,
    REG_DIG_RSTN_MASK, REG_DIG_RSTN_RESET);
    udelay(1);
    phy_update_bits(inno, REGISTER_PART_DIGITAL, 0x00,
    REG_DIG_RSTN_MASK, REG_DIG_RSTN_NORMAL);
    txbyteclkhs = inno.pll.rate / 8;
    t_txbyteclkhs = div_u64(PSEC_PER_SEC, txbyteclkhs);
    esc_clk_div = DIV_ROUND_UP(txbyteclkhs, 20000000);
    txclkesc = txbyteclkhs / esc_clk_div;
    t_txclkesc = div_u64(PSEC_PER_SEC, txclkesc);
//
// The value of counter for HS Ths-exit
// Ths-exit = Tpin_txbyteclkhs * value
//
    hs_exit = DIV_ROUND_UP(cfg.hs_exit, t_txbyteclkhs);
//
// The value of counter for HS Tclk-post
// Tclk-post = Tpin_txbyteclkhs * value
//
    clk_post = DIV_ROUND_UP(cfg.clk_post, t_txbyteclkhs);
//
// The value of counter for HS Tclk-pre
// Tclk-pre = Tpin_txbyteclkhs * value
//
    clk_pre = DIV_ROUND_UP(cfg.clk_pre, BITS_PER_BYTE);
//
// The value of counter for HS Tta-go
// Tta-go for turnaround
// Tta-go = Ttxclkesc * value
//
    ta_go = DIV_ROUND_UP(cfg.ta_go, t_txclkesc);
//
// The value of counter for HS Tta-sure
// Tta-sure for turnaround
// Tta-sure = Ttxclkesc * value
//
    ta_sure = DIV_ROUND_UP(cfg.ta_sure, t_txclkesc);
//
// The value of counter for HS Tta-wait
// Tta-wait for turnaround
// Tta-wait = Ttxclkesc * value
//
    ta_wait = DIV_ROUND_UP(cfg.ta_get, t_txclkesc);
    for (i = 0; i < inno.pdata.num_timings; i++)
    if (inno.pll.rate <= timings[i].rate)
    break;
    if (i == inno.pdata.num_timings)
    --i;
//
// The value of counter for HS Tlpx Time
// Tlpx = Tpin_txbyteclkhs * (2 + value)
//
    if (inno.pdata.max_rate == MAX_1GHZ) {
    lpx = DIV_ROUND_UP(cfg.lpx, t_txbyteclkhs);
    if (lpx >= 2)
    lpx -= 2;
    } else
    lpx = timings[i].lpx;
    hs_prepare = timings[i].hs_prepare;
    hs_trail = timings[i].hs_trail;
    clk_lane_hs_zero = timings[i].clk_lane_hs_zero;
    data_lane_hs_zero = timings[i].data_lane_hs_zero;
    wakeup = 0x3ff;
    for (i = REGISTER_PART_CLOCK_LANE; i <= REGISTER_PART_DATA3_LANE; i++) {
    if (i == REGISTER_PART_CLOCK_LANE)
    hs_zero = clk_lane_hs_zero;
    else
    hs_zero = data_lane_hs_zero;
    phy_update_bits(inno, i, 0x05, T_LPX_CNT_MASK,
    T_LPX_CNT(lpx));
    phy_update_bits(inno, i, 0x06, T_HS_PREPARE_CNT_MASK,
    T_HS_PREPARE_CNT(hs_prepare));
    if (inno.pdata.max_rate == MAX_2_5GHZ)
    phy_update_bits(inno, i, 0x06, T_HS_ZERO_CNT_HI_MASK,
    T_HS_ZERO_CNT_HI(hs_zero >> 6));
    phy_update_bits(inno, i, 0x07, T_HS_ZERO_CNT_LO_MASK,
    T_HS_ZERO_CNT_LO(hs_zero));
    phy_update_bits(inno, i, 0x08, T_HS_TRAIL_CNT_MASK,
    T_HS_TRAIL_CNT(hs_trail));
    if (inno.pdata.max_rate == MAX_2_5GHZ)
    phy_update_bits(inno, i, 0x11, T_HS_EXIT_CNT_HI_MASK,
    T_HS_EXIT_CNT_HI(hs_exit >> 5));
    phy_update_bits(inno, i, 0x09, T_HS_EXIT_CNT_LO_MASK,
    T_HS_EXIT_CNT_LO(hs_exit));
    if (inno.pdata.max_rate == MAX_2_5GHZ)
    phy_update_bits(inno, i, 0x10, T_CLK_POST_CNT_HI_MASK,
    T_CLK_POST_CNT_HI(clk_post >> 4));
    phy_update_bits(inno, i, 0x0a, T_CLK_POST_CNT_LO_MASK,
    T_CLK_POST_CNT_LO(clk_post));
    phy_update_bits(inno, i, 0x0e, T_CLK_PRE_CNT_MASK,
    T_CLK_PRE_CNT(clk_pre));
    phy_update_bits(inno, i, 0x0c, T_WAKEUP_CNT_HI_MASK,
    T_WAKEUP_CNT_HI(wakeup >> 8));
    phy_update_bits(inno, i, 0x0d, T_WAKEUP_CNT_LO_MASK,
    T_WAKEUP_CNT_LO(wakeup));
    phy_update_bits(inno, i, 0x10, T_TA_GO_CNT_MASK,
    T_TA_GO_CNT(ta_go));
    phy_update_bits(inno, i, 0x11, T_TA_SURE_CNT_MASK,
    T_TA_SURE_CNT(ta_sure));
    phy_update_bits(inno, i, 0x12, T_TA_WAIT_CNT_MASK,
    T_TA_WAIT_CNT(ta_wait));
    }
// Enable lanes on analog part
    switch (inno.pdata.max_lanes) {
    case 1:
    val =  LANE_EN_0;
    break;
    case 2:
    val =  LANE_EN_0 | LANE_EN_1;
    break;
    case 3:
    val = LANE_EN_0 | LANE_EN_1 | LANE_EN_2;
    break;
    case 4:
    default:
    val = LANE_EN_0 | LANE_EN_1 | LANE_EN_2 | LANE_EN_3;
    break;
    }
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00,
    LANE_EN_MASK, LANE_EN_CK | val);
    }
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_lvds_mode_enable(inno: *mut inno_dsidphy) {
    static void inno_dsidphy_lvds_mode_enable(struct inno_dsidphy *inno)
    {
    let mut prediv: u8 = 2;
    let mut fbdiv: u16 = 28;
// Sample clock reverse direction
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x08,
    SAMPLE_CLOCK_DIRECTION_MASK | LOWFRE_EN_MASK,
    SAMPLE_CLOCK_DIRECTION_REVERSE |
    PLL_OUTPUT_FREQUENCY_DIV_BY_1);
// Select LVDS mode
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x03,
    MODE_ENABLE_MASK, LVDS_MODE_ENABLE);
// Configure PLL
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x03,
    REG_PREDIV_MASK, REG_PREDIV(prediv));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x03,
    REG_FBDIV_HI_MASK, REG_FBDIV_HI(fbdiv));
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x04,
    REG_FBDIV_LO_MASK, REG_FBDIV_LO(fbdiv));
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x08, 0xff, 0xfc);
// Enable PLL and Bandgap
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x0b,
    LVDS_PLL_POWER_MASK | LVDS_BANDGAP_POWER_MASK,
    LVDS_PLL_POWER_ON | LVDS_BANDGAP_POWER_ON);
    msleep(20);
// Select PLL mode
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x1e,
    PLL_MODE_SEL_MASK, PLL_MODE_SEL_LVDS_MODE);
// Reset LVDS digital logic
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x00,
    LVDS_DIGITAL_INTERNAL_RESET_MASK,
    LVDS_DIGITAL_INTERNAL_RESET_ENABLE);
    udelay(1);
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x00,
    LVDS_DIGITAL_INTERNAL_RESET_MASK,
    LVDS_DIGITAL_INTERNAL_RESET_DISABLE);
// Enable LVDS digital logic
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x01,
    LVDS_DIGITAL_INTERNAL_ENABLE_MASK,
    LVDS_DIGITAL_INTERNAL_ENABLE);
// Enable LVDS analog driver
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x0b,
    LVDS_LANE_EN_MASK, LVDS_CLK_LANE_EN |
    LVDS_DATA_LANE0_EN | LVDS_DATA_LANE1_EN |
    LVDS_DATA_LANE2_EN | LVDS_DATA_LANE3_EN);
    }
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_power_on(phy: *mut phy) -> c_int {
    static int inno_dsidphy_power_on(struct phy *phy)
    {
    struct inno_dsidphy *inno = phy_get_drvdata(phy);
    clk_prepare_enable(inno.pclk_phy);
    clk_prepare_enable(inno.ref_clk);
    pm_runtime_get_sync(inno.dev);
// Bandgap power on
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00,
    BANDGAP_POWER_MASK, BANDGAP_POWER_ON);
// Enable power work
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00,
    POWER_WORK_MASK, POWER_WORK_ENABLE);
    switch (inno.mode) {
    case PHY_MODE_MIPI_DPHY:
    inno_dsidphy_mipi_mode_enable(inno);
    break;
    case PHY_MODE_LVDS:
    inno_dsidphy_lvds_mode_enable(inno);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_power_off(phy: *mut phy) -> c_int {
    static int inno_dsidphy_power_off(struct phy *phy)
    {
    struct inno_dsidphy *inno = phy_get_drvdata(phy);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00, LANE_EN_MASK, 0);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x01,
    REG_LDOPD_MASK | REG_PLLPD_MASK,
    REG_LDOPD_POWER_DOWN | REG_PLLPD_POWER_DOWN);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00,
    POWER_WORK_MASK, POWER_WORK_DISABLE);
    phy_update_bits(inno, REGISTER_PART_ANALOG, 0x00,
    BANDGAP_POWER_MASK, BANDGAP_POWER_DOWN);
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x0b, LVDS_LANE_EN_MASK, 0);
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x01,
    LVDS_DIGITAL_INTERNAL_ENABLE_MASK,
    LVDS_DIGITAL_INTERNAL_DISABLE);
    phy_update_bits(inno, REGISTER_PART_LVDS, 0x0b,
    LVDS_PLL_POWER_MASK | LVDS_BANDGAP_POWER_MASK,
    LVDS_PLL_POWER_OFF | LVDS_BANDGAP_POWER_DOWN);
    pm_runtime_put(inno.dev);
    clk_disable_unprepare(inno.ref_clk);
    clk_disable_unprepare(inno.pclk_phy);
    return 0;
    }
    static int inno_dsidphy_set_mode(struct phy *phy, enum phy_mode mode,
    int submode)
    {
    struct inno_dsidphy *inno = phy_get_drvdata(phy);
    switch (mode) {
    case PHY_MODE_MIPI_DPHY:
    case PHY_MODE_LVDS:
    inno.mode = mode;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int inno_dsidphy_configure(struct phy *phy,
    union phy_configure_opts *opts)
    {
    struct inno_dsidphy *inno = phy_get_drvdata(phy);
    int ret;
    if (inno.mode != PHY_MODE_MIPI_DPHY)
    return -EINVAL;
    ret = phy_mipi_dphy_config_validate(&opts.mipi_dphy);
    if (ret)
    return ret;
    memcpy(&inno.dphy_cfg, &opts.mipi_dphy, sizeof(inno.dphy_cfg));
    return 0;
    }
    static const struct phy_ops inno_dsidphy_ops = {
    .configure = inno_dsidphy_configure,
    .set_mode = inno_dsidphy_set_mode,
    .power_on = inno_dsidphy_power_on,
    .power_off = inno_dsidphy_power_off,
    .owner = THIS_MODULE,
    };
    static const struct inno_video_phy_plat_data max_1ghz_video_phy_plat_data = {
    .inno_mipi_dphy_timing_table = inno_mipi_dphy_timing_table_max_1ghz,
    .num_timings = ARRAY_SIZE(inno_mipi_dphy_timing_table_max_1ghz),
    .max_rate = MAX_1GHZ,
    .max_lanes = 4,
    };
    static const struct inno_video_phy_plat_data max_1_5ghz_video_phy_plat_data = {
    .inno_mipi_dphy_timing_table = inno_mipi_dphy_timing_table_max_1_5ghz,
    .num_timings = ARRAY_SIZE(inno_mipi_dphy_timing_table_max_1_5ghz),
    .max_rate = MAX_1_5GHZ,
    .max_lanes = 2,
    };
    static const struct inno_video_phy_plat_data max_2_5ghz_video_phy_plat_data = {
    .inno_mipi_dphy_timing_table = inno_mipi_dphy_timing_table_max_2_5ghz,
    .num_timings = ARRAY_SIZE(inno_mipi_dphy_timing_table_max_2_5ghz),
    .max_rate = MAX_2_5GHZ,
    .max_lanes = 4,
    };
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_probe(pdev: *mut platform_device) -> c_int {
    static int inno_dsidphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct inno_dsidphy *inno;
    struct phy_provider *phy_provider;
    struct phy *phy;
    int ret;
    inno = devm_kzalloc(dev, sizeof(*inno), GFP_KERNEL);
    if (!inno)
    return -ENOMEM;
    inno.dev = dev;
    inno.pdata = of_device_get_match_data(inno.dev);
    platform_set_drvdata(pdev, inno);
    inno.phy_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(inno.phy_base))
    return PTR_ERR(inno.phy_base);
    inno.ref_clk = devm_clk_get(dev, "ref");
    if (IS_ERR(inno.ref_clk)) {
    ret = PTR_ERR(inno.ref_clk);
    dev_err(dev, "failed to get ref clock: %d\n", ret);
    return ret;
    }
    inno.pclk_phy = devm_clk_get(dev, "pclk");
    if (IS_ERR(inno.pclk_phy)) {
    ret = PTR_ERR(inno.pclk_phy);
    dev_err(dev, "failed to get phy pclk: %d\n", ret);
    return ret;
    }
    inno.rst = devm_reset_control_get(dev, "apb");
    if (IS_ERR(inno.rst)) {
    ret = PTR_ERR(inno.rst);
    dev_err(dev, "failed to get system reset control: %d\n", ret);
    return ret;
    }
    phy = devm_phy_create(dev, core::ptr::null_mut(), &inno_dsidphy_ops);
    if (IS_ERR(phy)) {
    ret = PTR_ERR(phy);
    dev_err(dev, "failed to create phy: %d\n", ret);
    return ret;
    }
    phy_set_drvdata(phy, inno);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    ret = PTR_ERR(phy_provider);
    dev_err(dev, "failed to register phy provider: %d\n", ret);
    return ret;
    }
    pm_runtime_enable(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inno_dsidphy_remove(pdev: *mut platform_device) {
    static void inno_dsidphy_remove(struct platform_device *pdev)
    {
    struct inno_dsidphy *inno = platform_get_drvdata(pdev);
    pm_runtime_disable(inno.dev);
    }
    static const struct of_device_id inno_dsidphy_of_match[] = {
    {
    .compatible = "rockchip,px30-dsi-dphy",
    .data = &max_1ghz_video_phy_plat_data,
    }, {
    .compatible = "rockchip,rk3128-dsi-dphy",
    .data = &max_1ghz_video_phy_plat_data,
    }, {
    .compatible = "rockchip,rk3368-dsi-dphy",
    .data = &max_1ghz_video_phy_plat_data,
    }, {
    .compatible = "rockchip,rk3506-dsi-dphy",
    .data = &max_1_5ghz_video_phy_plat_data,
    }, {
    .compatible = "rockchip,rk3568-dsi-dphy",
    .data = &max_2_5ghz_video_phy_plat_data,
    }, {
    .compatible = "rockchip,rv1126-dsi-dphy",
    .data = &max_2_5ghz_video_phy_plat_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, inno_dsidphy_of_match);
    static struct platform_driver inno_dsidphy_driver = {
    .driver = {
    .name = "inno-dsidphy",
    .of_match_table	= of_match_ptr(inno_dsidphy_of_match),
    },
    .probe = inno_dsidphy_probe,
    .remove = inno_dsidphy_remove,
    };
    module_platform_driver(inno_dsidphy_driver);
    MODULE_AUTHOR("Wyon Bi <bivvy.bi@rock-chips.com>");
    MODULE_DESCRIPTION("Innosilicon MIPI/LVDS/TTL Video Combo PHY driver");
    MODULE_LICENSE("GPL v2");
