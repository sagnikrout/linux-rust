//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rk3308_codec.h
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
// Rockchip RK3308 internal audio codec driver -- register definitions
//
// Copyright (c) 2018, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.
// Copyright (c) 2022, Vivax-Metrotech Ltd
//
pub const RK3308_GLB_CON: c_uint = 0x00;
// ADC DIGITAL REGISTERS
//
// The ADC group are 0 ~ 3, that control:
//
// CH0: left_0(ADC1) and right_0(ADC2)
// CH1: left_1(ADC3) and right_1(ADC4)
// CH2: left_2(ADC5) and right_2(ADC6)
// CH3: left_3(ADC7) and right_3(ADC8)
//

// DAC DIGITAL REGISTERS
pub const RK3308_DAC_DIG_OFFSET: c_uint = 0x300;

// ADC ANALOG REGISTERS
//
// The ADC group are 0 ~ 3, that control:
//
// CH0: left_0(ADC1) and right_0(ADC2)
// CH1: left_1(ADC3) and right_1(ADC4)
// CH2: left_2(ADC5) and right_2(ADC6)
// CH3: left_3(ADC7) and right_3(ADC8)
//

// DAC ANALOG REGISTERS
pub const RK3308_DAC_ANA_OFFSET: c_uint = 0x440;

//
// These are the bits for registers
//
// RK3308_GLB_CON - REG: 0x0000

// RK3308_ADC_DIG_CON01 - REG: 0x0004

pub const RK3308_ADC_I2S_VALID_LEN_SFT: c_int = 5;

pub const RK3308_ADC_I2S_MODE_SFT: c_int = 3;

// RK3308_ADC_DIG_CON02 - REG: 0x0008

pub const RK3308_ADC_I2S_FRAME_LEN_SFT: c_int = 2;

// RK3308_ADC_DIG_CON03 - REG: 0x000c
pub const RK3308_ADC_L_CH_BIST_SFT: c_int = 2;

pub const RK3308_ADC_R_CH_BIST_SFT: c_int = 0;

// RK3308_ADC_DIG_CON04 - REG: 0x0010

pub const RK3308_ADC_HPF_CUTOFF_SFT: c_int = 0;

// RK3308_ADC_DIG_CON07 - REG: 0x001c
pub const RK3308_ADCL_DATA_SFT: c_int = 4;
pub const RK3308_ADCR_DATA_SFT: c_int = 2;

//
// RK3308_ALC_L_DIG_CON00 - REG: 0x0040 + ch * 0xc0
// RK3308_ALC_R_DIG_CON00 - REG: 0x0080 + ch * 0xc0
//

pub const RK3308_CTRL_GEN_SFT: c_int = 4;

pub const RK3308_AGC_HOLD_TIME_SFT: c_int = 0;

//
// RK3308_ALC_L_DIG_CON01 - REG: 0x0044 + ch * 0xc0
// RK3308_ALC_R_DIG_CON01 - REG: 0x0084 + ch * 0xc0
//
pub const RK3308_AGC_DECAY_TIME_SFT: c_int = 4;
pub const RK3308_AGC_ATTACK_TIME_SFT: c_int = 0;
//
// RK3308_ALC_L_DIG_CON02 - REG: 0x0048 + ch * 0xc0
// RK3308_ALC_R_DIG_CON02 - REG: 0x0088 + ch * 0xc0
//

pub const RK3308_AGC_NOISE_GATE_THRESH_SFT: c_int = 0;

//
// RK3308_ALC_L_DIG_CON03 - REG: 0x004c + ch * 0xc0
// RK3308_ALC_R_DIG_CON03 - REG: 0x008c + ch * 0xc0
//

pub const RK3308_AGC_PGA_GAIN_MAX: c_uint = 0x1f;
pub const RK3308_AGC_PGA_GAIN_MIN: c_int = 0;
pub const RK3308_AGC_PGA_GAIN_SFT: c_int = 0;
//
// RK3308_ALC_L_DIG_CON04 - REG: 0x0050 + ch * 0xc0
// RK3308_ALC_R_DIG_CON04 - REG: 0x0090 + ch * 0xc0
//

pub const RK3308_AGC_APPROX_RATE_SFT: c_int = 0;

