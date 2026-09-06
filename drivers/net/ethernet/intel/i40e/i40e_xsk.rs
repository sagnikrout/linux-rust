//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_xsk.h
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
// Copyright(c) 2018 Intel Corporation.

// This value should match the pragma in the unrolled_count()
// macro. Why 4? It is strictly empirical. It seems to be a good
// compromise between the advantage of having simultaneous outstanding
// reads to the DMA array that can hide each others latency and the
// disadvantage of having a larger code path.
//
pub const PKTS_PER_BATCH: c_int = 4;
extern "C" {
    pub fn i40e_queue_pair_disable(vsi: *mut i40e_vsi, queue_pair: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_queue_pair_enable(vsi: *mut i40e_vsi, queue_pair: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_alloc_rx_buffers_zc(rx_ring: *mut i40e_ring, cleaned_count: u16) -> bool;
}
extern "C" {
    pub fn i40e_clean_rx_irq_zc(rx_ring: *mut i40e_ring, budget: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_clean_xdp_tx_irq(vsi: *mut i40e_vsi, tx_ring: *mut i40e_ring) -> bool;
}
extern "C" {
    pub fn i40e_xsk_wakeup(dev: *mut net_device, queue_id: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn i40e_realloc_rx_bi_zc(vsi: *mut i40e_vsi, zc: bool) -> c_int;
}
extern "C" {
    pub fn i40e_clear_rx_bi_zc(rx_ring: *mut i40e_ring);
}
