//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/internal.h
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
// internal AFS stuff
//
// Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

pub const AFS_CELL_MAX_ADDRS: c_int = 15;
//
// Partial file-locking emulation mode.  (The problem being that AFS3 only
// allows whole-file locks and no upgrading/downgrading).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_flock_mode {
    afs_flock_mode_unset,
    afs_flock_mode_local,	/* Local locking only */
    afs_flock_mode_openafs,	/* Don't get server lock for a partial lock */
    afs_flock_mode_strict,	/* Always get a server lock for a partial lock */
    afs_flock_mode_write,	/* Get an exclusive server lock for a partial lock */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_fs_context {
    pub /: *mut *mut bool force; / T to force cell type,
    pub /: *mut *mut bool autocell; / T if set auto mount operation,
    pub /: *mut *mut bool dyn_root; / T if dynamic root,
    pub /: *mut *mut bool no_cell; / T if the source is "none" (for dynroot),
    pub /: *mut *mut afs_flock_mode flock_mode; / Partial file-locking emulation mode,
    pub /: *mut *mut afs_voltype_t type; / type of volume requested,
    pub /: *mut *mut unsigned int volnamesz; / size of volume name,
    pub /: *const *const *const char volname; / name of volume to mount,
    pub /: *mut *mut *mut afs_net net; / the AFS net namespace stuff,
    pub /: *mut *mut *mut afs_cell cell; / cell in which to find volume,
    pub /: *mut *mut *mut afs_volume volume; / volume record,
    pub /: *mut *mut *mut key key; / key to use for secure mounting,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_call_state {
    AFS_CALL_CL_REQUESTING,		/* Client: Request is being sent */
    AFS_CALL_CL_AWAIT_REPLY,	/* Client: Awaiting reply */
    AFS_CALL_CL_PROC_REPLY,		/* Client: rxrpc call complete; processing reply */
    AFS_CALL_SV_AWAIT_OP_ID,	/* Server: Awaiting op ID */
    AFS_CALL_SV_AWAIT_REQUEST,	/* Server: Awaiting request data */
    AFS_CALL_SV_REPLYING,		/* Server: Replying */
    AFS_CALL_SV_AWAIT_ACK,		/* Server: Awaiting final ACK */
    AFS_CALL_COMPLETE,		/* Completed or failed */
}

//
// Address preferences.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_addr_preference {
    pub /: *mut *mut in_addr ipv4_addr; / AF_INET address to compare against,
    pub /: *mut *mut in6_addr ipv6_addr; / AF_INET6 address to compare against,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_addr_preference_list {
    pub rcu: rcu_head,
    pub /: *mut *mut u16 version; / Incremented when prefs list changes,
    pub /: *mut *mut u8 ipv6_off; / Offset of IPv6 addresses,
    pub /: *mut *mut u8 nr; / Number of addresses in total,
    pub /: *mut *mut u8 max_prefs; / Number of prefs allocated,
    pub __counted_by(max_prefs): afs_addr_preference prefs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_address {
    pub peer: *mut rxrpc_peer,
    pub /: *mut *mut short last_error; / Last error from this address,
    pub /: *mut *mut u16 prio; / Address priority,
}

//
// List of server addresses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_addr_list {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub /: *mut *mut u32 version; / Version,
    pub debug_id: c_uint,
    pub /: *mut *mut unsigned int addr_pref_version; / Version of address preference list,
    pub max_addrs: c_uchar,
    pub nr_addrs: c_uchar,
    pub /: *mut *mut unsigned char preferred; / Preferred address,
    pub /: *mut *mut unsigned char nr_ipv4; / Number of IPv4 addresses,
    pub source:8: dns_record_source,
    pub status:8: dns_lookup_status,
    pub /: *mut *mut unsigned long probe_failed; / Mask of addrs that failed locally/ICMP,
    pub /: *mut *mut unsigned long responded; / Mask of addrs that responded,
    pub __counted_by(max_addrs): afs_address addrs[],

}

//
// a record of an in-progress RxRPC call
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_call {
    pub /: *const *const *const afs_call_type type; / type of call,
    pub /: *mut *mut wait_queue_head_t waitq; / processes awaiting completion,
    pub /: *mut *mut work_async_work; / async I/O processor,
    pub /: *mut *mut work_work; / actual work processor,
    pub /: *mut *mut work_free_work; / Deferred free processor,
    pub /: *mut *mut *mut rxrpc_call rxcall; / RxRPC call handle,
    pub /: *mut *mut *mut rxrpc_peer peer; / Remote endpoint,
    pub /: *mut *mut *mut key key; / security for this call,
    pub /: *mut *mut *mut afs_net net; / The network namespace,
    pub /: *mut *mut *mut afs_server server; / The fileserver record if fs op (pins ref),
    pub /: *mut *mut *mut afs_vlserver vlserver; / The vlserver record if vl op,
    pub /: *mut *mut *mut void request; / request data (first part),
    pub /: *mut *mut *mut size_t iov_len; / Size of iter to be used,
    pub /: *mut *mut iov_iter def_iter; / Default buffer/data iterator,
    pub /: *mut *mut *mut iov_iter write_iter; / Iterator defining write to be made,
    pub /: *mut *mut *mut iov_iter iter; / Iterator currently in use,
    pub kvec: [kvec; 1],
    pub bvec: [bio_vec; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_call_type {
    pub name: *const c_char,
    pub /: *mut *mut unsigned int op; / Really enum afs_fs_operation,
// deliver request or reply data to an call
// - returning an error will cause the call to be aborted
//
    pub call): *mut *mut int (deliver)(struct afs_call,
// clean up a call
    pub call): *mut *mut void (destructor)(struct afs_call,
// Async receive processing function
    pub work): *mut *mut void (async_rx)(struct work_struct,
// Work function
    pub work): *mut *mut void (work)(struct work_struct,
// Call done function (gets called immediately on success or failure)
    pub call): *mut *mut void (done)(struct afs_call,
// Handle a call being immediately cancelled.
    pub call): *mut *mut void (immediate_cancel)(struct afs_call,
}

//
// Key available for writeback on a file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_wb_key {
    pub usage: refcount_t,
    pub key: *mut key,
    pub /: *mut *mut list_head vnode_link; / Link in vnode->wb_keys,
}

//
// AFS open file information record.  Pointed to by file->private_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_file {
    pub /: *mut *mut *mut key key; / The key this file was opened with,
    pub /: *mut *mut *mut afs_wb_key wb; / Writeback key record for this file,
}

//
// AFS superblock private data
// - there's one superblock per volume
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_super_info {
    pub /: *mut *mut *mut net net_ns; / Network namespace,
    pub /: *mut *mut *mut afs_cell cell; / The cell in which the volume resides,
    pub /: *mut *mut *mut afs_volume volume; / volume record,
    pub /: *mut *mut afs_flock_mode flock_mode:8; / File locking emulation mode,
    pub /: *mut *mut bool dyn_root; / True if dynamic root,
}

//
// Set of substitutes for @sys.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_sysnames {
pub const AFS_NR_SYSNAME: c_int = 16;
    pub subs: [*mut c_char; AFS_NR_SYSNAME],
    pub usage: refcount_t,
    pub nr: c_ushort,
    pub blank: [c_char; 1],
}

