//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_adv_rss.h
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
// Copyright (c) 2021, Intel Corporation.
// State of advanced RSS configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_adv_rss_state_t {
    IAVF_ADV_RSS_ADD_REQUEST,	/* User requests to add RSS */
    IAVF_ADV_RSS_ADD_PENDING,	/* RSS pending add by the PF */
    IAVF_ADV_RSS_DEL_REQUEST,	/* Driver requests to delete RSS */
    IAVF_ADV_RSS_DEL_PENDING,	/* RSS pending delete by the PF */
    IAVF_ADV_RSS_ACTIVE,		/* RSS configuration is active */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_adv_rss_flow_seg_hdr {
    IAVF_ADV_RSS_FLOW_SEG_HDR_NONE	= 0x00000000,
    IAVF_ADV_RSS_FLOW_SEG_HDR_IPV4	= 0x00000001,
    IAVF_ADV_RSS_FLOW_SEG_HDR_IPV6	= 0x00000002,
    IAVF_ADV_RSS_FLOW_SEG_HDR_TCP	= 0x00000004,
    IAVF_ADV_RSS_FLOW_SEG_HDR_UDP	= 0x00000008,
    IAVF_ADV_RSS_FLOW_SEG_HDR_SCTP	= 0x00000010,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPC		= 0x00000400,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPC_TEID	= 0x00000800,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPU_IP	= 0x00001000,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPU_EH	= 0x00002000,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPU_DWN	= 0x00004000,
    IAVF_ADV_RSS_FLOW_SEG_HDR_GTPU_UP	= 0x00008000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_adv_rss_flow_field {
// L3
    IAVF_ADV_RSS_FLOW_FIELD_IDX_IPV4_SA,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_IPV4_DA,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_IPV6_SA,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_IPV6_DA,
// L4
    IAVF_ADV_RSS_FLOW_FIELD_IDX_TCP_SRC_PORT,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_TCP_DST_PORT,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_UDP_SRC_PORT,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_UDP_DST_PORT,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_SCTP_SRC_PORT,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_SCTP_DST_PORT,
// GTPC_TEID
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPC_TEID,
// GTPU_IP
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPU_IP_TEID,
// GTPU_EH
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPU_EH_TEID,
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPU_EH_QFI,
// GTPU_UP
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPU_UP_TEID,
// GTPU_DWN
    IAVF_ADV_RSS_FLOW_FIELD_IDX_GTPU_DWN_TEID,

// The total number of enums must not exceed 64
    IAVF_ADV_RSS_FLOW_FIELD_IDX_MAX
}

pub const IAVF_ADV_RSS_HASH_INVALID: c_int = 0;

// bookkeeping of advanced RSS configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_adv_rss {
    pub state: iavf_adv_rss_state_t,
    pub list: list_head,
    pub packet_hdrs: u32,
    pub hash_flds: u64,
    pub symm: bool,
    pub cfg_msg: virtchnl_rss_cfg,
}
