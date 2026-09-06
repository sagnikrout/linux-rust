//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/efx.h
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
//
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
//

// TX
extern "C" {
    pub fn efx_siena_init_tx_queue_core_txq(tx_queue: *mut efx_tx_queue);
}
// RX
extern "C" {
    pub fn __efx_siena_rx_packet(channel: *mut efx_channel);
}
// Maximum number of TCP segments we support for soft-TSO
pub const EFX_TSO_MAX_SEGS: c_int = 100;
// The smallest [rt]xq_entries that the driver supports.  RX minimum
// is a bit arbitrary.  For TX, we must have space for at least 2
// TSO skbs.
//

// All EF10 architecture NICs steal one bit of the DMAQ size for various
// other purposes when counting TxQ entries, so we halve the queue size.
//

// Filters
//
// efx_filter_insert_filter - add or replace a filter
// @efx: NIC in which to insert the filter
// @spec: Specification for the filter
// @replace_equal: Flag for whether the specified filter may replace an
// existing filter with equal priority
//
// On success, return the filter ID.
// On failure, return a negative error code.
//
// If existing filters have equal match values to the new filter spec,
// then the new filter might replace them or the function might fail,
// as follows.
//
// 1. If the existing filters have lower priority, or @replace_equal
// is set and they have equal priority, replace them.
//
// 2. If the existing filters have higher priority, return -%EPERM.
//
// 3. If !efx_siena_filter_is_mc_recipient(@spec), or the NIC does not
// support delivery to multiple recipients, return -%EEXIST.
//
// This implies that filters for multiple multicast recipients must
// all be inserted with the same priority and @replace_equal = %false.
//
// efx_filter_remove_id_safe - remove a filter by ID, carefully
// @efx: NIC from which to remove the filter
// @priority: Priority of filter, as passed to @efx_filter_insert_filter
// @filter_id: ID of filter, as returned by @efx_filter_insert_filter
//
// This function will range-check @filter_id, so it is safe to call
// with a value passed from userland.
//
// efx_filter_get_filter_safe - retrieve a filter by ID, carefully
// @efx: NIC from which to remove the filter
// @priority: Priority of filter, as passed to @efx_filter_insert_filter
// @filter_id: ID of filter, as returned by @efx_filter_insert_filter
// @spec: Buffer in which to store filter specification
//
// This function will range-check @filter_id, so it is safe to call
// with a value passed from userland.
//
// RSS contexts
// Ethtool support
// Global
extern "C" {
    pub fn efx_siena_usecs_to_ticks(efx: *mut efx_nic, usecs: c_uint) -> c_uint;
}
// Update the generic software stats in the passed stats array
extern "C" {
    pub fn efx_siena_update_sw_stats(efx: *mut efx_nic, stats: *mut u64);
}
// MTD

extern "C" {
    pub fn efx_siena_mtd_rename(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_mtd_remove(efx: *mut efx_nic);
}

// Lock/freeze all TX queues so that we can be sure the
// TX scheduler is stopped when we're done and before
// netif_device_present() becomes false.
//