//
// AFS network namespace record.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_net {
    pub /: *mut *mut *mut net net; / Backpointer to the owning net namespace,
    pub uuid: afs_uuid,
    pub /: *mut *mut bool live; / F if this namespace is being removed,
// AF_RXRPC I/O stuff
    pub socket: *mut socket,
    pub spare_incoming_call: *mut afs_call,
    pub charge_preallocation_work: work_struct,
    pub rx_oob_work: work_struct,
    pub socket_mutex: mutex,
    pub nr_outstanding_calls: core::sync::atomic::AtomicI32,
    pub nr_superblocks: core::sync::atomic::AtomicI32,
// Cell database
    pub cells: rb_root,
    pub /: *mut *mut idr cells_dyn_ino; / cell->dynroot_ino mapping,
    pub ws_cell: *mut afs_cell __rcu,
    pub cells_outstanding: core::sync::atomic::AtomicI32,
    pub cells_lock: rw_semaphore,
    pub cells_alias_lock: mutex,
    pub proc_cells_lock: mutex,
    pub proc_cells: hlist_head,
// Known servers.  Theoretically each fileserver can only be in one
// cell, but in practice, people create aliases and subsets and there's
// no easy way to distinguish them.
//
    pub /: *mut *mut *mut seqlock_t fs_lock; / For fs_probe_, fs_proc,
    pub /: *mut *mut list_head fs_probe_fast; / List of afs_server to probe at 30s intervals,
    pub /: *mut *mut list_head fs_probe_slow; / List of afs_server to probe at 5m intervals,
    pub /: *mut *mut hlist_head fs_proc; / procfs servers list,
    pub /: *mut *mut *mut key fs_cm_token_key; / Key for creating CM tokens,
    pub fs_prober: work_struct,
    pub fs_probe_timer: timer_list,
    pub servers_outstanding: core::sync::atomic::AtomicI32,
// File locking renewal management
    pub lock_manager_mutex: mutex,
// Misc
    pub /: *mut *mut *mut super_block dynroot_sb; / Dynamic root mount superblock,
    pub /: *mut *mut *mut proc_dir_entry proc_afs; / /proc/net/afs directory,
    pub sysnames: *mut afs_sysnames,
    pub sysnames_lock: rwlock_t,
    pub address_prefs: *mut afs_addr_preference_list __rcu,
    pub address_pref_version: u16,
// Statistics counters
    pub /: *mut *mut atomic_t n_lookup; / Number of lookups done,
    pub /: *mut *mut atomic_t n_reval; / Number of dentries needing revalidation,
    pub /: *mut *mut atomic_t n_inval; / Number of invalidations by the server,
    pub /: *mut *mut atomic_t n_relpg; / Number of invalidations by release_folio,
    pub /: *mut *mut atomic_t n_read_dir; / Number of directory pages read,
    pub /: *mut *mut atomic_t n_dir_cr; / Number of directory entry creation edits,
    pub /: *mut *mut atomic_t n_dir_rm; / Number of directory entry removal edits,
    pub /: *mut *mut atomic_t n_stores; / Number of store ops,
    pub /: *mut *mut atomic_long_t n_store_bytes; / Number of bytes stored,
    pub /: *mut *mut atomic_long_t n_fetch_bytes; / Number of bytes fetched,
    pub /: *mut *mut atomic_t n_fetches; / Number of data fetch ops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_cell_state {
    AFS_CELL_SETTING_UP,
    AFS_CELL_UNLOOKED,
    AFS_CELL_ACTIVE,
    AFS_CELL_REMOVING,
    AFS_CELL_DEAD,
}

//
// AFS cell record.
//
// This is a tricky concept to get right as it is possible to create aliases
// simply by pointing AFSDB/SRV records for two names at the same set of VL
// servers; it is also possible to do things like setting up two sets of VL
// servers, one of which provides a superset of the volumes provided by the
// other (for internal/external division, for example).
//
// Cells only exist in the sense that (a) a cell's name maps to a set of VL
// servers and (b) a cell's name is used by the client to select the key to use
// for authentication and encryption.  The cell name is not typically used in
// the protocol.
//
// Two cells are determined to be aliases if they have an explicit alias (YFS
// only), share any VL servers in common or have at least one volume in common.
// "In common" means that the address list of the VL servers or the fileservers
// share at least one endpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_cell {
    pub rcu: rcu_head,
    pub /: *mut *mut rb_node net_node; / Node in net->cells,
}

// The volumes belonging to this cell
// Active fileserver interaction state.
// VL server list.
//
// Volume Location server record.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vlserver {
    pub rcu: rcu_head,
    pub /: *mut *mut *mut afs_addr_list __rcu addresses; / List of addresses for this VL server,
    pub flags: c_ulong,

    pub /: *mut *mut rwlock_t lock; / Lock on addresses,
    pub ref: refcount_t,
    pub /: *mut *mut unsigned int rtt; / Server's current RTT in uS,
    pub debug_id: c_uint,
// Probe state
    pub probe_wq: wait_queue_head_t,
    pub probe_outstanding: core::sync::atomic::AtomicI32,
    pub probe_lock: spinlock_t,
    pub /: *mut *mut unsigned int rtt; / Best RTT in uS (or UINT_MAX),
    pub abort_code: u32,
    pub error: c_short,
    pub flags: c_ushort,
pub const AFS_VLSERVER_PROBE_RESPONDED: c_uint = 0x01 /* At least once response (may be abort) */;
pub const AFS_VLSERVER_PROBE_IS_YFS: c_uint = 0x02 /* The peer appears to be YFS */;
pub const AFS_VLSERVER_PROBE_NOT_YFS: c_uint = 0x04 /* The peer appears not to be YFS */;
pub const AFS_VLSERVER_PROBE_LOCAL_FAILURE: c_uint = 0x08 /* A local failure prevented a probe */;
    pub probe: },
    pub /: *mut *mut u16 service_id; / Service ID we're using,
    pub port: u16,
    pub /: *mut *mut u16 name_len; / Length of name,
    pub /: *mut *mut char name[]; / Server name, case-flattened,
}

