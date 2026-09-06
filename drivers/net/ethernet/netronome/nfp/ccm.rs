//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/ccm.h
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
// Copyright (C) 2016-2019 Netronome Systems, Inc.
pub const NFP_CCM_H: c_int = 1;

// Firmware ABI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_ccm_type {
    NFP_CCM_TYPE_BPF_MAP_ALLOC	= 1,
    NFP_CCM_TYPE_BPF_MAP_FREE	= 2,
    NFP_CCM_TYPE_BPF_MAP_LOOKUP	= 3,
    NFP_CCM_TYPE_BPF_MAP_UPDATE	= 4,
    NFP_CCM_TYPE_BPF_MAP_DELETE	= 5,
    NFP_CCM_TYPE_BPF_MAP_GETNEXT	= 6,
    NFP_CCM_TYPE_BPF_MAP_GETFIRST	= 7,
    NFP_CCM_TYPE_BPF_BPF_EVENT	= 8,
    NFP_CCM_TYPE_CRYPTO_RESET	= 9,
    NFP_CCM_TYPE_CRYPTO_ADD		= 10,
    NFP_CCM_TYPE_CRYPTO_DEL		= 11,
    NFP_CCM_TYPE_CRYPTO_UPDATE	= 12,
    NFP_CCM_TYPE_CRYPTO_RESYNC	= 13,
    __NFP_CCM_TYPE_MAX,
}

pub const NFP_CCM_ABI_VERSION: c_int = 1;
pub const NFP_CCM_TYPE_REPLY_BIT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_ccm_hdr {
    pub type: u8,
    pub ver: u8,
    pub tag: __be16,
}

extern "C" {
    pub fn be16_to_cpu(_arg: __nfp_ccm_get_tag(skb)) -> return;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_ccm_mbox_tlv_type {
    NFP_NET_MBOX_TLV_TYPE_UNKNOWN	= 0,
    NFP_NET_MBOX_TLV_TYPE_END	= 1,
    NFP_NET_MBOX_TLV_TYPE_MSG	= 2,
    NFP_NET_MBOX_TLV_TYPE_MSG_NOSUP	= 3,
    NFP_NET_MBOX_TLV_TYPE_RESV	= 4,
}

// Implementation
//
// struct nfp_ccm - common control message handling
// @app:		APP handle
//
// @tag_allocator:	bitmap of control message tags in use
// @tag_alloc_next:	next tag bit to allocate
// @tag_alloc_last:	next tag bit to be freed
//
// @replies:		received cmsg replies waiting to be consumed
// @wq:			work queue for waiting for cmsg replies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_ccm {
    pub app: *mut nfp_app,
    pub 1): DECLARE_BITMAP(tag_allocator, U16_MAX +,
    pub tag_alloc_next: u16,
    pub tag_alloc_last: u16,
    pub replies: sk_buff_head,
    pub wq: wait_queue_head_t,
}

extern "C" {
    pub fn nfp_ccm_init(ccm: *mut nfp_ccm, app: *mut nfp_app) -> c_int;
}
extern "C" {
    pub fn nfp_ccm_clean(ccm: *mut nfp_ccm);
}
extern "C" {
    pub fn nfp_ccm_rx(ccm: *mut nfp_ccm, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_ccm_mbox_alloc(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_ccm_mbox_free(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_ccm_mbox_init(nn: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_ccm_mbox_clean(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_ccm_mbox_fits(nn: *mut nfp_net, size: c_uint) -> bool;
}
