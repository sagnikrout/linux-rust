//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1019.h
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
// rt1019.h  --  RT1019 ALSA SoC audio amplifier driver
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//
pub const RT1019_DEVICE_ID_VAL: c_uint = 0x1019;
pub const RT1019_DEVICE_ID_VAL2: c_uint = 0x6731;
pub const RT1019_RESET: c_uint = 0x0000;
pub const RT1019_IDS_CTRL: c_uint = 0x0011;
pub const RT1019_ASEL_CTRL: c_uint = 0x0013;
pub const RT1019_PWR_STRP_2: c_uint = 0x0019;
pub const RT1019_BEEP_TONE: c_uint = 0x001b;
pub const RT1019_VER_ID: c_uint = 0x005c;
pub const RT1019_VEND_ID_1: c_uint = 0x005e;
pub const RT1019_VEND_ID_2: c_uint = 0x005f;
pub const RT1019_DEV_ID_1: c_uint = 0x0061;
pub const RT1019_DEV_ID_2: c_uint = 0x0062;
pub const RT1019_SDB_CTRL: c_uint = 0x0066;
pub const RT1019_CLK_TREE_1: c_uint = 0x0100;
pub const RT1019_CLK_TREE_2: c_uint = 0x0101;
pub const RT1019_CLK_TREE_3: c_uint = 0x0102;
pub const RT1019_PLL_1: c_uint = 0x0311;
pub const RT1019_PLL_2: c_uint = 0x0312;
pub const RT1019_PLL_3: c_uint = 0x0313;
pub const RT1019_TDM_1: c_uint = 0x0400;
pub const RT1019_TDM_2: c_uint = 0x0401;
pub const RT1019_TDM_3: c_uint = 0x0402;
pub const RT1019_DMIX_MONO_1: c_uint = 0x0504;
pub const RT1019_DMIX_MONO_2: c_uint = 0x0505;
pub const RT1019_BEEP_1: c_uint = 0x0b00;
pub const RT1019_BEEP_2: c_uint = 0x0b01;
// 0x0019 Power On Strap Control-2

// 0x0100 Clock Tree Control-1

pub const RT1019_CLK_SYS_PRE_SEL_SFT: c_int = 7;

pub const RT1019_PLL_SRC_SFT: c_int = 4;

// 0x0101 clock tree control-2

pub const RT1019_ASRC_256FS_MASK: c_uint = 0x3;
pub const RT1019_ASRC_256FS_DIV1: c_uint = 0x0;
pub const RT1019_ASRC_256FS_DIV2: c_uint = 0x1;
pub const RT1019_ASRC_256FS_DIV4: c_uint = 0x2;
// 0x0102 clock tree control-3

// 0x0311 PLL-1

pub const RT1019_PLL_M_SFT: c_int = 4;

pub const RT1019_PLL_M_BP_SFT: c_int = 1;

// 0x0312 PLL-2
pub const RT1019_PLL_Q_7_0_MASK: c_uint = 0xff;
// 0x0313 PLL-3
pub const RT1019_PLL_K_MASK: c_uint = 0x1f;
// 0x0400 TDM Control-1

// 0x0401 TDM Control-2

pub const RT1019_I2S_CH_TX_SFT: c_int = 6;

pub const RT1019_I2S_DF_SFT: c_int = 3;

pub const RT1019_I2S_DL_MASK: c_uint = 0x7;
pub const RT1019_I2S_DL_SFT: c_int = 0;
pub const RT1019_I2S_DL_16: c_uint = 0x0;
pub const RT1019_I2S_DL_20: c_uint = 0x1;
pub const RT1019_I2S_DL_24: c_uint = 0x2;
pub const RT1019_I2S_DL_32: c_uint = 0x3;
pub const RT1019_I2S_DL_8: c_uint = 0x4;
// TDM1 Control-3 (0x0402)

pub const RT1019_TDM_I2S_TX_R_DAC1_1_MASK: c_uint = 0x7;
pub const RT1019_TDM_I2S_TX_L_DAC1_1_SFT: c_int = 4;
pub const RT1019_TDM_I2S_TX_R_DAC1_1_SFT: c_int = 0;
// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1019_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub bclk_ratio: c_uint,
}
