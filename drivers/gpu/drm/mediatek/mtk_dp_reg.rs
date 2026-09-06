//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_dp_reg.h
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
// Copyright (c) 2019-2022 MediaTek Inc.
// Copyright (c) 2022 BayLibre
//
pub const SEC_OFFSET: c_uint = 0x4000;

// offset: 0x0
pub const DP_PHY_GLB_BIAS_GEN_00: c_uint = 0x0;

pub const DP_PHY_GLB_DPAUX_TX: c_uint = 0x8;

pub const MTK_DP_0034: c_uint = 0x34;

pub const DP_PHY_LANE_TX_0: c_uint = 0x104;

pub const DP_PHY_LANE_TX_1: c_uint = 0x204;

pub const DP_PHY_LANE_TX_2: c_uint = 0x304;

pub const DP_PHY_LANE_TX_3: c_uint = 0x404;

pub const MTK_DP_1040: c_uint = 0x1040;

// offset: TOP_OFFSET (0x2000)
pub const MTK_DP_TOP_PWR_STATE: c_uint = 0x2000;

pub const MTK_DP_TOP_SWING_EMP: c_uint = 0x2004;

pub const DP_TX0_VOLT_SWING_SHIFT: c_int = 0;

pub const DP_TX0_PRE_EMPH_SHIFT: c_int = 2;

pub const DP_TX1_VOLT_SWING_SHIFT: c_int = 8;

pub const MTK_DP_TOP_RESET_AND_PROBE: c_uint = 0x2020;

pub const MTK_DP_TOP_IRQ_MASK: c_uint = 0x202c;

pub const MTK_DP_TOP_MEM_PD: c_uint = 0x2038;

// offset: ENC0_OFFSET (0x3000)
pub const MTK_DP_ENC0_P0_3000: c_uint = 0x3000;

pub const MTK_DP_ENC0_P0_3004: c_uint = 0x3004;

pub const MTK_DP_ENC0_P0_3010: c_uint = 0x3010;

pub const MTK_DP_ENC0_P0_3014: c_uint = 0x3014;

pub const MTK_DP_ENC0_P0_3018: c_uint = 0x3018;

pub const MTK_DP_ENC0_P0_301C: c_uint = 0x301c;

pub const MTK_DP_ENC0_P0_3020: c_uint = 0x3020;

pub const MTK_DP_ENC0_P0_3024: c_uint = 0x3024;

pub const MTK_DP_ENC0_P0_3028: c_uint = 0x3028;

pub const MTK_DP_ENC0_P0_302C: c_uint = 0x302c;

pub const MTK_DP_ENC0_P0_3030: c_uint = 0x3030;

pub const MTK_DP_ENC0_P0_3034: c_uint = 0x3034;
pub const MTK_DP_ENC0_P0_3038: c_uint = 0x3038;

pub const MTK_DP_ENC0_P0_303C: c_uint = 0x303c;

pub const MTK_DP_ENC0_P0_3040: c_uint = 0x3040;
pub const SDP_DOWN_CNT_DP_ENC0_P0_VAL: c_uint = 0x20;

pub const MTK_DP_ENC0_P0_304C: c_uint = 0x304c;

pub const MTK_DP_ENC0_P0_3064: c_uint = 0x3064;

pub const MTK_DP_ENC0_P0_3088: c_uint = 0x3088;

pub const MTK_DP_ENC0_P0_308C: c_uint = 0x308c;

pub const MTK_DP_ENC0_P0_3090: c_uint = 0x3090;

pub const MTK_DP_ENC0_P0_3094: c_uint = 0x3094;

pub const MTK_DP_ENC0_P0_30A4: c_uint = 0x30a4;

pub const MTK_DP_ENC0_P0_30A8: c_uint = 0x30a8;
pub const MTK_DP_ENC0_P0_30BC: c_uint = 0x30bc;

pub const MTK_DP_ENC0_P0_30D8: c_uint = 0x30d8;
pub const MTK_DP_ENC0_P0_312C: c_uint = 0x312c;

pub const MTK_DP_ENC0_P0_3154: c_uint = 0x3154;

pub const MTK_DP_ENC0_P0_3158: c_uint = 0x3158;

pub const MTK_DP_ENC0_P0_315C: c_uint = 0x315c;

pub const MTK_DP_ENC0_P0_3160: c_uint = 0x3160;

pub const MTK_DP_ENC0_P0_3164: c_uint = 0x3164;

pub const MTK_DP_ENC0_P0_3168: c_uint = 0x3168;

pub const MTK_DP_ENC0_P0_316C: c_uint = 0x316c;

pub const MTK_DP_ENC0_P0_3170: c_uint = 0x3170;

pub const MTK_DP_ENC0_P0_3174: c_uint = 0x3174;

