//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/rmnet/rmnet_config.h
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
// Copyright (c) 2013-2014, 2016-2018, 2021 The Linux Foundation.
// All rights reserved.
//
// RMNET Data configuration engine
//

pub const RMNET_MAX_LOGICAL_EP: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_endpoint {
    pub mux_id: u8,
    pub egress_dev: *mut net_device,
    pub hlnode: hlist_node,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_egress_agg_params {
    pub bytes: u32,
    pub count: u32,
    pub time_nsec: u64,
}

// One instance of this structure is instantiated for each real_dev associated
// with rmnet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_port {
    pub dev: *mut net_device,
    pub data_format: u32,
    pub nr_rmnet_devs: u8,
    pub rmnet_mode: u8,
    pub muxed_ep: [hlist_head; RMNET_MAX_LOGICAL_EP],
    pub bridge_ep: *mut net_device,
    pub rmnet_dev: *mut net_device,
// Egress aggregation information
    pub egress_agg_params: rmnet_egress_agg_params,
// Protect aggregation related elements
    pub agg_lock: spinlock_t,
    pub skbagg_head: *mut sk_buff,
    pub skbagg_tail: *mut sk_buff,
    pub agg_state: c_int,
    pub agg_count: u8,
    pub agg_time: timespec64,
    pub agg_last: timespec64,
    pub hrtimer: hrtimer,
    pub agg_wq: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_vnd_stats {
    pub rx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
    pub tx_drops: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_pcpu_stats {
    pub stats: rmnet_vnd_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_priv_stats {
    pub csum_ok: u64,
    pub csum_ip4_header_bad: u64,
    pub csum_valid_unset: u64,
    pub csum_validation_failed: u64,
    pub csum_err_bad_buffer: u64,
    pub csum_err_invalid_ip_version: u64,
    pub csum_err_invalid_transport: u64,
    pub csum_fragmented_pkt: u64,
    pub csum_skipped: u64,
    pub csum_sw: u64,
    pub csum_hw: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmnet_priv {
    pub mux_id: u8,
    pub real_dev: *mut net_device,
    pub pcpu_stats: *mut rmnet_pcpu_stats __percpu,
    pub gro_cells: gro_cells,
    pub stats: rmnet_priv_stats,
}
