//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/vc4_hdmi_phy.c
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
// Copyright (C) 2015 Broadcom
// Copyright (c) 2014 The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_2_PREEMP_SHIFT: c_int = 29;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_2_MAINDRV_SHIFT: c_int = 24;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_1_PREEMP_SHIFT: c_int = 21;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_1_MAINDRV_SHIFT: c_int = 16;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_0_PREEMP_SHIFT: c_int = 13;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_0_MAINDRV_SHIFT: c_int = 8;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_CK_PREEMP_SHIFT: c_int = 5;

pub const VC4_HDMI_TX_PHY_CTL_0_PREEMP_CK_MAINDRV_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA2_SHIFT: c_int = 15;

pub const VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA1_SHIFT: c_int = 10;

pub const VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA0_SHIFT: c_int = 5;

pub const VC4_HDMI_TX_PHY_CTL_1_RES_SEL_CK_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_CTL_2_VCO_GAIN_SHIFT: c_int = 16;

pub const VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA2_SHIFT: c_int = 12;

pub const VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA1_SHIFT: c_int = 8;

pub const VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA0_SHIFT: c_int = 4;

pub const VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELCK_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_CTL_3_RP_SHIFT: c_int = 17;

pub const VC4_HDMI_TX_PHY_CTL_3_RZ_SHIFT: c_int = 12;

pub const VC4_HDMI_TX_PHY_CTL_3_CP1_SHIFT: c_int = 10;

pub const VC4_HDMI_TX_PHY_CTL_3_CP_SHIFT: c_int = 8;

pub const VC4_HDMI_TX_PHY_CTL_3_CZ_SHIFT: c_int = 6;

pub const VC4_HDMI_TX_PHY_CTL_3_ICP_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_PLL_CTL_0_VCO_SEL_SHIFT: c_int = 9;

pub const VC4_HDMI_TX_PHY_PLL_CTL_1_CPP_SHIFT: c_int = 16;

pub const VC4_HDMI_TX_PHY_PLL_CTL_1_FREQ_DOUBLER_DELAY_SHIFT: c_int = 14;

pub const VC4_HDMI_TX_PHY_PLL_CTL_1_POST_RST_SEL_SHIFT: c_int = 11;

pub const VC4_HDMI_TX_PHY_CLK_DIV_VCO_SHIFT: c_int = 8;

pub const VC4_HDMI_TX_PHY_PLL_CFG_PDIV_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_CHANNEL_SWAP_TXCK_OUT_SEL_SHIFT: c_int = 12;

pub const VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX2_OUT_SEL_SHIFT: c_int = 8;

pub const VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX1_OUT_SEL_SHIFT: c_int = 4;

pub const VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX0_OUT_SEL_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1_MIN_LIMIT_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2_MAX_LIMIT_SHIFT: c_int = 0;

pub const VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4_STABLE_THRESHOLD_SHIFT: c_int = 16;

pub const VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4_HOLD_THRESHOLD_SHIFT: c_int = 0;

pub const VC4_HDMI_RM_OFFSET_OFFSET_SHIFT: c_int = 0;

pub const VC4_HDMI_RM_FORMAT_SHIFT_SHIFT: c_int = 24;

pub const OSCILLATOR_FREQUENCY: c_int = 54000000;
    void vc4_hdmi_phy_init(struct vc4_hdmi *vc4_hdmi,
    struct drm_connector_state *conn_state)
    {
    unsigned long flags;
// PHY should be in reset, like
// vc4_hdmi_encoder_disable() does.
//
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0xf << 16);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc4_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi) {
    void vc4_hdmi_phy_disable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0xf << 16);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc4_hdmi_phy_rng_enable(vc4_hdmi: *mut vc4_hdmi) {
    void vc4_hdmi_phy_rng_enable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_CTL_0,
    HDMI_READ(HDMI_TX_PHY_CTL_0) &
    ~VC4_HDMI_TX_PHY_RNG_PWRDN);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc4_hdmi_phy_rng_disable(vc4_hdmi: *mut vc4_hdmi) {
    void vc4_hdmi_phy_rng_disable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_CTL_0,
    HDMI_READ(HDMI_TX_PHY_CTL_0) |
    VC4_HDMI_TX_PHY_RNG_PWRDN);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
    static unsigned long long
    phy_get_vco_freq(unsigned long long clock, u8 *vco_sel, u8 *vco_div)
    {
    let mut vco_freq: c_ulonglong = clock;
    let mut _vco_div: c_uint = 0;
    let mut _vco_sel: c_uint = 0;
    while (vco_freq < 3000000000ULL) {
    _vco_div++;
    vco_freq = clock * _vco_div * 10;
    }
    if (vco_freq > 4500000000ULL)
    _vco_sel = 1;
// vco_sel = _vco_sel;
// vco_div = _vco_div;
    return vco_freq;
    }
