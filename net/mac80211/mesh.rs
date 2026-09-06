//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/mesh.h
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
// Copyright (c) 2008, 2009 open80211s Ltd.
// Copyright (C) 2023-2024 Intel Corporation
// Authors:    Luis Carlos Cobo <luisca@cozybit.com>
// Javier Cardona <javier@cozybit.com>
//

// Data structures
//
// enum mesh_path_flags - mac80211 mesh path flags
//
// @MESH_PATH_ACTIVE: the mesh path can be used for forwarding
// @MESH_PATH_RESOLVING: the discovery process is running for this mesh path
// @MESH_PATH_SN_VALID: the mesh path contains a valid destination sequence
// number
// @MESH_PATH_FIXED: the mesh path has been manually set and should not be
// modified
// @MESH_PATH_RESOLVED: the mesh path can has been resolved
// @MESH_PATH_REQ_QUEUED: there is an unsent path request for this destination
// already queued up, waiting for the discovery process to start.
// @MESH_PATH_DELETED: the mesh path has been deleted and should no longer
// be used
//
// MESH_PATH_RESOLVED is used by the mesh path timer to
// decide when to stop or cancel the mesh path discovery.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mesh_path_flags {
    MESH_PATH_ACTIVE =	BIT(0),
    MESH_PATH_RESOLVING =	BIT(1),
    MESH_PATH_SN_VALID =	BIT(2),
    MESH_PATH_FIXED	=	BIT(3),
    MESH_PATH_RESOLVED =	BIT(4),
    MESH_PATH_REQ_QUEUED =	BIT(5),
    MESH_PATH_DELETED =	BIT(6),
}

//
// enum mesh_deferred_task_flags - mac80211 mesh deferred tasks
//
// @MESH_WORK_HOUSEKEEPING: run the periodic mesh housekeeping tasks
// @MESH_WORK_ROOT: the mesh root station needs to send a frame
// @MESH_WORK_DRIFT_ADJUST: time to compensate for clock drift relative to other
// mesh nodes
// @MESH_WORK_MBSS_CHANGED: rebuild beacon and notify driver of BSS changes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mesh_deferred_task_flags {
    MESH_WORK_HOUSEKEEPING,
    MESH_WORK_ROOT,
    MESH_WORK_DRIFT_ADJUST,
    MESH_WORK_MBSS_CHANGED,
}

//
// struct mesh_path - mac80211 mesh path structure
//
// @dst: mesh path destination mac address
// @mpp: mesh proxy mac address
// @rhash: rhashtable list pointer
// @walk_list: linked list containing all mesh_path objects.
// @gate_list: list pointer for known gates list
// @sdata: mesh subif
// @next_hop: mesh neighbor to which frames for this destination will be
// forwarded
// @timer: mesh path discovery timer
// @frame_queue: pending queue for frames sent to this destination while the
// path is unresolved
// @rcu: rcu head for freeing mesh path
// @sn: target sequence number
// @metric: current metric to this destination
// @hop_count: hops to destination
// @exp_time: in jiffies, when the path will expire or when it expired
// @discovery_timeout: timeout (lapse in jiffies) used for the last discovery
// retry
// @discovery_retries: number of discovery retries
// @flags: mesh path flags, as specified on &enum mesh_path_flags
// @state_lock: mesh path state lock used to protect changes to the
// mpath itself.  No need to take this lock when adding or removing
// an mpath to a hash bucket on a path table.
// @rann_snd_addr: the RANN sender address
// @rann_metric: the aggregated path metric towards the root node
// @last_preq_to_root: Timestamp of last PREQ sent to root
// @is_root: the destination station of this path is a root node
// @is_gate: the destination station of this path is a mesh gate
// @path_change_count: the number of path changes to destination
// @fast_tx_check: timestamp of last fast-xmit enable attempt
//
// The dst address is unique in the mesh path table. Since the mesh_path is
// protected by RCU, deleting the next_hop STA must remove / substitute the
// mesh_path structure and wait until that is no longer reachable before
// destroying the STA completely.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_path {
    pub dst: [u8; ETH_ALEN],
    pub /: *mut *mut u8 mpp[ETH_ALEN]; / used for MPP or MAP,
    pub rhash: rhash_head,
    pub walk_list: hlist_node,
    pub gate_list: hlist_node,
    pub sdata: *mut ieee80211_sub_if_data,
    pub next_hop: *mut sta_info __rcu,
    pub timer: timer_list,
    pub frame_queue: sk_buff_head,
    pub rcu: rcu_head,
    pub sn: u32,
    pub metric: u32,
    pub hop_count: u8,
    pub exp_time: c_ulong,
    pub discovery_timeout: u32,
    pub discovery_retries: u8,
    pub flags: mesh_path_flags,
    pub state_lock: spinlock_t,
    pub rann_snd_addr: [u8; ETH_ALEN],
    pub rann_metric: u32,
    pub last_preq_to_root: c_ulong,
    pub fast_tx_check: c_ulong,
    pub is_root: bool,
    pub is_gate: bool,
    pub path_change_count: u32,
}

