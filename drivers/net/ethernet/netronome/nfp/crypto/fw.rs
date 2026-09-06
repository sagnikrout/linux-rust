//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/crypto/fw.h
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
// Copyright (C) 2019 Netronome Systems, Inc.
pub const NFP_CRYPTO_FW_H: c_int = 1;

pub const NFP_NET_CRYPTO_OP_TLS_1_2_AES_GCM_128_ENC: c_int = 0;
pub const NFP_NET_CRYPTO_OP_TLS_1_2_AES_GCM_128_DEC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_tls_resync_req {
    pub fw_handle: [__be32; 2],
    pub tcp_seq: __be32,
    pub l3_offset: u8,
    pub l4_offset: u8,
    pub resv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_reply_simple {
    pub hdr: nfp_ccm_hdr,
    pub error: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_reset {
    pub hdr: nfp_ccm_hdr,
    pub ep_id: __be32,
}

pub const NFP_NET_TLS_VLAN_UNUSED: c_int = 4095;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_add_front {
// New members MUST be added within the struct_group() macro below.
    pub hdr: nfp_ccm_hdr,
    pub ep_id: __be32,
    pub resv: [u8; 3],
    pub opcode: u8,
    pub key_len: u8,
    pub __packed: __be16 ipver_vlan,
    pub l4_proto: u8,
pub const NFP_NET_TLS_NON_ADDR_KEY_LEN: c_int = 8;
    pub l3_addrs: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_add_back {
    pub src_port: __be16,
    pub dst_port: __be16,
    pub key: [__be32; 8],
    pub salt: __be32,
    pub iv: [__be32; 2],
    pub counter: __be32,
    pub rec_no: [__be32; 2],
    pub tcp_seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_add_v4 {
    pub front: nfp_crypto_req_add_front_hdr,
    pub src_ip: __be32,
    pub dst_ip: __be32,
    pub back: nfp_crypto_req_add_back,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_add_v6 {
    pub front: nfp_crypto_req_add_front_hdr,
    pub src_ip: [__be32; 4],
    pub dst_ip: [__be32; 4],
    pub back: nfp_crypto_req_add_back,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_reply_add {
    pub hdr: nfp_ccm_hdr,
    pub error: __be32,
    pub handle: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_del {
    pub hdr: nfp_ccm_hdr,
    pub ep_id: __be32,
    pub handle: [__be32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_crypto_req_update {
    pub hdr: nfp_ccm_hdr,
    pub ep_id: __be32,
    pub resv: [u8; 3],
    pub opcode: u8,
    pub handle: [__be32; 2],
    pub rec_no: [__be32; 2],
    pub tcp_seq: __be32,
}
