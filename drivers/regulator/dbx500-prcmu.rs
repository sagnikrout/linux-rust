//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/dbx500-prcmu.h
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Bengt Jonsson <bengt.jonsson@stericsson.com> for ST-Ericsson,
// Jonas Aaberg <jonas.aberg@stericsson.com> for ST-Ericsson
//

//
// struct dbx500_regulator_info - dbx500 regulator information
// @desc: regulator description
// @is_enabled: status of the regulator
// @epod_id: id for EPOD (power domain)
// @is_ramret: RAM retention switch for EPOD (power domain)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbx500_regulator_info {
    pub desc: regulator_desc,
    pub is_enabled: bool,
    pub epod_id: u16,
    pub is_ramret: bool,
    pub exclude_from_power_state: bool,
}

extern "C" {
    pub fn power_state_active_enable();
}
extern "C" {
    pub fn power_state_active_disable() -> c_int;
}

extern "C" {
    pub fn ux500_regulator_debug_exit() -> c_int;
}

