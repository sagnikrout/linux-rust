//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/dbg.h
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
// Copyright (C) 2005-2014, 2018-2019, 2021-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2015-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_dbg_h__

//
// struct iwl_fw_dump_desc - describes the dump
// @len: length of trig_desc->data
// @trig_desc: the description of the dump
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_dump_desc {
    pub len: usize,
// must be last
    pub trig_desc: iwl_fw_error_dump_trigger_desc,
}

//
// struct iwl_fw_dbg_params - register values to restore
// @in_sample: DBGC_IN_SAMPLE value
// @out_ctrl: DBGC_OUT_CTRL value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_dbg_params {
    pub in_sample: u32,
    pub out_ctrl: u32,
}

// old-style dump entry point
extern "C" {
    pub fn iwl_fw_start_dbg_conf(fwrt: *mut iwl_fw_runtime, id: u8) -> c_int;
}

// If this is the first event checked, jump to update start ts
extern "C" {
    pub fn iwl_fw_dbg_trigger_stop_conf_match(_arg: fwrt, _arg: trig) -> return;
}

extern "C" {
    pub fn iwl_fw_error_dump_wk(work: *mut work_struct);
}
extern "C" {
    pub fn iwl_fw_dbg_read_d3_debug_data(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_fw_send_timestamp_marker_cmd(fwrt: *mut iwl_fw_runtime) -> c_int;
}

extern "C" {
    pub fn iwl_fw_trigger_timestamp(fwrt: *mut iwl_fw_runtime, delay: u32);
}

extern "C" {
    pub fn iwl_fw_dbg_stop_sync(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_fwrt_dump_error_logs(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_fwrt_read_err_table(trans: *mut iwl_trans, base: u32, err_id: *mut u32) -> bool;
}
extern "C" {
    pub fn iwl_fw_disable_dbg_asserts(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_fw_dbg_clear_monitor_buf(fwrt: *mut iwl_fw_runtime);
}

