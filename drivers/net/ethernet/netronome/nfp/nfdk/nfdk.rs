//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfdk/nfdk.h
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

pub const NFDK_TX_DESC_PER_SIMPLE_PKT: c_int = 2;

pub const NFDK_TX_DESC_BLOCK_SZ: c_int = 256;

pub const NFDK_TX_DESC_GATHER_MAX: c_int = 17;
// TX descriptor format

pub const NFDK_DESC_TX_TYPE_NOP: c_int = 0;
pub const NFDK_DESC_TX_TYPE_GATHER: c_int = 1;
pub const NFDK_DESC_TX_TYPE_TSO: c_int = 2;
pub const NFDK_DESC_TX_TYPE_SIMPLE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_nfdk_tx_desc {
    pub /: *mut *mut __le16 dma_addr_hi; / High bits of host buf address,
    pub /: *mut *mut __le16 dma_len_type; / Length to DMA for this desc,
    pub /: *mut *mut __le32 dma_addr_lo; / Low 32bit of host buf addr,
}

// The device don't make use of the 2 or 3 least significant bits of the address
// due to alignment constraints. The driver can make use of those bits to carry
// information about the buffer before giving it to the device.
//
// NOTE: The driver must clear the lower bits before handing the buffer to the
// device.
//
// - NFDK_TX_BUF_INFO_SOP - Start of a packet
// Mark the buffer as a start of a packet. This is used in the XDP TX process
// to stash virtual and DMA address so that they can be recycled when the TX
// operation is completed.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_nfdk_tx_buf {
// First slot
    pub skb: *mut sk_buff,
    pub frag: *mut c_void,
    pub val: c_ulong,
}

// 1 + nr_frags next slots
// TSO (optional)
// First descriptor fits less data, so adjust for that
extern "C" {
    pub fn nfp_nfdk_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn nfp_nfdk_tx(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn nfp_nfdk_ctrl_poll(t: *mut tasklet_struct);
}

extern "C" {
    pub fn nfp_nfdk_ipsec_tx(flags: u64, skb: *mut sk_buff) -> u64;
}

