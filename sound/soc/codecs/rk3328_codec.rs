//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rk3328_codec.h
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
// rk3328 ALSA SoC Audio driver
//
// Copyright (c) 2017, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.
//

// codec register

// REG00: CODEC_RESET

// REG03: DAC_INIT_CTRL1

// REG04: DAC_INIT_CTRL2

// REG05: DAC_INIT_CTRL3

// REG22: DAC_PRECHARGE_CTRL

pub const DAC_CHARGE_CURRENT_ALL_OFF: c_uint = 0x00;
pub const DAC_CHARGE_CURRENT_ALL_ON: c_uint = 0x7f;
// REG23: DAC_PWR_CTRL

// REG24: DAC_CLK_CTRL

// REG25: HPMIX_CTRL

// REG26: DAC_SELECT

// REG27: HPOUT_CTRL

// REG28: HPOUTL_GAIN_CTRL

// REG29: HPOUTR_GAIN_CTRL

// REG2a: HPOUT_POP_CTRL

pub const RK3328_HIFI: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk3328_reg_msk_val {
    pub reg: c_uint,
    pub msk: c_uint,
    pub val: c_uint,
}
