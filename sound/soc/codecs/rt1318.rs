//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1318.h
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
// rt1318.h -- Platform data for RT1318
//
// Copyright 2024 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1318_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt1318_platform_data,
    pub cali_work: work_struct,
    pub regmap: *mut regmap,
    pub r0_l_integer: c_uint,
    pub r0_l_factor: c_uint,
    pub r0_r_integer: c_uint,
    pub r0_r_factor: c_uint,
    pub rt1318_init: c_int,
    pub rt1318_dvol: c_int,
    pub sysclk_src: c_int,
    pub sysclk: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub master: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
}

pub const RT1318_PLL_INP_MAX: c_int = 40000000;
pub const RT1318_PLL_INP_MIN: c_int = 256000;
pub const RT1318_PLL_N_MAX: c_uint = 0x1ff;
pub const RT1318_PLL_K_MAX: c_uint = 0x1f;
pub const RT1318_PLL_M_MAX: c_uint = 0x1f;
pub const RT1318_LRCLK_192000: c_int = 192000;
pub const RT1318_LRCLK_96000: c_int = 96000;
pub const RT1318_LRCLK_48000: c_int = 48000;
pub const RT1318_LRCLK_44100: c_int = 44100;
pub const RT1318_LRCLK_16000: c_int = 16000;
pub const RT1318_DVOL_STEP: c_int = 383;
pub const RT1318_CLK1: c_uint = 0xc001;
pub const RT1318_CLK2: c_uint = 0xc003;
pub const RT1318_CLK3: c_uint = 0xc004;
pub const RT1318_CLK4: c_uint = 0xc005;
pub const RT1318_CLK5: c_uint = 0xc006;
pub const RT1318_CLK6: c_uint = 0xc007;
pub const RT1318_CLK7: c_uint = 0xc008;
pub const RT1318_PWR_STA1: c_uint = 0xc121;
pub const RT1318_SPK_VOL_TH: c_uint = 0xc130;
pub const RT1318_TCON: c_uint = 0xc203;
pub const RT1318_SRC_TCON: c_uint = 0xc204;
pub const RT1318_TCON_RELATE: c_uint = 0xc206;
pub const RT1318_DA_VOL_L_8: c_uint = 0xc20b;
pub const RT1318_DA_VOL_L_1_7: c_uint = 0xc20c;
pub const RT1318_DA_VOL_R_8: c_uint = 0xc20d;
pub const RT1318_DA_VOL_R_1_7: c_uint = 0xc20e;
pub const RT1318_FEEDBACK_PATH: c_uint = 0xc321;
pub const RT1318_STP_TEMP_L: c_uint = 0xdb00;
pub const RT1318_STP_SEL_L: c_uint = 0xdb08;
pub const RT1318_STP_R0_EN_L: c_uint = 0xdb12;
pub const RT1318_R0_CMP_L_FLAG: c_uint = 0xdb35;
pub const RT1318_PRE_R0_L_24: c_uint = 0xdbb5;
pub const RT1318_PRE_R0_L_23_16: c_uint = 0xdbb6;
pub const RT1318_PRE_R0_L_15_8: c_uint = 0xdbb7;
pub const RT1318_PRE_R0_L_7_0: c_uint = 0xdbb8;
pub const RT1318_R0_L_24: c_uint = 0xdbc5;
pub const RT1318_R0_L_23_16: c_uint = 0xdbc6;
pub const RT1318_R0_L_15_8: c_uint = 0xdbc7;
pub const RT1318_R0_L_7_0: c_uint = 0xdbc8;
pub const RT1318_STP_SEL_R: c_uint = 0xdd08;
pub const RT1318_STP_R0_EN_R: c_uint = 0xdd12;
pub const RT1318_R0_CMP_R_FLAG: c_uint = 0xdd35;
pub const RT1318_PRE_R0_R_24: c_uint = 0xddb5;
pub const RT1318_PRE_R0_R_23_16: c_uint = 0xddb6;
pub const RT1318_PRE_R0_R_15_8: c_uint = 0xddb7;
pub const RT1318_PRE_R0_R_7_0: c_uint = 0xddb8;
pub const RT1318_R0_R_24: c_uint = 0xddc5;
pub const RT1318_R0_R_23_16: c_uint = 0xddc6;
pub const RT1318_R0_R_15_8: c_uint = 0xddc7;
pub const RT1318_R0_R_7_0: c_uint = 0xddc8;
pub const RT1318_DEV_ID1: c_uint = 0xf012;
pub const RT1318_DEV_ID2: c_uint = 0xf013;
pub const RT1318_PLL1_K: c_uint = 0xf20d;
pub const RT1318_PLL1_M: c_uint = 0xf20f;
pub const RT1318_PLL1_N_8: c_uint = 0xf211;
pub const RT1318_PLL1_N_7_0: c_uint = 0xf212;
pub const RT1318_SINE_GEN0: c_uint = 0xf800;
pub const RT1318_TDM_CTRL1: c_uint = 0xf900;
pub const RT1318_TDM_CTRL2: c_uint = 0xf901;
pub const RT1318_TDM_CTRL3: c_uint = 0xf902;
pub const RT1318_TDM_CTRL9: c_uint = 0xf908;
// Clock-1  (0xC001)