//
// Weighted list of Volume Location servers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vlserver_entry {
    pub /: *mut *mut u16 priority; / Preference (as SRV),
    pub /: *mut *mut u16 weight; / Weight (as SRV),
    pub source:8: dns_record_source,
    pub status:8: dns_lookup_status,
    pub server: *mut afs_vlserver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vlserver_list {
    pub rcu: rcu_head,
    pub ref: refcount_t,
    pub nr_servers: u8,
    pub /: *mut *mut u8 index; / Server currently in use,
    pub /: *mut *mut u8 preferred; / Preferred server,
    pub source:8: dns_record_source,
    pub status:8: dns_lookup_status,
    pub lock: rwlock_t,
    pub servers: [afs_vlserver_entry; ],
}

//
// Cached VLDB entry.
//
// This is pointed to by cell->vldb_entries, indexed by name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vldb_entry {
    pub /: *mut *mut afs_volid_t vid[3]; / Volume IDs for R/W, R/O and Bak volumes,
    pub flags: c_ulong,
    pub fs_server: [uuid_t; AFS_NMAXNSERVERS],
    pub /: *mut *mut u32 addr_version[AFS_NMAXNSERVERS]; / Registration change counters,
    pub fs_mask: [u8; AFS_NMAXNSERVERS],
pub const AFS_VOL_VTM_RW: c_uint = 0x01 /* R/W version of the volume is available (on this server) */;
pub const AFS_VOL_VTM_RO: c_uint = 0x02 /* R/O version of the volume is available (on this server) */;
pub const AFS_VOL_VTM_BAK: c_uint = 0x04 /* backup version of the volume is available (on this server) */;
    pub vlsf_flags: [u8; AFS_NMAXNSERVERS],
    pub error: c_short,
    pub /: *mut *mut u8 nr_servers; / Number of server records,
    pub name_len: u8,
    pub /: *mut *mut u8 name[AFS_MAXVOLNAME + 1]; / NUL-padded volume name,
}

//
// Fileserver endpoint state.  The records the addresses of a fileserver's
// endpoints and the state and result of a round of probing on them.  This
// allows the rotation algorithm to access those results without them being
// erased by a subsequent round of probing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_endpoint_state {
    pub rcu: rcu_head,
    pub /: *mut *mut *mut afs_addr_list addresses; / The addresses being probed,
    pub /: *mut *mut unsigned long responsive_set; / Bitset of responsive endpoints,
    pub /: *mut *mut unsigned long failed_set; / Bitset of endpoints we failed to probe,
    pub ref: refcount_t,
    pub /: *mut *mut unsigned int server_id; / Debug ID of server,
    pub /: *mut *mut unsigned int probe_seq; / Probe sequence (from server::probe_counter),
    pub /: *mut *mut atomic_t nr_probing; / Number of outstanding probes,
    pub /: *mut *mut unsigned int rtt; / Best RTT in uS (or UINT_MAX),
    pub abort_code: i32,
    pub error: c_short,
    pub flags: c_ulong,

}

//
// Record of fileserver with which we're actively communicating.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_server {
    pub rcu: rcu_head,
    pub /: *mut *mut uuid_t uuid; / Server ID,
    pub _uuid: afs_uuid,
}

pub const AFS_SERVER_FL_UPDATING: c_int = 1;

// file service access
// Probe state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_ro_replicating {
    AFS_RO_NOT_REPLICATING,			/* Not doing replication */
    AFS_RO_REPLICATING_USE_OLD,		/* Replicating; use old version */
    AFS_RO_REPLICATING_USE_NEW,		/* Replicating; switch to new version */
    } __mode(byte);

