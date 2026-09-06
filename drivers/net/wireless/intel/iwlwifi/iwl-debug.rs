//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-debug.h
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
// Copyright(c) 2003 - 2014 Intel Corporation. All rights reserved.
// Copyright(c) 2018 - 2021, 2024-2025 Intel Corporation
//
// Portions of this file are derived from the ipw3945 project.
//

// Macro flag: #define __iwl_debug_h__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_err_mode {
    IWL_ERR_MODE_REGULAR,
    IWL_ERR_MODE_RFKILL,
    IWL_ERR_MODE_TRACE_ONLY,
    IWL_ERR_MODE_RATELIMIT,
}

extern "C" {
    pub fn __iwl_warn(dev: *mut device, fmt: *const c_char, __printf(2: ...), _arg: 3);
}
extern "C" {
    pub fn __iwl_info(dev: *mut device, fmt: *const c_char, __printf(2: ...), _arg: 3);
}
extern "C" {
    pub fn __iwl_crit(dev: *mut device, fmt: *const c_char, __printf(2: ...), _arg: 3);
}
// not all compilers can evaluate strlen() at compile time, so use sizeof()

// No matter what is m (priv, bus, trans), this will work

//
// To use the debug system:
//
// If you are defining a new debug classification, simply add it to the #define
// list here in the form of
//
// #define IWL_DL_xxxx VALUE
//
// where xxxx should be the name of the classification (for example, WEP).
//
// You then need to either add a IWL_xxxx_DEBUG() macro definition for your
// classification, or use IWL_DEBUG(IWL_DL_xxxx, ...) whenever you want
// to send output to that classification.
//
// The active debug levels can be accessed via files
//
// /sys/module/iwlwifi/parameters/debug
// when CONFIG_IWLWIFI_DEBUG=y.
//
// /sys/kernel/debug/phy0/iwlwifi/debug/debug_level
// when CONFIG_IWLWIFI_DEBUGFS=y.
//
// 0x0000000F - 0x00000001
pub const IWL_DL_INFO: c_uint = 0x00000001;
pub const IWL_DL_MAC80211: c_uint = 0x00000002;
pub const IWL_DL_HCMD: c_uint = 0x00000004;
pub const IWL_DL_TDLS: c_uint = 0x00000008;
// 0x000000F0 - 0x00000010
pub const IWL_DL_QUOTA: c_uint = 0x00000010;
pub const IWL_DL_TE: c_uint = 0x00000020;
pub const IWL_DL_EEPROM: c_uint = 0x00000040;
pub const IWL_DL_RADIO: c_uint = 0x00000080;
// 0x00000F00 - 0x00000100
pub const IWL_DL_POWER: c_uint = 0x00000100;
pub const IWL_DL_TEMP: c_uint = 0x00000200;
pub const IWL_DL_WOWLAN: c_uint = 0x00000400;
pub const IWL_DL_SCAN: c_uint = 0x00000800;
// 0x0000F000 - 0x00001000
pub const IWL_DL_ASSOC: c_uint = 0x00001000;
pub const IWL_DL_DROP: c_uint = 0x00002000;
pub const IWL_DL_LAR: c_uint = 0x00004000;
pub const IWL_DL_COEX: c_uint = 0x00008000;
// 0x000F0000 - 0x00010000
pub const IWL_DL_FW: c_uint = 0x00010000;
pub const IWL_DL_RF_KILL: c_uint = 0x00020000;
pub const IWL_DL_TPT: c_uint = 0x00040000;
pub const IWL_DL_PTP: c_uint = 0x00080000;
// 0x00F00000 - 0x00100000
pub const IWL_DL_RATE: c_uint = 0x00100000;
pub const IWL_DL_CALIB: c_uint = 0x00200000;
pub const IWL_DL_WEP: c_uint = 0x00400000;
pub const IWL_DL_TX: c_uint = 0x00800000;
// 0x0F000000 - 0x01000000
pub const IWL_DL_RX: c_uint = 0x01000000;
pub const IWL_DL_ISR: c_uint = 0x02000000;
pub const IWL_DL_HT: c_uint = 0x04000000;
pub const IWL_DL_EHT: c_uint = 0x08000000;
// 0xF0000000 - 0x10000000
pub const IWL_DL_11H: c_uint = 0x10000000;
pub const IWL_DL_STATS: c_uint = 0x20000000;
pub const IWL_DL_TX_REPLY: c_uint = 0x40000000;
pub const IWL_DL_TX_QUEUES: c_uint = 0x80000000;

