//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas571x.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// TAS571x amplifier audio driver
//
// Copyright (C) 2015 Google, Inc.
//
// device registers
pub const TAS571X_CLK_CTRL_REG: c_uint = 0x00;
pub const TAS571X_DEV_ID_REG: c_uint = 0x01;
pub const TAS571X_ERR_STATUS_REG: c_uint = 0x02;
pub const TAS571X_SYS_CTRL_1_REG: c_uint = 0x03;
pub const TAS571X_SDI_REG: c_uint = 0x04;
pub const TAS571X_SDI_FMT_MASK: c_uint = 0x0f;
pub const TAS571X_SYS_CTRL_2_REG: c_uint = 0x05;
pub const TAS571X_SYS_CTRL_2_SDN_MASK: c_uint = 0x40;
pub const TAS571X_SOFT_MUTE_REG: c_uint = 0x06;
pub const TAS571X_SOFT_MUTE_CH1_SHIFT: c_int = 0;
pub const TAS571X_SOFT_MUTE_CH2_SHIFT: c_int = 1;
pub const TAS571X_SOFT_MUTE_CH3_SHIFT: c_int = 2;
pub const TAS571X_MVOL_REG: c_uint = 0x07;
pub const TAS571X_CH1_VOL_REG: c_uint = 0x08;
pub const TAS571X_CH2_VOL_REG: c_uint = 0x09;
pub const TAS571X_CH3_VOL_REG: c_uint = 0x0a;
pub const TAS571X_VOL_CFG_REG: c_uint = 0x0e;
pub const TAS571X_MODULATION_LIMIT_REG: c_uint = 0x10;
pub const TAS571X_IC_DELAY_CH1_REG: c_uint = 0x11;
pub const TAS571X_IC_DELAY_CH2_REG: c_uint = 0x12;
pub const TAS571X_IC_DELAY_CH3_REG: c_uint = 0x13;
pub const TAS571X_IC_DELAY_CH4_REG: c_uint = 0x14;
pub const TAS571X_PWM_CH_SDN_GROUP_REG: c_uint = 0x19	/* N/A on TAS5717, TAS5719 */;

