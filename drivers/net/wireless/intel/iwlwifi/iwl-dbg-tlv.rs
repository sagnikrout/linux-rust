//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-dbg-tlv.h
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
// Copyright (C) 2018-2023, 2025 Intel Corporation
//

// Macro flag: #define __iwl_dbg_tlv_h__

pub const IWL_DBG_TLV_MAX_PRESET: c_int = 15;

//
// struct iwl_dbg_tlv_node - debug TLV node
// @list: list of &struct iwl_dbg_tlv_node
// @tlv: debug TLV
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dbg_tlv_node {
    pub list: list_head,
    pub tlv: iwl_ucode_tlv,
}

//
// union iwl_dbg_tlv_tp_data - data that is given in a time point
// @fw_pkt: a packet received from the FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iwl_dbg_tlv_tp_data {
    pub fw_pkt: *mut iwl_rx_packet,
}

//
// struct iwl_dbg_tlv_time_point_data - debug time point data
// @trig_list: list of triggers
// @active_trig_list: list of active triggers
// @hcmd_list: list of host commands
// @config_list: list of configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dbg_tlv_time_point_data {
    pub trig_list: list_head,
    pub active_trig_list: list_head,
    pub hcmd_list: list_head,
    pub config_list: list_head,
}

extern "C" {
    pub fn iwl_dbg_tlv_load_bin(dev: *mut device, trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_dbg_tlv_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_dbg_tlv_init(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_dbg_tlv_init_cfg(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_dbg_tlv_del_timers(trans: *mut iwl_trans);
}
