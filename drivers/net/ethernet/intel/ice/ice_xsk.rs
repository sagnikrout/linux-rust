//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_xsk.h
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
// Copyright (c) 2019, Intel Corporation.

pub const PKTS_PER_BATCH: c_int = 8;

extern "C" {
    pub fn ice_xsk_wakeup(netdev: *mut net_device, queue_id: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn ice_xsk_any_rx_ring_ena(vsi: *mut ice_vsi) -> bool;
}
extern "C" {
    pub fn ice_xsk_clean_rx_ring(rx_ring: *mut ice_rx_ring);
}
extern "C" {
    pub fn ice_xsk_clean_xdp_ring(xdp_ring: *mut ice_tx_ring);
}
extern "C" {
    pub fn ice_xmit_zc(xdp_ring: *mut ice_tx_ring, xsk_pool: *mut xsk_buff_pool) -> bool;
}
extern "C" {
    pub fn ice_realloc_rx_xdp_bufs(rx_ring: *mut ice_rx_ring, pool_present: bool) -> c_int;
}
extern "C" {
    pub fn ice_qvec_ena_irq(vsi: *mut ice_vsi, q_vector: *mut ice_q_vector);
}