//
// Replaceable volume server list.
//
    struct afs_server_entry {
    struct afs_server	*server;
    struct afs_volume	*volume;
    struct list_head	slink;		/* Link in server->volumes */
    time64_t		cb_expires_at;	/* Time at which volume-level callback expires */
    unsigned long		flags;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_server_list {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub /: *mut *mut bool attached; / T if attached to servers,
    pub /: *mut *mut afs_ro_replicating ro_replicating; / RW->RO update (probably) in progress,
    pub nr_servers: c_uchar,
    pub /: *mut *mut unsigned short vnovol_mask; / Servers to be skipped due to VNOVOL,
    pub /: *mut *mut unsigned int seq; / Set to ->servers_seq when installed,
    pub lock: rwlock_t,
    pub servers: [afs_server_entry; ],
}

//
// Live AFS volume management.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_volume {
    pub rcu: rcu_head,
    pub /: *mut *mut afs_volid_t vid; / The volume ID of this volume,
    pub /: *mut *mut afs_volid_t vids[AFS_MAXTYPES]; / All associated volume IDs,
    pub ref: refcount_t,
    pub /: *mut *mut unsigned int debug_id; / Debugging ID for traces,
    pub /: *mut *mut time64_t update_at; / Time at which to next update,
    pub /: *mut *mut *mut afs_cell cell; / Cell to which belongs (pins ref),
    pub /: *mut *mut rb_node cell_node; / Link in cell->volumes,
    pub /: *mut *mut hlist_node proc_link; / Link in cell->proc_volumes,
    pub /: *mut *mut *mut super_block __rcu sb; / Superblock on which inodes reside,
    pub /: *mut *mut work_destructor; / Deferred destructor,
    pub flags: c_ulong,

    pub /: *mut *mut *mut fscache_volume cache; / Caching cookie,

    pub /: *mut *mut *mut afs_server_list __rcu servers; / List of servers on which volume resides,
    pub /: *mut *mut rwlock_t servers_lock; / Lock for ->servers,
    pub /: *mut *mut unsigned int servers_seq; / Incremented each time ->servers changes,
// RO release tracking
    pub /: *mut *mut mutex volsync_lock; / Time/state evaluation lock,
    pub /: *mut *mut time64_t creation_time; / Volume creation time (or TIME64_MIN),
    pub /: *mut *mut time64_t update_time; / Volume update time (or TIME64_MIN),
// Callback management
    pub /: *mut *mut mutex cb_check_lock; / Lock to control race to check after v_break,
    pub /: *mut *mut time64_t cb_expires_at; / Earliest volume callback expiry time,
    pub /: *mut *mut atomic_t cb_ro_snapshot; / RO volume update-from-snapshot counter,
    pub /: *mut *mut atomic_t cb_v_break; / Volume-break event counter.,
    pub /: *mut *mut atomic_t cb_v_check; / Volume-break has-been-checked counter.,
    pub /: *mut *mut atomic_t cb_scrub; / Scrub-all-data event counter.,
    pub cb_v_break_lock: rwlock_t,
    pub open_mmaps_lock: rw_semaphore,
    pub /: *mut *mut list_head open_mmaps; / List of vnodes that are mmapped,
    pub /: *mut *mut afs_voltype_t type; / type of volume,
    pub /: *mut *mut char type_force; / force volume type (suppress R/O -> R/W),
    pub name_len: u8,
    pub /: *mut *mut u8 name[AFS_MAXVOLNAME + 1]; / NUL-padded volume name,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_lock_state {
    AFS_VNODE_LOCK_NONE,		/* The vnode has no lock on the server */
    AFS_VNODE_LOCK_WAITING_FOR_CB,	/* We're waiting for the server to break the callback */
    AFS_VNODE_LOCK_SETTING,		/* We're asking the server for a lock */
    AFS_VNODE_LOCK_GRANTED,		/* We have a lock on the server */
    AFS_VNODE_LOCK_EXTENDING,	/* We're extending a lock on the server */
    AFS_VNODE_LOCK_NEED_UNLOCK,	/* We need to unlock on the server */
    AFS_VNODE_LOCK_UNLOCKING,	/* We're telling the server to unlock */
    AFS_VNODE_LOCK_DELETED,		/* The vnode has been deleted whilst we have a lock */
}

//
// AFS inode private data.
//
// Note that afs_alloc_inode() *must* reset anything that could incorrectly
// leak from one inode to another.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vnode {
    pub /: *mut *mut netfs_inode netfs; / Netfslib context and vfs inode,
    pub /: *mut *mut *mut afs_volume volume; / volume on which vnode resides,
    pub /: *mut *mut afs_fid fid; / the file identifier for this inode,
    pub /: *mut *mut afs_file_status status; / AFS status info for this file,
    pub /: *mut *mut afs_dataversion_t invalid_before; / Child dentries are invalid before this,
    pub /: *mut *mut *mut afs_permits __rcu permit_cache; / cache of permits so far obtained,
    pub /: *mut *mut list_head io_lock_waiters; / Threads waiting for the I/O lock,
    pub /: *mut *mut rw_semaphore validate_lock; / lock for validating this vnode,
    pub /: *mut *mut rw_semaphore rmdir_lock; / Lock for rmdir vs sillyrename,
    pub /: *mut *mut *mut key silly_key; / Silly rename key,
    pub /: *mut *mut spinlock_t wb_lock; / lock for wb_keys,
    pub /: *mut *mut spinlock_t lock; / waitqueue/flags lock,
    pub flags: c_ulong,

    pub /: *mut *mut *mut folio_queue directory; / Directory contents,
    pub /: *mut *mut *mut afs_symlink __rcu symlink; / Symlink content,
    pub /: *mut *mut list_head wb_keys; / List of keys available for writeback,
    pub /: *mut *mut list_head pending_locks; / locks waiting to be granted,
    pub /: *mut *mut list_head granted_locks; / locks granted on this file,
    pub /: *mut *mut delayed_work lock_work; / work to be done in locking,
    pub /: *mut *mut *mut key lock_key; / Key to be used in lock ops,
    pub /: *mut *mut ktime_t locked_at; / Time at which lock obtained,
    pub 8: afs_lock_state lock_state :,
    pub 8: afs_lock_type_t lock_type :,
    pub /: *mut *mut unsigned int directory_size; / Amount of space in ->directory,
// outstanding callback notification on this file
    pub /: *mut *mut work_cb_work; / Work for mmap'd files,
    pub /: *mut *mut list_head cb_mmap_link; / Link in cell->fs_open_mmaps,
    pub /: *mut *mut *mut void cb_server; / Server with callback/filelock,
    pub /: *mut *mut atomic_t cb_nr_mmap; / Number of mmaps,
    pub /: *mut *mut unsigned int cb_ro_snapshot; / RO volume release counter on ->volume,
    pub /: *mut *mut unsigned int cb_scrub; / Scrub counter on ->volume,
    pub /: *mut *mut unsigned int cb_break; / Break counter on vnode,
    pub /: *mut *mut unsigned int cb_v_check; / Break check counter on ->volume,
    pub /: *mut *mut *mut seqlock_t cb_lock; / Lock for ->cb_server, ->status, ->cb_break,
    pub /: *mut *mut atomic64_t cb_expires_at; / time at which callback expires,

}

extern "C" {
    pub fn netfs_i_cookie(_arg: &vnode->netfs) -> return;
}

//
// cached security record for one user's attempt to access a vnode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_permit {
    pub /: *mut *mut *mut key key; / RxRPC ticket holding a security context,
    pub /: *mut *mut afs_access_t access; / CallerAccess value for this key,
}

//
// Immutable cache of CallerAccess records from attempts to access vnodes.
// These may be shared between multiple vnodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_permits {
    pub rcu: rcu_head,
    pub /: *mut *mut hlist_node hash_node; / Link in hash,
    pub /: *mut *mut unsigned long h; / Hash value for this permit list,
    pub usage: refcount_t,
    pub /: *mut *mut unsigned short nr_permits; / Number of records,
    pub /: *mut *mut bool invalidated; / Invalidated due to key change,
    pub /: *mut *mut afs_permit permits[] __counted_by(nr_permits); / List of permits sorted by key pointer,
}

//
// Copy of symlink content for normal use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_symlink {
    pub rcu: rcu_head,
    pub ref: refcount_t,
    pub content: [c_char; ],
}

//
// Error prioritisation and accumulation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_error {
    pub /: *mut *mut s32 abort_code; / Cumulative abort code,
    pub /: *mut *mut short error; / Cumulative error,
    pub /: *mut *mut bool responded; / T if server responded,
    pub /: *mut *mut bool aborted; / T if ->error is from an abort,
}

//
// Cursor for iterating over a set of volume location servers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vl_cursor {
    pub /: *mut *mut *mut afs_cell cell; / The cell we're querying,
    pub /: *mut *mut *mut afs_vlserver_list server_list; / Current server list (pins ref),
    pub /: *mut *mut *mut afs_vlserver server; / Server on which this resides,
    pub /: *mut *mut *mut afs_addr_list alist; / Current address list (pins ref),
    pub /: *mut *mut *mut key key; / Key for the server,
    pub /: *mut *mut unsigned long untried_servers; / Bitmask of untried servers,
    pub /: *mut *mut unsigned long addr_tried; / Tried addresses,
    pub /: *mut *mut afs_error cumul_error; / Cumulative error,
    pub debug_id: c_uint,
    pub call_abort_code: i32,
    pub /: *mut *mut short call_error; / Error from single call,
    pub /: *mut *mut short server_index; / Current server,
    pub /: *mut *mut signed char addr_index; / Current address,
    pub flags: c_ushort,
