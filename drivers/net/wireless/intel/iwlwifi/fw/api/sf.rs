//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/sf.h
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
// Copyright (C) 2012-2014 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_sf_h__
// Smart Fifo state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sf_state {
    SF_LONG_DELAY_ON = 0, /* should never be called by driver */
    SF_FULL_ON,
    SF_UNINIT,
    SF_INIT_OFF,
    SF_HW_NUM_STATES
}

// Smart Fifo possible scenario
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sf_scenario {
    SF_SCENARIO_SINGLE_UNICAST,
    SF_SCENARIO_AGG_UNICAST,
    SF_SCENARIO_MULTICAST,
    SF_SCENARIO_BA_RESP,
    SF_SCENARIO_TX_RESP,
    SF_NUM_SCENARIO
}

// smart FIFO default values
pub const SF_W_MARK_SISO: c_int = 6144;
pub const SF_W_MARK_MIMO2: c_int = 8192;
pub const SF_W_MARK_MIMO3: c_int = 6144;
pub const SF_W_MARK_LEGACY: c_int = 4096;
pub const SF_W_MARK_SCAN: c_int = 4096;
// SF Scenarios timers for default configuration (aligned to 32 uSec)

// SF Scenarios timers for BSS MAC configuration (aligned to 32 uSec)

//
// struct iwl_sf_cfg_cmd - Smart Fifo configuration command.
// @state: smart fifo state, types listed in &enum iwl_sf_state.
// @watermark: Minimum allowed available free space in RXF for transient state.
// @long_delay_timeouts: aging and idle timer values for each scenario
// in long delay state.
// @full_on_timeouts: timer values for each scenario in full on state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sf_cfg_cmd {
    pub state: __le32,
    pub watermark: [__le32; SF_TRANSIENT_STATES_NUMBER],
    pub long_delay_timeouts: [__le32; SF_NUM_SCENARIO][SF_NUM_TIMEOUT_TYPES],
    pub full_on_timeouts: [__le32; SF_NUM_SCENARIO][SF_NUM_TIMEOUT_TYPES],
    pub /: *mut *mut } __packed; / SF_CFG_API_S_VER_2,
