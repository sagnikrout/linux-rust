//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/wifi7/hal_qcn9274.h
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
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

extern "C" {
    pub fn ath12k_hal_rx_desc_get_l3_pad_bytes_qcn9274(desc: *mut hal_rx_desc) -> u8;
}
extern "C" {
    pub fn ath12k_hal_rx_desc_get_mpdu_ppdu_id_qcn9274(desc: *mut hal_rx_desc) -> u32;
}
extern "C" {
    pub fn ath12k_hal_rx_desc_set_msdu_len_qcn9274(desc: *mut hal_rx_desc, len: u16);
}
extern "C" {
    pub fn ath12k_hal_rx_desc_get_mpdu_start_offset_qcn9274() -> u32;
}
extern "C" {
    pub fn ath12k_hal_rx_desc_get_msdu_end_offset_qcn9274() -> u32;
}
extern "C" {
    pub fn ath12k_hal_get_rx_desc_size_qcn9274() -> u32;
}
extern "C" {
    pub fn ath12k_hal_rx_desc_get_msdu_src_link_qcn9274(desc: *mut hal_rx_desc) -> u8;
}
extern "C" {
    pub fn ath12k_hal_rx_mpdu_start_wmask_get_qcn9274() -> u16;
}
extern "C" {
    pub fn ath12k_hal_rx_msdu_end_wmask_get_qcn9274() -> u32;
}