//
// RK3308_ALC_L_DIG_CON05 - REG: 0x0054 + ch * 0xc0
// RK3308_ALC_R_DIG_CON05 - REG: 0x0094 + ch * 0xc0
//
pub const RK3308_AGC_LO_8BITS_AGC_MAX_MSK: c_uint = 0xff;
//
// RK3308_ALC_L_DIG_CON06 - REG: 0x0058 + ch * 0xc0
// RK3308_ALC_R_DIG_CON06 - REG: 0x0098 + ch * 0xc0
//
pub const RK3308_AGC_HI_8BITS_AGC_MAX_MSK: c_uint = 0xff;
//
// RK3308_ALC_L_DIG_CON07 - REG: 0x005c + ch * 0xc0
// RK3308_ALC_R_DIG_CON07 - REG: 0x009c + ch * 0xc0
//
pub const RK3308_AGC_LO_8BITS_AGC_MIN_MSK: c_uint = 0xff;
//
// RK3308_ALC_L_DIG_CON08 - REG: 0x0060 + ch * 0xc0
// RK3308_ALC_R_DIG_CON08 - REG: 0x00a0 + ch * 0xc0
//
pub const RK3308_AGC_HI_8BITS_AGC_MIN_MSK: c_uint = 0xff;
//
// RK3308_ALC_L_DIG_CON09 - REG: 0x0064 + ch * 0xc0
// RK3308_ALC_R_DIG_CON09 - REG: 0x00a4 + ch * 0xc0
//

pub const RK3308_AGC_MAX_GAIN_PGA_MAX: c_uint = 0x7;
pub const RK3308_AGC_MAX_GAIN_PGA_MIN: c_int = 0;
pub const RK3308_AGC_MAX_GAIN_PGA_SFT: c_int = 3;

pub const RK3308_AGC_MIN_GAIN_PGA_MAX: c_uint = 0x7;
pub const RK3308_AGC_MIN_GAIN_PGA_MIN: c_int = 0;
pub const RK3308_AGC_MIN_GAIN_PGA_SFT: c_int = 0;

//
// RK3308_ALC_L_DIG_CON12 - REG: 0x0068 + ch * 0xc0
// RK3308_ALC_R_DIG_CON12 - REG: 0x00a8 + ch * 0xc0
//
pub const RK3308_AGC_GAIN_MSK: c_uint = 0x1f;
// RK3308_DAC_DIG_CON01 - REG: 0x0304

pub const RK3308_DAC_I2S_VALID_LEN_SFT: c_int = 5;

pub const RK3308_DAC_I2S_MODE_SFT: c_int = 3;

// RK3308_DAC_DIG_CON02 - REG: 0x0308

pub const RK3308_DAC_I2S_FRAME_LEN_SFT: c_int = 2;

// RK3308_DAC_DIG_CON03 - REG: 0x030C
pub const RK3308_DAC_L_CH_BIST_SFT: c_int = 2;

pub const RK3308_DAC_R_CH_BIST_SFT: c_int = 0;

// RK3308_DAC_DIG_CON04 - REG: 0x0310
// Versions up to B:
pub const RK3308_DAC_MODULATOR_GAIN_SFT: c_int = 4;

pub const RK3308_DAC_CIC_IF_GAIN_SFT: c_int = 0;

// Version C:
pub const RK3308BS_DAC_DIG_GAIN_SFT: c_int = 0;

// RK3308BS_ADC_DIG_CON05..06 (Version C only)
pub const RK3308_ADC_DIG_VOL_CON_x_SFT: c_int = 0;

// RK3308_DAC_DIG_CON05 - REG: 0x0314

// RK3308_DAC_DIG_CON10 - REG: 0x0328

// RK3308_DAC_DIG_CON11 - REG: 0x032c

// RK3308_ADC_ANA_CON00 - REG: 0x0340

pub const RK3308_ADC_CH1_CH2_MIC_ALL: c_uint = 0xff;

// RK3308_ADC_ANA_CON01 - REG: 0x0344
//
// The PGA of MIC-INs:
// - HW version A:
// 0x0 - MIC1~MIC8  0 dB (recommended when ADC used as loopback)
// 0x3 - MIC1~MIC8 20 dB (recommended when ADC used as MIC input)
// - HW version B:
// 0x0 - MIC1~MIC8   0 dB
// 0x1 - MIC1~MIC8 6.6 dB
// 0x2 - MIC1~MIC8  13 dB
// 0x3 - MIC1~MIC8  20 dB
//
pub const RK3308_ADC_CH2_MIC_GAIN_MAX: c_uint = 0x3;
pub const RK3308_ADC_CH2_MIC_GAIN_MIN: c_int = 0;
pub const RK3308_ADC_CH2_MIC_GAIN_SFT: c_int = 4;

pub const RK3308_ADC_CH1_MIC_GAIN_MAX: c_uint = 0x3;
pub const RK3308_ADC_CH1_MIC_GAIN_MIN: c_int = 0;
pub const RK3308_ADC_CH1_MIC_GAIN_SFT: c_int = 0;

// RK3308_ADC_ANA_CON02 - REG: 0x0348

// RK3308_ADC_ANA_CON03 - REG: 0x034c
pub const RK3308_ADC_CH1_ALC_GAIN_MAX: c_uint = 0x1f;
pub const RK3308_ADC_CH1_ALC_GAIN_MIN: c_int = 0;
pub const RK3308_ADC_CH1_ALC_GAIN_SFT: c_int = 0;

