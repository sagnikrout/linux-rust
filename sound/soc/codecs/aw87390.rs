//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw87390.h
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
// aw87390.h  --  aw87390 ALSA SoC Audio driver
//
// Copyright (c) 2023 awinic Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

pub const AW87391_REG_CP_OVP_6_50V: c_int = 0;
pub const AW87391_REG_CP_OVP_6_75V: c_int = 1;
pub const AW87391_REG_CP_OVP_7_00V: c_int = 2;
pub const AW87391_REG_CP_OVP_7_25V: c_int = 3;
pub const AW87391_REG_CP_OVP_7_50V: c_int = 4;
pub const AW87391_REG_CP_OVP_7_75V: c_int = 5;
pub const AW87391_REG_CP_OVP_8_00V: c_int = 6;
pub const AW87391_REG_CP_OVP_8_25V: c_int = 7;
pub const AW87391_REG_CP_OVP_8_50V: c_int = 8;

pub const AW87391_GAIN_12DB: c_int = 0;
pub const AW87391_GAIN_15DB: c_int = 1;
pub const AW87391_GAIN_18DB: c_int = 2;
pub const AW87391_GAIN_21DB: c_int = 3;
pub const AW87391_GAIN_24DB: c_int = 4;

// AGC2PO supports values between 500mW (0000) to 1600mW (1011)

pub const AW87391_AK2F_S_10_24: c_int = 0;
pub const AW87391_AK2F_S_20_48: c_int = 1;
pub const AW87391_AK2F_S_41: c_int = 2;
pub const AW87391_AK2F_S_82: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw87390_id {
    AW87390_CHIP_ID = 0x76,
    AW87391_CHIP_ID = 0xc1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw87390 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub vdd_reg: *mut regulator,
}
