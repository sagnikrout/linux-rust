//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_fdir.h
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
// State of Flow Director filter
//
// *_REQUEST states are used to mark filter to be sent to PF driver to perform
// an action (either add or delete filter). *_PENDING states are an indication
// that request was sent to PF and the driver is waiting for response.
//
// Both DELETE and DISABLE states are being used to delete a filter in PF.
// The difference is that after a successful response filter in DEL_PENDING
// state is being deleted from VF driver as well and filter in DIS_PENDING state
// is being changed to INACTIVE state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_fdir_fltr_state_t {
    IAVF_FDIR_FLTR_ADD_REQUEST,	/* User requests to add filter */
    IAVF_FDIR_FLTR_ADD_PENDING,	/* Filter pending add by the PF */
    IAVF_FDIR_FLTR_DEL_REQUEST,	/* User requests to delete filter */
    IAVF_FDIR_FLTR_DEL_PENDING,	/* Filter pending delete by the PF */
    IAVF_FDIR_FLTR_DIS_REQUEST,	/* Filter scheduled to be disabled */
    IAVF_FDIR_FLTR_DIS_PENDING,	/* Filter pending disable by the PF */
    IAVF_FDIR_FLTR_INACTIVE,	/* Filter inactive on link down */
    IAVF_FDIR_FLTR_ACTIVE,		/* Filter is active */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_fdir_flow_type {
// NONE - used for undef/error
    IAVF_FDIR_FLOW_NONE = 0,
    IAVF_FDIR_FLOW_IPV4_TCP,
    IAVF_FDIR_FLOW_IPV4_UDP,
    IAVF_FDIR_FLOW_IPV4_SCTP,
    IAVF_FDIR_FLOW_IPV4_AH,
    IAVF_FDIR_FLOW_IPV4_ESP,
    IAVF_FDIR_FLOW_IPV4_OTHER,
    IAVF_FDIR_FLOW_IPV6_TCP,
    IAVF_FDIR_FLOW_IPV6_UDP,
    IAVF_FDIR_FLOW_IPV6_SCTP,
    IAVF_FDIR_FLOW_IPV6_AH,
    IAVF_FDIR_FLOW_IPV6_ESP,
    IAVF_FDIR_FLOW_IPV6_OTHER,
    IAVF_FDIR_FLOW_NON_IP_L2,
// MAX - this must be last and add anything new just above it
    IAVF_FDIR_FLOW_PTYPE_MAX,
}

// Must not exceed the array element number of '__be32 data[2]' in the ethtool
// 'struct ethtool_rx_flow_spec.m_ext.data[2]' to express the flex-byte (word).
//
pub const IAVF_FLEX_WORD_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_flex_word {
    pub offset: u16,
    pub word: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_ipv4_addrs {
    pub src_ip: __be32,
    pub dst_ip: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_ipv6_addrs {
    pub src_ip: in6_addr,
    pub dst_ip: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_fdir_eth {
    pub etype: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_fdir_ip {
    pub v4_addrs: iavf_ipv4_addrs,
    pub v6_addrs: iavf_ipv6_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_fdir_extra {
    pub usr_def: [u32; IAVF_FLEX_WORD_NUM],
}

// bookkeeping of Flow Director filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_fdir_fltr {
    pub state: iavf_fdir_fltr_state_t,
    pub list: list_head,
    pub flow_type: iavf_fdir_flow_type,
    pub eth_data: iavf_fdir_eth,
    pub eth_mask: iavf_fdir_eth,
    pub ip_data: iavf_fdir_ip,
    pub ip_mask: iavf_fdir_ip,
    pub ext_data: iavf_fdir_extra,
    pub ext_mask: iavf_fdir_extra,
    pub action: virtchnl_action,
// flex byte filter data
    pub /: *mut *mut u8 ip_ver; / used to adjust the flex offset, 4 : IPv4, 6 : IPv6,
    pub flex_cnt: u8,
    pub flex_words: [iavf_flex_word; IAVF_FLEX_WORD_NUM],
    pub flow_id: u32,
    pub /: *mut *mut u32 cls_u32_handle; / for FDIR added via tc u32,
    pub /: *mut *mut u32 loc; / Rule location inside the flow table,
    pub q_index: u32,
    pub vc_add_msg: virtchnl_fdir_add,
}

extern "C" {
    pub fn iavf_fill_fdir_add_msg(adapter: *mut iavf_adapter, fltr: *mut iavf_fdir_fltr) -> c_int;
}
extern "C" {
    pub fn iavf_print_fdir_fltr(adapter: *mut iavf_adapter, fltr: *mut iavf_fdir_fltr);
}
extern "C" {
    pub fn iavf_fdir_is_dup_fltr(adapter: *mut iavf_adapter, fltr: *mut iavf_fdir_fltr) -> bool;
}
extern "C" {
    pub fn iavf_fdir_del_fltr(adapter: *mut iavf_adapter, is_raw: bool, data: u32) -> c_int;
}
