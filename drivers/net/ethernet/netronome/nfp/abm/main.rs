//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/abm/main.h
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
// Copyright (C) 2018 Netronome Systems, Inc.
pub const __NFP_ABM_H__: c_int = 1;

// Dump of 64 PRIOs and 256 REDs seems to take 850us on Xeon v4 @ 2.20GHz;
// 2.5ms / 400Hz seems more than sufficient for stats resolution.
//

// The possible actions if thresholds are exceeded
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_abm_q_action {
// mark if ECN capable, otherwise drop
    NFP_ABM_ACT_MARK_DROP		= 0,
// mark if ECN capable, otherwise goto QM
    NFP_ABM_ACT_MARK_QUEUE		= 1,
    NFP_ABM_ACT_DROP		= 2,
    NFP_ABM_ACT_QUEUE		= 3,
    NFP_ABM_ACT_NOQUEUE		= 4,
}

//
// struct nfp_abm - ABM NIC app structure
// @app:	back pointer to nfp_app
// @pf_id:	ID of our PF link
//
// @red_support:	is RED offload supported
// @num_prios:	number of supported DSCP priorities
// @num_bands:	number of supported DSCP priority bands
// @action_mask:	bitmask of supported actions
//
// @thresholds:		current threshold configuration
// @threshold_undef:	bitmap of thresholds which have not been set
// @actions:		current FW action configuration
// @num_thresholds:	number of @thresholds and bits in @threshold_undef
//
// @prio_map_len:	computed length of FW priority map (in bytes)
// @dscp_mask:		mask FW will apply on DSCP field
//
// @eswitch_mode:	devlink eswitch mode, advanced functions only visible
// in switchdev mode
//
// @q_lvls:	queue level control area
// @qm_stats:	queue statistics symbol
// @q_stats:	basic queue statistics (only in per-band case)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_abm {
    pub app: *mut nfp_app,
    pub pf_id: c_uint,
    pub red_support: c_uint,
    pub num_prios: c_uint,
    pub num_bands: c_uint,
    pub action_mask: c_uint,
    pub thresholds: *mut u32,
    pub threshold_undef: *mut c_ulong,
    pub actions: *mut u8,
    pub num_thresholds: usize,
    pub prio_map_len: c_uint,
    pub dscp_mask: u8,
    pub eswitch_mode: devlink_eswitch_mode,
    pub q_lvls: *const nfp_rtsym,
    pub qm_stats: *const nfp_rtsym,
    pub q_stats: *const nfp_rtsym,
}

//
// struct nfp_alink_stats - ABM NIC statistics
// @tx_pkts:		number of TXed packets
// @tx_bytes:		number of TXed bytes
// @backlog_pkts:	momentary backlog length (packets)
// @backlog_bytes:	momentary backlog length (bytes)
// @overlimits:		number of ECN marked TXed packets (accumulative)
// @drops:		number of tail-dropped packets (accumulative)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_alink_stats {
    pub tx_pkts: u64,
    pub tx_bytes: u64,
    pub backlog_pkts: u64,
    pub backlog_bytes: u64,
    pub overlimits: u64,
    pub drops: u64,
}

//
// struct nfp_alink_xstats - extended ABM NIC statistics
// @ecn_marked:		number of ECN marked TXed packets
// @pdrop:		number of hard drops due to queue limit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_alink_xstats {
    pub ecn_marked: u64,
    pub pdrop: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_qdisc_type {
    NFP_QDISC_NONE = 0,
    NFP_QDISC_MQ,
    NFP_QDISC_RED,
    NFP_QDISC_GRED,
}

