//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/wifi7/hal_tx.h
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

// TODO: check all these data can be managed with struct ath12k_tx_desc_info for perf
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_info {
    pub /: *mut *mut u16 meta_data_flags; / %HAL_TCL_DATA_CMD_INFO0_META_,
    pub ring_id: u8,
    pub rbm_id: u8,
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
    pub vdev_id: u8,
    pub dscp_tid_tbl_idx: u8,
    pub enable_mesh: bool,
    pub bank_id: c_int,
    pub lookup_override: bool,
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
    pub pkt_type: hal_tx_rate_stats_pkt_type,
    pub sgi: hal_tx_rate_stats_sgi,
    pub bw: ath12k_supported_bw,
    pub mcs: u8,
    pub tones: u16,
    pub ofdma: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_phy_desc {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_status_prot {
    pub reserved: __le64,
    pub info0: __le32,
    pub info1: __le32,
    pub reserved1: [__le32; 11],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_status_user_ppdu {
    pub reserved: __le64,
    pub info0: __le32,
    pub reserved1: [__le32; 3],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_status_start_prot {
    pub info0: __le32,
    pub info1: __le32,
    pub reserved: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_status_start {
    pub reserved: __le32,
    pub info0: __le32,
    pub reserved1: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_queue_exten {
    pub info0: __le32,
    pub info1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_setup {
    pub schedule_id: __le32,
    pub info0: __le32,
    pub reserved: __le64,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_pcu_ppdu_setup_init {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub reserved: __le32,
    pub info4: __le32,
    pub info5: __le32,
    pub info6: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_tx_fes_status_end {
    pub reserved: [__le32; 2],
    pub info0: __le32,
    pub reserved1: [__le32; 19],
    pub __packed: },

// STA mode will have MCAST_PKT_CTRL instead of DSCP_TID_MAP bitfield

    pub id): *mut *mut void ath12k_wifi7_hal_tx_set_dscp_tid_map(struct ath12k_base ab, int,
    pub ti): *mut hal_tx_info,
    pub cmd): *mut ath12k_hal_reo_cmd,
    pub bank_id): u8,
