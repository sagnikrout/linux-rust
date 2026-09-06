//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_ring.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_ring.h: Declaration of functions for Rx/Tx rings.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rxpage {
    pub page: *mut page,
    pub daddr: dma_addr_t,
    pub order: c_uint,
    pub pg_off: c_uint,
}

// TxC       SOP        DX         EOP
// +----------+----------+----------+-----------
// 8bytes|len l3,l4 | pa       | pa       | pa
// +----------+----------+----------+-----------
// 4/8bytes|len pkt   |len pkt   |          | skb
// +----------+----------+----------+-----------
// 4/8bytes|is_gso    |len,flags |len       |len,is_eop
// +----------+----------+----------+-----------
//
// This aq_ring_buff_s doesn't have endianness dependency.
// It is __packed for cache line optimizations.
//
// RX/TX
// RX
// EOP
// TxC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_ring_stats_rx_s {
    pub /: *mut *mut u64_stats_sync syncp; / must be first,
    pub errors: u64,
    pub packets: u64,
    pub bytes: u64,
    pub lro_packets: u64,
    pub jumbo_packets: u64,
    pub alloc_fails: u64,
    pub skb_alloc_fails: u64,
    pub polls: u64,
    pub pg_losts: u64,
    pub pg_flips: u64,
    pub pg_reuses: u64,
    pub xdp_aborted: u64,
    pub xdp_drop: u64,
    pub xdp_pass: u64,
    pub xdp_tx: u64,
    pub xdp_invalid: u64,
    pub xdp_redirect: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_ring_stats_tx_s {
    pub /: *mut *mut u64_stats_sync syncp; / must be first,
    pub errors: u64,
    pub packets: u64,
    pub bytes: u64,
    pub queue_restarts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union aq_ring_stats_s {
    pub rx: aq_ring_stats_rx_s,
    pub tx: aq_ring_stats_tx_s,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl_ring_type {
    ATL_RING_TX,
    ATL_RING_RX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_ring_s {
    pub buff_ring: *mut aq_ring_buff_s,
    pub /: *mut *mut *mut u8 dx_ring; / descriptors ring, dma shared mem,
    pub aq_nic: *mut aq_nic_s,
    pub /: *mut *mut unsigned int idx; / for HW layer registers operations,
    pub hw_head: c_uint,
    pub sw_head: c_uint,
    pub sw_tail: c_uint,
    pub /: *mut *mut unsigned int size; / descriptors number,
    pub /: *mut *mut unsigned int dx_size; / TX or RX descriptor size,,
// stored here for fater math
    pub page_order: u16,
    pub page_offset: u16,
    pub frame_max: u16,
    pub tail_size: u16,
    pub stats: aq_ring_stats_s,
    pub dx_ring_pa: dma_addr_t,
    pub xdp_prog: *mut bpf_prog,
    pub ring_type: atl_ring_type,
    pub xdp_rxq: xdp_rxq_info,
    pub ptp_ts_deadline: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_ring_param_s {
    pub vec_idx: c_uint,
    pub cpu: c_uint,
    pub affinity_mask: cpumask_t,
}

extern "C" {
    pub fn aq_ring_init(self: *mut aq_ring_s, ring_type: atl_ring_type) -> c_int;
}
extern "C" {
    pub fn aq_ring_rx_deinit(self: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_free(self: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_update_queue_state(ring: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_queue_wake(ring: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_queue_stop(ring: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_tx_clean(self: *mut aq_ring_s) -> bool;
}
extern "C" {
    pub fn aq_ring_tx_deinit(self: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_rx_fill(self: *mut aq_ring_s) -> c_int;
}
extern "C" {
    pub fn aq_ring_hwts_rx_free(self: *mut aq_ring_s);
}
extern "C" {
    pub fn aq_ring_hwts_rx_clean(self: *mut aq_ring_s, aq_nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_ring_fill_stats_data(self: *mut aq_ring_s, data: *mut u64) -> c_uint;
}