//
// struct nfp_qdisc - tracked TC Qdisc
// @netdev:		netdev on which Qdisc was created
// @type:		Qdisc type
// @handle:		handle of this Qdisc
// @parent_handle:	handle of the parent (unreliable if Qdisc was grafted)
// @use_cnt:		number of attachment points in the hierarchy
// @num_children:	current size of the @children array
// @children:		pointers to children
//
// @params_ok:		parameters of this Qdisc are OK for offload
// @offload_mark:	offload refresh state - selected for offload
// @offloaded:		Qdisc is currently offloaded to the HW
//
// @mq:			MQ Qdisc specific parameters and state
// @mq.stats:		current stats of the MQ Qdisc
// @mq.prev_stats:	previously reported @mq.stats
//
// @red:		RED Qdisc specific parameters and state
// @red.num_bands:	Number of valid entries in the @red.band table
// @red.band:		Per-band array of RED instances
// @red.band.ecn:		ECN marking is enabled (rather than drop)
// @red.band.threshold:		ECN marking threshold
// @red.band.stats:		current stats of the RED Qdisc
// @red.band.prev_stats:	previously reported @red.stats
// @red.band.xstats:		extended stats for RED - current
// @red.band.prev_xstats:	extended stats for RED - previously reported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_qdisc {
    pub netdev: *mut net_device,
    pub type: nfp_qdisc_type,
    pub handle: u32,
    pub parent_handle: u32,
    pub use_cnt: c_uint,
    pub num_children: c_uint,
    pub children: *mut nfp_qdisc,
    pub params_ok: bool,
    pub offload_mark: bool,
    pub offloaded: bool,
// NFP_QDISC_MQ
    pub stats: nfp_alink_stats,
    pub prev_stats: nfp_alink_stats,
    pub mq: },
// TC_SETUP_QDISC_RED, TC_SETUP_QDISC_GRED
    pub num_bands: c_uint,
    pub ecn: bool,
    pub threshold: u32,
    pub stats: nfp_alink_stats,
    pub prev_stats: nfp_alink_stats,
    pub xstats: nfp_alink_xstats,
    pub prev_xstats: nfp_alink_xstats,
    pub band: [}; MAX_DPs],
    pub red: },
}

//
// struct nfp_abm_link - port tuple of a ABM NIC
// @abm:	back pointer to nfp_abm
// @vnic:	data vNIC
// @id:		id of the data vNIC
// @queue_base:	id of base to host queue within PCIe (not QC idx)
// @total_queues:	number of PF queues
//
// @last_stats_update:	ktime of last stats update
//
// @prio_map:		current map of priorities
// @has_prio:		@prio_map is valid
//
// @def_band:		default band to use
// @dscp_map:		list of DSCP to band mappings
//
// @root_qdisc:	pointer to the current root of the Qdisc hierarchy
// @qdiscs:	all qdiscs recorded by major part of the handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_abm_link {
    pub abm: *mut nfp_abm,
    pub vnic: *mut nfp_net,
    pub id: c_uint,
    pub queue_base: c_uint,
    pub total_queues: c_uint,
    pub last_stats_update: u64,
    pub prio_map: *mut u32,
    pub has_prio: bool,
    pub def_band: u8,
    pub dscp_map: list_head,
    pub root_qdisc: *mut nfp_qdisc,
    pub qdiscs: radix_tree_root,
}

extern "C" {
    pub fn nfp_abm_qdisc_offload_update(alink: *mut nfp_abm_link);
}
extern "C" {
    pub fn nfp_abm_ctrl_read_params(alink: *mut nfp_abm_link) -> c_int;
}
extern "C" {
    pub fn nfp_abm_ctrl_find_addrs(abm: *mut nfp_abm) -> c_int;
}
extern "C" {
    pub fn __nfp_abm_ctrl_set_q_lvl(abm: *mut nfp_abm, id: c_uint, val: u32) -> c_int;
}
extern "C" {
    pub fn nfp_abm_ctrl_stat_non_sto(alink: *mut nfp_abm_link, i: c_uint) -> u64;
}
extern "C" {
    pub fn nfp_abm_ctrl_stat_sto(alink: *mut nfp_abm_link, i: c_uint) -> u64;
}
extern "C" {
    pub fn nfp_abm_ctrl_qm_enable(abm: *mut nfp_abm) -> c_int;
}
extern "C" {
    pub fn nfp_abm_ctrl_qm_disable(abm: *mut nfp_abm) -> c_int;
}
extern "C" {
    pub fn nfp_abm_prio_map_update(abm: *mut nfp_abm);
}
extern "C" {
    pub fn nfp_abm_ctrl_prio_map_update(alink: *mut nfp_abm_link, packed: *mut u32) -> c_int;
}