pub const MTK_DP_ENC0_P0_3178: c_uint = 0x3178;

pub const MTK_DP_ENC0_P0_31B0: c_uint = 0x31b0;
pub const PGEN_PATTERN_SEL_VAL: c_int = 4;

pub const MTK_DP_ENC0_P0_31EC: c_uint = 0x31ec;

// offset: ENC1_OFFSET (0x3200)
pub const MTK_DP_ENC1_P0_3200: c_uint = 0x3200;
pub const MTK_DP_ENC1_P0_3280: c_uint = 0x3280;

pub const MTK_DP_ENC1_P0_3300: c_uint = 0x3300;
pub const VIDEO_AFIFO_RDY_SEL_DP_ENC1_P0_VAL: c_int = 2;

pub const MTK_DP_ENC1_P0_3304: c_uint = 0x3304;

pub const MTK_DP_ENC1_P0_3324: c_uint = 0x3324;

pub const AUDIO_SOURCE_MUX_DP_ENC1_P0_DPRX: c_int = 0;
pub const MTK_DP_ENC1_P0_3364: c_uint = 0x3364;
pub const SDP_DOWN_CNT_IN_HBLANK_DP_ENC1_P0_VAL: c_uint = 0x20;

pub const FIFO_READ_START_POINT_DP_ENC1_P0_VAL: c_int = 4;

pub const MTK_DP_ENC1_P0_3368: c_uint = 0x3368;

pub const BS2BS_MODE_DP_ENC1_P0_VAL: c_int = 1;

pub const MTK_DP_ENC1_P0_3374: c_uint = 0x3374;

pub const MTK_DP_ENC1_P0_33F4: c_uint = 0x33f4;

// offset: TRANS_OFFSET (0x3400)
pub const MTK_DP_TRANS_P0_3400: c_uint = 0x3400;

pub const MTK_DP_TRANS_P0_3404: c_uint = 0x3404;

pub const MTK_DP_TRANS_P0_340C: c_uint = 0x340c;

pub const MTK_DP_TRANS_P0_3410: c_uint = 0x3410;

pub const MTK_DP_TRANS_P0_3414: c_uint = 0x3414;

pub const MTK_DP_TRANS_P0_3418: c_uint = 0x3418;

pub const MTK_DP_TRANS_P0_342C: c_uint = 0x342c;

pub const MTK_DP_TRANS_P0_3430: c_uint = 0x3430;

pub const MTK_DP_TRANS_P0_34A4: c_uint = 0x34a4;

pub const MTK_DP_TRANS_P0_3540: c_uint = 0x3540;

pub const MTK_DP_TRANS_P0_3580: c_uint = 0x3580;

pub const MTK_DP_TRANS_P0_35C8: c_uint = 0x35c8;

pub const MTK_DP_TRANS_P0_35D0: c_uint = 0x35d0;

pub const MTK_DP_TRANS_P0_35F0: c_uint = 0x35f0;

// offset: AUX_OFFSET (0x3600)
pub const MTK_DP_AUX_P0_360C: c_uint = 0x360c;

pub const AUX_TIMEOUT_THR_AUX_TX_P0_VAL: c_uint = 0x1595;
pub const MTK_DP_AUX_P0_3614: c_uint = 0x3614;

pub const AUX_RX_UI_CNT_THR_AUX_FOR_26M: c_int = 13;
pub const MTK_DP_AUX_P0_3618: c_uint = 0x3618;

pub const MTK_DP_AUX_P0_3620: c_uint = 0x3620;

pub const MTK_DP_AUX_P0_3624: c_uint = 0x3624;

pub const MTK_DP_AUX_P0_3628: c_uint = 0x3628;

pub const MTK_DP_AUX_P0_362C: c_uint = 0x362c;

pub const MTK_DP_AUX_P0_3630: c_uint = 0x3630;

pub const MTK_DP_AUX_P0_3634: c_uint = 0x3634;

pub const AUX_TX_OVER_SAMPLE_RATE_FOR_26M: c_int = 25;
pub const MTK_DP_AUX_P0_3640: c_uint = 0x3640;

pub const MTK_DP_AUX_P0_3644: c_uint = 0x3644;

pub const MTK_DP_AUX_P0_3648: c_uint = 0x3648;

pub const MTK_DP_AUX_P0_364C: c_uint = 0x364c;

pub const MTK_DP_AUX_P0_3650: c_uint = 0x3650;

pub const MTK_DP_AUX_P0_3658: c_uint = 0x3658;

pub const MTK_DP_AUX_P0_3690: c_uint = 0x3690;

pub const MTK_DP_AUX_P0_3704: c_uint = 0x3704;

pub const MTK_DP_AUX_P0_3708: c_uint = 0x3708;
pub const MTK_DP_AUX_P0_37C8: c_uint = 0x37c8;