// Clock-2  (0xC003)

pub const RT1318_DIV_AP_SFT: c_int = 4;

pub const RT1318_DIV_DAMOD_SFT: c_int = 0;

// Clock-3  (0xC004)

pub const RT1318_AD_STO1_SFT: c_int = 4;

pub const RT1318_AD_STO2_SFT: c_int = 0;

pub const RT1318_AD_STO2_SFT: c_int = 0;
// Clock-4  (0xC005)

pub const RT1318_AD_ANA_STO1_SFT: c_int = 4;

pub const RT1318_AD_ANA_STO2_SFT: c_int = 0;
// Clock-5  (0xC006)

pub const RT1318_DIV_FIFO_IN_SFT: c_int = 4;

pub const RT1318_DIV_FIFO_OUT_SFT: c_int = 0;
// Clock-6  (0xC007)

pub const RT1318_DIV_NLMS_SFT: c_int = 6;

pub const RT1318_DIV_AD_MONO_SFT: c_int = 3;

pub const RT1318_DIV_POST_G_SFT: c_int = 0;

// Power Status 1  (0xC121)

pub const RT1318_PDB_CTRL_SFT: c_int = 0;
// SRC Tcon(0xc204)

// R0 Compare Flag  (0xDB35)

// PLL internal setting (0xF20D), K value

// PLL internal setting (0xF20F), M value

// PLL internal setting (0xF211), N_8 value

// PLL internal setting (0xF212), N_7_0 value

// TDM CTRL 1  (0xf900)

pub const RT1318_I2S_FMT_SFT: c_int = 0;
// TDM CTRL 2  (0xf901)

pub const RT1318_I2S_DL_MASK: c_uint = 0x7;
pub const RT1318_I2S_DL_SFT: c_int = 0;
pub const RT1318_I2S_DL_16: c_uint = 0x0;
pub const RT1318_I2S_DL_20: c_uint = 0x1;
pub const RT1318_I2S_DL_24: c_uint = 0x2;
pub const RT1318_I2S_DL_32: c_uint = 0x3;
pub const RT1318_I2S_DL_8: c_uint = 0x4;
// TDM CTRL 3  (0xf902)

pub const RT1318_I2S_TX_CHL_SFT: c_int = 4;

pub const RT1318_I2S_RX_CHL_SFT: c_int = 0;

// TDM CTRL 9  (0xf908)

pub const RT1318_TDM_I2S_TX_R_DAC1_1_MASK: c_uint = 0x7;
pub const RT1318_TDM_I2S_TX_L_DAC1_1_SFT: c_int = 4;
pub const RT1318_TDM_I2S_TX_R_DAC1_1_SFT: c_int = 0;
pub const RT1318_REG_DISP_LEN: c_int = 23;
// System Clock Source
// PLL Source
// TDM channel
// R0 calibration result
// PLL pre-defined M/N/K
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_calc_map {
    pub pll_in: c_uint,
    pub pll_out: c_uint,
    pub k: c_int,
    pub n: c_int,
    pub m: c_int,
    pub m_bp: bool,
    pub k_bp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1318_pll_code {
    pub /: *mut *mut bool m_bp; / Indicates bypass m code or not.,
    pub /: *mut *mut bool k_bp; / Indicates bypass k code or not.,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}
