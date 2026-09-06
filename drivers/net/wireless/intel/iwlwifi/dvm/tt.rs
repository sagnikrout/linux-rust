//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/tt.h
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
// Copyright(c) 2007 - 2014, 2023-2024 Intel Corporation. All rights reserved.
//
// Portions of this file are derived from the ipw3945 project, as well
// as portions of the ieee80211 subsystem header files.
//

// Macro flag: #define __iwl_tt_setting_h__

pub const IWL_ABSOLUTE_ZERO: c_int = 0;
pub const IWL_ABSOLUTE_MAX: c_uint = 0xFFFFFFFF;
pub const IWL_TT_INCREASE_MARGIN: c_int = 5;
pub const IWL_TT_CT_KILL_MARGIN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_antenna_ok {
    IWL_ANT_OK_NONE,
    IWL_ANT_OK_SINGLE,
    IWL_ANT_OK_MULTI,
}

// Thermal Throttling State Machine states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tt_state {
    IWL_TI_0,	/* normal temperature, system power state */
    IWL_TI_1,	/* high temperature detect, low power state */
    IWL_TI_2,	/* higher temperature detected, lower power state */
    IWL_TI_CT_KILL, /* critical temperature detected, lowest power state */
    IWL_TI_STATE_MAX
}

//
// struct iwl_tt_restriction - Thermal Throttling restriction table
// @tx_stream: number of tx stream allowed
// @is_ht: ht enable/disable
// @rx_stream: number of rx stream allowed
//
// This table is used by advance thermal throttling management
// based on the current thermal throttling state, and determines
// the number of tx/rx streams and the status of HT operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tt_restriction {
    pub tx_stream: iwl_antenna_ok,
    pub rx_stream: iwl_antenna_ok,
    pub is_ht: bool,
}

//
// struct iwl_tt_trans - Thermal Throttling transaction table
// @next_state:  next thermal throttling mode
// @tt_low: low temperature threshold to change state
// @tt_high: high temperature threshold to change state
//
// This is used by the advanced thermal throttling algorithm
// to determine the next thermal state to go based on the
// current temperature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tt_trans {
    pub next_state: iwl_tt_state,
    pub tt_low: u32,
    pub tt_high: u32,
}

//
// struct iwl_tt_mgmt - Thermal Throttling Management structure
// @advanced_tt:    advanced thermal throttle required
// @state:          current Thermal Throttling state
// @tt_power_mode:  Thermal Throttling power mode index
// being used to set power level when
// when thermal throttling state != IWL_TI_0
// the tt_power_mode should set to different
// power mode based on the current tt state
// @tt_previous_temp: last measured temperature
// @restriction: ptr to restriction tbl, used by advance
// thermal throttling to determine how many tx/rx streams
// should be used in tt state; and can HT be enabled or not
// @transaction: ptr to adv trans table, used by advance thermal throttling
// state transaction
// @ct_kill_toggle: used to toggle the CSR bit when checking uCode temperature
// @ct_kill_exit_tm: timer to exit thermal kill
// @ct_kill_waiting_tm: timer to enter thermal kill
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tt_mgmt {
    pub state: iwl_tt_state,
    pub advanced_tt: bool,
    pub tt_power_mode: u8,
    pub ct_kill_toggle: bool,

    pub tt_previous_temp: i32,

    pub restriction: *mut iwl_tt_restriction,
    pub transaction: *mut iwl_tt_trans,
    pub ct_kill_exit_tm: timer_list,
    pub ct_kill_waiting_tm: timer_list,
}

extern "C" {
    pub fn iwl_tt_current_power_mode(priv: *mut iwl_priv) -> u8;
}
extern "C" {
    pub fn iwl_tt_is_low_power_state(priv: *mut iwl_priv) -> bool;
}
extern "C" {
    pub fn iwl_ht_enabled(priv: *mut iwl_priv) -> bool;
}
extern "C" {
    pub fn iwl_tx_ant_restriction(priv: *mut iwl_priv) -> iwl_antenna_ok;
}
extern "C" {
    pub fn iwl_tt_enter_ct_kill(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_tt_exit_ct_kill(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_tt_handler(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_tt_initialize(priv: *mut iwl_priv);
}
extern "C" {
    pub fn iwl_tt_exit(priv: *mut iwl_priv);
}