// RK3308_ADC_ANA_CON04 - REG: 0x0350
pub const RK3308_ADC_CH2_ALC_GAIN_MAX: c_uint = 0x1f;
pub const RK3308_ADC_CH2_ALC_GAIN_MIN: c_int = 0;
pub const RK3308_ADC_CH2_ALC_GAIN_SFT: c_int = 0;

// RK3308_ADC_ANA_CON05 - REG: 0x0354

// RK3308_ADC_ANA_CON06 - REG: 0x0358

// RK3308_ADC_ANA_CON07 - REG: 0x035c
// Note: The register configuration is only valid for ADC2
pub const RK3308_ADC_CH2_IN_SEL_SFT: c_int = 6;

// Note: The register configuration is only valid for ADC1
pub const RK3308_ADC_CH1_IN_SEL_SFT: c_int = 4;

pub const RK3308_ADC_LEVEL_RANGE_MICBIAS_MAX: c_int = 7;
pub const RK3308_ADC_LEVEL_RANGE_MICBIAS_SFT: c_int = 0;

// RK3308_ADC_ANA_CON08 - REG: 0x0360

// RK3308_ADC_ANA_CON10 - REG: 0x0368

pub const RK3308_ADC_CURRENT_CHARGE_SFT: c_int = 0;

// RK3308_ADC_ANA_CON11 - REG: 0x036c

// RK3308_DAC_ANA_CON00 - REG: 0x0440

// RK3308_DAC_ANA_CON01 - REG: 0x0444

pub const RK3308_DAC_HPOUT_POP_SOUND_R_SFT: c_int = 4;
pub const RK3308_DAC_HPOUT_POP_SOUND_L_SFT: c_int = 0;
// unshifted values for both L and R:
pub const RK3308_DAC_HPOUT_POP_SOUND_x_MSK: c_uint = 0x3;
pub const RK3308_DAC_HPOUT_POP_SOUND_x_WORK: c_uint = 0x2;
pub const RK3308_DAC_HPOUT_POP_SOUND_x_INIT: c_uint = 0x1;
// RK3308_DAC_ANA_CON02 - REG: 0x0448

// RK3308_DAC_ANA_CON03 - REG: 0x044c

pub const RK3308_DAC_R_HPOUT_MUTE_SFT: c_int = 4;

pub const RK3308_DAC_L_HPOUT_MUTE_SFT: c_int = 0;
// RK3308_DAC_ANA_CON04 - REG: 0x0450
pub const RK3308_DAC_x_LINEOUT_GAIN_MAX: c_uint = 0x3;
pub const RK3308_DAC_R_LINEOUT_GAIN_SFT: c_int = 6;

pub const RK3308_DAC_R_LINEOUT_MUTE_SFT: c_int = 5;

pub const RK3308_DAC_L_LINEOUT_GAIN_SFT: c_int = 2;

pub const RK3308_DAC_L_LINEOUT_MUTE_SFT: c_int = 1;

// RK3308_DAC_ANA_CON05 - REG: 0x0454, step is 1.5db
// RK3308_DAC_ANA_CON06 - REG: 0x0458, step is 1.5db
pub const RK3308_DAC_x_HPOUT_GAIN_MAX: c_uint = 0x1e;
pub const RK3308_DAC_x_HPOUT_GAIN_SFT: c_int = 0;

// RK3308_DAC_ANA_CON07 - REG: 0x045c
pub const RK3308_DAC_R_HPOUT_DRV_SFT: c_int = 4;

pub const RK3308_DAC_L_HPOUT_DRV_SFT: c_int = 0;

// RK3308_DAC_ANA_CON08 - REG: 0x0460
pub const RK3308_DAC_R_LINEOUT_DRV_SFT: c_int = 4;

pub const RK3308_DAC_L_LINEOUT_DRV_SFT: c_int = 0;

// RK3308_DAC_ANA_CON12 - REG: 0x0470
pub const RK3308_DAC_R_HPMIX_SEL_SFT: c_int = 6;

pub const RK3308_DAC_L_HPMIX_SEL_SFT: c_int = 2;

pub const RK3308_DAC_x_HPMIX_GAIN_MIN: c_uint = 0x1 /* 0x0 and 0x3 are reserved */;
pub const RK3308_DAC_x_HPMIX_GAIN_MAX: c_uint = 0x2;
pub const RK3308_DAC_R_HPMIX_GAIN_SFT: c_int = 4;

pub const RK3308_DAC_L_HPMIX_GAIN_SFT: c_int = 0;

// RK3308_DAC_ANA_CON13 - REG: 0x0474

// RK3308_DAC_ANA_CON14 - REG: 0x0478

pub const RK3308_DAC_CURRENT_CHARGE_SFT: c_int = 0;

// RK3308_DAC_ANA_CON15 - REG: 0x047C
pub const RK3308_DAC_LINEOUT_POP_SOUND_R_SFT: c_int = 4;

pub const RK3308_DAC_LINEOUT_POP_SOUND_L_SFT: c_int = 0;