pub const AFS_VL_CURSOR_STOP: c_uint = 0x0001		/* Set to cease iteration */;
pub const AFS_VL_CURSOR_RETRY: c_uint = 0x0002		/* Set to do a retry */;
pub const AFS_VL_CURSOR_RETRIED: c_uint = 0x0004		/* Set if started a retry */;
    pub /: *mut *mut short nr_iterations; / Number of server iterations,
    pub /: *mut *mut bool call_responded; / T if the current address responded,
}

//
// Fileserver state tracking for an operation.  An array of these is kept,
// indexed by server index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_server_state {
// Tracking of fileserver probe state.  Other operations may interfere
// by probing a fileserver when accessing other volumes.
//
    pub probe_seq: c_uint,
    pub /: *mut *mut unsigned long untried_addrs; / Addresses we haven't tried yet,
    pub probe_waiter: wait_queue_entry,
    pub /: *mut *mut *mut afs_endpoint_state endpoint_state; / Endpoint state being monitored,
}

//
// Fileserver operation methods.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_operation_ops {
    pub op): *mut *mut void (issue_afs_rpc)(struct afs_operation,
    pub op): *mut *mut void (issue_yfs_rpc)(struct afs_operation,
    pub op): *mut *mut void (success)(struct afs_operation,
    pub op): *mut *mut void (aborted)(struct afs_operation,
    pub op): *mut *mut void (failed)(struct afs_operation,
    pub op): *mut *mut void (edit_dir)(struct afs_operation,
    pub op): *mut *mut void (put)(struct afs_operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vnode_param {
    pub vnode: *mut afs_vnode,
    pub /: *mut *mut afs_fid fid; / Fid to access,
    pub /: *mut *mut afs_status_cb scb; / Returned status and callback promise,
    pub /: *mut *mut afs_dataversion_t dv_before; / Data version before the call,
    pub /: *mut *mut unsigned int cb_break_before; / cb_break before the call,
    pub /: *mut *mut u8 dv_delta; / Expected change in data version,
    pub /: *mut *mut bool put_vnode:1; / T if we have a ref on the vnode,
    pub /: *mut *mut bool need_io_lock:1; / T if we need the I/O lock on this,
    pub /: *mut *mut bool update_ctime:1; / Need to update the ctime,
    pub /: *mut *mut bool set_size:1; / Must update i_size,
    pub /: *mut *mut bool op_unlinked:1; / True if file was unlinked by op,
    pub /: *mut *mut bool speculative:1; / T if speculative status fetch (no vnode lock),
    pub /: *mut *mut bool modification:1; / Set if the content gets modified,
}

//
// Fileserver operation wrapper, handling server and address rotation
// asynchronously.  May make simultaneous calls to multiple servers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_operation {
    pub /: *mut *mut *mut afs_net net; / Network namespace,
    pub /: *mut *mut *mut key key; / Key for the cell,
    pub /: *const *const *const afs_call_type type; / Type of call done,
    pub ops: *const afs_operation_ops,
// Parameters/results for the operation
    pub /: *mut *mut *mut afs_volume volume; / Volume being accessed,
    pub file: [afs_vnode_param; 2],
    pub more_files: *mut afs_vnode_param,
    pub /: *mut *mut afs_volsync pre_volsync; / Volsync before op,
    pub /: *mut *mut afs_volsync volsync; / Volsync returned by op,
    pub /: *mut *mut *mut dentry dentry; / Dentry to be altered,
    pub /: *mut *mut *mut dentry dentry_2; / Second dentry to be altered,
    pub /: *mut *mut timespec64 mtime; / Modification time to record,
    pub /: *mut *mut timespec64 ctime; / Change time to set,
    pub /: *mut *mut afs_error cumul_error; / Cumulative error,
    pub /: *mut *mut short nr_files; / Number of entries in file[], more_files,
    pub debug_id: c_uint,
    pub /: *mut *mut unsigned int cb_v_break; / Volume break counter before op,
    pub /: *mut *mut int which; / Which ->file[] to fetch for,
    pub fetch_status: },
    pub /: *mut *mut int reason; / enum afs_edit_dir_reason,
    pub mode: mode_t,
    pub symlink: *mut afs_symlink,
    pub create: },
    pub need_rehash: bool,
    pub unlink: },
    pub rehash: *mut dentry,
    pub tmp: *mut dentry,
    pub rename_flags: c_uint,
    pub new_negative: bool,
    pub rename: },
    pub subreq: *mut netfs_io_subrequest,
    pub fetch: },
    pub type: afs_lock_type_t,
    pub lock: },
    pub write_iter: *mut iov_iter,
    pub pos: loff_t,
    pub size: loff_t,
    pub i_size: loff_t,
    pub store: },
    pub attr: *mut iattr,
    pub old_i_size: loff_t,
    pub setattr: },
    pub acl: *mut afs_acl,
    pub yacl: *mut yfs_acl,
    pub vs: afs_volume_status,
    pub buf: *mut kstatfs,
    pub volstatus: },
}

// Fileserver iteration state
pub const AFS_OPERATION_STOP: c_uint = 0x0001	/* Set to cease iteration */;
pub const AFS_OPERATION_VBUSY: c_uint = 0x0002	/* Set if seen VBUSY */;
pub const AFS_OPERATION_VMOVED: c_uint = 0x0004	/* Set if seen VMOVED */;
pub const AFS_OPERATION_VNOVOL: c_uint = 0x0008	/* Set if seen VNOVOL */;
pub const AFS_OPERATION_CUR_ONLY: c_uint = 0x0010	/* Set if current server only (file lock held) */;
pub const AFS_OPERATION_NO_VSLEEP: c_uint = 0x0020	/* Set to prevent sleep on VBUSY, VOFFLINE, ... */;
pub const AFS_OPERATION_UNINTR: c_uint = 0x0040	/* Set if op is uninterruptible */;
pub const AFS_OPERATION_DOWNGRADE: c_uint = 0x0080	/* Set to retry with downgraded opcode */;
pub const AFS_OPERATION_LOCK_0: c_uint = 0x0100	/* Set if have io_lock on file[0] */;
pub const AFS_OPERATION_LOCK_1: c_uint = 0x0200	/* Set if have io_lock on file[1] */;
pub const AFS_OPERATION_TRIED_ALL: c_uint = 0x0400	/* Set if we've tried all the fileservers */;
pub const AFS_OPERATION_RETRY_SERVER: c_uint = 0x0800	/* Set if we should retry the current server */;
pub const AFS_OPERATION_DIR_CONFLICT: c_uint = 0x1000	/* Set if we detected a 3rd-party dir change */;
pub const AFS_OPERATION_ASYNC: c_uint = 0x2000	/* Set if should run asynchronously */;
//
// Cache auxiliary data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_vnode_cache_aux {
    pub data_version: __be64,
    pub __packed: },
    pub cpu_to_be64(vnode->status.data_version): aux->data_version =,
    pub aux: afs_vnode_cache_aux,
    pub &aux): afs_set_cache_aux(vnode,,
    pub flags): i_size_read(&vnode->netfs.inode),,
