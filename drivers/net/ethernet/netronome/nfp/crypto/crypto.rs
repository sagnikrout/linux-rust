//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/crypto/crypto.h
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
pub const NFP_CRYPTO_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_net_tls_offload_ctx {
    pub fw_handle: [__be32; 2],
    pub rx_end: [u8; 0],
// Tx only fields follow - Rx side does not have enough driver state
// to fit these
//
    pub next_seq: u32,
}

extern "C" {
    pub fn nfp_net_tls_init(nn: *mut nfp_net) -> c_int;
}

// IPsec related structures and functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_ipsec_offload {
    pub seq_hi: u32,
    pub seq_low: u32,
    pub handle: u32,
}

extern "C" {
    pub fn nfp_net_ipsec_init(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_ipsec_clean(nn: *mut nfp_net);
}
extern "C" {
    pub fn nfp_net_ipsec_rx(meta: *mut nfp_meta_parsed, skb: *mut sk_buff) -> c_int;
}

