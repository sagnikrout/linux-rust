//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_netdev.h
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

pub const FBNIC_MIN_RXD_PER_FRAME: c_int = 2;
// Natively supported tunnel GSO features (not thru GSO_PARTIAL)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_net {
    pub xdp_prog: *mut bpf_prog,
    pub FBNIC_MAX_XDPQS]: *mut *mut fbnic_ring tx[FBNIC_MAX_TXQS +,
    pub rx: [*mut fbnic_ring; FBNIC_MAX_RXQS],
    pub napi: [*mut fbnic_napi_vector; FBNIC_MAX_NAPI_VECTORS],
    pub netdev: *mut net_device,
    pub fbd: *mut fbnic_dev,
    pub txq_size: u32,
    pub hpq_size: u32,
    pub ppq_size: u32,
    pub rcq_size: u32,
    pub hds_thresh: u32,
    pub rx_usecs: u16,
    pub tx_usecs: u16,
    pub rx_max_frames: u32,
    pub num_napi: u16,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub pcs: *mut phylink_pcs,
    pub aui: u8,
    pub fec: u8,
// Cached top bits of the HW time counter for 40b -> 64b conversion
    pub time_high: u32,
// Protect readers of @time_offset, writers take @time_lock.
    pub time_seq: u64_stats_sync,
// Offset in ns between free running NIC PHC and time set via PTP
// clock callbacks
//
    pub time_offset: i64,
    pub num_tx_queues: u16,
    pub num_rx_queues: u16,
    pub indir_tbl: [u8; FBNIC_RPC_RSS_TBL_COUNT][FBNIC_RPC_RSS_TBL_SIZE],
    pub rss_key: [u32; FBNIC_RPC_RSS_KEY_DWORD_LEN],
    pub rss_flow_hash: [u32; FBNIC_NUM_HASH_OPT],
// Storage for stats after ring destruction
    pub tx_stats: fbnic_queue_stats,
    pub rx_stats: fbnic_queue_stats,
    pub bdq_stats: fbnic_queue_stats,
    pub link_down_events: u64,
// Time stamping filter config
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub tx_pause: bool,
}

extern "C" {
    pub fn __fbnic_open(fbn: *mut fbnic_net) -> c_int;
}
extern "C" {
    pub fn fbnic_up(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_down(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_down_noidle(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_netdev_free(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_netdev_register(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fbnic_netdev_unregister(netdev: *mut net_device);
}
extern "C" {
    pub fn fbnic_set_ethtool_ops(dev: *mut net_device);
}
extern "C" {
    pub fn fbnic_ptp_setup(fbd: *mut fbnic_dev) -> c_int;
}
extern "C" {
    pub fn fbnic_ptp_destroy(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_time_init(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_time_start(fbn: *mut fbnic_net) -> c_int;
}
extern "C" {
    pub fn fbnic_time_stop(fbn: *mut fbnic_net);
}
extern "C" {
    pub fn fbnic_clear_rx_mode(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_phylink_create(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fbnic_phylink_destroy(netdev: *mut net_device);
}
extern "C" {
    pub fn fbnic_phylink_init(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fbnic_phylink_pmd_training_complete_notify(netdev: *mut net_device);
}
