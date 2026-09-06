//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/binding.h
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
// Copyright (C) 2012-2014, 2020, 2022, 2024 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_binding_h__

//
// struct iwl_binding_cmd_v1 - configuring bindings
// ( BINDING_CONTEXT_CMD = 0x2b )
// @id_and_color: ID and color of the relevant Binding,
// &enum iwl_ctxt_id_and_color
// @action: action to perform, see &enum iwl_ctxt_action
// @macs: array of MAC id and colors which belong to the binding,
// &enum iwl_ctxt_id_and_color
// @phy: PHY id and color which belongs to the binding,
// &enum iwl_ctxt_id_and_color
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_binding_cmd_v1 {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
// BINDING_DATA_API_S_VER_1
    pub macs: [__le32; MAX_MACS_IN_BINDING],
    pub phy: __le32,
    pub /: *mut *mut } __packed; / BINDING_CMD_API_S_VER_1,
//
// struct iwl_binding_cmd - configuring bindings
// ( BINDING_CONTEXT_CMD = 0x2b )
// @id_and_color: ID and color of the relevant Binding,
// &enum iwl_ctxt_id_and_color
// @action: action to perform, see &enum iwl_ctxt_action
// @macs: array of MAC id and colors which belong to the binding
// &enum iwl_ctxt_id_and_color
// @phy: PHY id and color which belongs to the binding
// &enum iwl_ctxt_id_and_color
// @lmac_id: the lmac id the binding belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_binding_cmd {
// COMMON_INDEX_HDR_API_S_VER_1
    pub id_and_color: __le32,
    pub action: __le32,
// BINDING_DATA_API_S_VER_1
    pub macs: [__le32; MAX_MACS_IN_BINDING],
    pub phy: __le32,
    pub lmac_id: __le32,
    pub /: *mut *mut } __packed; / BINDING_CMD_API_S_VER_2,

// The maximal number of fragments in the FW's schedule session
pub const IWL_MVM_MAX_QUOTA: c_int = 128;
//
// struct iwl_time_quota_data_v1 - configuration of time quota per binding
// @id_and_color: ID and color of the relevant Binding,
// &enum iwl_ctxt_id_and_color
// @quota: absolute time quota in TU. The scheduler will try to divide the
// remainig quota (after Time Events) according to this quota.
// @max_duration: max uninterrupted context duration in TU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_quota_data_v1 {
    pub id_and_color: __le32,
    pub quota: __le32,
    pub max_duration: __le32,
    pub /: *mut *mut } __packed; / TIME_QUOTA_DATA_API_S_VER_1,
//
// struct iwl_time_quota_cmd_v1 - configuration of time quota between bindings
// ( TIME_QUOTA_CMD = 0x2c )
// @quotas: allocations per binding
// Note: on non-CDB the fourth one is the auxilary mac and is
// essentially zero.
// On CDB the fourth one is a regular binding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_quota_cmd_v1 {
    pub quotas: [iwl_time_quota_data_v1; MAX_BINDINGS],
    pub /: *mut *mut } __packed; / TIME_QUOTA_ALLOCATION_CMD_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_quota_low_latency {
    IWL_QUOTA_LOW_LATENCY_NONE = 0,
    IWL_QUOTA_LOW_LATENCY_TX = BIT(0),
    IWL_QUOTA_LOW_LATENCY_RX = BIT(1),
    IWL_QUOTA_LOW_LATENCY_TX_RX =
    IWL_QUOTA_LOW_LATENCY_TX | IWL_QUOTA_LOW_LATENCY_RX,
}

//
// struct iwl_time_quota_data - configuration of time quota per binding
// @id_and_color: ID and color of the relevant Binding.
// @quota: absolute time quota in TU. The scheduler will try to divide the
// remainig quota (after Time Events) according to this quota.
// @max_duration: max uninterrupted context duration in TU
// @low_latency: low latency status, &enum iwl_quota_low_latency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_quota_data {
    pub id_and_color: __le32,
    pub quota: __le32,
    pub max_duration: __le32,
    pub low_latency: __le32,
    pub /: *mut *mut } __packed; / TIME_QUOTA_DATA_API_S_VER_2,
//
// struct iwl_time_quota_cmd - configuration of time quota between bindings
// ( TIME_QUOTA_CMD = 0x2c )
// Note: on non-CDB the fourth one is the auxilary mac and is essentially zero.
// On CDB the fourth one is a regular binding.
//
// @quotas: allocations per binding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_time_quota_cmd {
    pub quotas: [iwl_time_quota_data; MAX_BINDINGS],
    pub /: *mut *mut } __packed; / TIME_QUOTA_ALLOCATION_CMD_API_S_VER_2,
