//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dlm/dlmcommon.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// dlmcommon.h
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//

pub const DLM_LOCKID_NAME_MAX: c_int = 32;

// Intended to make it easier for us to switch out hash functions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_mle_type {
    DLM_MLE_BLOCK = 0,
    DLM_MLE_MASTER = 1,
    DLM_MLE_MIGRATION = 2,
    DLM_MLE_NUM_TYPES = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_master_list_entry {
    pub master_hash_node: hlist_node,
    pub hb_events: list_head,
    pub dlm: *mut dlm_ctxt,
    pub spinlock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub woken: core::sync::atomic::AtomicI32,
    pub mle_refs: kref,
    pub inuse: c_int,
    pub maybe_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    pub vote_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    pub response_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    pub node_map: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
    pub master: u8,
    pub new_master: u8,
    pub type: dlm_mle_type,
    pub mle_hb_up: o2hb_callback_func,
    pub mle_hb_down: o2hb_callback_func,
    pub mleres: *mut dlm_lock_resource,
    pub mname: [c_uchar; DLM_LOCKID_NAME_MAX],
    pub mnamelen: c_uint,
    pub mnamehash: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_ast_type {
    DLM_AST = 0,
    DLM_BAST = 1,
    DLM_ASTUNLOCK = 2,
}

pub const DLM_RECOVERY_LOCK_NAME_LEN: c_int = 9;
pub const DLM_RECO_STATE_ACTIVE: c_uint = 0x0001;
pub const DLM_RECO_STATE_FINALIZE: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_ctxt_state {
    DLM_CTXT_NEW = 0,
    DLM_CTXT_JOINED = 1,
    DLM_CTXT_IN_SHUTDOWN = 2,
    DLM_CTXT_LEAVING = 3,
}

// these give a really vague idea of the system load
// NOTE: Next three are protected by dlm_domain_lock
// The filesystem specifies this at domain registration.  We
// cache it here to know what to tell other nodes.
// This is the inter-dlm communication version
// these keventd work queue items are for less-frequently
// called functions that cannot be directly called from the
// net message handlers for some reason, usually because
// they need to send net messages of their own.
extern "C" {
    pub fn dlm_dispatch_work(work: *mut work_struct);
}
extern "C" {
    pub fn void(: *mut dlm_workfunc_t)(struct dlm_work_item, : *mut c_void) -> typedef;
}
pub const DLM_LOCK_RES_UNINITED: c_uint = 0x00000001;
pub const DLM_LOCK_RES_RECOVERING: c_uint = 0x00000002;
pub const DLM_LOCK_RES_READY: c_uint = 0x00000004;
pub const DLM_LOCK_RES_DIRTY: c_uint = 0x00000008;
pub const DLM_LOCK_RES_IN_PROGRESS: c_uint = 0x00000010;
pub const DLM_LOCK_RES_MIGRATING: c_uint = 0x00000020;
pub const DLM_LOCK_RES_DROPPING_REF: c_uint = 0x00000040;
pub const DLM_LOCK_RES_BLOCK_DIRTY: c_uint = 0x00001000;
pub const DLM_LOCK_RES_SETREF_INPROG: c_uint = 0x00002000;
pub const DLM_LOCK_RES_RECOVERY_WAITING: c_uint = 0x00004000;
// max milliseconds to wait to sync up a network failure with a node death

// WARNING: Please see the comment in dlm_init_lockres before
// adding fields here.
//
// Please keep granted, converting, and blocked in this order,
// as some funcs want to iterate over all lists.
//
// All four lists are protected by the hash's reference.
//
// These two lists require you to hold an additional reference
// while they are on the list.
//
// Added during init and removed during release
// unused lock resources have their last_used stamped and are
// put on a list for the dlm thread to run.
// these 3 are just padding for the in-memory structure, but
// list and flags are actually used when sent over the wire
// ast and bast must be callable while holding a spinlock!
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_lockres_list {
    DLM_GRANTED_LIST = 0,
    DLM_CONVERTING_LIST = 1,
    DLM_BLOCKED_LIST = 2,
}

pub const DLM_ASSERT_RESPONSE_REASSERT: c_uint = 0x00000001;
pub const DLM_ASSERT_RESPONSE_MASTERY_REF: c_uint = 0x00000002;
pub const DLM_ASSERT_MASTER_MLE_CLEANUP: c_uint = 0x00000001;
pub const DLM_ASSERT_MASTER_REQUERY: c_uint = 0x00000002;
pub const DLM_ASSERT_MASTER_FINISH_MIGRATION: c_uint = 0x00000004;
pub const DLM_MIGRATE_RESPONSE_MASTERY_REF: c_uint = 0x00000001;
pub const DLM_MRES_RECOVERY: c_uint = 0x01;
pub const DLM_MRES_MIGRATION: c_uint = 0x02;
pub const DLM_MRES_ALL_DONE: c_uint = 0x04;
//
// We would like to get one whole lockres into a single network
// message whenever possible.  Generally speaking, there will be
// at most one dlm_lock on a lockres for each node in the cluster,
// plus (infrequently) any additional locks coming in from userdlm.
//
// struct _dlm_lockres_page
// {
// dlm_migratable_lockres mres;
// dlm_migratable_lock ml[DLM_MAX_MIGRATABLE_LOCKS];
// u8 pad[DLM_MIG_LOCKRES_RESERVED];
// };
//
// from ../cluster/tcp.h
// O2NET_MAX_PAYLOAD_BYTES  (4096 - sizeof(net_msg))
// (roughly 4080 bytes)
// and sizeof(dlm_migratable_lockres) = 112 bytes
// and sizeof(dlm_migratable_lock) = 16 bytes
//
// Choosing DLM_MAX_MIGRATABLE_LOCKS=240 and
// DLM_MIG_LOCKRES_RESERVED=128 means we have this:
//
// (DLM_MAX_MIGRATABLE_LOCKS * sizeof(dlm_migratable_lock)) +
// sizeof(dlm_migratable_lockres) + DLM_MIG_LOCKRES_RESERVED =
// NET_MAX_PAYLOAD_BYTES
// (240 * 16) + 112 + 128 = 4080
//
// So a lockres would need more than 240 locks before it would
// use more than one network packet to recover.  Not too bad.
//
pub const DLM_MAX_MIGRATABLE_LOCKS: c_int = 240;
// or zero if not needed
// 16 bytes
// 48 bytes
// 112 bytes

// from above, 128 bytes
// for some undetermined future use

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_query_join_response_code {
    JOIN_DISALLOW = 0,
    JOIN_OK = 1,
    JOIN_OK_NO_MAP = 2,
    JOIN_PROTOCOL_MISMATCH = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_query_join_packet {
    pub fs_minor: *mut *mut u8 code; / Response code. dlm_minor and,
    pub the: *mut *mut u8 dlm_minor; / The minor version of the protocol,
    pub the: *mut *mut u8 fs_minor; / The minor version of the protocol,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dlm_query_join_response {
    pub intval: __be32,
    pub packet: dlm_query_join_packet,
}

// unused for now
// eventually we can use this to attempt
// lvb recovery based on each node's info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_query_region {
    pub qr_node: u8,
    pub qr_numregions: u8,
    pub qr_namelen: u8,
    pub pad1: u8,
    pub qr_domain: [u8; O2NM_MAX_NAME_LEN],
    pub O2NM_MAX_REGIONS]: *mut *mut u8 qr_regions[O2HB_MAX_REGION_NAME_LEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_node_info {
    pub ni_nodenum: u8,
    pub pad1: u8,
    pub ni_ipv4_port: __be16,
    pub ni_ipv4_address: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_query_nodeinfo {
    pub qn_nodenum: u8,
    pub qn_numnodes: u8,
    pub qn_namelen: u8,
    pub pad1: u8,
    pub qn_domain: [u8; O2NM_MAX_NAME_LEN],
    pub qn_nodes: [dlm_node_info; O2NM_MAX_NODES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_deref_lockres_done {
    pub pad1: u32,
    pub pad2: u16,
    pub node_idx: u8,
    pub namelen: u8,
    pub name: [u8; O2NM_MAX_NAME_LEN],
}

extern "C" {
    pub fn dlm_lock_get(lock: *mut dlm_lock);
}
extern "C" {
    pub fn dlm_lock_put(lock: *mut dlm_lock);
}
extern "C" {
    pub fn dlm_launch_thread(dlm: *mut dlm_ctxt) -> c_int;
}
extern "C" {
    pub fn dlm_complete_thread(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_launch_recovery_thread(dlm: *mut dlm_ctxt) -> c_int;
}
extern "C" {
    pub fn dlm_complete_recovery_thread(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_wait_for_recovery(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_kick_recovery_thread(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_is_node_dead(dlm: *mut dlm_ctxt, node: u8) -> c_int;
}
extern "C" {
    pub fn dlm_wait_for_node_death(dlm: *mut dlm_ctxt, node: u8, timeout: c_int);
}
extern "C" {
    pub fn dlm_wait_for_node_recovery(dlm: *mut dlm_ctxt, node: u8, timeout: c_int);
}
extern "C" {
    pub fn dlm_put(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_domain_fully_joined(dlm: *mut dlm_ctxt) -> c_int;
}
// This is called on every lookup, so it might be worth
// inlining.
extern "C" {
    pub fn dlm_lockres_put(res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn __dlm_unhash_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn __dlm_insert_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn dlm_is_host_down(errno: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_queue_ast(dlm: *mut dlm_ctxt, lock: *mut dlm_lock);
}
extern "C" {
    pub fn __dlm_queue_ast(dlm: *mut dlm_ctxt, lock: *mut dlm_lock);
}
extern "C" {
    pub fn __dlm_queue_bast(dlm: *mut dlm_ctxt, lock: *mut dlm_lock);
}
extern "C" {
    pub fn dlm_print_one_lock_resource(res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn __dlm_print_one_lock_resource(res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn dlm_kick_thread(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn __dlm_dirty_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn dlm_hb_node_down_cb(node: *mut o2nm_node, idx: c_int, data: *mut c_void);
}
extern "C" {
    pub fn dlm_hb_node_up_cb(node: *mut o2nm_node, idx: c_int, data: *mut c_void);
}
extern "C" {
    pub fn dlm_empty_lockres(dlm: *mut dlm_ctxt, res: *mut dlm_lock_resource) -> c_int;
}
extern "C" {
    pub fn __dlm_lockres_reserve_ast(res: *mut dlm_lock_resource);
}
extern "C" {
    pub fn dlm_assert_master_post_handler(status: c_int, data: *mut c_void, ret_data: *mut c_void);
}
// will exit holding res->spinlock, but may drop in function
extern "C" {
    pub fn __dlm_wait_on_lockres_flags(res: *mut dlm_lock_resource, flags: c_int);
}
// will exit holding res->spinlock, but may drop in function
extern "C" {
    pub fn __dlm_unlink_mle(dlm: *mut dlm_ctxt, mle: *mut dlm_master_list_entry);
}
extern "C" {
    pub fn __dlm_insert_mle(dlm: *mut dlm_ctxt, mle: *mut dlm_master_list_entry);
}
// create/destroy slab caches
extern "C" {
    pub fn dlm_init_master_caches() -> c_int;
}
extern "C" {
    pub fn dlm_destroy_master_caches();
}
extern "C" {
    pub fn dlm_init_lock_cache() -> c_int;
}
extern "C" {
    pub fn dlm_destroy_lock_cache();
}
extern "C" {
    pub fn dlm_init_mle_cache() -> c_int;
}
extern "C" {
    pub fn dlm_destroy_mle_cache();
}
extern "C" {
    pub fn dlm_hb_event_notify_attached(dlm: *mut dlm_ctxt, idx: c_int, node_up: c_int);
}
extern "C" {
    pub fn dlm_force_free_mles(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_lock_basts_flushed(dlm: *mut dlm_ctxt, lock: *mut dlm_lock) -> c_int;
}
extern "C" {
    pub fn __dlm_lockres_has_locks(res: *mut dlm_lock_resource) -> c_int;
}
extern "C" {
    pub fn __dlm_lockres_unused(res: *mut dlm_lock_resource) -> c_int;
}
// NO_LOCK compatible with all
// EX incompatible with all non-NO_LOCK
// request must be PR, which is compatible with PR
