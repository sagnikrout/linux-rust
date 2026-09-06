//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8192/mt8192-afe-clk.h
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
// mt8192-afe-clk.h  --  Mediatek 8192 afe clock ctrl definition
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//
pub const AP_PLL_CON3: c_uint = 0x0014;
pub const APLL1_CON0: c_uint = 0x0318;
pub const APLL1_CON1: c_uint = 0x031c;
pub const APLL1_CON2: c_uint = 0x0320;
pub const APLL1_CON4: c_uint = 0x0328;
pub const APLL1_TUNER_CON0: c_uint = 0x0040;
pub const APLL2_CON0: c_uint = 0x032c;
pub const APLL2_CON1: c_uint = 0x0330;
pub const APLL2_CON2: c_uint = 0x0334;
pub const APLL2_CON4: c_uint = 0x033c;
pub const APLL2_TUNER_CON0: c_uint = 0x0044;
pub const CLK_CFG_7: c_uint = 0x0080;
pub const CLK_CFG_8: c_uint = 0x0090;
pub const CLK_CFG_11: c_uint = 0x00c0;
pub const CLK_CFG_12: c_uint = 0x00d0;
pub const CLK_CFG_13: c_uint = 0x00e0;
pub const CLK_CFG_15: c_uint = 0x0100;
pub const CLK_AUDDIV_0: c_uint = 0x0320;
pub const CLK_AUDDIV_2: c_uint = 0x0328;
pub const CLK_AUDDIV_3: c_uint = 0x0334;
pub const CLK_AUDDIV_4: c_uint = 0x0338;
pub const CKSYS_AUD_TOP_CFG: c_uint = 0x032c;
pub const CKSYS_AUD_TOP_MON: c_uint = 0x0330;
pub const PERI_BUS_DCM_CTRL: c_uint = 0x0074;
pub const MODULE_SW_CG_1_STA: c_uint = 0x0094;
pub const MODULE_SW_CG_2_STA: c_uint = 0x00ac;
// CLK_AUDDIV_0
pub const APLL12_DIV0_PDN_SFT: c_int = 0;
pub const APLL12_DIV0_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV1_PDN_SFT: c_int = 1;
pub const APLL12_DIV1_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV2_PDN_SFT: c_int = 2;
pub const APLL12_DIV2_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV3_PDN_SFT: c_int = 3;
pub const APLL12_DIV3_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV4_PDN_SFT: c_int = 4;
pub const APLL12_DIV4_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIVB_PDN_SFT: c_int = 5;
pub const APLL12_DIVB_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV5_PDN_SFT: c_int = 6;
pub const APLL12_DIV5_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV6_PDN_SFT: c_int = 7;
pub const APLL12_DIV6_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV7_PDN_SFT: c_int = 8;
pub const APLL12_DIV7_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV8_PDN_SFT: c_int = 9;
pub const APLL12_DIV8_PDN_MASK: c_uint = 0x1;

pub const APLL12_DIV9_PDN_SFT: c_int = 10;
pub const APLL12_DIV9_PDN_MASK: c_uint = 0x1;

pub const APLL_I2S0_MCK_SEL_SFT: c_int = 16;
pub const APLL_I2S0_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S1_MCK_SEL_SFT: c_int = 17;
pub const APLL_I2S1_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S2_MCK_SEL_SFT: c_int = 18;
pub const APLL_I2S2_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S3_MCK_SEL_SFT: c_int = 19;
pub const APLL_I2S3_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S4_MCK_SEL_SFT: c_int = 20;
pub const APLL_I2S4_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S5_MCK_SEL_SFT: c_int = 21;
pub const APLL_I2S5_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S6_MCK_SEL_SFT: c_int = 22;
pub const APLL_I2S6_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S7_MCK_SEL_SFT: c_int = 23;
pub const APLL_I2S7_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S8_MCK_SEL_SFT: c_int = 24;
pub const APLL_I2S8_MCK_SEL_MASK: c_uint = 0x1;

pub const APLL_I2S9_MCK_SEL_SFT: c_int = 25;
pub const APLL_I2S9_MCK_SEL_MASK: c_uint = 0x1;

// CLK_AUDDIV_2
pub const APLL12_CK_DIV0_SFT: c_int = 0;
pub const APLL12_CK_DIV0_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV1_SFT: c_int = 8;
pub const APLL12_CK_DIV1_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV2_SFT: c_int = 16;
pub const APLL12_CK_DIV2_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV3_SFT: c_int = 24;
pub const APLL12_CK_DIV3_MASK: c_uint = 0xff;

// CLK_AUDDIV_3
pub const APLL12_CK_DIV4_SFT: c_int = 0;
pub const APLL12_CK_DIV4_MASK: c_uint = 0xff;

pub const APLL12_CK_DIVB_SFT: c_int = 8;
pub const APLL12_CK_DIVB_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV5_SFT: c_int = 16;
pub const APLL12_CK_DIV5_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV6_SFT: c_int = 24;
pub const APLL12_CK_DIV6_MASK: c_uint = 0xff;

// CLK_AUDDIV_4
pub const APLL12_CK_DIV7_SFT: c_int = 0;
pub const APLL12_CK_DIV7_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV8_SFT: c_int = 8;
pub const APLL12_CK_DIV8_MASK: c_uint = 0xff;

pub const APLL12_CK_DIV9_SFT: c_int = 16;
pub const APLL12_CK_DIV9_MASK: c_uint = 0xff;

// AUD_TOP_CFG
pub const AUD_TOP_CFG_SFT: c_int = 0;
pub const AUD_TOP_CFG_MASK: c_uint = 0xffffffff;

// AUD_TOP_MON
pub const AUD_TOP_MON_SFT: c_int = 0;
pub const AUD_TOP_MON_MASK: c_uint = 0xffffffff;

// CLK_AUDDIV_3
pub const APLL12_CK_DIV5_MSB_SFT: c_int = 0;
pub const APLL12_CK_DIV5_MSB_MASK: c_uint = 0xf;

pub const RESERVED0_SFT: c_int = 4;
pub const RESERVED0_MASK: c_uint = 0xfffffff;

// APLL

// apll related mux
extern "C" {
    pub fn mt8192_init_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8192_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8192_afe_disable_clock(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8192_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8192_apll1_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8192_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8192_apll2_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8192_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
}
extern "C" {
    pub fn mt8192_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8192_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
}
// these will be replaced by using CCF
extern "C" {
    pub fn mt8192_mck_enable(afe: *mut mtk_base_afe, mck_id: c_int, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8192_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int);
}