pub const MESH_FAST_TX_CACHE_MAX_SIZE: c_int = 512;
pub const MESH_FAST_TX_CACHE_THRESHOLD_SIZE: c_int = 384;

//
// enum ieee80211_mesh_fast_tx_type - cached mesh fast tx entry type
//
// @MESH_FAST_TX_TYPE_LOCAL: tx from the local vif address as SA
// @MESH_FAST_TX_TYPE_PROXIED: local tx with a different SA (e.g. bridged)
// @MESH_FAST_TX_TYPE_FORWARDED: forwarded from a different mesh point
// @NUM_MESH_FAST_TX_TYPE: number of entry types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_mesh_fast_tx_type {
    MESH_FAST_TX_TYPE_LOCAL,
    MESH_FAST_TX_TYPE_PROXIED,
    MESH_FAST_TX_TYPE_FORWARDED,

// must be last
    NUM_MESH_FAST_TX_TYPE
}

//
// struct ieee80211_mesh_fast_tx_key - cached mesh fast tx entry key
//
// @addr: The Ethernet DA for this entry
// @type: cache entry type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_fast_tx_key {
    pub __aligned(2): u8 addr[ETH_ALEN],
    pub type: u16,
}

//
// struct ieee80211_mesh_fast_tx - cached mesh fast tx entry
// @rhash: rhashtable pointer
// @key: the lookup key for this cache entry
// @fast_tx: base fast_tx data
// @hdr: cached mesh and rfc1042 headers
// @hdrlen: length of mesh + rfc1042
// @walk_list: list containing all the fast tx entries
// @mpath: mesh path corresponding to the Mesh DA
// @mppath: MPP entry corresponding to this DA
// @timestamp: Last used time of this entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_fast_tx {
    pub rhash: rhash_head,
    pub key: ieee80211_mesh_fast_tx_key,
    pub fast_tx: ieee80211_fast_tx,
    pub sizeof(rfc1042_header)]: u8 hdr[sizeof(struct ieee80211s_hdr) +,
    pub hdrlen: u16,
    pub mppath: *mut *mut mesh_path mpath,,
    pub walk_list: hlist_node,
    pub timestamp: c_ulong,
}

// Recent multicast cache
// RMC_BUCKETS must be a power of 2, maximum 256
pub const RMC_BUCKETS: c_int = 256;
pub const RMC_QUEUE_MAX_LEN: c_int = 4;

//
// struct rmc_entry - entry in the Recent Multicast Cache
//
// @seqnum: mesh sequence number of the frame
// @exp_time: expiration time of the entry, in jiffies
// @sa: source address of the frame
// @list: hashtable list pointer
//
// The Recent Multicast Cache keeps track of the latest multicast frames that
// have been received by a mesh interface and discards received multicast frames
// that are found in the cache.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmc_entry {
    pub list: hlist_node,
    pub exp_time: c_ulong,
    pub seqnum: u32,
    pub sa: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_rmc {
    pub bucket: [hlist_head; RMC_BUCKETS],
    pub idx_mask: u32,
}

