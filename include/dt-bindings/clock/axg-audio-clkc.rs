//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axg-audio-clkc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 Baylibre SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//
pub const AUD_CLKID_DDR_ARB: c_int = 29;
pub const AUD_CLKID_PDM: c_int = 30;
pub const AUD_CLKID_TDMIN_A: c_int = 31;
pub const AUD_CLKID_TDMIN_B: c_int = 32;
pub const AUD_CLKID_TDMIN_C: c_int = 33;
pub const AUD_CLKID_TDMIN_LB: c_int = 34;
pub const AUD_CLKID_TDMOUT_A: c_int = 35;
pub const AUD_CLKID_TDMOUT_B: c_int = 36;
pub const AUD_CLKID_TDMOUT_C: c_int = 37;
pub const AUD_CLKID_FRDDR_A: c_int = 38;
pub const AUD_CLKID_FRDDR_B: c_int = 39;
pub const AUD_CLKID_FRDDR_C: c_int = 40;
pub const AUD_CLKID_TODDR_A: c_int = 41;
pub const AUD_CLKID_TODDR_B: c_int = 42;
pub const AUD_CLKID_TODDR_C: c_int = 43;
pub const AUD_CLKID_LOOPBACK: c_int = 44;
pub const AUD_CLKID_SPDIFIN: c_int = 45;
pub const AUD_CLKID_SPDIFOUT: c_int = 46;
pub const AUD_CLKID_RESAMPLE: c_int = 47;
pub const AUD_CLKID_POWER_DETECT: c_int = 48;
pub const AUD_CLKID_MST_A_MCLK: c_int = 49;
pub const AUD_CLKID_MST_B_MCLK: c_int = 50;
pub const AUD_CLKID_MST_C_MCLK: c_int = 51;
pub const AUD_CLKID_MST_D_MCLK: c_int = 52;
pub const AUD_CLKID_MST_E_MCLK: c_int = 53;
pub const AUD_CLKID_MST_F_MCLK: c_int = 54;
pub const AUD_CLKID_SPDIFOUT_CLK: c_int = 55;
pub const AUD_CLKID_SPDIFIN_CLK: c_int = 56;
pub const AUD_CLKID_PDM_DCLK: c_int = 57;
pub const AUD_CLKID_PDM_SYSCLK: c_int = 58;
pub const AUD_CLKID_MST_A_MCLK_SEL: c_int = 59;
pub const AUD_CLKID_MST_B_MCLK_SEL: c_int = 60;
pub const AUD_CLKID_MST_C_MCLK_SEL: c_int = 61;
pub const AUD_CLKID_MST_D_MCLK_SEL: c_int = 62;
pub const AUD_CLKID_MST_E_MCLK_SEL: c_int = 63;
pub const AUD_CLKID_MST_F_MCLK_SEL: c_int = 64;
pub const AUD_CLKID_MST_A_MCLK_DIV: c_int = 65;
pub const AUD_CLKID_MST_B_MCLK_DIV: c_int = 66;
pub const AUD_CLKID_MST_C_MCLK_DIV: c_int = 67;
pub const AUD_CLKID_MST_D_MCLK_DIV: c_int = 68;
pub const AUD_CLKID_MST_E_MCLK_DIV: c_int = 69;
pub const AUD_CLKID_MST_F_MCLK_DIV: c_int = 70;
pub const AUD_CLKID_SPDIFOUT_CLK_SEL: c_int = 71;
pub const AUD_CLKID_SPDIFOUT_CLK_DIV: c_int = 72;
pub const AUD_CLKID_SPDIFIN_CLK_SEL: c_int = 73;
pub const AUD_CLKID_SPDIFIN_CLK_DIV: c_int = 74;
pub const AUD_CLKID_PDM_DCLK_SEL: c_int = 75;
pub const AUD_CLKID_PDM_DCLK_DIV: c_int = 76;
pub const AUD_CLKID_PDM_SYSCLK_SEL: c_int = 77;
pub const AUD_CLKID_PDM_SYSCLK_DIV: c_int = 78;
pub const AUD_CLKID_MST_A_SCLK: c_int = 79;
pub const AUD_CLKID_MST_B_SCLK: c_int = 80;
pub const AUD_CLKID_MST_C_SCLK: c_int = 81;
pub const AUD_CLKID_MST_D_SCLK: c_int = 82;
pub const AUD_CLKID_MST_E_SCLK: c_int = 83;
pub const AUD_CLKID_MST_F_SCLK: c_int = 84;
pub const AUD_CLKID_MST_A_LRCLK: c_int = 86;
pub const AUD_CLKID_MST_B_LRCLK: c_int = 87;
pub const AUD_CLKID_MST_C_LRCLK: c_int = 88;
pub const AUD_CLKID_MST_D_LRCLK: c_int = 89;
pub const AUD_CLKID_MST_E_LRCLK: c_int = 90;
pub const AUD_CLKID_MST_F_LRCLK: c_int = 91;
pub const AUD_CLKID_MST_A_SCLK_PRE_EN: c_int = 92;
pub const AUD_CLKID_MST_B_SCLK_PRE_EN: c_int = 93;
pub const AUD_CLKID_MST_C_SCLK_PRE_EN: c_int = 94;
pub const AUD_CLKID_MST_D_SCLK_PRE_EN: c_int = 95;
pub const AUD_CLKID_MST_E_SCLK_PRE_EN: c_int = 96;
pub const AUD_CLKID_MST_F_SCLK_PRE_EN: c_int = 97;
pub const AUD_CLKID_MST_A_SCLK_DIV: c_int = 98;
pub const AUD_CLKID_MST_B_SCLK_DIV: c_int = 99;
pub const AUD_CLKID_MST_C_SCLK_DIV: c_int = 100;
pub const AUD_CLKID_MST_D_SCLK_DIV: c_int = 101;
pub const AUD_CLKID_MST_E_SCLK_DIV: c_int = 102;
pub const AUD_CLKID_MST_F_SCLK_DIV: c_int = 103;
pub const AUD_CLKID_MST_A_SCLK_POST_EN: c_int = 104;
pub const AUD_CLKID_MST_B_SCLK_POST_EN: c_int = 105;
pub const AUD_CLKID_MST_C_SCLK_POST_EN: c_int = 106;
pub const AUD_CLKID_MST_D_SCLK_POST_EN: c_int = 107;
pub const AUD_CLKID_MST_E_SCLK_POST_EN: c_int = 108;
pub const AUD_CLKID_MST_F_SCLK_POST_EN: c_int = 109;
pub const AUD_CLKID_MST_A_LRCLK_DIV: c_int = 110;
pub const AUD_CLKID_MST_B_LRCLK_DIV: c_int = 111;
pub const AUD_CLKID_MST_C_LRCLK_DIV: c_int = 112;
pub const AUD_CLKID_MST_D_LRCLK_DIV: c_int = 113;
pub const AUD_CLKID_MST_E_LRCLK_DIV: c_int = 114;
pub const AUD_CLKID_MST_F_LRCLK_DIV: c_int = 115;
pub const AUD_CLKID_TDMIN_A_SCLK_SEL: c_int = 116;
pub const AUD_CLKID_TDMIN_B_SCLK_SEL: c_int = 117;
pub const AUD_CLKID_TDMIN_C_SCLK_SEL: c_int = 118;
pub const AUD_CLKID_TDMIN_LB_SCLK_SEL: c_int = 119;
pub const AUD_CLKID_TDMOUT_A_SCLK_SEL: c_int = 120;
pub const AUD_CLKID_TDMOUT_B_SCLK_SEL: c_int = 121;
pub const AUD_CLKID_TDMOUT_C_SCLK_SEL: c_int = 122;
pub const AUD_CLKID_TDMIN_A_SCLK: c_int = 123;
pub const AUD_CLKID_TDMIN_B_SCLK: c_int = 124;
pub const AUD_CLKID_TDMIN_C_SCLK: c_int = 125;
pub const AUD_CLKID_TDMIN_LB_SCLK: c_int = 126;
pub const AUD_CLKID_TDMOUT_A_SCLK: c_int = 127;
pub const AUD_CLKID_TDMOUT_B_SCLK: c_int = 128;
pub const AUD_CLKID_TDMOUT_C_SCLK: c_int = 129;
pub const AUD_CLKID_TDMIN_A_LRCLK: c_int = 130;
pub const AUD_CLKID_TDMIN_B_LRCLK: c_int = 131;
pub const AUD_CLKID_TDMIN_C_LRCLK: c_int = 132;
pub const AUD_CLKID_TDMIN_LB_LRCLK: c_int = 133;
pub const AUD_CLKID_TDMOUT_A_LRCLK: c_int = 134;
pub const AUD_CLKID_TDMOUT_B_LRCLK: c_int = 135;
pub const AUD_CLKID_TDMOUT_C_LRCLK: c_int = 136;
pub const AUD_CLKID_TDMIN_A_SCLK_PRE_EN: c_int = 137;
pub const AUD_CLKID_TDMIN_B_SCLK_PRE_EN: c_int = 138;
pub const AUD_CLKID_TDMIN_C_SCLK_PRE_EN: c_int = 139;
pub const AUD_CLKID_TDMIN_LB_SCLK_PRE_EN: c_int = 140;
pub const AUD_CLKID_TDMOUT_A_SCLK_PRE_EN: c_int = 141;
pub const AUD_CLKID_TDMOUT_B_SCLK_PRE_EN: c_int = 142;
pub const AUD_CLKID_TDMOUT_C_SCLK_PRE_EN: c_int = 143;
pub const AUD_CLKID_TDMIN_A_SCLK_POST_EN: c_int = 144;
pub const AUD_CLKID_TDMIN_B_SCLK_POST_EN: c_int = 145;
pub const AUD_CLKID_TDMIN_C_SCLK_POST_EN: c_int = 146;
pub const AUD_CLKID_TDMIN_LB_SCLK_POST_EN: c_int = 147;
pub const AUD_CLKID_TDMOUT_A_SCLK_POST_EN: c_int = 148;
pub const AUD_CLKID_TDMOUT_B_SCLK_POST_EN: c_int = 149;
pub const AUD_CLKID_TDMOUT_C_SCLK_POST_EN: c_int = 150;
pub const AUD_CLKID_SPDIFOUT_B: c_int = 151;
pub const AUD_CLKID_SPDIFOUT_B_CLK: c_int = 152;
pub const AUD_CLKID_SPDIFOUT_B_CLK_SEL: c_int = 153;
pub const AUD_CLKID_SPDIFOUT_B_CLK_DIV: c_int = 154;
pub const AUD_CLKID_TDM_MCLK_PAD0: c_int = 155;
pub const AUD_CLKID_TDM_MCLK_PAD1: c_int = 156;
pub const AUD_CLKID_TDM_LRCLK_PAD0: c_int = 157;
pub const AUD_CLKID_TDM_LRCLK_PAD1: c_int = 158;
pub const AUD_CLKID_TDM_LRCLK_PAD2: c_int = 159;
pub const AUD_CLKID_TDM_SCLK_PAD0: c_int = 160;
pub const AUD_CLKID_TDM_SCLK_PAD1: c_int = 161;
pub const AUD_CLKID_TDM_SCLK_PAD2: c_int = 162;
pub const AUD_CLKID_TOP: c_int = 163;
pub const AUD_CLKID_TORAM: c_int = 164;
pub const AUD_CLKID_EQDRC: c_int = 165;
pub const AUD_CLKID_RESAMPLE_B: c_int = 166;
pub const AUD_CLKID_TOVAD: c_int = 167;
pub const AUD_CLKID_LOCKER: c_int = 168;
pub const AUD_CLKID_SPDIFIN_LB: c_int = 169;
pub const AUD_CLKID_FRDDR_D: c_int = 170;
pub const AUD_CLKID_TODDR_D: c_int = 171;
pub const AUD_CLKID_LOOPBACK_B: c_int = 172;
pub const AUD_CLKID_CLK81_EN: c_int = 173;
pub const AUD_CLKID_SYSCLK_A_DIV: c_int = 174;
pub const AUD_CLKID_SYSCLK_B_DIV: c_int = 175;
pub const AUD_CLKID_SYSCLK_A_EN: c_int = 176;
pub const AUD_CLKID_SYSCLK_B_EN: c_int = 177;
pub const AUD_CLKID_EARCRX: c_int = 178;
pub const AUD_CLKID_EARCRX_CMDC_SEL: c_int = 179;
pub const AUD_CLKID_EARCRX_CMDC_DIV: c_int = 180;
pub const AUD_CLKID_EARCRX_CMDC: c_int = 181;
pub const AUD_CLKID_EARCRX_DMAC_SEL: c_int = 182;
pub const AUD_CLKID_EARCRX_DMAC_DIV: c_int = 183;
pub const AUD_CLKID_EARCRX_DMAC: c_int = 184;
