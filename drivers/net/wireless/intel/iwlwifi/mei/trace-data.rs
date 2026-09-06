//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mei/trace-data.h
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
// Copyright(c) 2021        Intel Corporation
//

// Macro flag: #define trace_iwlmei_sap_data(...)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_sap_data_trace_type {
    IWL_SAP_RX_DATA_TO_AIR,
    IWL_SAP_TX_DATA_FROM_AIR,
    IWL_SAP_RX_DATA_DROPPED_FROM_AIR,
    IWL_SAP_TX_DHCP,
}

extern "C" {
    pub fn sizeof(iwl_sap_hdr: struct) -> return;
}
extern "C" {
    pub fn sizeof(iwl_sap_cb_data: struct) -> return;
}

//
// If you add something here, add a stub in case
// !defined(CONFIG_IWLWIFI_DEVICE_TRACING)
//

