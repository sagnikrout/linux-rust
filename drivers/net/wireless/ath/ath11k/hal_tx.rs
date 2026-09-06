//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/hal_tx.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const HAL_TX_ADDRX_EN: c_int = 1;
pub const HAL_TX_ADDRY_EN: c_int = 2;
pub const HAL_TX_ADDR_SEARCH_DEFAULT: c_int = 0;
pub const HAL_TX_ADDR_SEARCH_INDEX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_info {
    pub /: *mut *mut u16 meta_data_flags; / %HAL_TCL_DATA_CMD_INFO0_META_,
    pub ring_id: u8,
    pub desc_id: u32,
    pub type: hal_tcl_desc_type,
    pub encap_type: hal_tcl_encap_type,
    pub paddr: dma_addr_t,
    pub data_len: u32,
    pub pkt_offset: u32,
    pub encrypt_type: hal_encrypt_type,
    pub /: *mut *mut u32 flags0; / %HAL_TCL_DATA_CMD_INFO1_,
    pub /: *mut *mut u32 flags1; / %HAL_TCL_DATA_CMD_INFO2_,
    pub /: *mut *mut u16 addr_search_flags; / %HAL_TCL_DATA_CMD_INFO0_ADDR(X/Y)_,
    pub bss_ast_hash: u16,
    pub bss_ast_idx: u16,
    pub tid: u8,
    pub /: *mut *mut u8 search_type; / %HAL_TX_ADDR_SEARCH_,
    pub lmac_id: u8,
    pub dscp_tid_tbl_idx: u8,
    pub enable_mesh: bool,
    pub rbm_id: u8,
}

// TODO: Check if the actual desc macros can be used instead

// Tx status parsed from srng desc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_status {
    pub buf_rel_source: hal_wbm_rel_src_module,
    pub status: hal_wbm_tqm_rel_reason,
    pub ack_rssi: i8,
    pub /: *mut *mut u32 flags; / %HAL_TX_STATUS_FLAGS_,
    pub ppdu_id: u32,
    pub try_cnt: u8,
    pub tid: u8,
    pub peer_id: u16,
    pub rate_stats: u32,
}

extern "C" {
    pub fn ath11k_hal_tx_set_dscp_tid_map(ab: *mut ath11k_base, id: c_int);
}
