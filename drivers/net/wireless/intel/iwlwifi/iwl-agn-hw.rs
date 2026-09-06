//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-agn-hw.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014 Intel Corporation
//
// Please use this file (iwl-agn-hw.h) only for hardware-related definitions.
//

// Macro flag: #define __iwl_agn_hw_h__

// RSSI to dBm
pub const IWLAGN_RSSI_OFFSET: c_int = 44;
pub const IWLAGN_DEFAULT_TX_RETRY: c_int = 15;
pub const IWLAGN_MGMT_DFAULT_RETRY_LIMIT: c_int = 3;
pub const IWLAGN_RTS_DFAULT_RETRY_LIMIT: c_int = 60;
pub const IWLAGN_BAR_DFAULT_RETRY_LIMIT: c_int = 60;
pub const IWLAGN_LOW_RETRY_LIMIT: c_int = 7;
// Limit range of txpower output target to be between these values

// EEPROM
pub const IWLAGN_EEPROM_IMG_SIZE: c_int = 2048;
// high blocks contain PAPD data

pub const IWLAGN_NUM_QUEUES: c_int = 20;
