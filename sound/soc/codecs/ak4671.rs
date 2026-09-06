//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ak4671.h
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
// ak4671.h  --  audio driver for AK4671
//
// Copyright (C) 2009 Samsung Electronics Co.Ltd
// Author: Joonyoung Shim <jy0922.shim@samsung.com>
//
pub const AK4671_AD_DA_POWER_MANAGEMENT: c_uint = 0x00;
pub const AK4671_PLL_MODE_SELECT0: c_uint = 0x01;
pub const AK4671_PLL_MODE_SELECT1: c_uint = 0x02;
pub const AK4671_FORMAT_SELECT: c_uint = 0x03;
pub const AK4671_MIC_SIGNAL_SELECT: c_uint = 0x04;
pub const AK4671_MIC_AMP_GAIN: c_uint = 0x05;
pub const AK4671_MIXING_POWER_MANAGEMENT0: c_uint = 0x06;
pub const AK4671_MIXING_POWER_MANAGEMENT1: c_uint = 0x07;
pub const AK4671_OUTPUT_VOLUME_CONTROL: c_uint = 0x08;
pub const AK4671_LOUT1_SIGNAL_SELECT: c_uint = 0x09;
pub const AK4671_ROUT1_SIGNAL_SELECT: c_uint = 0x0a;
pub const AK4671_LOUT2_SIGNAL_SELECT: c_uint = 0x0b;
pub const AK4671_ROUT2_SIGNAL_SELECT: c_uint = 0x0c;
pub const AK4671_LOUT3_SIGNAL_SELECT: c_uint = 0x0d;
pub const AK4671_ROUT3_SIGNAL_SELECT: c_uint = 0x0e;
pub const AK4671_LOUT1_POWER_MANAGERMENT: c_uint = 0x0f;
pub const AK4671_LOUT2_POWER_MANAGERMENT: c_uint = 0x10;
pub const AK4671_LOUT3_POWER_MANAGERMENT: c_uint = 0x11;
pub const AK4671_LCH_INPUT_VOLUME_CONTROL: c_uint = 0x12;
pub const AK4671_RCH_INPUT_VOLUME_CONTROL: c_uint = 0x13;
pub const AK4671_ALC_REFERENCE_SELECT: c_uint = 0x14;
pub const AK4671_DIGITAL_MIXING_CONTROL: c_uint = 0x15;
pub const AK4671_ALC_TIMER_SELECT: c_uint = 0x16;
pub const AK4671_ALC_MODE_CONTROL: c_uint = 0x17;
pub const AK4671_MODE_CONTROL1: c_uint = 0x18;
pub const AK4671_MODE_CONTROL2: c_uint = 0x19;
pub const AK4671_LCH_OUTPUT_VOLUME_CONTROL: c_uint = 0x1a;
pub const AK4671_RCH_OUTPUT_VOLUME_CONTROL: c_uint = 0x1b;
pub const AK4671_SIDETONE_A_CONTROL: c_uint = 0x1c;
pub const AK4671_DIGITAL_FILTER_SELECT: c_uint = 0x1d;
pub const AK4671_FIL3_COEFFICIENT0: c_uint = 0x1e;
pub const AK4671_FIL3_COEFFICIENT1: c_uint = 0x1f;
pub const AK4671_FIL3_COEFFICIENT2: c_uint = 0x20;
pub const AK4671_FIL3_COEFFICIENT3: c_uint = 0x21;
pub const AK4671_EQ_COEFFICIENT0: c_uint = 0x22;
pub const AK4671_EQ_COEFFICIENT1: c_uint = 0x23;
pub const AK4671_EQ_COEFFICIENT2: c_uint = 0x24;
pub const AK4671_EQ_COEFFICIENT3: c_uint = 0x25;
pub const AK4671_EQ_COEFFICIENT4: c_uint = 0x26;
pub const AK4671_EQ_COEFFICIENT5: c_uint = 0x27;
pub const AK4671_FIL1_COEFFICIENT0: c_uint = 0x28;
pub const AK4671_FIL1_COEFFICIENT1: c_uint = 0x29;
pub const AK4671_FIL1_COEFFICIENT2: c_uint = 0x2a;
pub const AK4671_FIL1_COEFFICIENT3: c_uint = 0x2b;
pub const AK4671_FIL2_COEFFICIENT0: c_uint = 0x2c;
pub const AK4671_FIL2_COEFFICIENT1: c_uint = 0x2d;
pub const AK4671_FIL2_COEFFICIENT2: c_uint = 0x2e;
pub const AK4671_FIL2_COEFFICIENT3: c_uint = 0x2f;
pub const AK4671_DIGITAL_FILTER_SELECT2: c_uint = 0x30;
pub const AK4671_E1_COEFFICIENT0: c_uint = 0x32;
pub const AK4671_E1_COEFFICIENT1: c_uint = 0x33;
pub const AK4671_E1_COEFFICIENT2: c_uint = 0x34;
pub const AK4671_E1_COEFFICIENT3: c_uint = 0x35;
pub const AK4671_E1_COEFFICIENT4: c_uint = 0x36;
pub const AK4671_E1_COEFFICIENT5: c_uint = 0x37;
pub const AK4671_E2_COEFFICIENT0: c_uint = 0x38;
pub const AK4671_E2_COEFFICIENT1: c_uint = 0x39;
pub const AK4671_E2_COEFFICIENT2: c_uint = 0x3a;
pub const AK4671_E2_COEFFICIENT3: c_uint = 0x3b;
pub const AK4671_E2_COEFFICIENT4: c_uint = 0x3c;
pub const AK4671_E2_COEFFICIENT5: c_uint = 0x3d;
pub const AK4671_E3_COEFFICIENT0: c_uint = 0x3e;
pub const AK4671_E3_COEFFICIENT1: c_uint = 0x3f;
pub const AK4671_E3_COEFFICIENT2: c_uint = 0x40;
pub const AK4671_E3_COEFFICIENT3: c_uint = 0x41;
pub const AK4671_E3_COEFFICIENT4: c_uint = 0x42;
pub const AK4671_E3_COEFFICIENT5: c_uint = 0x43;
pub const AK4671_E4_COEFFICIENT0: c_uint = 0x44;
pub const AK4671_E4_COEFFICIENT1: c_uint = 0x45;
pub const AK4671_E4_COEFFICIENT2: c_uint = 0x46;
pub const AK4671_E4_COEFFICIENT3: c_uint = 0x47;
pub const AK4671_E4_COEFFICIENT4: c_uint = 0x48;
pub const AK4671_E4_COEFFICIENT5: c_uint = 0x49;
pub const AK4671_E5_COEFFICIENT0: c_uint = 0x4a;
pub const AK4671_E5_COEFFICIENT1: c_uint = 0x4b;
pub const AK4671_E5_COEFFICIENT2: c_uint = 0x4c;
pub const AK4671_E5_COEFFICIENT3: c_uint = 0x4d;
pub const AK4671_E5_COEFFICIENT4: c_uint = 0x4e;
pub const AK4671_E5_COEFFICIENT5: c_uint = 0x4f;
pub const AK4671_EQ_CONTROL_250HZ_100HZ: c_uint = 0x50;
pub const AK4671_EQ_CONTROL_3500HZ_1KHZ: c_uint = 0x51;
pub const AK4671_EQ_CONTRO_10KHZ: c_uint = 0x52;
pub const AK4671_PCM_IF_CONTROL0: c_uint = 0x53;
pub const AK4671_PCM_IF_CONTROL1: c_uint = 0x54;
pub const AK4671_PCM_IF_CONTROL2: c_uint = 0x55;
pub const AK4671_DIGITAL_VOLUME_B_CONTROL: c_uint = 0x56;
pub const AK4671_DIGITAL_VOLUME_C_CONTROL: c_uint = 0x57;
pub const AK4671_SIDETONE_VOLUME_CONTROL: c_uint = 0x58;
pub const AK4671_DIGITAL_MIXING_CONTROL2: c_uint = 0x59;
pub const AK4671_SAR_ADC_CONTROL: c_uint = 0x5a;
// Bitfield Definitions
// AK4671_AD_DA_POWER_MANAGEMENT (0x00) Fields
pub const AK4671_PMVCM: c_uint = 0x01;
// AK4671_PLL_MODE_SELECT0 (0x01) Fields
pub const AK4671_PLL: c_uint = 0x0f;

pub const AK4671_FS: c_uint = 0xf0;

// AK4671_PLL_MODE_SELECT1 (0x02) Fields
pub const AK4671_PMPLL: c_uint = 0x01;
pub const AK4671_M_S: c_uint = 0x02;
// AK4671_FORMAT_SELECT (0x03) Fields
pub const AK4671_DIF: c_uint = 0x03;

pub const AK4671_BCKP: c_uint = 0x04;
pub const AK4671_MSBS: c_uint = 0x08;
pub const AK4671_SDOD: c_uint = 0x10;
// AK4671_LOUT2_POWER_MANAGEMENT (0x10) Fields
pub const AK4671_MUTEN: c_uint = 0x04;