#[no_mangle]
unsafe extern "C" fn phy_get_cp_current(vco_freq: c_ulong) -> u8 {
    static u8 phy_get_cp_current(unsigned long vco_freq)
    {
    if (vco_freq < 3700000000ULL)
    return 0x1c;
    return 0x18;
    }
#[no_mangle]
unsafe extern "C" fn phy_get_rm_offset(vco_freq: c_ulonglong) -> u32 {
    static u32 phy_get_rm_offset(unsigned long long vco_freq)
    {
    let mut fref: c_ulonglong = OSCILLATOR_FREQUENCY;
    let mut offset: u64 = 0;
// RM offset is stored as 9.22 format
    offset = vco_freq * 2;
    offset = offset << 22;
    do_div(offset, fref);
    offset >>= 2;
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn phy_get_vco_gain(vco_freq: c_ulonglong) -> u8 {
    static u8 phy_get_vco_gain(unsigned long long vco_freq)
    {
    if (vco_freq < 3350000000ULL)
    return 0xf;
    if (vco_freq < 3700000000ULL)
    return 0xc;
    if (vco_freq < 4050000000ULL)
    return 0x6;
    if (vco_freq < 4800000000ULL)
    return 0x5;
    if (vco_freq < 5200000000ULL)
    return 0x7;
    return 0x2;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_lane_settings {
    struct {
    pub preemphasis: u8,
    pub main_driver: u8,
    pub amplitude: },
    pub res_sel_data: u8,
    pub term_res_sel_data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_settings {
    pub min_rate: c_ulonglong,
    pub max_rate: c_ulonglong,
    pub channel: [phy_lane_settings; 3],
    pub clock: phy_lane_settings,
}

    static const struct phy_settings vc5_hdmi_phy_settings[] = {
    {
    0, 50000000,
    {
    {{0x0, 0x0A}, 0x12, 0x0},
    {{0x0, 0x0A}, 0x12, 0x0},
    {{0x0, 0x0A}, 0x12, 0x0}
    },
    {{0x0, 0x0A}, 0x18, 0x0},
    },
    {
    50000001, 75000000,
    {
    {{0x0, 0x09}, 0x12, 0x0},
    {{0x0, 0x09}, 0x12, 0x0},
    {{0x0, 0x09}, 0x12, 0x0}
    },
    {{0x0, 0x0C}, 0x18, 0x3},
    },
    {
    75000001,   165000000,
    {
    {{0x0, 0x09}, 0x12, 0x0},
    {{0x0, 0x09}, 0x12, 0x0},
    {{0x0, 0x09}, 0x12, 0x0}
    },
    {{0x0, 0x0C}, 0x18, 0x3},
    },
    {
    165000001,  250000000,
    {
    {{0x0, 0x0F}, 0x12, 0x1},
    {{0x0, 0x0F}, 0x12, 0x1},
    {{0x0, 0x0F}, 0x12, 0x1}
    },
    {{0x0, 0x0C}, 0x18, 0x3},
    },
    {
    250000001,  340000000,
    {
    {{0x2, 0x0D}, 0x12, 0x1},
    {{0x2, 0x0D}, 0x12, 0x1},
    {{0x2, 0x0D}, 0x12, 0x1}
    },
    {{0x0, 0x0C}, 0x18, 0xF},
    },
    {
    340000001,  450000000,
    {
    {{0x0, 0x1B}, 0x12, 0xF},
    {{0x0, 0x1B}, 0x12, 0xF},
    {{0x0, 0x1B}, 0x12, 0xF}
    },
    {{0x0, 0x0A}, 0x12, 0xF},
    },
    {
    450000001,  600000000,
    {
    {{0x0, 0x1C}, 0x12, 0xF},
    {{0x0, 0x1C}, 0x12, 0xF},
    {{0x0, 0x1C}, 0x12, 0xF}
    },
    {{0x0, 0x0B}, 0x13, 0xF},
    },
    };
    static const struct phy_settings *phy_get_settings(unsigned long long tmds_rate)
    {
    let mut count: c_uint = ARRAY_SIZE(vc5_hdmi_phy_settings);
    unsigned int i;
    for (i = 0; i < count; i++) {
    const struct phy_settings *s = &vc5_hdmi_phy_settings[i];
    if (tmds_rate >= s.min_rate && tmds_rate <= s.max_rate)
    return s;
    }
//
// If the pixel clock exceeds our max setting, try the max
// setting anyway.
//
    return &vc5_hdmi_phy_settings[count - 1];
    }
    static const struct phy_lane_settings *
    phy_get_channel_settings(enum vc4_hdmi_phy_channel chan,
    unsigned long long tmds_rate)
    {
    const struct phy_settings *settings = phy_get_settings(tmds_rate);
    if (chan == PHY_LANE_CK)
    return &settings.clock;
    return &settings.channel[chan];
    }
#[no_mangle]
unsafe extern "C" fn vc5_hdmi_reset_phy(vc4_hdmi: *mut vc4_hdmi) {
    static void vc5_hdmi_reset_phy(struct vc4_hdmi *vc4_hdmi)
    {
    lockdep_assert_held(&vc4_hdmi.hw_lock);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0x0f);
    HDMI_WRITE(HDMI_TX_PHY_POWERDOWN_CTL, BIT(10));
    }
    void vc5_hdmi_phy_init(struct vc4_hdmi *vc4_hdmi,
    struct drm_connector_state *conn_state)
    {
    const struct phy_lane_settings *chan0_settings, *chan1_settings, *chan2_settings, *clock_settings;
    const struct vc4_hdmi_variant *variant = vc4_hdmi.variant;
    let mut pixel_freq: c_ulonglong = conn_state.hdmi.tmds_char_rate;
    unsigned long long vco_freq;
    unsigned char word_sel;
    unsigned long flags;
    u8 vco_sel, vco_div;
    vco_freq = phy_get_vco_freq(pixel_freq, &vco_sel, &vco_div);
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    vc5_hdmi_reset_phy(vc4_hdmi);
    HDMI_WRITE(HDMI_TX_PHY_POWERDOWN_CTL,
    VC4_HDMI_TX_PHY_POWERDOWN_CTL_RNDGEN_PWRDN);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL,
    HDMI_READ(HDMI_TX_PHY_RESET_CTL) &
    ~VC4_HDMI_TX_PHY_RESET_CTL_TX_0_RESET &
    ~VC4_HDMI_TX_PHY_RESET_CTL_TX_1_RESET &
    ~VC4_HDMI_TX_PHY_RESET_CTL_TX_2_RESET &
    ~VC4_HDMI_TX_PHY_RESET_CTL_TX_CK_RESET);
    HDMI_WRITE(HDMI_RM_CONTROL,
    HDMI_READ(HDMI_RM_CONTROL) |
    VC4_HDMI_RM_CONTROL_EN_FREEZE_COUNTERS |
    VC4_HDMI_RM_CONTROL_EN_LOAD_INTEGRATOR |
    VC4_HDMI_RM_CONTROL_FREE_RUN);
    HDMI_WRITE(HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1,
    (HDMI_READ(HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1) &
    ~VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1_MIN_LIMIT_MASK) |
    VC4_SET_FIELD(0, VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_1_MIN_LIMIT));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2,
    (HDMI_READ(HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2) &
    ~VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2_MAX_LIMIT_MASK) |
    VC4_SET_FIELD(0, VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_2_MAX_LIMIT));
    HDMI_WRITE(HDMI_RM_OFFSET,
    VC4_SET_FIELD(phy_get_rm_offset(vco_freq),
    VC4_HDMI_RM_OFFSET_OFFSET) |
    VC4_HDMI_RM_OFFSET_ONLY);
    HDMI_WRITE(HDMI_TX_PHY_CLK_DIV,
    VC4_SET_FIELD(vco_div, VC4_HDMI_TX_PHY_CLK_DIV_VCO));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4,
    VC4_SET_FIELD(0xe147, VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4_HOLD_THRESHOLD) |
    VC4_SET_FIELD(0xe14, VC4_HDMI_TX_PHY_PLL_CALIBRATION_CONFIG_4_STABLE_THRESHOLD));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CTL_0,
    VC4_HDMI_TX_PHY_PLL_CTL_0_ENA_VCO_CLK |
    VC4_HDMI_TX_PHY_PLL_CTL_0_VCO_CONT_EN |
    VC4_HDMI_TX_PHY_PLL_CTL_0_MASH11_MODE |
    VC4_SET_FIELD(vco_sel, VC4_HDMI_TX_PHY_PLL_CTL_0_VCO_SEL));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CTL_1,
    HDMI_READ(HDMI_TX_PHY_PLL_CTL_1) |
    VC4_HDMI_TX_PHY_PLL_CTL_1_FREQ_DOUBLER_ENABLE |
    VC4_SET_FIELD(3, VC4_HDMI_TX_PHY_PLL_CTL_1_POST_RST_SEL) |
    VC4_SET_FIELD(1, VC4_HDMI_TX_PHY_PLL_CTL_1_FREQ_DOUBLER_DELAY) |
    VC4_SET_FIELD(0x8a, VC4_HDMI_TX_PHY_PLL_CTL_1_CPP));
    HDMI_WRITE(HDMI_RM_FORMAT,
    HDMI_READ(HDMI_RM_FORMAT) |
    VC4_SET_FIELD(2, VC4_HDMI_RM_FORMAT_SHIFT));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CFG,
    HDMI_READ(HDMI_TX_PHY_PLL_CFG) |
    VC4_SET_FIELD(1, VC4_HDMI_TX_PHY_PLL_CFG_PDIV));
    if (pixel_freq >= 340000000)
    word_sel = 3;
    else
    word_sel = 0;
    HDMI_WRITE(HDMI_TX_PHY_TMDS_CLK_WORD_SEL, word_sel);
    HDMI_WRITE(HDMI_TX_PHY_CTL_3,
    VC4_SET_FIELD(phy_get_cp_current(vco_freq),
    VC4_HDMI_TX_PHY_CTL_3_ICP) |
    VC4_SET_FIELD(1, VC4_HDMI_TX_PHY_CTL_3_CP) |
    VC4_SET_FIELD(1, VC4_HDMI_TX_PHY_CTL_3_CP1) |
    VC4_SET_FIELD(3, VC4_HDMI_TX_PHY_CTL_3_CZ) |
    VC4_SET_FIELD(4, VC4_HDMI_TX_PHY_CTL_3_RP) |
    VC4_SET_FIELD(6, VC4_HDMI_TX_PHY_CTL_3_RZ));
    chan0_settings =
    phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_0],
    pixel_freq);
    chan1_settings =
    phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_1],
    pixel_freq);
    chan2_settings =
    phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_2],
    pixel_freq);
    clock_settings =
    phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_CK],
    pixel_freq);
    HDMI_WRITE(HDMI_TX_PHY_CTL_0,
    VC4_SET_FIELD(chan0_settings.amplitude.preemphasis,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_0_PREEMP) |
    VC4_SET_FIELD(chan0_settings.amplitude.main_driver,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_0_MAINDRV) |
    VC4_SET_FIELD(chan1_settings.amplitude.preemphasis,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_1_PREEMP) |
    VC4_SET_FIELD(chan1_settings.amplitude.main_driver,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_1_MAINDRV) |
    VC4_SET_FIELD(chan2_settings.amplitude.preemphasis,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_2_PREEMP) |
    VC4_SET_FIELD(chan2_settings.amplitude.main_driver,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_2_MAINDRV) |
    VC4_SET_FIELD(clock_settings.amplitude.preemphasis,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_CK_PREEMP) |
    VC4_SET_FIELD(clock_settings.amplitude.main_driver,
    VC4_HDMI_TX_PHY_CTL_0_PREEMP_CK_MAINDRV));
    HDMI_WRITE(HDMI_TX_PHY_CTL_1,
    HDMI_READ(HDMI_TX_PHY_CTL_1) |
    VC4_SET_FIELD(chan0_settings.res_sel_data,
    VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA0) |
    VC4_SET_FIELD(chan1_settings.res_sel_data,
    VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA1) |
    VC4_SET_FIELD(chan2_settings.res_sel_data,
    VC4_HDMI_TX_PHY_CTL_1_RES_SEL_DATA2) |
    VC4_SET_FIELD(clock_settings.res_sel_data,
    VC4_HDMI_TX_PHY_CTL_1_RES_SEL_CK));
    HDMI_WRITE(HDMI_TX_PHY_CTL_2,
    VC4_SET_FIELD(chan0_settings.term_res_sel_data,
    VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA0) |
    VC4_SET_FIELD(chan1_settings.term_res_sel_data,
    VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA1) |
    VC4_SET_FIELD(chan2_settings.term_res_sel_data,
    VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELDATA2) |
    VC4_SET_FIELD(clock_settings.term_res_sel_data,
    VC4_HDMI_TX_PHY_CTL_2_TERM_RES_SELCK) |
    VC4_SET_FIELD(phy_get_vco_gain(vco_freq),
    VC4_HDMI_TX_PHY_CTL_2_VCO_GAIN));
    HDMI_WRITE(HDMI_TX_PHY_CHANNEL_SWAP,
    VC4_SET_FIELD(variant.phy_lane_mapping[PHY_LANE_0],
    VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX0_OUT_SEL) |
    VC4_SET_FIELD(variant.phy_lane_mapping[PHY_LANE_1],
    VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX1_OUT_SEL) |
    VC4_SET_FIELD(variant.phy_lane_mapping[PHY_LANE_2],
    VC4_HDMI_TX_PHY_CHANNEL_SWAP_TX2_OUT_SEL) |
    VC4_SET_FIELD(variant.phy_lane_mapping[PHY_LANE_CK],
    VC4_HDMI_TX_PHY_CHANNEL_SWAP_TXCK_OUT_SEL));
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL,
    HDMI_READ(HDMI_TX_PHY_RESET_CTL) &
    ~(VC4_HDMI_TX_PHY_RESET_CTL_PLL_RESETB |
    VC4_HDMI_TX_PHY_RESET_CTL_PLLDIV_RESETB));
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL,
    HDMI_READ(HDMI_TX_PHY_RESET_CTL) |
    VC4_HDMI_TX_PHY_RESET_CTL_PLL_RESETB |
    VC4_HDMI_TX_PHY_RESET_CTL_PLLDIV_RESETB);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc5_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi) {
    void vc5_hdmi_phy_disable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    vc5_hdmi_reset_phy(vc4_hdmi);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc5_hdmi_phy_rng_enable(vc4_hdmi: *mut vc4_hdmi) {
    void vc5_hdmi_phy_rng_enable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_POWERDOWN_CTL,
    HDMI_READ(HDMI_TX_PHY_POWERDOWN_CTL) &
    ~VC4_HDMI_TX_PHY_POWERDOWN_CTL_RNDGEN_PWRDN);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc5_hdmi_phy_rng_disable(vc4_hdmi: *mut vc4_hdmi) {
    void vc5_hdmi_phy_rng_disable(struct vc4_hdmi *vc4_hdmi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    HDMI_WRITE(HDMI_TX_PHY_POWERDOWN_CTL,
    HDMI_READ(HDMI_TX_PHY_POWERDOWN_CTL) |
    VC4_HDMI_TX_PHY_POWERDOWN_CTL_RNDGEN_PWRDN);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }

    static unsigned long long
    vc6_phy_get_vco_freq(unsigned long long tmds_rate, unsigned int *vco_div)
    {
    unsigned int min_div;
    unsigned int max_div;
    unsigned int div;
    div = 0;
    while (tmds_rate * div * 10 < VC6_VCO_MIN_FREQ)
    div++;
    min_div = div;
    while (tmds_rate * (div + 1) * 10 < VC6_VCO_MAX_FREQ)
    div++;
    max_div = div;
    div = min_div + (max_div - min_div) / 2;
// vco_div = div;
    return tmds_rate * div * 10;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc6_phy_lane_settings {
    pub ext_current_ctl:4: c_uint,
    pub ffe_enable:1: c_uint,
    pub slew_rate_ctl:1: c_uint,
    pub ffe_post_tap_en:1: c_uint,
    pub ldmos_bias_ctl:2: c_uint,
    pub com_mode_ldmos_en:1: c_uint,
    pub edge_sel:1: c_uint,
    pub ext_current_src_hs_en:1: c_uint,
    pub term_ctl:2: c_uint,
    pub ext_current_src_en:1: c_uint,
    pub int_current_src_en:1: c_uint,
    pub int_current_ctl:4: c_uint,
    pub int_current_src_hs_en:1: c_uint,
    pub main_tap_current_select:3: c_uint,
    pub post_tap_current_select:3: c_uint,
    pub slew_ctl_slow_loading:2: c_uint,
    pub slew_ctl_slow_driving:2: c_uint,
    pub ffe_pre_tap_en:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc6_phy_settings {
    pub min_rate: c_ulonglong,
    pub max_rate: c_ulonglong,
    pub channel: [vc6_phy_lane_settings; 3],
    pub clock: vc6_phy_lane_settings,
}

    static const struct vc6_phy_settings vc6_hdmi_phy_settings[] = {
    {
    0, 222000000,
    {
    {
// 200mA
    .ext_current_ctl = 8,
// 0.85V
    .ldmos_bias_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    {
// 200mA
    .ext_current_ctl = 8,
// 0.85V
    .ldmos_bias_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    {
// 200mA
    .ext_current_ctl = 8,
// 0.85V
    .ldmos_bias_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    },
    {
// 200mA
    .ext_current_ctl = 8,
// 0.85V
    .ldmos_bias_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    },
    {
    222000001, 297000000,
    {
    {
// 200mA and 180mA ?!
    .ext_current_ctl = 12,
// 0.85V
    .ldmos_bias_ctl = 1,
// 100 Ohm
    .term_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
    },
    {
// 200mA and 180mA ?!
    .ext_current_ctl = 12,
// 0.85V
    .ldmos_bias_ctl = 1,
// 100 Ohm
    .term_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
    },
    {
// 200mA and 180mA ?!
    .ext_current_ctl = 12,
// 0.85V
    .ldmos_bias_ctl = 1,
// 100 Ohm
    .term_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
    },
    },
    {
// 200mA and 180mA ?!
    .ext_current_ctl = 12,
// 0.85V
    .ldmos_bias_ctl = 1,
// 100 Ohm
    .term_ctl = 1,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
// Internal Current Source Half Swing Enable
    .int_current_src_hs_en = 1,
    },
    },
    {
    297000001, 597000044,
    {
    {
// 200mA
    .ext_current_ctl = 8,
// Normal Slew Rate Control
    .slew_rate_ctl = 1,
// 0.85V
    .ldmos_bias_ctl = 1,
// 50 Ohms
    .term_ctl = 3,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    {
// 200mA
    .ext_current_ctl = 8,
// Normal Slew Rate Control
    .slew_rate_ctl = 1,
// 0.85V
    .ldmos_bias_ctl = 1,
// 50 Ohms
    .term_ctl = 3,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    {
// 200mA
    .ext_current_ctl = 8,
// Normal Slew Rate Control
    .slew_rate_ctl = 1,
// 0.85V
    .ldmos_bias_ctl = 1,
// 50 Ohms
    .term_ctl = 3,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    },
    {
// 200mA
    .ext_current_ctl = 8,
// Normal Slew Rate Control
    .slew_rate_ctl = 1,
// 0.85V
    .ldmos_bias_ctl = 1,
// External Current Source Half Swing Enable
    .ext_current_src_hs_en = 1,
// 50 Ohms
    .term_ctl = 3,
// Enable External Current Source
    .ext_current_src_en = 1,
// Enable Internal Current Source
    .int_current_src_en = 1,
// 200mA
    .int_current_ctl = 8,
// Internal Current Source Half Swing Enable
    .int_current_src_hs_en = 1,
// 17.6 mA
    .main_tap_current_select = 7,
    },
    },
    };
    static const struct vc6_phy_settings *
    vc6_phy_get_settings(unsigned long long tmds_rate)
    {
    let mut count: c_uint = ARRAY_SIZE(vc6_hdmi_phy_settings);
    unsigned int i;
    for (i = 0; i < count; i++) {
    const struct vc6_phy_settings *s = &vc6_hdmi_phy_settings[i];
    if (tmds_rate >= s.min_rate && tmds_rate <= s.max_rate)
    return s;
    }
//
// If the pixel clock exceeds our max setting, try the max
// setting anyway.
//
    return &vc6_hdmi_phy_settings[count - 1];
    }
    static const struct vc6_phy_lane_settings *
    vc6_phy_get_channel_settings(enum vc4_hdmi_phy_channel chan,
    unsigned long long tmds_rate)
    {
    const struct vc6_phy_settings *settings = vc6_phy_get_settings(tmds_rate);
    if (chan == PHY_LANE_CK)
    return &settings.clock;
    return &settings.channel[chan];
    }
#[no_mangle]
unsafe extern "C" fn vc6_hdmi_reset_phy(vc4_hdmi: *mut vc4_hdmi) {
    static void vc6_hdmi_reset_phy(struct vc4_hdmi *vc4_hdmi)
    {
    lockdep_assert_held(&vc4_hdmi.hw_lock);
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0);
    HDMI_WRITE(HDMI_TX_PHY_POWERUP_CTL, 0);
    }
    void vc6_hdmi_phy_init(struct vc4_hdmi *vc4_hdmi,
    struct drm_connector_state *conn_state)
    {
    const struct vc6_phy_lane_settings *chan0_settings;
    const struct vc6_phy_lane_settings *chan1_settings;
    const struct vc6_phy_lane_settings *chan2_settings;
    const struct vc6_phy_lane_settings *clock_settings;
    const struct vc4_hdmi_variant *variant = vc4_hdmi.variant;
    let mut pixel_freq: c_ulonglong = conn_state.hdmi.tmds_char_rate;
    unsigned long long vco_freq;
    unsigned char word_sel;
    unsigned long flags;
    unsigned int vco_div;
    vco_freq = vc6_phy_get_vco_freq(pixel_freq, &vco_div);
    spin_lock_irqsave(&vc4_hdmi.hw_lock, flags);
    vc6_hdmi_reset_phy(vc4_hdmi);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_0, 0x810c6000);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_1, 0x00b8c451);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_2, 0x46402e31);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_3, 0x00b8c005);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_4, 0x42410261);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_5, 0xcc021001);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_6, 0xc8301c80);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_7, 0xb0804444);
    HDMI_WRITE(HDMI_TX_PHY_PLL_MISC_8, 0xf80f8000);
    HDMI_WRITE(HDMI_TX_PHY_PLL_REFCLK,
    VC6_HDMI_TX_PHY_PLL_REFCLK_REFCLK_SEL_CMOS |
    VC4_SET_FIELD(54, VC6_HDMI_TX_PHY_PLL_REFCLK_REFFRQ));
    HDMI_WRITE(HDMI_TX_PHY_RESET_CTL, 0x7f);
    HDMI_WRITE(HDMI_RM_OFFSET,
    VC4_HDMI_RM_OFFSET_ONLY |
    VC4_SET_FIELD(phy_get_rm_offset(vco_freq),
    VC4_HDMI_RM_OFFSET_OFFSET));
    HDMI_WRITE(HDMI_TX_PHY_PLL_VCOCLK_DIV,
    VC6_HDMI_TX_PHY_PLL_VCOCLK_DIV_VCODIV_EN |
    VC4_SET_FIELD(vco_div,
    VC6_HDMI_TX_PHY_PLL_VCOCLK_DIV_VCODIV));
    HDMI_WRITE(HDMI_TX_PHY_PLL_CFG,
    VC4_SET_FIELD(0, VC4_HDMI_TX_PHY_PLL_CFG_PDIV));
    HDMI_WRITE(HDMI_TX_PHY_PLL_POST_KDIV,
    VC4_SET_FIELD(2, VC6_HDMI_TX_PHY_PLL_POST_KDIV_CLK0_SEL) |
    VC4_SET_FIELD(1, VC6_HDMI_TX_PHY_PLL_POST_KDIV_KDIV));
    chan0_settings =
    vc6_phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_0],
    pixel_freq);
    HDMI_WRITE(HDMI_TX_PHY_CTL_0,
    VC4_SET_FIELD(chan0_settings.ext_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_CTL) |
    VC4_SET_FIELD(chan0_settings.ffe_enable,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_ENABLE) |
    VC4_SET_FIELD(chan0_settings.slew_rate_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_RATE_CTL) |
    VC4_SET_FIELD(chan0_settings.ffe_post_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_POST_TAP_EN) |
    VC4_SET_FIELD(chan0_settings.ldmos_bias_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_LDMOS_BIAS_CTL) |
    VC4_SET_FIELD(chan0_settings.com_mode_ldmos_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_COM_MODE_LDMOS_EN) |
    VC4_SET_FIELD(chan0_settings.edge_sel,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EDGE_SEL) |
    VC4_SET_FIELD(chan0_settings.ext_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan0_settings.term_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_TERM_CTL) |
    VC4_SET_FIELD(chan0_settings.ext_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan0_settings.int_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan0_settings.int_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_CTL) |
    VC4_SET_FIELD(chan0_settings.int_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan0_settings.main_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_MAIN_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan0_settings.post_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_POST_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan0_settings.slew_ctl_slow_loading,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_LOADING) |
    VC4_SET_FIELD(chan0_settings.slew_ctl_slow_driving,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_DRIVING) |
    VC4_SET_FIELD(chan0_settings.ffe_pre_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_PRE_TAP_EN));
    chan1_settings =
    vc6_phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_1],
    pixel_freq);
    HDMI_WRITE(HDMI_TX_PHY_CTL_1,
    VC4_SET_FIELD(chan1_settings.ext_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_CTL) |
    VC4_SET_FIELD(chan1_settings.ffe_enable,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_ENABLE) |
    VC4_SET_FIELD(chan1_settings.slew_rate_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_RATE_CTL) |
    VC4_SET_FIELD(chan1_settings.ffe_post_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_POST_TAP_EN) |
    VC4_SET_FIELD(chan1_settings.ldmos_bias_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_LDMOS_BIAS_CTL) |
    VC4_SET_FIELD(chan1_settings.com_mode_ldmos_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_COM_MODE_LDMOS_EN) |
    VC4_SET_FIELD(chan1_settings.edge_sel,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EDGE_SEL) |
    VC4_SET_FIELD(chan1_settings.ext_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan1_settings.term_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_TERM_CTL) |
    VC4_SET_FIELD(chan1_settings.ext_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan1_settings.int_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan1_settings.int_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_CTL) |
    VC4_SET_FIELD(chan1_settings.int_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan1_settings.main_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_MAIN_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan1_settings.post_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_POST_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan1_settings.slew_ctl_slow_loading,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_LOADING) |
    VC4_SET_FIELD(chan1_settings.slew_ctl_slow_driving,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_DRIVING) |
    VC4_SET_FIELD(chan1_settings.ffe_pre_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_PRE_TAP_EN));
    chan2_settings =
    vc6_phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_2],
    pixel_freq);
    HDMI_WRITE(HDMI_TX_PHY_CTL_2,
    VC4_SET_FIELD(chan2_settings.ext_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_CTL) |
    VC4_SET_FIELD(chan2_settings.ffe_enable,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_ENABLE) |
    VC4_SET_FIELD(chan2_settings.slew_rate_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_RATE_CTL) |
    VC4_SET_FIELD(chan2_settings.ffe_post_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_POST_TAP_EN) |
    VC4_SET_FIELD(chan2_settings.ldmos_bias_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_LDMOS_BIAS_CTL) |
    VC4_SET_FIELD(chan2_settings.com_mode_ldmos_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_COM_MODE_LDMOS_EN) |
    VC4_SET_FIELD(chan2_settings.edge_sel,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EDGE_SEL) |
    VC4_SET_FIELD(chan2_settings.ext_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan2_settings.term_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_TERM_CTL) |
    VC4_SET_FIELD(chan2_settings.ext_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan2_settings.int_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(chan2_settings.int_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_CTL) |
    VC4_SET_FIELD(chan2_settings.int_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(chan2_settings.main_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_MAIN_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan2_settings.post_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_POST_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(chan2_settings.slew_ctl_slow_loading,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_LOADING) |
    VC4_SET_FIELD(chan2_settings.slew_ctl_slow_driving,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_DRIVING) |
    VC4_SET_FIELD(chan2_settings.ffe_pre_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_PRE_TAP_EN));
    clock_settings =
    vc6_phy_get_channel_settings(variant.phy_lane_mapping[PHY_LANE_CK],
    pixel_freq);
    HDMI_WRITE(HDMI_TX_PHY_CTL_CK,
    VC4_SET_FIELD(clock_settings.ext_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_CTL) |
    VC4_SET_FIELD(clock_settings.ffe_enable,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_ENABLE) |
    VC4_SET_FIELD(clock_settings.slew_rate_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_RATE_CTL) |
    VC4_SET_FIELD(clock_settings.ffe_post_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_POST_TAP_EN) |
    VC4_SET_FIELD(clock_settings.ldmos_bias_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_LDMOS_BIAS_CTL) |
    VC4_SET_FIELD(clock_settings.com_mode_ldmos_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_COM_MODE_LDMOS_EN) |
    VC4_SET_FIELD(clock_settings.edge_sel,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EDGE_SEL) |
    VC4_SET_FIELD(clock_settings.ext_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(clock_settings.term_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_TERM_CTL) |
    VC4_SET_FIELD(clock_settings.ext_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_EXT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(clock_settings.int_current_src_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_EN) |
    VC4_SET_FIELD(clock_settings.int_current_ctl,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_CTL) |
    VC4_SET_FIELD(clock_settings.int_current_src_hs_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_INT_CURRENT_SRC_HS_EN) |
    VC4_SET_FIELD(clock_settings.main_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_MAIN_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(clock_settings.post_tap_current_select,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_POST_TAP_CURRENT_SELECT) |
    VC4_SET_FIELD(clock_settings.slew_ctl_slow_loading,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_LOADING) |
    VC4_SET_FIELD(clock_settings.slew_ctl_slow_driving,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_SLEW_CTL_SLOW_DRIVING) |
    VC4_SET_FIELD(clock_settings.ffe_pre_tap_en,
    VC6_HDMI_TX_PHY_HDMI_CTRL_CHX_FFE_PRE_TAP_EN));
    if (pixel_freq >= 340000000)
    word_sel = 3;
    else
    word_sel = 0;
    HDMI_WRITE(HDMI_TX_PHY_TMDS_CLK_WORD_SEL, word_sel);
    HDMI_WRITE(HDMI_TX_PHY_POWERUP_CTL,
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_BG_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_LDO_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_BIAS_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_TX_CK_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_TX_2_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_TX_1_PWRUP |
    VC6_HDMI_TX_PHY_HDMI_POWERUP_CTL_TX_0_PWRUP);
    HDMI_WRITE(HDMI_TX_PHY_PLL_POWERUP_CTL,
    VC6_HDMI_TX_PHY_PLL_POWERUP_CTL_PLL_PWRUP);
    HDMI_WRITE(HDMI_TX_PHY_PLL_RESET_CTL,
    HDMI_READ(HDMI_TX_PHY_PLL_RESET_CTL) &
    ~VC6_HDMI_TX_PHY_PLL_RESET_CTL_PLL_RESETB);
    HDMI_WRITE(HDMI_TX_PHY_PLL_RESET_CTL,
    HDMI_READ(HDMI_TX_PHY_PLL_RESET_CTL) |
    VC6_HDMI_TX_PHY_PLL_RESET_CTL_PLL_RESETB);
    spin_unlock_irqrestore(&vc4_hdmi.hw_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn vc6_hdmi_phy_disable(vc4_hdmi: *mut vc4_hdmi) {
    void vc6_hdmi_phy_disable(struct vc4_hdmi *vc4_hdmi)
    {
    }
