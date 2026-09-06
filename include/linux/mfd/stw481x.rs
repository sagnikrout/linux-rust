//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stw481x.h
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
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

// These registers are accessed from more than one driver
pub const STW_CONF1: c_uint = 0x11U;
pub const STW_CONF1_PDN_VMMC: c_uint = 0x01U;
pub const STW_CONF1_VMMC_MASK: c_uint = 0x0eU;
pub const STW_CONF1_VMMC_1_8V: c_uint = 0x02U;
pub const STW_CONF1_VMMC_2_85V: c_uint = 0x04U;
pub const STW_CONF1_VMMC_3V: c_uint = 0x06U;
pub const STW_CONF1_VMMC_1_85V: c_uint = 0x08U;
pub const STW_CONF1_VMMC_2_6V: c_uint = 0x0aU;
pub const STW_CONF1_VMMC_2_7V: c_uint = 0x0cU;
pub const STW_CONF1_VMMC_3_3V: c_uint = 0x0eU;
pub const STW_CONF1_MMC_LS_STATUS: c_uint = 0x10U;
pub const STW_PCTL_REG_LO: c_uint = 0x1eU;
pub const STW_PCTL_REG_HI: c_uint = 0x1fU;
pub const STW_CONF1_V_MONITORING: c_uint = 0x20U;
pub const STW_CONF1_IT_WARN: c_uint = 0x40U;
pub const STW_CONF1_PDN_VAUX: c_uint = 0x80U;
pub const STW_CONF2: c_uint = 0x20U;
pub const STW_CONF2_MASK_TWARN: c_uint = 0x01U;
pub const STW_CONF2_VMMC_EXT: c_uint = 0x02U;
pub const STW_CONF2_MASK_IT_WAKE_UP: c_uint = 0x04U;
pub const STW_CONF2_GPO1: c_uint = 0x08U;
pub const STW_CONF2_GPO2: c_uint = 0x10U;
pub const STW_VCORE_SLEEP: c_uint = 0x21U;
//
// struct stw481x - state holder for the Stw481x drivers
// @i2c_client: corresponding I2C client
// @map: regmap handle to access device registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stw481x {
    pub client: *mut i2c_client,
    pub map: *mut regmap,
}