//
// Directory iteration management.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_dir_iter {
    pub dvnode: *mut afs_vnode,
    pub block: *mut afs_xdr_dir_block,
    pub fq: *mut folio_queue,
    pub fpos: c_uint,
    pub fq_slot: c_int,
    pub loop_check: c_uint,
    pub nr_slots: u8,
    pub bucket: u8,
    pub prev_entry: c_uint,
}

//
// addr_list.c
//
extern "C" {
    pub fn afs_put_addrlist(alist: *mut afs_addr_list, reason: afs_alist_trace);
}
//
// addr_prefs.c
//
extern "C" {
    pub fn afs_proc_addr_prefs_write(file: *mut file, buf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn afs_get_address_preferences_rcu(net: *mut afs_net, alist: *mut afs_addr_list);
}
extern "C" {
    pub fn afs_get_address_preferences(net: *mut afs_net, alist: *mut afs_addr_list);
}
//
// callback.c
//
extern "C" {
    pub fn afs_invalidate_mmap_work(: *mut work_struct);
}
extern "C" {
    pub fn afs_init_callback_state(: *mut afs_server);
}
extern "C" {
    pub fn __afs_break_callback(: *mut afs_vnode, afs_cb_break_reason: enum);
}
extern "C" {
    pub fn afs_break_callback(: *mut afs_vnode, afs_cb_break_reason: enum);
}
extern "C" {
    pub fn afs_break_callbacks(: *mut afs_server, _arg: usize, : *mut afs_callback_break);
}
//
// cell.c
//
extern "C" {
    pub fn afs_cell_init(: *mut afs_net, : *const c_char) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_lookup_cell_for {
    AFS_LOOKUP_CELL_DYNROOT,
    AFS_LOOKUP_CELL_MOUNTPOINT,
    AFS_LOOKUP_CELL_DIRECT_MOUNT,
    AFS_LOOKUP_CELL_PRELOAD,
    AFS_LOOKUP_CELL_ROOTCELL,
    AFS_LOOKUP_CELL_ALIAS_CHECK,
}

extern "C" {
    pub fn afs_unuse_cell(cell: *mut afs_cell, reason: afs_cell_trace);
}
extern "C" {
    pub fn afs_see_cell(: *mut afs_cell, afs_cell_trace: enum);
}
extern "C" {
    pub fn afs_put_cell(: *mut afs_cell, afs_cell_trace: enum);
}
extern "C" {
    pub fn afs_queue_cell(: *mut afs_cell, afs_cell_trace: enum);
}
extern "C" {
    pub fn afs_set_cell_timer(cell: *mut afs_cell, delay_secs: c_uint);
}
extern "C" {
    pub fn afs_cell_purge(: *mut afs_net) -> void __net_exit;
}
//
// cmservice.c
//
extern "C" {
    pub fn afs_cm_incoming_call(: *mut afs_call) -> bool;
}
//
// cm_security.c
//
extern "C" {
    pub fn afs_process_oob_queue(work: *mut work_struct);
}

extern "C" {
    pub fn afs_create_token_key(net: *mut afs_net, socket: *mut socket) -> c_int;
}

//
// dir.c
//
extern "C" {
    pub fn afs_d_release(: *mut dentry);
}
extern "C" {
    pub fn afs_check_for_remote_deletion(: *mut afs_operation);
}
//
// dir_edit.c
//
extern "C" {
    pub fn afs_edit_dir_remove(: *mut afs_vnode, : *const qstr, afs_edit_dir_reason: enum);
}
extern "C" {
    pub fn afs_mkdir_init_dir(dvnode: *mut afs_vnode, parent_vnode: *mut afs_vnode);
}
//
// dir_search.c
//
extern "C" {
    pub fn afs_dir_hash_name(name: *const qstr) -> c_uint;
}
extern "C" {
    pub fn afs_dir_init_iter(iter: *mut afs_dir_iter, name: *const qstr) -> bool;
}
//
// dir_silly.c
//
extern "C" {
    pub fn afs_silly_iput(: *mut dentry, : *mut inode) -> c_int;
}
//
// dynroot.c
//
// file.c
//
extern "C" {
    pub fn afs_cache_wb_key(: *mut afs_vnode, : *mut afs_file) -> c_int;
}
extern "C" {
    pub fn afs_put_wb_key(: *mut afs_wb_key);
}
extern "C" {
    pub fn afs_open(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn afs_release(: *mut inode, : *mut file) -> c_int;
}
extern "C" {
    pub fn afs_fetch_data_async_rx(work: *mut work_struct);
}
extern "C" {
    pub fn afs_fetch_data_immediate_cancel(call: *mut afs_call);
}
extern "C" {
    pub fn afs_set_i_size(vnode: *mut afs_vnode, new_i_size: loff_t);
}
//
// flock.c
//
extern "C" {
    pub fn afs_lock_op_done(: *mut afs_call);
}
extern "C" {
    pub fn afs_lock_work(: *mut work_struct);
}
extern "C" {
    pub fn afs_lock_may_be_available(: *mut afs_vnode);
}
extern "C" {
    pub fn afs_lock(: *mut file, _arg: c_int, : *mut file_lock) -> c_int;
}
extern "C" {
    pub fn afs_flock(: *mut file, _arg: c_int, : *mut file_lock) -> c_int;
}
//
// fsclient.c
//
extern "C" {
    pub fn afs_fs_fetch_status(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_fetch_data(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_create_file(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_make_dir(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_remove_file(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_remove_dir(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_link(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_symlink(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_rename(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_store_data(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_setattr(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_get_volume_status(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_set_lock(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_extend_lock(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_release_lock(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_inline_bulk_status(: *mut afs_operation);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_acl {
    pub size: u32,
    pub __counted_by(size): u8 data[],
}

extern "C" {
    pub fn afs_fs_fetch_acl(: *mut afs_operation);
}
extern "C" {
    pub fn afs_fs_store_acl(: *mut afs_operation);
}
//
// fs_operation.c
//
extern "C" {
    pub fn afs_put_operation(: *mut afs_operation) -> c_int;
}
extern "C" {
    pub fn afs_begin_vnode_operation(: *mut afs_operation) -> bool;
}
extern "C" {
    pub fn afs_end_vnode_operation(op: *mut afs_operation);
}
extern "C" {
    pub fn afs_wait_for_operation(: *mut afs_operation);
}
extern "C" {
    pub fn afs_do_sync_operation(: *mut afs_operation) -> c_int;
}
//
// fs_probe.c
//
extern "C" {
    pub fn afs_put_endpoint_state(estate: *mut afs_endpoint_state, where: afs_estate_trace);
}
extern "C" {
    pub fn afs_fileserver_probe_result(: *mut afs_call);
}
extern "C" {
    pub fn afs_wait_for_fs_probes(op: *mut afs_operation, states: *mut afs_server_state, intr: bool) -> c_int;
}
extern "C" {
    pub fn afs_probe_fileserver(: *mut afs_net, : *mut afs_server);
}
extern "C" {
    pub fn afs_fs_probe_dispatcher(: *mut work_struct);
}
extern "C" {
    pub fn afs_fs_probe_cleanup(: *mut afs_net);
}
//
// inode.c
//
extern "C" {
    pub fn afs_vnode_commit_status(: *mut afs_operation, : *mut afs_vnode_param);
}
extern "C" {
    pub fn afs_fetch_status(: *mut afs_vnode, : *mut key, _arg: bool, : *mut afs_access_t) -> c_int;
}
extern "C" {
    pub fn afs_ilookup5_test_by_fid(: *mut inode, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn afs_setattr(idmap: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn afs_evict_inode(: *mut inode);
}
extern "C" {
    pub fn afs_drop_inode(: *mut inode) -> c_int;
}
//
// main.c
//
extern "C" {
    pub fn net_generic(_arg: net, _arg: afs_net_id) -> return;
}
extern "C" {
    pub fn afs_net(_arg: AFS_FS_S(sb)->net_ns) -> return;
}
extern "C" {
    pub fn afs_sb2net(_arg: dentry->d_sb) -> return;
}
extern "C" {
    pub fn afs_sb2net(_arg: inode->i_sb) -> return;
}
extern "C" {
    pub fn afs_i2net(_arg: &vnode->netfs.inode) -> return;
}
extern "C" {
    pub fn net_generic(_arg: sock_net(sk), _arg: afs_net_id) -> return;
}

//
// misc.c
//
extern "C" {
    pub fn afs_abort_to_error(_arg: u32) -> c_int;
}
extern "C" {
    pub fn afs_prioritise_error(: *mut afs_error, _arg: c_int, _arg: u32);
}
//
// mntpt.c
//
extern "C" {
    pub fn afs_mntpt_kill_timer();
}
//
// proc.c
//

extern "C" {
    pub fn afs_proc_init(: *mut afs_net) -> int __net_init;
}
extern "C" {
    pub fn afs_proc_cleanup(: *mut afs_net) -> void __net_exit;
}
extern "C" {
    pub fn afs_proc_cell_setup(: *mut afs_cell) -> c_int;
}
extern "C" {
    pub fn afs_proc_cell_remove(: *mut afs_cell);
}
extern "C" {
    pub fn afs_put_sysnames(: *mut afs_sysnames);
}

//
// rotate.c
//
extern "C" {
    pub fn afs_clear_server_states(op: *mut afs_operation);
}
extern "C" {
    pub fn afs_select_fileserver(: *mut afs_operation) -> bool;
}
extern "C" {
    pub fn afs_dump_edestaddrreq(: *const afs_operation);
}
//
// rxrpc.c
//
extern "C" {
    pub fn afs_open_socket(: *mut afs_net) -> int __net_init;
}
extern "C" {
    pub fn afs_close_socket(: *mut afs_net) -> void __net_exit;
}
extern "C" {
    pub fn afs_charge_preallocation(: *mut work_struct);
}
extern "C" {
    pub fn afs_put_call(: *mut afs_call);
}
extern "C" {
    pub fn afs_deferred_put_call(call: *mut afs_call);
}
extern "C" {
    pub fn afs_make_call(call: *mut afs_call, gfp: gfp_t);
}
extern "C" {
    pub fn afs_deliver_to_call(call: *mut afs_call);
}
extern "C" {
    pub fn afs_wait_for_call_to_complete(call: *mut afs_call);
}
extern "C" {
    pub fn afs_flat_call_destructor(: *mut afs_call);
}
extern "C" {
    pub fn afs_send_empty_reply(: *mut afs_call);
}
extern "C" {
    pub fn afs_send_simple_reply(: *mut afs_call, : *const c_void, _arg: usize);
}
extern "C" {
    pub fn afs_extract_data(: *mut afs_call, _arg: bool) -> c_int;
}
extern "C" {
    pub fn afs_protocol_error(: *mut afs_call, afs_eproto_cause: enum) -> c_int;
}
extern "C" {
    pub fn afs_extract_data(_arg: call, _arg: false) -> return;
}
// Asynchronous calls have two refs to release - one from the alloc and
// one queued with the work item - and we can't just deallocate the
// call because the work item may be queued again.
//
// security.c
//
extern "C" {
    pub fn afs_put_permits(: *mut afs_permits);
}
extern "C" {
    pub fn afs_clear_permits(: *mut afs_vnode);
}
extern "C" {
    pub fn afs_check_permit(: *mut afs_vnode, : *mut key, : *mut afs_access_t) -> c_int;
}
extern "C" {
    pub fn afs_permission(: *mut mnt_idmap, : *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn afs_clean_up_permit_cache() -> void __exit;
}
//
// server.c
//
extern "C" {
    pub fn afs_put_server(: *mut afs_net, : *mut afs_server, afs_server_trace: enum);
}
extern "C" {
    pub fn afs_purge_servers(cell: *mut afs_cell);
}
extern "C" {
    pub fn afs_fs_probe_timer(: *mut timer_list);
}
extern "C" {
    pub fn afs_wait_for_servers(net: *mut afs_net) -> void __net_exit;
}
extern "C" {
    pub fn afs_check_server_record(op: *mut afs_operation, server: *mut afs_server, key: *mut key) -> bool;
}
extern "C" {
    pub fn list_empty(_arg: &server->probe_link) -> return;
}
//
// server_list.c
//
extern "C" {
    pub fn afs_put_serverlist(: *mut afs_net, : *mut afs_server_list);
}
extern "C" {
    pub fn afs_annotate_server_list(: *mut afs_server_list, : *mut afs_server_list) -> bool;
}
extern "C" {
    pub fn afs_attach_volume_to_servers(volume: *mut afs_volume, slist: *mut afs_server_list);
}
extern "C" {
    pub fn afs_detach_volume_from_servers(volume: *mut afs_volume, slist: *mut afs_server_list);
}
//
// super.c
//
extern "C" {
    pub fn afs_fs_init() -> int __init;
}
extern "C" {
    pub fn afs_fs_exit();
}
//
// symlink.c
//
extern "C" {
    pub fn afs_invalidate_symlink(vnode: *mut afs_vnode);
}
extern "C" {
    pub fn afs_evict_symlink(vnode: *mut afs_vnode);
}
extern "C" {
    pub fn afs_init_new_symlink(vnode: *mut afs_vnode, op: *mut afs_operation);
}
extern "C" {
    pub fn afs_readlink(dentry: *mut dentry, buffer: *mut char __user, buflen: c_int) -> c_int;
}
//
// validation.c
//
extern "C" {
    pub fn afs_check_validity(vnode: *const afs_vnode) -> bool;
}
extern "C" {
    pub fn afs_update_volume_state(op: *mut afs_operation) -> c_int;
}
extern "C" {
    pub fn afs_validate(vnode: *mut afs_vnode, key: *mut key) -> c_int;
}
//
// vlclient.c
//
// vl_alias.c
//
extern "C" {
    pub fn afs_cell_detect_alias(: *mut afs_cell, : *mut key) -> c_int;
}
//
// vl_probe.c
//
extern "C" {
    pub fn afs_vlserver_probe_result(: *mut afs_call);
}
extern "C" {
    pub fn afs_send_vl_probes(: *mut afs_net, : *mut key, : *mut afs_vlserver_list) -> c_int;
}
extern "C" {
    pub fn afs_wait_for_vl_probes(: *mut afs_vlserver_list, long: unsigned) -> c_int;
}
//
// vl_rotate.c
//
extern "C" {
    pub fn afs_select_vlserver(: *mut afs_vl_cursor) -> bool;
}
extern "C" {
    pub fn afs_select_current_vlserver(: *mut afs_vl_cursor) -> bool;
}
extern "C" {
    pub fn afs_end_vlserver_operation(: *mut afs_vl_cursor) -> c_int;
}
//
// vlserver_list.c
//
extern "C" {
    pub fn afs_put_vlserver(: *mut afs_net, : *mut afs_vlserver);
}
extern "C" {
    pub fn afs_put_vlserverlist(: *mut afs_net, : *mut afs_vlserver_list);
}
//
// volume.c
//
extern "C" {
    pub fn afs_activate_volume(: *mut afs_volume) -> c_int;
}
extern "C" {
    pub fn afs_deactivate_volume(: *mut afs_volume);
}
extern "C" {
    pub fn afs_try_get_volume(volume: *mut afs_volume, reason: afs_volume_trace) -> bool;
}
extern "C" {
    pub fn afs_put_volume(volume: *mut afs_volume, reason: afs_volume_trace);
}
extern "C" {
    pub fn afs_check_volume_status(: *mut afs_volume, : *mut afs_operation) -> c_int;
}
//
// write.c
//
extern "C" {
    pub fn afs_prepare_write(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn afs_issue_write(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn afs_begin_writeback(wreq: *mut netfs_io_request);
}
extern "C" {
    pub fn afs_retry_request(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream);
}
extern "C" {
    pub fn afs_writepages(: *mut address_space, : *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn afs_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn afs_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn afs_prune_wb_keys(: *mut afs_vnode);
}
//
// xattr.c
//
// yfsclient.c
//
extern "C" {
    pub fn yfs_fs_fetch_data(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_create_file(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_make_dir(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_remove_file2(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_remove_file(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_remove_dir(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_link(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_symlink(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_rename(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_rename_replace(op: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_rename_noreplace(op: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_rename_exchange(op: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_store_data(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_setattr(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_get_volume_status(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_set_lock(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_extend_lock(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_release_lock(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_fetch_status(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_inline_bulk_status(: *mut afs_operation);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_acl {
    pub /: *mut *mut *mut afs_acl acl; / Dir/file/symlink ACL,
    pub /: *mut *mut *mut afs_acl vol_acl; / Whole volume ACL,
    pub /: *mut *mut u32 inherit_flag; / True if ACL is inherited from parent dir,
    pub /: *mut *mut u32 num_cleaned; / Number of ACEs removed due to subject removal,
    pub flags: c_uint,
pub const YFS_ACL_WANT_ACL: c_uint = 0x01	/* Set if caller wants ->acl */;
pub const YFS_ACL_WANT_VOL_ACL: c_uint = 0x02	/* Set if caller wants ->vol_acl */;
}

extern "C" {
    pub fn yfs_free_opaque_acl(: *mut yfs_acl);
}
extern "C" {
    pub fn yfs_fs_fetch_opaque_acl(: *mut afs_operation);
}
extern "C" {
    pub fn yfs_fs_store_opaque_acl2(: *mut afs_operation);
}
//
// Miscellaneous inline functions.
//
extern "C" {
    pub fn container_of(_arg: inode, afs_vnode: struct, _arg: netfs.inode) -> return;
}
//
// Note that a dentry got changed.  We need to set d_fsdata to the data version
// number derived from the result of the operation.  It doesn't matter if
// d_fsdata goes backwards as we'll just revalidate.
//
// Check for a conflicting operation on a directory that we just unlinked from.
// If someone managed to sneak a link or an unlink in on the file we just
// unlinked, we won't be able to trust nlink on an AFS file (but not YFS).
//
// Set the callback promise on a vnode.
//
// Clear the callback promise on a vnode, returning true if it was promised.
//
// Mark a directory as being invalid.
//
// debug tracing
//

pub const AFS_DEBUG_KENTER: c_uint = 0x01;
pub const AFS_DEBUG_KLEAVE: c_uint = 0x02;
pub const AFS_DEBUG_KDEBUG: c_uint = 0x04;

//
// debug assertion checking
//

