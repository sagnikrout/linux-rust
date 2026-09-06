//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_hw_def.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025 Broadcom

pub const TX_BD_HSIZE_SHIFT: c_int = 16;

pub const TX_BD_CFA_ACTION_SHIFT: c_int = 16;
pub const TX_BD_CFA_META_MASK: c_uint = 0xfffffff;
pub const TX_BD_CFA_META_VID_MASK: c_uint = 0xfff;

pub const TX_BD_CFA_META_PRI_SHIFT: c_int = 12;

pub const TX_BD_CFA_META_TPID_SHIFT: c_int = 16;

pub const TX_BD_CFA_META_KEY_SHIFT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_bd_ext {
    pub tx_bd_hsize_lflags: __le32,
    pub tx_bd_mss: __le32,
    pub tx_bd_cfa_action: __le32,
    pub tx_bd_cfa_meta: __le32,
}

pub const RX_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;
pub const RX_CMP_FLAGS_ITYPES_MASK: c_uint = 0xf000;

pub const RX_CMP_LEN_SHIFT: c_int = 16;

pub const RX_CMP_AGG_BUFS_SHIFT: c_int = 1;

pub const RX_CMP_RSS_HASH_TYPE_SHIFT: c_int = 9;

pub const RX_CMP_V3_RSS_EXT_OP_LEGACY_SHIFT: c_int = 12;

pub const RX_CMP_V3_RSS_EXT_OP_NEW_SHIFT: c_int = 8;

pub const RX_CMP_PAYLOAD_OFFSET_SHIFT: c_int = 16;

pub const RX_CMP_SUB_NS_TS_SHIFT: c_int = 16;

pub const RX_CMP_METADATA1_SHIFT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_cmp {
    pub rx_cmp_len_flags_type: __le32,
    pub rx_cmp_opaque: u32,
    pub rx_cmp_misc_v1: __le32,
    pub rx_cmp_rss_hash: __le32,
}

pub const RX_CMP_FLAGS2_METADATA_TPID_SFT: c_int = 16;

pub const RX_CMPL_ERRORS_SFT: c_int = 1;

pub const RX_CMPL_CFA_CODE_SFT: c_int = 16;

pub const RX_CMPL_METADATA0_SFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_cmp_ext {
    pub rx_cmp_flags2: __le32,
    pub rx_cmp_meta_data: __le32,
    pub rx_cmp_cfa_code_errors_v2: __le32,
    pub rx_cmp_timestamp: __le32,
}

pub const RX_AGG_CMP_LEN_SHIFT: c_int = 16;

pub const RX_AGG_CMP_AGG_ID_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_agg_cmp {
    pub rx_agg_cmp_len_flags_type: __le32,
    pub rx_agg_cmp_opaque: u32,
    pub rx_agg_cmp_v: __le32,
    pub rx_agg_cmp_unused: __le32,
}

pub const EXT_OP_INNER_4: c_uint = 0x0;
pub const EXT_OP_OUTER_4: c_uint = 0x2;
pub const EXT_OP_INNFL_3: c_uint = 0x8;
pub const EXT_OP_OUTFL_3: c_uint = 0xa;

pub const RX_TPA_START_CMP_FLAGS_SHIFT: c_int = 6;

pub const RX_TPA_START_CMP_FLAGS_PLACEMENT_SHIFT: c_int = 7;

pub const RX_TPA_START_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;

pub const RX_TPA_START_CMP_LEN_SHIFT: c_int = 16;

pub const RX_TPA_START_CMP_RSS_HASH_TYPE_SHIFT: c_int = 9;

pub const RX_TPA_START_CMP_V3_RSS_HASH_TYPE_SHIFT: c_int = 7;

pub const RX_TPA_START_CMP_AGG_ID_SHIFT: c_int = 16;

pub const RX_TPA_START_CMP_METADATA1_SHIFT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_start_cmp {
    pub rx_tpa_start_cmp_len_flags_type: __le32,
    pub rx_tpa_start_cmp_opaque: u32,
    pub rx_tpa_start_cmp_misc_v1: __le32,
    pub rx_tpa_start_cmp_rss_hash: __le32,
}

pub const RX_TPA_START_CMP_FLAGS2_EXT_META_FORMAT_SHIFT: c_int = 10;

pub const RX_TPA_START_CMP_FLAGS2_CSUM_CMPL_SHIFT: c_int = 16;

pub const RX_TPA_START_CMP_ERRORS_BUFFER_ERROR_SHIFT: c_int = 1;

pub const RX_TPA_START_CMPL_CFA_CODE_SHIFT: c_int = 16;

pub const RX_TPA_START_CMP_METADATA0_SFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_start_cmp_ext {
    pub rx_tpa_start_cmp_flags2: __le32,
    pub rx_tpa_start_cmp_metadata: __le32,
    pub rx_tpa_start_cmp_cfa_code_v2: __le32,
    pub rx_tpa_start_cmp_hdr_info: __le32,
}

pub const RX_TPA_END_CMP_FLAGS_SHIFT: c_int = 6;

pub const RX_TPA_END_CMP_FLAGS_PLACEMENT_SHIFT: c_int = 7;

pub const RX_TPA_END_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;

pub const RX_TPA_END_CMP_LEN_SHIFT: c_int = 16;

pub const RX_TPA_END_CMP_TPA_SEGS_SHIFT: c_int = 8;

pub const RX_TPA_END_CMP_AGG_ID_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_end_cmp {
    pub rx_tpa_end_cmp_len_flags_type: __le32,
    pub rx_tpa_end_cmp_opaque: u32,
    pub rx_tpa_end_cmp_misc_v1: __le32,
    pub rx_tpa_end_cmp_tsdelta: __le32,
}

pub const RX_TPA_END_CMP_PAYLOAD_OFFSET_SHIFT: c_int = 16;

pub const RX_TPA_END_CMP_AGG_BUFS_SHIFT: c_int = 24;

pub const RX_TPA_END_CMPL_ERRORS_SHIFT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_end_cmp_ext {
    pub rx_tpa_end_cmp_dup_acks: __le32,
    pub rx_tpa_end_cmp_seg_len: __le32,
    pub rx_tpa_end_cmp_errors_v2: __le32,
    pub rx_tpa_end_cmp_start_opaque: u32,
}

