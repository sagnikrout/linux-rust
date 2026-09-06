//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65090.h
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
// Core driver interface for TI TPS65090 PMIC family
//
// Copyright (C) 2012 NVIDIA Corporation
//

// TPS65090 IRQs
// TPS65090 Regulator ID
// Last entry for maximum ID
// Register addresses
pub const TPS65090_REG_INTR_STS: c_uint = 0x00;
pub const TPS65090_REG_INTR_STS2: c_uint = 0x01;
pub const TPS65090_REG_INTR_MASK: c_uint = 0x02;
pub const TPS65090_REG_INTR_MASK2: c_uint = 0x03;
pub const TPS65090_REG_CG_CTRL0: c_uint = 0x04;
pub const TPS65090_REG_CG_CTRL1: c_uint = 0x05;
pub const TPS65090_REG_CG_CTRL2: c_uint = 0x06;
pub const TPS65090_REG_CG_CTRL3: c_uint = 0x07;
pub const TPS65090_REG_CG_CTRL4: c_uint = 0x08;
pub const TPS65090_REG_CG_CTRL5: c_uint = 0x09;
pub const TPS65090_REG_CG_STATUS1: c_uint = 0x0a;
pub const TPS65090_REG_CG_STATUS2: c_uint = 0x0b;
pub const TPS65090_REG_AD_OUT1: c_uint = 0x17;
pub const TPS65090_REG_AD_OUT2: c_uint = 0x18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65090 {
    pub dev: *mut device,
    pub rmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
}

//
// struct tps65090_regulator_plat_data
//
// @reg_init_data: The regulator init data.
// @enable_ext_control: Enable extrenal control or not. Only available for
// DCDC1, DCDC2 and DCDC3.
// @gpiod: Gpio descriptor if external control is enabled and controlled through
// gpio
// @overcurrent_wait_valid: True if the overcurrent_wait should be applied.
// @overcurrent_wait: Value to set as the overcurrent wait time.  This is the
// actual bitfield value, not a time in ms (valid value are 0 - 3).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65090_regulator_plat_data {
    pub reg_init_data: *mut regulator_init_data,
    pub enable_ext_control: bool,
    pub gpiod: *mut gpio_desc,
    pub overcurrent_wait_valid: bool,
    pub overcurrent_wait: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65090_platform_data {
    pub irq_base: c_int,
    pub supplied_to: *mut c_char,
    pub num_supplicants: usize,
    pub enable_low_current_chrg: c_int,
    pub reg_pdata: [*mut tps65090_regulator_plat_data; TPS65090_REGULATOR_MAX],
}

//
// NOTE: the functions below are not intended for use outside
// of the TPS65090 sub-device drivers
//
extern "C" {
    pub fn regmap_write(_arg: tps->rmap, _arg: reg, _arg: val) -> return;
}
// val = temp_val;
extern "C" {
    pub fn regmap_update_bits(_arg: tps->rmap, _arg: reg, _arg: BIT(bit_num), _arg: ~0u) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: tps->rmap, _arg: reg, _arg: BIT(bit_num), _arg: 0u) -> return;
}