pub const TAS571X_START_STOP_PERIOD_REG: c_uint = 0x1a;
pub const TAS571X_OSC_TRIM_REG: c_uint = 0x1b;
pub const TAS571X_BKND_ERR_REG: c_uint = 0x1c;
pub const TAS571X_INPUT_MUX_REG: c_uint = 0x20;
pub const TAS571X_CH4_SRC_SELECT_REG: c_uint = 0x21;
pub const TAS571X_PWM_MUX_REG: c_uint = 0x25;
// 20-byte biquad registers
pub const TAS5707_CH1_BQ0_REG: c_uint = 0x29;
pub const TAS5707_CH1_BQ1_REG: c_uint = 0x2a;
pub const TAS5707_CH1_BQ2_REG: c_uint = 0x2b;
pub const TAS5707_CH1_BQ3_REG: c_uint = 0x2c;
pub const TAS5707_CH1_BQ4_REG: c_uint = 0x2d;
pub const TAS5707_CH1_BQ5_REG: c_uint = 0x2e;
pub const TAS5707_CH1_BQ6_REG: c_uint = 0x2f;
pub const TAS5707_CH2_BQ0_REG: c_uint = 0x30;
pub const TAS5707_CH2_BQ1_REG: c_uint = 0x31;
pub const TAS5707_CH2_BQ2_REG: c_uint = 0x32;
pub const TAS5707_CH2_BQ3_REG: c_uint = 0x33;
pub const TAS5707_CH2_BQ4_REG: c_uint = 0x34;
pub const TAS5707_CH2_BQ5_REG: c_uint = 0x35;
pub const TAS5707_CH2_BQ6_REG: c_uint = 0x36;
pub const TAS5717_CH1_BQ0_REG: c_uint = 0x26;
pub const TAS5717_CH1_BQ1_REG: c_uint = 0x27;
pub const TAS5717_CH1_BQ2_REG: c_uint = 0x28;
pub const TAS5717_CH1_BQ3_REG: c_uint = 0x29;
pub const TAS5717_CH1_BQ4_REG: c_uint = 0x2a;
pub const TAS5717_CH1_BQ5_REG: c_uint = 0x2b;
pub const TAS5717_CH1_BQ6_REG: c_uint = 0x2c;
pub const TAS5717_CH1_BQ7_REG: c_uint = 0x2d;
pub const TAS5717_CH1_BQ8_REG: c_uint = 0x2e;
pub const TAS5717_CH1_BQ9_REG: c_uint = 0x2f;
pub const TAS5717_CH2_BQ0_REG: c_uint = 0x30;
pub const TAS5717_CH2_BQ1_REG: c_uint = 0x31;
pub const TAS5717_CH2_BQ2_REG: c_uint = 0x32;
pub const TAS5717_CH2_BQ3_REG: c_uint = 0x33;
pub const TAS5717_CH2_BQ4_REG: c_uint = 0x34;
pub const TAS5717_CH2_BQ5_REG: c_uint = 0x35;
pub const TAS5717_CH2_BQ6_REG: c_uint = 0x36;
pub const TAS5717_CH2_BQ7_REG: c_uint = 0x37;
pub const TAS5717_CH2_BQ8_REG: c_uint = 0x38;
pub const TAS5717_CH2_BQ9_REG: c_uint = 0x39;
pub const TAS5717_CH1_BQ10_REG: c_uint = 0x58;
pub const TAS5717_CH1_BQ11_REG: c_uint = 0x59;
pub const TAS5717_CH4_BQ0_REG: c_uint = 0x5a;
pub const TAS5717_CH4_BQ1_REG: c_uint = 0x5b;
pub const TAS5717_CH2_BQ10_REG: c_uint = 0x5c;
pub const TAS5717_CH2_BQ11_REG: c_uint = 0x5d;
pub const TAS5717_CH3_BQ0_REG: c_uint = 0x5e;
pub const TAS5717_CH3_BQ1_REG: c_uint = 0x5f;
pub const TAS5717_CH1_RIGHT_CH_MIX_REG: c_uint = 0x72;
pub const TAS5717_CH1_LEFT_CH_MIX_REG: c_uint = 0x73;
pub const TAS5717_CH2_LEFT_CH_MIX_REG: c_uint = 0x76;
pub const TAS5717_CH2_RIGHT_CH_MIX_REG: c_uint = 0x77;
pub const TAS5733_CH1_BQ0_REG: c_uint = 0x26;
pub const TAS5733_CH1_BQ1_REG: c_uint = 0x27;
pub const TAS5733_CH1_BQ2_REG: c_uint = 0x28;
pub const TAS5733_CH1_BQ3_REG: c_uint = 0x29;
pub const TAS5733_CH1_BQ4_REG: c_uint = 0x2a;
pub const TAS5733_CH1_BQ5_REG: c_uint = 0x2b;
pub const TAS5733_CH1_BQ6_REG: c_uint = 0x2c;
pub const TAS5733_CH1_BQ7_REG: c_uint = 0x2d;
pub const TAS5733_CH1_BQ8_REG: c_uint = 0x2e;
pub const TAS5733_CH1_BQ9_REG: c_uint = 0x2f;
pub const TAS5733_CH2_BQ0_REG: c_uint = 0x30;
pub const TAS5733_CH2_BQ1_REG: c_uint = 0x31;
pub const TAS5733_CH2_BQ2_REG: c_uint = 0x32;
pub const TAS5733_CH2_BQ3_REG: c_uint = 0x33;
pub const TAS5733_CH2_BQ4_REG: c_uint = 0x34;
pub const TAS5733_CH2_BQ5_REG: c_uint = 0x35;
pub const TAS5733_CH2_BQ6_REG: c_uint = 0x36;
pub const TAS5733_CH2_BQ7_REG: c_uint = 0x37;
pub const TAS5733_CH2_BQ8_REG: c_uint = 0x38;
pub const TAS5733_CH2_BQ9_REG: c_uint = 0x39;
pub const TAS5733_CH1_BQ10_REG: c_uint = 0x58;
pub const TAS5733_CH1_CBQ0_REG: c_uint = 0x59;
pub const TAS5733_CH1_CBQ1_REG: c_uint = 0x5a;
pub const TAS5733_CH1_CBQ2_REG: c_uint = 0x5b;
pub const TAS5733_CH1_CBQ3_REG: c_uint = 0x5c;
pub const TAS5733_CH2_BQ10_REG: c_uint = 0x5d;
pub const TAS5733_CH2_CBQ0_REG: c_uint = 0x5e;
pub const TAS5733_CH2_CBQ1_REG: c_uint = 0x5f;
pub const TAS5733_CH2_CBQ2_REG: c_uint = 0x60;
pub const TAS5733_CH2_CBQ3_REG: c_uint = 0x61;
