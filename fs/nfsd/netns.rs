//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/netns.h
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
// per net namespace data structures for nfsd
//
// Copyright (C) 2012, Jeff Layton <jlayton@redhat.com>
//

// Hash tables for nfs4_clientid state
pub const CLIENT_HASH_BITS: c_int = 4;

pub const SESSION_HASH_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd_net_flag {
    NFSD_NET_GRACE_ENDED,
    NFSD_NET_GRACE_END_FORCED,
    NFSD_NET_IN_GRACE,
    NFSD_NET_SOMEBODY_RECLAIMED,
    NFSD_NET_TRACK_RECLAIM_COMPLETES,
    NFSD_NET_UP,
    NFSD_NET_LOCKD_UP,
}

// cache misses due only to checksum comparison failures
// amount of memory (in bytes) currently consumed by the DRC

//
// Per-netns NFSv4 callback (backchannel) per-operation counters, indexed
// directly by RFC 8881 callback opcode (OP_CB_GETATTR..OP_CB_OFFLOAD).
//

//
// Represents a nfsd "container". With respect to nfsv4 state tracking, the
// fields of interest are the *_id_hashtbls and the *_name_tree. These track
// the nfs4_client objects by either short or long form clientid.
//
// Each nfsd_net runs a nfs4_laundromat workqueue job when necessary to clean
// up expired clients and delegations within the container.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_net {
    pub cld_net: *mut cld_net,
    pub svc_expkey_cache: *mut cache_detail,
    pub svc_export_cache: *mut cache_detail,
    pub idtoname_cache: *mut cache_detail,
    pub nametoid_cache: *mut cache_detail,
    pub nfsd4_manager: lock_manager,
    pub flags: c_ulong,
    pub boot_time: time64_t,
    pub /: *mut *mut time64_t boot_time_bt; / same instant in CLOCK_BOOTTIME,
    pub nfsd_client_dir: *mut dentry,
//
// reclaim_str_hashtbl[] holds known client info from previous reset/reboot
// used in reboot/reset lease grace period processing
//
// conf_id_hashtbl[], and conf_name_tree hold confirmed
// setclientid_confirmed info.
//
// unconf_str_hastbl[] and unconf_name_tree hold unconfirmed
// setclientid info.
//
    pub reclaim_str_hashtbl: *mut list_head,
    pub reclaim_str_hashtbl_size: c_int,
    pub reclaim_str_hashtbl_lock: rw_semaphore,
    pub conf_id_hashtbl: *mut list_head,
    pub conf_name_tree: rb_root,
    pub unconf_id_hashtbl: *mut list_head,
    pub unconf_name_tree: rb_root,
    pub sessionid_hashtbl: *mut list_head,
//
// client_lru holds client queue ordered by nfs4_client.cl_time
// for lease renewal.
//
// close_lru holds (open) stateowner queue ordered by nfs4_stateowner.so_time
// for last close replay.
//
// reclaim_str_hashtbl[], reclaim_str_hashtbl_size are protected by
// reclaim_str_hashtbl_lock.
//
// All of the remaining fields are protected by the client_lock.
//
    pub client_lru: list_head,
    pub close_lru: list_head,
// protects del_recall_lru and delegation hash/unhash;
// nests outside client_lock
    pub ____cacheline_aligned: spinlock_t deleg_lock,
    pub del_recall_lru: list_head,
// protected by blocked_locks_lock
    pub blocked_locks_lru: list_head,
    pub laundromat_work: delayed_work,
// client_lock protects the client lru list and session hash
// table; nests inside deleg_lock
    pub client_lock: spinlock_t,
// protects blocked_locks_lru
    pub blocked_locks_lock: spinlock_t,
    pub rec_file: *mut file,
    pub client_tracking_ops: *const nfsd4_client_tracking_ops,
    pub nfsd4_lease: time64_t,
    pub nfsd4_grace: time64_t,
    pub nr_reclaim_complete: core::sync::atomic::AtomicI32,
    pub writeverf_lock: seqlock_t,
    pub writeverf: [c_uchar; 8],
