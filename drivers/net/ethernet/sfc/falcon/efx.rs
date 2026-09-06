//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/falcon/efx.h
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

// All controllers use BAR 0 for I/O space and BAR 2(&3) for memory
// All VFs use BAR 0/1 for memory
pub const EF4_MEM_BAR: c_int = 2;
pub const EF4_MEM_VF_BAR: c_int = 0;
extern "C" {
    pub fn ef4_net_open(net_dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ef4_net_stop(net_dev: *mut net_device) -> c_int;
}
// TX
extern "C" {
    pub fn ef4_probe_tx_queue(tx_queue: *mut ef4_tx_queue) -> c_int;
}
extern "C" {
    pub fn ef4_remove_tx_queue(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_init_tx_queue(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_init_tx_queue_core_txq(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_fini_tx_queue(tx_queue: *mut ef4_tx_queue);
}
extern "C" {
    pub fn ef4_enqueue_skb(tx_queue: *mut ef4_tx_queue, skb: *mut sk_buff) -> netdev_tx_t;
}
extern "C" {
    pub fn ef4_xmit_done(tx_queue: *mut ef4_tx_queue, index: c_uint);
}
extern "C" {
    pub fn ef4_tx_max_skb_descs(efx: *mut ef4_nic) -> c_uint;
}
// RX
extern "C" {
    pub fn ef4_set_default_rx_indir_table(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_rx_config_page_split(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_probe_rx_queue(rx_queue: *mut ef4_rx_queue) -> c_int;
}
extern "C" {
    pub fn ef4_remove_rx_queue(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_init_rx_queue(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_fini_rx_queue(rx_queue: *mut ef4_rx_queue);
}
extern "C" {
    pub fn ef4_fast_push_rx_descriptors(rx_queue: *mut ef4_rx_queue, atomic: bool);
}
extern "C" {
    pub fn ef4_rx_slow_fill(t: *mut timer_list);
}
extern "C" {
    pub fn __ef4_rx_packet(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_schedule_slow_fill(rx_queue: *mut ef4_rx_queue);
}

// Maximum number of TCP segments we support for soft-TSO
pub const EF4_TSO_MAX_SEGS: c_int = 100;
// The smallest [rt]xq_entries that the driver supports.  RX minimum
// is a bit arbitrary.  For TX, we must have space for at least 2
// TSO skbs.
//

// Filters
extern "C" {
    pub fn ef4_mac_reconfigure(efx: *mut ef4_nic);
}
//
// ef4_filter_insert_filter - add or replace a filter
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
// 3. If !ef4_filter_is_mc_recipient(@spec), or the NIC does not
// support delivery to multiple recipients, return -%EEXIST.
//
// This implies that filters for multiple multicast recipients must
// all be inserted with the same priority and @replace_equal = %false.
//
// ef4_filter_remove_id_safe - remove a filter by ID, carefully
// @efx: NIC from which to remove the filter
// @priority: Priority of filter, as passed to @ef4_filter_insert_filter
// @filter_id: ID of filter, as returned by @ef4_filter_insert_filter
//
// This function will range-check @filter_id, so it is safe to call
// with a value passed from userland.
//
// ef4_filter_get_filter_safe - retrieve a filter by ID, carefully
// @efx: NIC from which to remove the filter
// @priority: Priority of filter, as passed to @ef4_filter_insert_filter
// @filter_id: ID of filter, as returned by @ef4_filter_insert_filter
// @spec: Buffer in which to store filter specification
//
// This function will range-check @filter_id, so it is safe to call
// with a value passed from userland.
//

extern "C" {
    pub fn __ef4_filter_rfs_expire(efx: *mut ef4_nic, quota: unsigned) -> bool;
}
pub const ef4_filter_rfs_enabled(): c_int = 1;

pub const ef4_filter_rfs_enabled(): c_int = 0;

extern "C" {
    pub fn ef4_filter_is_mc_recipient(spec: *const ef4_filter_spec) -> bool;
}
// Channels
extern "C" {
    pub fn ef4_channel_dummy_op_int(channel: *mut ef4_channel) -> c_int;
}
extern "C" {
    pub fn ef4_channel_dummy_op_void(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_realloc_channels(efx: *mut ef4_nic, rxq_entries: u32, txq_entries: u32) -> c_int;
}
// Ports
extern "C" {
    pub fn ef4_reconfigure_port(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn __ef4_reconfigure_port(efx: *mut ef4_nic) -> c_int;
}
// Ethtool support
// Reset handling
extern "C" {
    pub fn ef4_reset(efx: *mut ef4_nic, method: reset_type) -> c_int;
}
extern "C" {
    pub fn ef4_reset_down(efx: *mut ef4_nic, method: reset_type);
}
extern "C" {
    pub fn ef4_reset_up(efx: *mut ef4_nic, method: reset_type, ok: bool) -> c_int;
}
extern "C" {
    pub fn ef4_try_recovery(efx: *mut ef4_nic) -> c_int;
}
// Global
extern "C" {
    pub fn ef4_schedule_reset(efx: *mut ef4_nic, type: reset_type);
}
extern "C" {
    pub fn ef4_usecs_to_ticks(efx: *mut ef4_nic, usecs: c_uint) -> c_uint;
}
extern "C" {
    pub fn ef4_stop_eventq(channel: *mut ef4_channel);
}
extern "C" {
    pub fn ef4_start_eventq(channel: *mut ef4_channel);
}
// Dummy PHY ops for PHY drivers
extern "C" {
    pub fn ef4_port_dummy_op_int(efx: *mut ef4_nic) -> c_int;
}
extern "C" {
    pub fn ef4_port_dummy_op_void(efx: *mut ef4_nic);
}
// Update the generic software stats in the passed stats array
extern "C" {
    pub fn ef4_update_sw_stats(efx: *mut ef4_nic, stats: *mut u64);
}
// MTD

extern "C" {
    pub fn ef4_mtd_rename(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_mtd_remove(efx: *mut ef4_nic);
}

extern "C" {
    pub fn ef4_link_status_changed(efx: *mut ef4_nic);
}
extern "C" {
    pub fn ef4_link_set_advertising(efx: *mut ef4_nic, _arg: u32);
}
extern "C" {
    pub fn ef4_link_set_wanted_fc(efx: *mut ef4_nic, _arg: u8);
}
// Lock/freeze all TX queues so that we can be sure the
// TX scheduler is stopped when we're done and before
// netif_device_present() becomes false.
//
