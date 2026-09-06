//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_txrx_common.h
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
pub const IXGBE_XDP_PASS: c_int = 0;

extern "C" {
    pub fn ixgbe_xdp_ring_update_tail(ring: *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_xdp_ring_update_tail_locked(ring: *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_irq_rearm_queues(adapter: *mut ixgbe_adapter, qmask: u64);
}
extern "C" {
    pub fn ixgbe_txrx_ring_disable(adapter: *mut ixgbe_adapter, ring: c_int);
}
extern "C" {
    pub fn ixgbe_txrx_ring_enable(adapter: *mut ixgbe_adapter, ring: c_int);
}
extern "C" {
    pub fn ixgbe_alloc_rx_buffers_zc(rx_ring: *mut ixgbe_ring, cleaned_count: u16) -> bool;
}
extern "C" {
    pub fn ixgbe_xsk_clean_rx_ring(rx_ring: *mut ixgbe_ring);
}
extern "C" {
    pub fn ixgbe_xsk_wakeup(dev: *mut net_device, queue_id: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_xsk_clean_tx_ring(tx_ring: *mut ixgbe_ring);
}
