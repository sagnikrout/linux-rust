//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/dhc-utils.h
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
// Copyright (C) 2021, 2025 Intel Corporation
//

// Macro flag: #define __iwl_fw_dhc_utils_h__

//
// iwl_dhc_resp_status - return status of DHC response
// @fw: firwmware image information
// @pkt: response packet, must not be %NULL
//
// Returns: the status value of the DHC command or (u32)-1 if the
// response was too short.
//
extern "C" {
    pub fn le32_to_cpu(_arg: resp->status) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: resp->status) -> return;
}
//
// iwl_dhc_resp_data - return data pointer of DHC response
// @fw: firwmware image information
// @pkt: response packet, must not be %NULL
// @len: where to store the length
//
// Returns: The data pointer, or an ERR_PTR() if the data was
// not valid (too short).
//
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// len = iwl_rx_packet_payload_len(pkt) - sizeof(*resp);
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// len = iwl_rx_packet_payload_len(pkt) - sizeof(*resp);
