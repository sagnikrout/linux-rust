//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8523.h
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
// wm8523.h  --  WM8523 ASoC driver
//
// Copyright 2009 Wolfson Microelectronics, plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Based on wm8753.h
//
// Register values.
//
pub const WM8523_DEVICE_ID: c_uint = 0x00;
pub const WM8523_REVISION: c_uint = 0x01;
pub const WM8523_PSCTRL1: c_uint = 0x02;
pub const WM8523_AIF_CTRL1: c_uint = 0x03;
pub const WM8523_AIF_CTRL2: c_uint = 0x04;
pub const WM8523_DAC_CTRL3: c_uint = 0x05;
pub const WM8523_DAC_GAINL: c_uint = 0x06;
pub const WM8523_DAC_GAINR: c_uint = 0x07;
pub const WM8523_ZERO_DETECT: c_uint = 0x08;
pub const WM8523_REGISTER_COUNT: c_int = 9;
pub const WM8523_MAX_REGISTER: c_uint = 0x08;
//
// Field Definitions.
//
// R0 (0x00) - DEVICE_ID
//
pub const WM8523_CHIP_ID_MASK: c_uint = 0xFFFF  /* CHIP_ID - [15:0] */;

//
// R1 (0x01) - REVISION
//
pub const WM8523_CHIP_REV_MASK: c_uint = 0x0007  /* CHIP_REV - [2:0] */;

//
// R2 (0x02) - PSCTRL1
//
pub const WM8523_SYS_ENA_MASK: c_uint = 0x0003  /* SYS_ENA - [1:0] */;

//
// R3 (0x03) - AIF_CTRL1
//
pub const WM8523_TDM_MODE_MASK: c_uint = 0x1800  /* TDM_MODE - [12:11] */;

pub const WM8523_TDM_SLOT_MASK: c_uint = 0x0600  /* TDM_SLOT - [10:9] */;

pub const WM8523_DEEMPH: c_uint = 0x0100  /* DEEMPH  */;
pub const WM8523_DEEMPH_MASK: c_uint = 0x0100  /* DEEMPH  */;

pub const WM8523_AIF_MSTR: c_uint = 0x0080  /* AIF_MSTR  */;
pub const WM8523_AIF_MSTR_MASK: c_uint = 0x0080  /* AIF_MSTR  */;

pub const WM8523_LRCLK_INV: c_uint = 0x0040  /* LRCLK_INV  */;
pub const WM8523_LRCLK_INV_MASK: c_uint = 0x0040  /* LRCLK_INV  */;

pub const WM8523_BCLK_INV: c_uint = 0x0020  /* BCLK_INV  */;
pub const WM8523_BCLK_INV_MASK: c_uint = 0x0020  /* BCLK_INV  */;

pub const WM8523_WL_MASK: c_uint = 0x0018  /* WL - [4:3] */;

pub const WM8523_FMT_MASK: c_uint = 0x0007  /* FMT - [2:0] */;

//
// R4 (0x04) - AIF_CTRL2
//
pub const WM8523_DAC_OP_MUX_MASK: c_uint = 0x00C0  /* DAC_OP_MUX - [7:6] */;

pub const WM8523_BCLKDIV_MASK: c_uint = 0x0038  /* BCLKDIV - [5:3] */;

pub const WM8523_SR_MASK: c_uint = 0x0007  /* SR - [2:0] */;

//
// R5 (0x05) - DAC_CTRL3
//
pub const WM8523_ZC: c_uint = 0x0010  /* ZC  */;
pub const WM8523_ZC_MASK: c_uint = 0x0010  /* ZC  */;

pub const WM8523_DACR: c_uint = 0x0008  /* DACR  */;
pub const WM8523_DACR_MASK: c_uint = 0x0008  /* DACR  */;

pub const WM8523_DACL: c_uint = 0x0004  /* DACL  */;
pub const WM8523_DACL_MASK: c_uint = 0x0004  /* DACL  */;

pub const WM8523_VOL_UP_RAMP: c_uint = 0x0002  /* VOL_UP_RAMP  */;
pub const WM8523_VOL_UP_RAMP_MASK: c_uint = 0x0002  /* VOL_UP_RAMP  */;

pub const WM8523_VOL_DOWN_RAMP: c_uint = 0x0001  /* VOL_DOWN_RAMP  */;
pub const WM8523_VOL_DOWN_RAMP_MASK: c_uint = 0x0001  /* VOL_DOWN_RAMP  */;

//
// R6 (0x06) - DAC_GAINL
//
pub const WM8523_DACL_VU: c_uint = 0x0200  /* DACL_VU  */;
pub const WM8523_DACL_VU_MASK: c_uint = 0x0200  /* DACL_VU  */;

pub const WM8523_DACL_VOL_MASK: c_uint = 0x01FF  /* DACL_VOL - [8:0] */;

//
// R7 (0x07) - DAC_GAINR
//
pub const WM8523_DACR_VU: c_uint = 0x0200  /* DACR_VU  */;
pub const WM8523_DACR_VU_MASK: c_uint = 0x0200  /* DACR_VU  */;

pub const WM8523_DACR_VOL_MASK: c_uint = 0x01FF  /* DACR_VOL - [8:0] */;

//
// R8 (0x08) - ZERO_DETECT
//
pub const WM8523_ZD_COUNT_MASK: c_uint = 0x0003  /* ZD_COUNT - [1:0] */;

