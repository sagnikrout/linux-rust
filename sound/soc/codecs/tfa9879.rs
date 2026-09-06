//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tfa9879.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// tfa9879.h  --  driver for NXP Semiconductors TFA9879
//
// Copyright (C) 2014 Axentia Technologies AB
// Author: Peter Rosin <peda@axentia.se>
//
pub const TFA9879_DEVICE_CONTROL: c_uint = 0x00;
pub const TFA9879_SERIAL_INTERFACE_1: c_uint = 0x01;
pub const TFA9879_PCM_IOM2_FORMAT_1: c_uint = 0x02;
pub const TFA9879_SERIAL_INTERFACE_2: c_uint = 0x03;
pub const TFA9879_PCM_IOM2_FORMAT_2: c_uint = 0x04;
pub const TFA9879_EQUALIZER_A1: c_uint = 0x05;
pub const TFA9879_EQUALIZER_A2: c_uint = 0x06;
pub const TFA9879_EQUALIZER_B1: c_uint = 0x07;
pub const TFA9879_EQUALIZER_B2: c_uint = 0x08;
pub const TFA9879_EQUALIZER_C1: c_uint = 0x09;
pub const TFA9879_EQUALIZER_C2: c_uint = 0x0a;
pub const TFA9879_EQUALIZER_D1: c_uint = 0x0b;
pub const TFA9879_EQUALIZER_D2: c_uint = 0x0c;
pub const TFA9879_EQUALIZER_E1: c_uint = 0x0d;
pub const TFA9879_EQUALIZER_E2: c_uint = 0x0e;
pub const TFA9879_BYPASS_CONTROL: c_uint = 0x0f;
pub const TFA9879_DYNAMIC_RANGE_COMPR: c_uint = 0x10;
pub const TFA9879_BASS_TREBLE: c_uint = 0x11;
pub const TFA9879_HIGH_PASS_FILTER: c_uint = 0x12;
pub const TFA9879_VOLUME_CONTROL: c_uint = 0x13;
pub const TFA9879_MISC_CONTROL: c_uint = 0x14;
pub const TFA9879_MISC_STATUS: c_uint = 0x15;
// TFA9879_DEVICE_CONTROL
pub const TFA9879_INPUT_SEL_MASK: c_uint = 0x0010;
pub const TFA9879_INPUT_SEL_SHIFT: c_int = 4;
pub const TFA9879_OPMODE_MASK: c_uint = 0x0008;
pub const TFA9879_OPMODE_SHIFT: c_int = 3;
pub const TFA9879_RESET_MASK: c_uint = 0x0002;
pub const TFA9879_RESET_SHIFT: c_int = 1;
pub const TFA9879_POWERUP_MASK: c_uint = 0x0001;
pub const TFA9879_POWERUP_SHIFT: c_int = 0;
// TFA9879_SERIAL_INTERFACE
pub const TFA9879_MONO_SEL_MASK: c_uint = 0x0c00;
pub const TFA9879_MONO_SEL_SHIFT: c_int = 10;
pub const TFA9879_MONO_SEL_LEFT: c_int = 0;
pub const TFA9879_MONO_SEL_RIGHT: c_int = 1;
pub const TFA9879_MONO_SEL_BOTH: c_int = 2;
pub const TFA9879_I2S_FS_MASK: c_uint = 0x03c0;
pub const TFA9879_I2S_FS_SHIFT: c_int = 6;
pub const TFA9879_I2S_FS_8000: c_int = 0;
pub const TFA9879_I2S_FS_11025: c_int = 1;
pub const TFA9879_I2S_FS_12000: c_int = 2;
pub const TFA9879_I2S_FS_16000: c_int = 3;
pub const TFA9879_I2S_FS_22050: c_int = 4;
pub const TFA9879_I2S_FS_24000: c_int = 5;
pub const TFA9879_I2S_FS_32000: c_int = 6;
pub const TFA9879_I2S_FS_44100: c_int = 7;
pub const TFA9879_I2S_FS_48000: c_int = 8;
pub const TFA9879_I2S_FS_64000: c_int = 9;
pub const TFA9879_I2S_FS_88200: c_int = 10;
pub const TFA9879_I2S_FS_96000: c_int = 11;
pub const TFA9879_I2S_SET_MASK: c_uint = 0x0038;
pub const TFA9879_I2S_SET_SHIFT: c_int = 3;
pub const TFA9879_I2S_SET_MSB_J_24: c_int = 2;
pub const TFA9879_I2S_SET_I2S_24: c_int = 3;
pub const TFA9879_I2S_SET_LSB_J_16: c_int = 4;
pub const TFA9879_I2S_SET_LSB_J_18: c_int = 5;
pub const TFA9879_I2S_SET_LSB_J_20: c_int = 6;
pub const TFA9879_I2S_SET_LSB_J_24: c_int = 7;
pub const TFA9879_SCK_POL_MASK: c_uint = 0x0004;
pub const TFA9879_SCK_POL_SHIFT: c_int = 2;
pub const TFA9879_SCK_POL_NORMAL: c_int = 0;
pub const TFA9879_SCK_POL_INVERSE: c_int = 1;
pub const TFA9879_I_MODE_MASK: c_uint = 0x0003;
pub const TFA9879_I_MODE_SHIFT: c_int = 0;
pub const TFA9879_I_MODE_I2S: c_int = 0;
pub const TFA9879_I_MODE_PCM_IOM2_SHORT: c_int = 1;
pub const TFA9879_I_MODE_PCM_IOM2_LONG: c_int = 2;
// TFA9879_PCM_IOM2_FORMAT
pub const TFA9879_PCM_FS_MASK: c_uint = 0x0800;
pub const TFA9879_PCM_FS_SHIFT: c_int = 11;
pub const TFA9879_A_LAW_MASK: c_uint = 0x0400;
pub const TFA9879_A_LAW_SHIFT: c_int = 10;
pub const TFA9879_PCM_COMP_MASK: c_uint = 0x0200;
pub const TFA9879_PCM_COMP_SHIFT: c_int = 9;
pub const TFA9879_PCM_DL_MASK: c_uint = 0x0100;
pub const TFA9879_PCM_DL_SHIFT: c_int = 8;
pub const TFA9879_D1_SLOT_MASK: c_uint = 0x00f0;
pub const TFA9879_D1_SLOT_SHIFT: c_int = 4;
pub const TFA9879_D2_SLOT_MASK: c_uint = 0x000f;
pub const TFA9879_D2_SLOT_SHIFT: c_int = 0;
// TFA9879_EQUALIZER_X1
pub const TFA9879_T1_MASK: c_uint = 0x8000;
pub const TFA9879_T1_SHIFT: c_int = 15;
pub const TFA9879_K1M_MASK: c_uint = 0x7ff0;
pub const TFA9879_K1M_SHIFT: c_int = 4;
pub const TFA9879_K1E_MASK: c_uint = 0x000f;
pub const TFA9879_K1E_SHIFT: c_int = 0;
// TFA9879_EQUALIZER_X2
pub const TFA9879_T2_MASK: c_uint = 0x8000;
pub const TFA9879_T2_SHIFT: c_int = 15;
pub const TFA9879_K2M_MASK: c_uint = 0x7800;
pub const TFA9879_K2M_SHIFT: c_int = 11;
pub const TFA9879_K2E_MASK: c_uint = 0x0700;
pub const TFA9879_K2E_SHIFT: c_int = 8;
pub const TFA9879_K0_MASK: c_uint = 0x00fe;
pub const TFA9879_K0_SHIFT: c_int = 1;
pub const TFA9879_S_MASK: c_uint = 0x0001;
pub const TFA9879_S_SHIFT: c_int = 0;
// TFA9879_BYPASS_CONTROL
pub const TFA9879_L_OCP_MASK: c_uint = 0x00c0;
pub const TFA9879_L_OCP_SHIFT: c_int = 6;
pub const TFA9879_L_OTP_MASK: c_uint = 0x0030;
pub const TFA9879_L_OTP_SHIFT: c_int = 4;
pub const TFA9879_CLIPCTRL_MASK: c_uint = 0x0008;
pub const TFA9879_CLIPCTRL_SHIFT: c_int = 3;
pub const TFA9879_HPF_BP_MASK: c_uint = 0x0004;
pub const TFA9879_HPF_BP_SHIFT: c_int = 2;
pub const TFA9879_DRC_BP_MASK: c_uint = 0x0002;
pub const TFA9879_DRC_BP_SHIFT: c_int = 1;
pub const TFA9879_EQ_BP_MASK: c_uint = 0x0001;
pub const TFA9879_EQ_BP_SHIFT: c_int = 0;
// TFA9879_DYNAMIC_RANGE_COMPR
pub const TFA9879_AT_LVL_MASK: c_uint = 0xf000;
pub const TFA9879_AT_LVL_SHIFT: c_int = 12;
pub const TFA9879_AT_RATE_MASK: c_uint = 0x0f00;
pub const TFA9879_AT_RATE_SHIFT: c_int = 8;
pub const TFA9879_RL_LVL_MASK: c_uint = 0x00f0;
pub const TFA9879_RL_LVL_SHIFT: c_int = 4;
pub const TFA9879_RL_RATE_MASK: c_uint = 0x000f;
pub const TFA9879_RL_RATE_SHIFT: c_int = 0;
// TFA9879_BASS_TREBLE
pub const TFA9879_G_TRBLE_MASK: c_uint = 0x3e00;
pub const TFA9879_G_TRBLE_SHIFT: c_int = 9;
pub const TFA9879_F_TRBLE_MASK: c_uint = 0x0180;
pub const TFA9879_F_TRBLE_SHIFT: c_int = 7;
pub const TFA9879_G_BASS_MASK: c_uint = 0x007c;
pub const TFA9879_G_BASS_SHIFT: c_int = 2;
pub const TFA9879_F_BASS_MASK: c_uint = 0x0003;
pub const TFA9879_F_BASS_SHIFT: c_int = 0;
// TFA9879_HIGH_PASS_FILTER
pub const TFA9879_HP_CTRL_MASK: c_uint = 0x00ff;
pub const TFA9879_HP_CTRL_SHIFT: c_int = 0;
// TFA9879_VOLUME_CONTROL
pub const TFA9879_ZR_CRSS_MASK: c_uint = 0x1000;
pub const TFA9879_ZR_CRSS_SHIFT: c_int = 12;
pub const TFA9879_VOL_MASK: c_uint = 0x00ff;
pub const TFA9879_VOL_SHIFT: c_int = 0;
// TFA9879_MISC_CONTROL
pub const TFA9879_DE_PHAS_MASK: c_uint = 0x0c00;
pub const TFA9879_DE_PHAS_SHIFT: c_int = 10;
pub const TFA9879_H_MUTE_MASK: c_uint = 0x0200;
pub const TFA9879_H_MUTE_SHIFT: c_int = 9;
pub const TFA9879_S_MUTE_MASK: c_uint = 0x0100;
pub const TFA9879_S_MUTE_SHIFT: c_int = 8;
pub const TFA9879_P_LIM_MASK: c_uint = 0x00ff;
pub const TFA9879_P_LIM_SHIFT: c_int = 0;
// TFA9879_MISC_STATUS
pub const TFA9879_PS_MASK: c_uint = 0x4000;
pub const TFA9879_PS_SHIFT: c_int = 14;
pub const TFA9879_PORA_MASK: c_uint = 0x2000;
pub const TFA9879_PORA_SHIFT: c_int = 13;
pub const TFA9879_AMP_MASK: c_uint = 0x0600;
pub const TFA9879_AMP_SHIFT: c_int = 9;
pub const TFA9879_IBP_2_MASK: c_uint = 0x0100;
pub const TFA9879_IBP_2_SHIFT: c_int = 8;
pub const TFA9879_OFP_2_MASK: c_uint = 0x0080;
pub const TFA9879_OFP_2_SHIFT: c_int = 7;
pub const TFA9879_UFP_2_MASK: c_uint = 0x0040;
pub const TFA9879_UFP_2_SHIFT: c_int = 6;
pub const TFA9879_IBP_1_MASK: c_uint = 0x0020;
pub const TFA9879_IBP_1_SHIFT: c_int = 5;
pub const TFA9879_OFP_1_MASK: c_uint = 0x0010;
pub const TFA9879_OFP_1_SHIFT: c_int = 4;
pub const TFA9879_UFP_1_MASK: c_uint = 0x0008;
pub const TFA9879_UFP_1_SHIFT: c_int = 3;
pub const TFA9879_OCPOKA_MASK: c_uint = 0x0004;
pub const TFA9879_OCPOKA_SHIFT: c_int = 2;
pub const TFA9879_OCPOKB_MASK: c_uint = 0x0002;
pub const TFA9879_OCPOKB_SHIFT: c_int = 1;
pub const TFA9879_OTPOK_MASK: c_uint = 0x0001;
pub const TFA9879_OTPOK_SHIFT: c_int = 0;
