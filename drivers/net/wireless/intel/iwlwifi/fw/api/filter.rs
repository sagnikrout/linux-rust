//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/filter.h
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

// Macro flag: #define __iwl_fw_api_filter_h__

pub const MAX_PORT_ID_NUM: c_int = 2;
pub const MAX_MCAST_FILTERING_ADDRESSES: c_int = 256;
//
// struct iwl_mcast_filter_cmd - configure multicast filter.
// @filter_own: Set 1 to filter out multicast packets sent by station itself
// @port_id:	Multicast MAC addresses array specifier. This is a strange way
// to identify network interface adopted in host-device IF.
// It is used by FW as index in array of addresses. This array has
// MAX_PORT_ID_NUM members.
// @count:	Number of MAC addresses in the array
// @pass_all:	Set 1 to pass all multicast packets.
// @bssid:	current association BSSID.
// @reserved:	reserved
// @addr_list:	Place holder for array of MAC addresses.
// IMPORTANT: add padding if necessary to ensure DWORD alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcast_filter_cmd {
    pub filter_own: u8,
    pub port_id: u8,
    pub count: u8,
    pub pass_all: u8,
    pub bssid: [u8; 6],
    pub reserved: [u8; 2],
    pub addr_list: [u8; ],
    pub /: *mut *mut } __packed; / MCAST_FILTERING_CMD_API_S_VER_1,