//
// Minimum number of threads to run per pool.  If 0 then the
// min == max requested number of threads.
//
    pub min_threads: c_uint,
    pub clientid_base: u32,
    pub clientid_counter: u32,
    pub clverifier_counter: u32,
    pub nfsd_info: svc_info,

    pub nfsd_net_ref: percpu_ref,
    pub nfsd_net_confirm_done: completion,
    pub nfsd_net_free_done: completion,
//
// clientid and stateid data for construction of net unique COPY
// stateids.
//
    pub s2s_cp_cl_id: u32,
    pub s2s_cp_stateids: idr,
    pub s2s_cp_lock: spinlock_t,
    pub pending_async_copies: core::sync::atomic::AtomicI32,
//
// Version information
//
    pub 1]: bool nfsd_versions[NFSD_MAXVERS +,
    pub 1]: bool nfsd4_minorversions[NFSD_SUPPORTED_MINOR_VERSION +,
//
// Duplicate reply cache
//
    pub drc_hashtbl: *mut nfsd_drc_bucket,
// max number of entries allowed in the cache
    pub max_drc_entries: c_uint,
// number of significant bits in the hash value
    pub maskbits: c_uint,
    pub drc_hashsize: c_uint,
//
// Stats and other tracking of on the duplicate reply cache.
// The longest_chain* fields are modified with only the per-bucket
// cache lock, which isn't really safe and should be fixed if we want
// these statistics to be completely accurate.
//
// total number of entries
    pub num_drc_entries: core::sync::atomic::AtomicI32,
// Per-netns stats counters
    pub counter: [percpu_counter; NFSD_STATS_COUNTERS_NUM],
// Per-netns NFSv4 callback (backchannel) per-operation counters
    pub cb_counter: [percpu_counter; NFSD_STATS_CB_OPS_NUM],
// sunrpc svc stats
    pub nfsd_svcstats: svc_stat,
// longest hash chain seen
    pub longest_chain: c_uint,
// size of cache when we saw the longest hash chain
    pub longest_chain_cachesize: c_uint,
    pub nfsd_reply_cache_shrinker: *mut shrinker,
// tracking server-to-server copy mounts
    pub nfsd_ssc_lock: spinlock_t,
    pub nfsd_ssc_mount_list: list_head,
    pub nfsd_ssc_waitq: wait_queue_head_t,
// utsname taken from the process that starts the server
    pub nfsd_name: [c_char; UNX_MAXNODENAME+1],
    pub fcache_dispose_lock: spinlock_t,
    pub fcache_dispose_list: list_head,
    pub siphash_key: siphash_key_t,
    pub nfs4_client_count: core::sync::atomic::AtomicI32,
    pub nfs4_max_clients: c_int,
    pub nfsd_courtesy_clients: core::sync::atomic::AtomicI32,
    pub nfsd_client_shrinker: *mut shrinker,
    pub nfsd_shrinker_work: work_struct,
// last time an admin-revoke happened for NFSv4.0
    pub nfs40_last_revoke: time64_t,

// Local clients to be invalidated when net is shut down
    pub local_clients_lock: spinlock_t,
    pub local_clients: list_head,

    pub fh_key: *mut siphash_key_t,
    pub nfsd_cb: *mut nfsd_net_cb,
}

// Simple check to find out if a given net was properly initialized

extern "C" {
    pub fn nfsd_support_version(vers: c_int) -> bool;
}
extern "C" {
    pub fn nfsd_net_try_get(net: *mut net) -> bool;
}
extern "C" {
    pub fn nfsd_net_put(net: *mut net);
}
extern "C" {
    pub fn nfsd_copy_write_verifier(verf[2]: __be32, nn: *mut nfsd_net);
}
extern "C" {
    pub fn nfsd_reset_write_verifier(nn: *mut nfsd_net);
}
