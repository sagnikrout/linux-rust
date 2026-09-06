//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_txrx.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

// Guarantee we have space needed for storing the buffer
// To store the buffer we need:
// 1 descriptor per page
// + 1 descriptor for skb head
// + 2 descriptors for metadata and optional metadata
// + 7 descriptors to keep tail out of the same cacheline as head
// If we cannot guarantee that then we should return TX_BUSY
//

// To receive the worst case packet we need:
// 1 descriptor for primary metadata
// + 1 descriptor for optional metadata
// + 1 descriptor for headers
// + 4 descriptors for payload
//
pub const FBNIC_MAX_RX_PKT_DESC: c_int = 7;

// These apply to TWQs, TCQ, RCQ

pub const FBNIC_TXQ_SIZE_DEFAULT: c_int = 1024;
pub const FBNIC_HPQ_SIZE_DEFAULT: c_int = 256;
pub const FBNIC_PPQ_SIZE_DEFAULT: c_int = 256;
pub const FBNIC_RCQ_SIZE_DEFAULT: c_int = 1024;
pub const FBNIC_TX_USECS_DEFAULT: c_int = 35;
pub const FBNIC_RX_USECS_DEFAULT: c_int = 30;
pub const FBNIC_RX_FRAMES_DEFAULT: c_int = 0;

pub const FBNIC_RX_HROOM_PAD: c_int = 128;

pub const FBNIC_RX_PAD: c_int = 0;
pub const FBNIC_RX_PAYLD_OFFSET: c_int = 0;
pub const FBNIC_RX_PAYLD_PG_CL: c_int = 0;

pub const FBNIC_HDR_BYTES_MIN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_pkt_buff {
    pub buff: xdp_buff,
    pub hwtstamp: ktime_t,
    pub add_frag_failed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub csum_partial: u64,
    pub lso: u64,
    pub ts_packets: u64,
    pub ts_lost: u64,
    pub stop: u64,
    pub wake: u64,
    pub twq: },
    pub alloc_failed: u64,
    pub csum_complete: u64,
    pub csum_none: u64,
    pub length_errors: u64,
    pub rx: },
    pub alloc_failed: u64,
    pub bdq: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rx_buf {
    pub netmem: netmem_ref,
    pub pagecnt_bias: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_ring {
// Pointer to buffer specific info
    pub /: *mut *mut *mut fbnic_pkt_buff pkt; / RCQ,
    pub /: *mut *mut *mut fbnic_rx_buf rx_buf; / BDQ,
    pub /: *mut *mut *mut *mut void tx_buf; / TWQ,
    pub /: *mut *mut *mut void buffer; / Generic pointer,
}

// Rx BDQs only
// Deferred_head is used to cache the head for TWQ1 if
// an attempt is made to clean TWQ1 with zero napi_budget.
// We do not use it for any other ring.
//
// Slow path fields follow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_q_triad {
    pub cmpl: fbnic_ring sub0, sub1,,
    pub xdp_rxq: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_napi_vector {
    pub napi: napi_struct,
    pub /: *mut *mut *mut device dev; / Device for DMA unmapping,
    pub fbd: *mut fbnic_dev,
    pub dbg_nv: *mut dentry,
    pub v_idx: u16,
    pub txt_count: u8,
    pub rxt_count: u8,
    pub qt: [fbnic_q_triad; ],
}

extern "C" {
    pub fn fbnic_xmit_frame(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn fbnic_alloc_napi_vectors(fbn: *mut fbnic_net) -> c_int;
}
extern "C" {
    pub fn fbnic_free_napi_vectors(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_alloc_resources(fbn: *mut fbnic_net) -> c_int;
}
extern "C" {
    pub fn fbnic_free_resources(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_set_netif_queues(fbn: *mut fbnic_net) -> c_int;
}
extern "C" {
    pub fn fbnic_reset_netif_queues(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_msix_clean_rings(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn fbnic_napi_enable(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_napi_disable(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_config_drop_mode(fbn: *mut fbnic_net, tx_pause: bool);
}
extern "C" {
    pub fn fbnic_enable(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_disable(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_dbg_up(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_dbg_down(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_flush(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_fill(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_napi_depletion_check(netdev: *mut net_device);
}
extern "C" {
    pub fn fbnic_wait_all_queues_idle(fbd: *mut fbnic_dev, may_fail: bool) -> c_int;
}
extern "C" {
    pub fn fbnic_dbg_nv_init(nv: *mut fbnic_napi_vector);
}
extern "C" {
    pub fn fbnic_dbg_nv_exit(nv: *mut fbnic_napi_vector);
}
