//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/inno_rk3036.h
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
// Driver of Inno Codec for rk3036 by Rockchip Inc.
//
// Author: Zheng ShunQian<zhengsq@rock-chips.com>
//
// codec registers
pub const INNO_R00: c_uint = 0x00;
pub const INNO_R01: c_uint = 0x0c;
pub const INNO_R02: c_uint = 0x10;
pub const INNO_R03: c_uint = 0x14;
pub const INNO_R04: c_uint = 0x88;
pub const INNO_R05: c_uint = 0x8c;
pub const INNO_R06: c_uint = 0x90;
pub const INNO_R07: c_uint = 0x94;
pub const INNO_R08: c_uint = 0x98;
pub const INNO_R09: c_uint = 0x9c;
pub const INNO_R10: c_uint = 0xa0;
// register bit filed

pub const INNO_R04_DACR_SW_SHIFT: c_int = 0;
pub const INNO_R04_DACL_SW_SHIFT: c_int = 1;
pub const INNO_R04_DACR_CLK_SHIFT: c_int = 2;
pub const INNO_R04_DACL_CLK_SHIFT: c_int = 3;
pub const INNO_R04_DACR_VREF_SHIFT: c_int = 4;
pub const INNO_R04_DACL_VREF_SHIFT: c_int = 5;
pub const INNO_R05_HPR_EN_SHIFT: c_int = 0;
pub const INNO_R05_HPL_EN_SHIFT: c_int = 1;
pub const INNO_R05_HPR_WORK_SHIFT: c_int = 2;
pub const INNO_R05_HPL_WORK_SHIFT: c_int = 3;
pub const INNO_R06_VOUTR_CZ_SHIFT: c_int = 0;
pub const INNO_R06_VOUTL_CZ_SHIFT: c_int = 1;
pub const INNO_R06_DACR_HILO_VREF_SHIFT: c_int = 2;
pub const INNO_R06_DACL_HILO_VREF_SHIFT: c_int = 3;
pub const INNO_R06_DAC_EN_SHIFT: c_int = 5;

pub const INNO_HP_GAIN_SHIFT: c_int = 0;
// Gain of output, 1.5db step: -39db(0x0) ~ 0db(0x1a) ~ 6db(0x1f)
pub const INNO_HP_GAIN_0DB: c_uint = 0x1a;
pub const INNO_HP_GAIN_N39DB: c_uint = 0x0;
pub const INNO_R09_HP_ANTIPOP_MSK: c_uint = 0x3;
pub const INNO_R09_HP_ANTIPOP_OFF: c_uint = 0x1;
pub const INNO_R09_HP_ANTIPOP_ON: c_uint = 0x2;
pub const INNO_R09_HPR_ANITPOP_SHIFT: c_int = 0;
pub const INNO_R09_HPL_ANITPOP_SHIFT: c_int = 2;
pub const INNO_R09_HPR_MUTE_SHIFT: c_int = 4;
pub const INNO_R09_HPL_MUTE_SHIFT: c_int = 5;
pub const INNO_R09_DACR_SWITCH_SHIFT: c_int = 6;
pub const INNO_R09_DACL_SWITCH_SHIFT: c_int = 7;

