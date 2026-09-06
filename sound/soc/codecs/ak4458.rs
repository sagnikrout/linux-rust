//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ak4458.h
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
// Audio driver for AK4458
//
// Copyright (C) 2016 Asahi Kasei Microdevices Corporation
// Copyright 2018 NXP
//

// Settings
pub const AK4458_00_CONTROL1: c_uint = 0x00;
pub const AK4458_01_CONTROL2: c_uint = 0x01;
pub const AK4458_02_CONTROL3: c_uint = 0x02;
pub const AK4458_03_LCHATT: c_uint = 0x03;
pub const AK4458_04_RCHATT: c_uint = 0x04;
pub const AK4458_05_CONTROL4: c_uint = 0x05;
pub const AK4458_06_DSD1: c_uint = 0x06;
pub const AK4458_07_CONTROL5: c_uint = 0x07;
pub const AK4458_08_SOUND_CONTROL: c_uint = 0x08;
pub const AK4458_09_DSD2: c_uint = 0x09;
pub const AK4458_0A_CONTROL6: c_uint = 0x0A;
pub const AK4458_0B_CONTROL7: c_uint = 0x0B;
pub const AK4458_0C_CONTROL8: c_uint = 0x0C;
pub const AK4458_0D_CONTROL9: c_uint = 0x0D;
pub const AK4458_0E_CONTROL10: c_uint = 0x0E;
pub const AK4458_0F_L2CHATT: c_uint = 0x0F;
pub const AK4458_10_R2CHATT: c_uint = 0x10;
pub const AK4458_11_L3CHATT: c_uint = 0x11;
pub const AK4458_12_R3CHATT: c_uint = 0x12;
pub const AK4458_13_L4CHATT: c_uint = 0x13;
pub const AK4458_14_R4CHATT: c_uint = 0x14;
// Bitfield Definitions
// AK4458_00_CONTROL1 (0x00) Fields
// Addr Register Name  D7     D6    D5    D4    D3    D2    D1    D0
// 00H  Control 1      ACKS   0     0     0     DIF2  DIF1  DIF0  RSTN
//
// Digital Filter (SD, SLOW, SSLOW)

// DIF2	1 0
// x	1 0 MSB justified  Figure 3 (default)
// x	1 1 I2S Compliment  Figure 4
//
pub const AK4458_DIF_SHIFT: c_int = 1;

// AK4458_00_CONTROL1 (0x00) D0 bit

// AK4458_0A_CONTROL6 Mode bits
pub const AK4458_MODE_SHIFT: c_int = 6;

// DAC Digital attenuator transition time setting
// Table 19
// Mode	ATS1	ATS2	ATT speed
// 0	0	0	4080/fs
// 1	0	1	2040/fs
// 2	1	0	510/fs
// 3	1	1	255/fs
//
pub const AK4458_ATS_SHIFT: c_int = 6;

