//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/coredump.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const ATH12K_FW_CRASH_DUMP_V2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_fw_crash_dump_type {
    FW_CRASH_DUMP_PAGING_DATA,
    FW_CRASH_DUMP_RDDM_DATA,
    FW_CRASH_DUMP_REMOTE_MEM_DATA,
    FW_CRASH_DUMP_PAGEABLE_DATA,
    FW_CRASH_DUMP_M3_DUMP,
    FW_CRASH_DUMP_NONE,
    FW_CRASH_DUMP_MLO_GLOBAL_DATA,

// keep last
    FW_CRASH_DUMP_TYPE_MAX,
}

pub const COREDUMP_TLV_HDR_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_tlv_dump_data {
// see ath11k_fw_crash_dump_type above
    pub type: __le32,
// in bytes
    pub tlv_len: __le32,
// pad to 32-bit boundaries as needed
    pub tlv_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dump_file_data {
// "ATH12K-FW-DUMP"
    pub df_magic: [c_char; 16],
// total dump len in bytes
    pub len: __le32,
// file dump version
    pub version: __le32,
// pci device id
    pub chip_id: __le32,
// qrtr instance id
    pub qrtr_id: __le32,
// pci domain id
    pub bus_id: __le32,
    pub guid: guid_t,
// time-of-day stamp
    pub tv_sec: __le64,
// time-of-day stamp, nano-seconds
    pub tv_nsec: __le64,
// room for growth w/out changing binary format
    pub unused: [u8; 128],
    pub data: [u8; ],
    pub __packed: },

    pub type): (enum ath12k_qmi_target_mem,
    pub work): *mut void ath12k_coredump_upload(struct work_struct,
    pub ab): *mut void ath12k_coredump_collect(struct ath12k_base,

    pub FW_CRASH_DUMP_TYPE_MAX: return,

