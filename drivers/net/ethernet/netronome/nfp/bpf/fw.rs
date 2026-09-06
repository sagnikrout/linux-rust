//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/bpf/fw.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.
pub const NFP_BPF_FW_H: c_int = 1;

// Kernel's enum bpf_reg_type is not uABI so people may change it breaking
// our FW ABI.  In that case we will do translation in the driver.
//
pub const NFP_BPF_SCALAR_VALUE: c_int = 1;
pub const NFP_BPF_MAP_VALUE: c_int = 4;
pub const NFP_BPF_STACK: c_int = 6;
pub const NFP_BPF_PACKET_DATA: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_cap_tlv_type {
    NFP_BPF_CAP_TYPE_FUNC		= 1,
    NFP_BPF_CAP_TYPE_ADJUST_HEAD	= 2,
    NFP_BPF_CAP_TYPE_MAPS		= 3,
    NFP_BPF_CAP_TYPE_RANDOM		= 4,
    NFP_BPF_CAP_TYPE_QUEUE_SELECT	= 5,
    NFP_BPF_CAP_TYPE_ADJUST_TAIL	= 6,
    NFP_BPF_CAP_TYPE_ABI_VERSION	= 7,
    NFP_BPF_CAP_TYPE_CMSG_MULTI_ENT	= 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_cap_tlv_func {
    pub func_id: __le32,
    pub func_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_cap_tlv_adjust_head {
    pub flags: __le32,
    pub off_min: __le32,
    pub off_max: __le32,
    pub guaranteed_sub: __le32,
    pub guaranteed_add: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_bpf_cap_tlv_maps {
    pub types: __le32,
    pub max_maps: __le32,
    pub max_elems: __le32,
    pub max_key_sz: __le32,
    pub max_val_sz: __le32,
    pub max_elem_sz: __le32,
}

//
// Types defined for map related control messages
//
// BPF ABIv2 fixed-length control message fields
pub const CMSG_MAP_KEY_LW: c_int = 16;
pub const CMSG_MAP_VALUE_LW: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_bpf_cmsg_status {
    CMSG_RC_SUCCESS			= 0,
    CMSG_RC_ERR_MAP_FD		= 1,
    CMSG_RC_ERR_MAP_NOENT		= 2,
    CMSG_RC_ERR_MAP_ERR		= 3,
    CMSG_RC_ERR_MAP_PARSE		= 4,
    CMSG_RC_ERR_MAP_EXIST		= 5,
    CMSG_RC_ERR_MAP_NOMEM		= 6,
    CMSG_RC_ERR_MAP_E2BIG		= 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_reply_map_simple {
    pub hdr: nfp_ccm_hdr,
    pub rc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_req_map_alloc_tbl {
    pub hdr: nfp_ccm_hdr,
    pub /: *mut *mut __be32 key_size; / in bytes,
    pub /: *mut *mut __be32 value_size; / in bytes,
    pub max_entries: __be32,
    pub map_type: __be32,
    pub /: *mut *mut __be32 map_flags; / reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_reply_map_alloc_tbl {
    pub reply_hdr: cmsg_reply_map_simple,
    pub tid: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_req_map_free_tbl {
    pub hdr: nfp_ccm_hdr,
    pub tid: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_reply_map_free_tbl {
    pub reply_hdr: cmsg_reply_map_simple,
    pub count: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_req_map_op {
    pub hdr: nfp_ccm_hdr,
    pub tid: __be32,
    pub count: __be32,
    pub flags: __be32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_reply_map_op {
    pub reply_hdr: cmsg_reply_map_simple,
    pub count: __be32,
    pub resv: __be32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_bpf_event {
    pub hdr: nfp_ccm_hdr,
    pub cpu_id: __be32,
    pub map_ptr: __be64,
    pub data_size: __be32,
    pub pkt_size: __be32,
    pub data: [u8; ],
}