// Default maximum number of plinks per interface
pub const MESH_MAX_PLINKS: c_int = 256;
// Maximum number of paths per interface
pub const MESH_MAX_MPATHS: c_int = 1024;
// Number of frames buffered per destination for unresolved destinations
pub const MESH_FRAME_QUEUE_LEN: c_int = 10;
// Public interfaces
// Various
extern "C" {
    pub fn mesh_rmc_free(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_rmc_init(sdata: *mut ieee80211_sub_if_data) -> c_int;
}
extern "C" {
    pub fn ieee80211s_init();
}
extern "C" {
    pub fn ieee80211_mesh_init_sdata(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_mesh_teardown_sdata(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_start_mesh(sdata: *mut ieee80211_sub_if_data) -> c_int;
}
extern "C" {
    pub fn ieee80211_stop_mesh(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_mesh_root_setup(ifmsh: *mut ieee80211_if_mesh);
}
// wrapper for ieee80211_bss_info_change_notify()
// mesh power save
extern "C" {
    pub fn ieee80211_mps_local_status_update(sdata: *mut ieee80211_sub_if_data) -> u64;
}
extern "C" {
    pub fn ieee80211_mps_sta_status_update(sta: *mut sta_info);
}
// Mesh paths
extern "C" {
    pub fn mesh_path_start_discovery(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_path_fix_nexthop(mpath: *mut mesh_path, next_hop: *mut sta_info);
}
extern "C" {
    pub fn mesh_path_expire(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_path_add_gate(mpath: *mut mesh_path) -> c_int;
}
extern "C" {
    pub fn mesh_path_send_to_gates(mpath: *mut mesh_path) -> c_int;
}
extern "C" {
    pub fn mesh_gate_num(sdata: *mut ieee80211_sub_if_data) -> c_int;
}
// Mesh plinks
extern "C" {
    pub fn mesh_peer_accepts_plinks(ie: *mut ieee802_11_elems) -> bool;
}
extern "C" {
    pub fn mesh_accept_plinks_update(sdata: *mut ieee80211_sub_if_data) -> u64;
}
extern "C" {
    pub fn mesh_plink_timer(t: *mut timer_list);
}
extern "C" {
    pub fn mesh_plink_broken(sta: *mut sta_info);
}
extern "C" {
    pub fn mesh_plink_deactivate(sta: *mut sta_info) -> u64;
}
extern "C" {
    pub fn mesh_plink_open(sta: *mut sta_info) -> u64;
}
extern "C" {
    pub fn mesh_plink_block(sta: *mut sta_info) -> u64;
}
extern "C" {
    pub fn mesh_sta_cleanup(sta: *mut sta_info);
}
// Private interfaces
// Mesh paths
extern "C" {
    pub fn mesh_path_assign_nexthop(mpath: *mut mesh_path, sta: *mut sta_info);
}
extern "C" {
    pub fn mesh_path_flush_pending(mpath: *mut mesh_path);
}
extern "C" {
    pub fn mesh_path_tx_pending(mpath: *mut mesh_path);
}
extern "C" {
    pub fn mesh_pathtbl_init(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_pathtbl_unregister(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_path_del(sdata: *mut ieee80211_sub_if_data, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn mesh_path_timer(t: *mut timer_list);
}
extern "C" {
    pub fn mesh_path_flush_by_nexthop(sta: *mut sta_info);
}
extern "C" {
    pub fn mesh_path_tx_root_frame(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_action_is_path_sel(mgmt: *mut ieee80211_mgmt) -> bool;
}
extern "C" {
    pub fn mesh_fast_tx_gc(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_fast_tx_flush_mpath(mpath: *mut mesh_path);
}

extern "C" {
    pub fn mesh_path_flush_by_iface(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn mesh_sync_adjust_tsf(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211s_stop();
}

