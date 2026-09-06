//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/power.h
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
// Copyright(c) 2007 - 2014 Intel Corporation. All rights reserved.
//
// Portions of this file are derived from the ipw3945 project, as well
// as portions of the ieee80211 subsystem header files.
//

// Macro flag: #define __iwl_power_setting_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_power_mgr {
    pub sleep_cmd: iwl_powertable_cmd,
    pub sleep_cmd_next: iwl_powertable_cmd,
    pub debug_sleep_level_override: c_int,
    pub bus_pm: bool,
}

extern "C" {
    pub fn iwl_power_update_mode(priv: *mut iwl_priv, force: bool) -> c_int;
}
extern "C" {
    pub fn iwl_power_initialize(priv: *mut iwl_priv);
}
