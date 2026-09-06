//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sma1303.h
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
// sma1303.h -- sma1303 ALSA SoC Audio driver
//
// Copyright 2023 Iron Device Corporation
//
// Author: Kiseok Jo <kiseok.jo@irondevice.com>
//
pub const SMA1303_I2C_ADDR_00: c_uint = 0x1e;
pub const SMA1303_I2C_ADDR_01: c_uint = 0x3e;
pub const SMA1303_I2C_ADDR_10: c_uint = 0x5e;
pub const SMA1303_I2C_ADDR_11: c_uint = 0x7e;
pub const SMA1303_EXTERNAL_CLOCK_19_2: c_uint = 0x00;
pub const SMA1303_EXTERNAL_CLOCK_24_576: c_uint = 0x01;
pub const SMA1303_PLL_CLKIN_MCLK: c_uint = 0x02;
pub const SMA1303_PLL_CLKIN_BCLK: c_uint = 0x03;
pub const SMA1303_MONO: c_uint = 0x00;
pub const SMA1303_STEREO: c_uint = 0x01;
pub const SMA1303_I2C_RETRY_COUNT: c_int = 3;
//
// SMA1303 Register Definition
//
// SMA1303 Register Addresses
pub const SMA1303_00_SYSTEM_CTRL: c_uint = 0x00;
pub const SMA1303_01_INPUT1_CTRL1: c_uint = 0x01;
pub const SMA1303_02_INPUT1_CTRL2: c_uint = 0x02;
pub const SMA1303_03_INPUT1_CTRL3: c_uint = 0x03;
pub const SMA1303_04_INPUT1_CTRL4: c_uint = 0x04;
// 0x05 ~ 0x08 : Reserved
pub const SMA1303_09_OUTPUT_CTRL: c_uint = 0x09;
pub const SMA1303_0A_SPK_VOL: c_uint = 0x0a;
pub const SMA1303_0B_BST_TEST: c_uint = 0x0b;
pub const SMA1303_0C_BST_TEST1: c_uint = 0x0c;
pub const SMA1303_0D_SPK_TEST: c_uint = 0x0d;
pub const SMA1303_0E_MUTE_VOL_CTRL: c_uint = 0x0e;
// 0x0F : Reserved
pub const SMA1303_10_SYSTEM_CTRL1: c_uint = 0x10;
pub const SMA1303_11_SYSTEM_CTRL2: c_uint = 0x11;
pub const SMA1303_12_SYSTEM_CTRL3: c_uint = 0x12;
// 0x13 : Reserved
pub const SMA1303_14_MODULATOR: c_uint = 0x14;
pub const SMA1303_15_BASS_SPK1: c_uint = 0x15;
pub const SMA1303_16_BASS_SPK2: c_uint = 0x16;
pub const SMA1303_17_BASS_SPK3: c_uint = 0x17;
pub const SMA1303_18_BASS_SPK4: c_uint = 0x18;
pub const SMA1303_19_BASS_SPK5: c_uint = 0x19;
pub const SMA1303_1A_BASS_SPK6: c_uint = 0x1a;
pub const SMA1303_1B_BASS_SPK7: c_uint = 0x1b;
// 0x1C ~ 0x22 : Reserved
pub const SMA1303_23_COMP_LIM1: c_uint = 0x23;
pub const SMA1303_24_COMP_LIM2: c_uint = 0x24;
pub const SMA1303_25_COMP_LIM3: c_uint = 0x25;
pub const SMA1303_26_COMP_LIM4: c_uint = 0x26;
// 0x27 ~ 0x32 : Reserved
pub const SMA1303_33_SDM_CTRL: c_uint = 0x33;
pub const SMA1303_34_OTP_DATA1: c_uint = 0x34;
// 0x35 : Reserved
pub const SMA1303_36_PROTECTION: c_uint = 0x36;
pub const SMA1303_37_SLOPE_CTRL: c_uint = 0x37;
pub const SMA1303_38_OTP_TRM0: c_uint = 0x38;
// 0x39 ~ 0x3A : Reserved
pub const SMA1303_3B_TEST1: c_uint = 0x3b;
pub const SMA1303_3C_TEST2: c_uint = 0x3c;
pub const SMA1303_3D_TEST3: c_uint = 0x3d;
pub const SMA1303_3E_ATEST1: c_uint = 0x3e;
pub const SMA1303_3F_ATEST2: c_uint = 0x3f;
// 0x40 ~ 0x8A : Reserved
pub const SMA1303_8B_PLL_POST_N: c_uint = 0x8b;
pub const SMA1303_8C_PLL_N: c_uint = 0x8c;
pub const SMA1303_8D_PLL_A_SETTING: c_uint = 0x8d;
pub const SMA1303_8E_PLL_CTRL: c_uint = 0x8e;
pub const SMA1303_8F_PLL_P_CP: c_uint = 0x8f;
pub const SMA1303_90_POSTSCALER: c_uint = 0x90;
pub const SMA1303_91_CLASS_G_CTRL: c_uint = 0x91;
pub const SMA1303_92_FDPEC_CTRL: c_uint = 0x92;
// 0x93 : Reserved
pub const SMA1303_94_BOOST_CTRL1: c_uint = 0x94;
pub const SMA1303_95_BOOST_CTRL2: c_uint = 0x95;
pub const SMA1303_96_BOOST_CTRL3: c_uint = 0x96;
pub const SMA1303_97_BOOST_CTRL4: c_uint = 0x97;
// 0x98 ~ 0x9F : Reserved
pub const SMA1303_A0_PAD_CTRL0: c_uint = 0xa0;
pub const SMA1303_A1_PAD_CTRL1: c_uint = 0xa1;
pub const SMA1303_A2_TOP_MAN1: c_uint = 0xa2;
pub const SMA1303_A3_TOP_MAN2: c_uint = 0xa3;
pub const SMA1303_A4_TOP_MAN3: c_uint = 0xa4;
pub const SMA1303_A5_TDM1: c_uint = 0xa5;
pub const SMA1303_A6_TDM2: c_uint = 0xa6;
pub const SMA1303_A7_CLK_MON: c_uint = 0xa7;
// 0xA8 ~ 0xF9 : Reserved
pub const SMA1303_FA_STATUS1: c_uint = 0xfa;
pub const SMA1303_FB_STATUS2: c_uint = 0xfb;
// 0xFC ~ 0xFE : Reserved
pub const SMA1303_FF_DEVICE_INDEX: c_uint = 0xff;
// SMA1303 Registers Bit Fields
// SYSTEM_CTRL : 0x00

// INTPUT CTRL1 : 0x01

// INTPUT CTRL2 : 0x02

// INTPUT CTRL3 : 0x03

// INTPUT CTRL4 : 0x04

// OUTPUT CTRL : 0x09

// BST_TEST : 0x0B

// BST_TEST1 : 0x0C

// SPK_TEST : 0x0D

// MUTE_VOL_CTRL : 0x0E

// SYSTEM_CTRL1 :0x10

// SYSTEM_CTRL2 : 0x11

// SYSTEM_CTRL3 : 0x12

// Modulator : 0x14

// SDM CONTROL : 0x33

// OTP_DATA1 : 0x34

// PROTECTION : 0x36

// TEST2 : 0x3C

// ATEST2 : 0x3F

// PLL_CTRL : 0x8E

// POSTSCALER : 0x90

// FDPEC CONTROL : 0x92

// BOOST_CONTROL4 : 0x97

// TOP_MAN1 : 0xA2

// TOP_MAN2 : 0xA3

// TOP_MAN3 0xA4

// TDM1 FORMAT : 0xA5

// TDM2 FORMAT : 0xA6

// STATUS1 : 0xFA

// STATUS2 : 0xFB

// DEVICE_INFO : 0xFF

