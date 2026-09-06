//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/rfi.h
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
// Copyright (C) 2020-2021, 2023 Intel Corporation
//

// Macro flag: #define __iwl_fw_api_rfi_h__
pub const IWL_RFI_LUT_ENTRY_CHANNELS_NUM: c_int = 15;
pub const IWL_RFI_LUT_SIZE: c_int = 24;
pub const IWL_RFI_LUT_INSTALLED_SIZE: c_int = 4;
//
// struct iwl_rfi_lut_entry - an entry in the RFI frequency LUT.
//
// @freq: frequency
// @channels: channels that can be interfered at frequency freq (at most 15)
// @bands: the corresponding bands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rfi_lut_entry {
    pub freq: __le16,
    pub channels: [u8; IWL_RFI_LUT_ENTRY_CHANNELS_NUM],
    pub bands: [u8; IWL_RFI_LUT_ENTRY_CHANNELS_NUM],
    pub __packed: },
//
// struct iwl_rfi_config_cmd - RFI configuration table
//
// @table: a table can have 24 frequency/channel mappings
// @oem: specifies if this is the default table or set by OEM
// @reserved: (reserved/padding)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rfi_config_cmd {
    pub table: [iwl_rfi_lut_entry; IWL_RFI_LUT_SIZE],
    pub oem: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / RFI_CONFIG_CMD_API_S_VER_1,
//
// enum iwl_rfi_freq_table_status - status of the frequency table query
// @RFI_FREQ_TABLE_OK: can be used
// @RFI_FREQ_TABLE_DVFS_NOT_READY: DVFS is not ready yet, should try later
// @RFI_FREQ_TABLE_DISABLED: the feature is disabled in FW
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_rfi_freq_table_status {
    RFI_FREQ_TABLE_OK,
    RFI_FREQ_TABLE_DVFS_NOT_READY,
    RFI_FREQ_TABLE_DISABLED,
}

//
// struct iwl_rfi_freq_table_resp_cmd - get the rfi freq table used by FW
//
// @table: table used by FW
// @status: see &iwl_rfi_freq_table_status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rfi_freq_table_resp_cmd {
    pub table: [iwl_rfi_lut_entry; IWL_RFI_LUT_INSTALLED_SIZE],
    pub status: __le32,
    pub /: *mut *mut } __packed; / RFI_CONFIG_CMD_API_S_VER_1,
//
// struct iwl_rfi_deactivate_notif - notifcation that FW disaled RFIm
//
// @reason: used only for a log message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rfi_deactivate_notif {
    pub reason: __le32,
    pub /: *mut *mut } __packed; / RFI_DEACTIVATE_NTF_S_VER_1,
