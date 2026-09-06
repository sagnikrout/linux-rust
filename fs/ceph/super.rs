//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/super.h
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

// large granularity for statfs utilization stats to facilitate
// large volume sizes on 32-bit machines.

// max size of osd read request, limited by libceph

// osd has a configurable limitation of max write size.
// CEPH_MSG_MAX_DATA_LEN should be small enough.

pub const CEPH_MAX_READDIR_DEFAULT: c_int = 1024;

//
// Delay telling the MDS we no longer want caps, in case we reopen
// the file.  Delay a minimum amount of time, even if we send a cap
// message for some other reason.  Otherwise, take the oppotunity to
// update the mds to avoid sending another message later.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mount_options {
    pub flags: c_uint,
    pub /: *mut *mut unsigned int wsize; / max write size,
    pub /: *mut *mut unsigned int rsize; / max read size,
    pub /: *mut *mut unsigned int rasize; / max readahead,
    pub /: *mut *mut unsigned int congestion_kb; / max writeback in flight,
    pub caps_wanted_delay_max: unsigned int caps_wanted_delay_min,,
    pub caps_max: c_int,
    pub /: *mut *mut unsigned int max_readdir; / max readdir result (entries),
    pub /: *mut *mut unsigned int max_readdir_bytes; / max readdir result (bytes),
    pub new_dev_syntax: bool,
//
// everything above this point can be memcmp'd; everything below
// is handled in compare_mount_options()
//
    pub /: *mut *mut *mut char snapdir_name; / default ".snap",
    pub /: *mut *mut *mut char mds_namespace; / default NULL,
    pub /: *mut *mut *mut char server_path; / default NULL (means "/"),
    pub /: *mut *mut *mut char fscache_uniq; / default NULL,
    pub mon_addr: *mut c_char,
    pub dummy_enc_policy: fscrypt_dummy_policy,
}

//
// Check if the mds namespace in ceph_mount_options matches
// the passed in namespace string. First time match (when
// ->mds_namespace is NULL) is treated specially, since
// ->mds_namespace needs to be initialized by the caller.
//
// mount state
pub const CEPH_ASYNC_CREATE_CONFLICT_BITS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_fs_client {
    pub sb: *mut super_block,
    pub metric_wakeup: list_head,
    pub mount_options: *mut ceph_mount_options,
    pub client: *mut ceph_client,
    pub mount_state: c_int,
    pub blocklisted: bool,
    pub have_copy_from2: bool,
    pub filp_gen: u32,
    pub max_file_size: loff_t,
    pub mdsc: *mut ceph_mds_client,
    pub writeback_count: atomic_long_t,
    pub write_congested: bool,
    pub inode_wq: *mut workqueue_struct,
    pub cap_wq: *mut workqueue_struct,
    pub CEPH_ASYNC_CREATE_CONFLICT_BITS): DECLARE_HASHTABLE(async_unlink_conflict,,
    pub async_unlink_conflict_lock: spinlock_t,

    pub debugfs_caps: *mut *mut dentry debugfs_dentry_lru,,
    pub debugfs_congestion_kb: *mut dentry,
    pub debugfs_bdi: *mut dentry,
    pub debugfs_mdsmap: *mut *mut dentry debugfs_mdsc,,
    pub debugfs_status: *mut dentry,
    pub debugfs_mds_sessions: *mut dentry,
    pub debugfs_metrics_dir: *mut dentry,
    pub debugfs_reset_dir: *mut dentry,
    pub debugfs_subvolume_metrics: *mut dentry,

    pub fscache: *mut fscache_volume,

    pub fsc_dummy_enc_policy: fscrypt_dummy_policy,

}

//
// File i/o capability.  This tracks shared state with the metadata
// server that allows us to cache or writeback attributes or to read
// and write data.  For any given inode, we should have one or more
// capabilities, one issued by each metadata server, and our
// cumulative access is the OR of all issued capabilities.
//
// Each cap is referenced by the inode's i_caps rbtree and by per-mds
// session capability lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_cap {
    pub ci: *mut ceph_inode_info,
//
// Per-ci cap tree.  Protected with
// `ceph_inode_info.i_ceph_lock`.
//
// Clearing this field with RB_CLEAR_NODE() requires holding
// both `ceph_inode_info.i_ceph_lock` and
// `ceph_mds_session->s_cap_lock`.  Calling RB_EMPTY_NODE()
// (via ceph_cap_is_removed()) requires holding at least one
// of these.
//
    pub ci_node: rb_node,
    pub session: *mut ceph_mds_session,
    pub /: *mut *mut list_head session_caps; / per-session caplist,
    pub /: *mut *mut u64 cap_id; / unique cap id (mds provided),
// in-use caps
    pub /: *mut *mut int issued; / latest, from the mds,
    pub of: *mut *mut int implemented; / implemented superset,
    pub /: *mut *mut int mds; / mds index for this cap,
    pub /: *mut *mut int mds_wanted; / caps wanted from this mds,
}

// caps to release

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_cap_flush {
    pub tid: u64,
    pub caps: c_int,
    pub /: *mut *mut bool wake; / wake up flush waiters when finish ?,
    pub /: *mut *mut bool is_capsnap; / true means capsnap,
    pub global: list_head g_list; //,
    pub inode: list_head i_list; // per,
    pub ci: *mut ceph_inode_info,
}

//
// Snapped cap state that is pending flush to mds.  When a snapshot occurs,
// we first complete any in-process sync writes and writeback any dirty
// data before flushing the snapped state (tracked here) back to the MDS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_cap_snap {
    pub nref: refcount_t,
    pub ci_item: list_head,
    pub cap_flush: ceph_cap_flush,
    pub follows: u64,
    pub dirty: int issued,,
    pub context: *mut ceph_snap_context,
    pub mode: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub xattr_blob: *mut ceph_buffer,
    pub xattr_version: u64,
    pub size: u64,
    pub change_attr: u64,
    pub btime: timespec64 mtime, atime, ctime,,
    pub time_warp_seq: u64,
    pub truncate_size: u64,
    pub truncate_seq: u32,
    pub /: *mut *mut int writing; / a sync write is still in progress,
    pub /: *mut *mut int dirty_pages; / dirty pages awaiting writeback,
    pub inline_data: bool,
    pub need_flush: bool,
}

//
// The frag tree describes how a directory is fragmented, potentially across
// multiple metadata servers.  It is also used to indicate points where
// metadata authority is delegated, and whether/where metadata is replicated.
//
// A _leaf_ frag will be present in the i_fragtree IFF there is
// delegation info.  That is, if mds >= 0 || ndist > 0.
//
pub const CEPH_MAX_DIRFRAG_REP: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_inode_frag {
    pub node: rb_node,
// fragtree state
    pub frag: u32,
    pub /: *mut *mut int split_by; / i.e. 2^(split_by) children,
// delegation and replication info
    pub /: *mut *mut int mds; / -1 if same authority as parent,
    pub /: *mut *mut int ndist; / >0 if replicated,
    pub dist: [c_int; CEPH_MAX_DIRFRAG_REP],
}

//
// We cache inode xattrs as an encoded blob until they are first used,
// at which point we parse them into an rbtree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_inode_xattr {
    pub node: rb_node,
    pub name: *const c_char,
    pub name_len: c_int,
    pub val: *const c_char,
    pub val_len: c_int,
    pub dirty: c_int,
    pub should_free_name: c_int,
    pub should_free_val: c_int,
}

//
// Ceph dentry state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_dentry_info {
    pub dentry: *mut dentry,
    pub lease_session: *mut ceph_mds_session,
    pub lease_list: list_head,
    pub hnode: hlist_node,
    pub flags: c_ulong,
    pub lease_shared_gen: c_int,
    pub lease_gen: u32,
    pub lease_seq: u32,
    pub lease_renew_from: unsigned long lease_renew_after,,
    pub time: c_ulong,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_inode_xattrs_info {
//
// (still encoded) xattr blob. we avoid the overhead of parsing
// this until someone actually calls getxattr, etc.
//
// blob->vec.iov_len == 4 implies there are no xattrs; blob ==
// NULL means we don't know.
//
    pub prealloc_blob: *mut *mut ceph_buffer blob,,
    pub index: rb_root,
    pub dirty: bool,
    pub count: c_int,
    pub names_size: c_int,
    pub vals_size: c_int,
    pub index_version: u64 version,,
}

//
// Ceph inode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_inode_info {
    pub /: *mut *mut netfs_inode netfs; / Netfslib context and vfs inode,
    pub /: *mut *mut ceph_vino i_vino; / ceph ino + snap,
    pub i_ceph_lock: spinlock_t,
    pub i_version: u64,
    pub i_inline_version: u64,
    pub i_time_warp_seq: u32,
    pub i_ceph_flags: c_ulong,
    pub i_release_count: core::sync::atomic::AtomicI64,
    pub i_ordered_count: core::sync::atomic::AtomicI64,
    pub i_complete_seq: [core::sync::atomic::AtomicI64; 2],
    pub i_dir_layout: ceph_dir_layout,
    pub i_layout: ceph_file_layout,
    pub creates: ceph_file_layout i_cached_layout; // for async,
    pub i_symlink: *mut c_char,
// for dirs
    pub i_rctime: timespec64,
    pub i_rsnaps: u64 i_rbytes, i_rfiles, i_rsubdirs,,
    pub i_subdirs: u64 i_files,,
// quotas
    pub i_max_files: u64 i_max_bytes,,
//
// Subvolume ID this inode belongs to. CEPH_SUBVOLUME_ID_NONE (0)
// means unknown/unset, matching the FUSE client convention.
// Once set to a valid (non-zero) value, it should not change
// during the inode's lifetime.
//
pub const CEPH_SUBVOLUME_ID_NONE: c_int = 0;
    pub i_subvolume_id: u64,
    pub i_dir_pin: i32,
    pub i_fragtree: rb_root,
    pub i_fragtree_nsplits: c_int,
    pub i_fragtree_mutex: mutex,
    pub i_xattrs: ceph_inode_xattrs_info,
// capabilities.  protected _both_ by i_ceph_lock and cap->session's
// s_mutex.
    pub /: *mut *mut rb_root i_caps; / cap list,
    pub /: *mut *mut *mut ceph_cap i_auth_cap; / authoritative cap, if any,
    pub /: *mut *mut unsigned i_dirty_caps, i_flushing_caps; / mask of dirtied fields,
//
// Link to the auth cap's session's s_cap_dirty list. s_cap_dirty
// is protected by the mdsc->cap_dirty_lock, but each individual item
// is also protected by the inode's i_ceph_lock. Walking s_cap_dirty
// requires the mdsc->cap_dirty_lock. List presence for an item can
// be tested under the i_ceph_lock. Changing anything requires both.
//
    pub i_dirty_item: list_head,
//
// Link to session's s_cap_flushing list. Protected in a similar
// fashion to i_dirty_item, but also by the s_mutex for changes. The
// s_cap_flushing list can be walked while holding either the s_mutex
// or msdc->cap_dirty_lock. List presence can also be checked while
// holding the i_ceph_lock for this inode.
//
    pub i_flushing_item: list_head,
// we need to track cap writeback on a per-cap-bit basis, to allow
// overlapping, pipelined cap flushes to the mds.  we can probably
// reduce the tid to 8 bits if we're concerned about inode size.
    pub i_prealloc_cap_flush: *mut ceph_cap_flush,
    pub i_cap_flush_list: list_head,
    pub /: *mut *mut wait_queue_head_t i_cap_wq; / threads waiting on a capability,
    pub /: *mut *mut unsigned long i_hold_caps_max; / jiffies,
    pub /: *mut *mut list_head i_cap_delay_list; / for delayed cap release to mds,
    pub i_cap_migration_resv: ceph_cap_reservation,
    pub /: *mut *mut list_head i_cap_snaps; / snapped state pending flush to mds,
    pub or: *mut *mut *mut ceph_snap_context i_head_snapc; / set if wr_buffer_head > 0,
    pub /: *mut *mut unsigned i_snap_caps; / cap bits for snapped files,
//
// Written under i_ceph_lock, read via READ_ONCE()
// from diagnostic paths.
//
    pub i_last_cap_flush_ack: u64,
    pub i_last_rd: c_ulong,
    pub i_last_wr: c_ulong,
    pub /: *mut *mut int i_nr_by_mode[CEPH_FILE_MODE_BITS]; / open file counts,
    pub i_truncate_mutex: mutex,
    pub /: *mut *mut u32 i_truncate_seq; / last truncate to smaller size,
    pub /: *mut *mut u64 i_truncate_size; / and the size we last truncated down to,
    pub /: *mut *mut int i_truncate_pending; / still need to call vmtruncate,
//
// For none fscrypt case it equals to i_truncate_size or it will
// equals to fscrypt_file_size
//
    pub i_truncate_pagecache_size: u64,
    pub /: *mut *mut u64 i_max_size; / max file size authorized by mds,
    pub /: *mut *mut u64 i_reported_size; / (max_)size reported to or requested of mds,
    pub /: *mut *mut u64 i_wanted_max_size; / offset we'd like to write too,
    pub /: *mut *mut u64 i_requested_max_size; / max_size we've requested,
// held references to caps
    pub i_pin_ref: c_int,
    pub i_fx_ref: int i_rd_ref, i_rdcache_ref, i_wr_ref, i_wb_ref,,
    pub i_wrbuffer_ref_head: int i_wrbuffer_ref,,
    pub i_filelock_ref: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t i_shared_gen; / increment each time we get FILE_SHARED,
    pub /: *mut *mut u32 i_rdcache_gen; / incremented each time we get FILE_CACHE.,
    pub /: *mut *mut u32 i_rdcache_revoking; / RDCACHE gen to async invalidate, if any,
    pub /: *mut *mut list_head i_unsafe_dirops; / uncommitted mds dir ops,
    pub /: *mut *mut list_head i_unsafe_iops; / uncommitted mds inode ops,
    pub i_unsafe_lock: spinlock_t,
    pub /: *mut *mut *mut ceph_snap_realm i_snap_realm; / snap realm (if caps),
    pub /: *mut *mut *mut ceph_snapid_map i_snapid_map; / snapid -> dev_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_netfs_request_data {
    pub caps: c_int,
//
// Maximum size of a file readahead request.
// The fadvise could update the bdi's default ra_pages.
//
    pub file_ra_pages: c_uint,
// Set it if fadvise disables file readahead entirely
    pub file_ra_disabled: bool,
}

extern "C" {
    pub fn container_of(_arg: inode, ceph_inode_info: struct, _arg: netfs.inode) -> return;
}
//
// Inode numbers in cephfs are 64 bits, but inode->i_ino is 32-bits on
// some arches. We generally do not use this value inside the ceph driver, but
// we do want to set it to something, so that generic vfs code has an
// appropriate value for tracepoints and the like.
//
extern "C" {
    pub fn ceph_ino_to_ino32(_arg: vino.ino) -> return;
}
// for printf-style formatting

//
// ceph_present_ino - format an inode number for presentation to userland
// @sb: superblock where the inode lives
// @ino: inode number to (possibly) convert
//
// If the user mounted with the ino32 option, then the 64-bit value needs
// to be converted to something that can fit inside 32 bits. Note that
// internal kernel code never uses this value, so this is entirely for
// userland consumption.
//
extern "C" {
    pub fn ceph_ino_to_ino32(_arg: ino) -> return;
}
extern "C" {
    pub fn ceph_present_ino(_arg: inode->i_sb, _arg: ceph_ino(inode)) -> return;
}
//
// The MDS reserves a set of inodes for its own usage. These should never
// be accessible by clients, and so the MDS has no reason to ever hand these
// out. The range is CEPH_MDS_INO_MDSDIR_OFFSET..CEPH_INO_SYSTEM_BASE.
//
// These come from src/mds/mdstypes.h in the ceph sources.
//
pub const CEPH_MAX_MDS: c_uint = 0x100;
pub const CEPH_NUM_STRAY: c_int = 10;

//
// Upper bound on the number of delegated inodes a single MDS session may
// hold. The MDS normally hands out a small preallocation window (the
// userspace mds_client_prealloc_inos option defaults to 1000) and refills
// it as the client consumes entries. This leaves generous headroom while
// bounding the CPU and memory a malformed delegation interval can consume.
//
pub const CEPH_MAX_DELEG_INOS: c_int = 8192;
// Don't warn on mdsdirs
//
// NB: The hashval will be run through the fs/inode.c hash function
// anyway, so there is no need to squash the inode number down to
// 32-bits first. Just use low-order bits on arches with 32-bit long.
//
extern "C" {
    pub fn ilookup5(_arg: sb, long)vino.ino: (unsigned, _arg: ceph_ino_compare, _arg: &vino) -> return;
}
//
// Ceph inode.
//

// bit 1 historically unused

// force a cap message to the MDS once
// the deferred work completes
//

//
// Masks of ceph inode work.
//
pub const CEPH_I_WORK_WRITEBACK: c_int = 0;
pub const CEPH_I_WORK_INVALIDATE_PAGES: c_int = 1;
pub const CEPH_I_WORK_VMTRUNCATE: c_int = 2;
pub const CEPH_I_WORK_CHECK_CAPS: c_int = 3;
pub const CEPH_I_WORK_FLUSH_SNAPS: c_int = 4;
//
// We set the ERROR_WRITE bit when we start seeing write errors on an inode
// and then clear it when they start succeeding. The write submission code
// just takes this as a hint, so we're not too worried if a few slip through
// in either direction.
//
// Makes sure operations that setup readdir cache (update page
// cache and i_size) are strongly ordered w.r.t. the following
// atomic64_set() operations.
//
// find a specific frag @f
//
// choose fragment for value @v.  copy frag content to pfrag, if leaf
// exists
//
// caps helpers
//
extern "C" {
    pub fn __ceph_caps_issued(ci: *mut ceph_inode_info, implemented: *mut c_int) -> c_int;
}
extern "C" {
    pub fn __ceph_caps_issued_mask(ci: *mut ceph_inode_info, mask: c_int, t: c_int) -> c_int;
}
extern "C" {
    pub fn ceph_free_cap_flush(cf: *mut ceph_cap_flush);
}
extern "C" {
    pub fn __ceph_caps_used(ci: *mut ceph_inode_info) -> c_int;
}
extern "C" {
    pub fn __ceph_caps_file_wanted(ci: *mut ceph_inode_info) -> c_int;
}
extern "C" {
    pub fn __ceph_caps_wanted(ci: *mut ceph_inode_info) -> c_int;
}
// what the mds thinks we want
extern "C" {
    pub fn __ceph_caps_mds_wanted(ci: *mut ceph_inode_info, check: bool) -> c_int;
}
extern "C" {
    pub fn ceph_caps_init(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_caps_finalize(mdsc: *mut ceph_mds_client);
}
//
// we keep buffered readdir results attached to file->private_data
//
pub const CEPH_F_SYNC: c_int = 1;
pub const CEPH_F_ATEND: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_file_info {
    pub /: *mut *mut short fmode; / initialized on open,
    pub /: *mut *mut *mut short flags; / CEPH_F_,
    pub rw_contexts_lock: spinlock_t,
    pub rw_contexts: list_head,
    pub filp_gen: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_dir_file_info {
    pub file_info: ceph_file_info,
// readdir: position within the dir
    pub frag: u32,
    pub last_readdir: *mut ceph_mds_request,
// readdir: position within a frag
    pub /: *mut *mut unsigned next_offset; / offset of next chunk (last_name's + 1),
    pub /: *mut *mut *mut char last_name; / last entry in previous chunk,
    pub dir_release_count: c_longlong,
    pub dir_ordered_count: c_longlong,
    pub readdir_cache_idx: c_int,
// used for -o dirstat read() on directory thing
    pub dir_info: *mut c_char,
    pub dir_info_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_rw_context {
    pub list: list_head,
    pub thread: *mut task_struct,
    pub caps: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_readdir_cache_control {
    pub folio: *mut folio,
    pub dentries: *mut dentry,
    pub index: c_int,
}

//
// A "snap realm" describes a subset of the file hierarchy sharing
// the same set of snapshots that apply to it.  The realms themselves
// are organized into a hierarchy, such that children inherit (some of)
// the snapshots of their parents.
//
// All inodes within the realm that have capabilities are linked into a
// per-realm list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_snap_realm {
    pub ino: u64,
    pub inode: *mut inode,
    pub nref: core::sync::atomic::AtomicI32,
    pub node: rb_node,
    pub seq: u64 created,,
    pub parent_ino: u64,
    pub /: *mut *mut u64 parent_since; / snapid when our current parent became so,
    pub /: *mut *mut *mut u64 prior_parent_snaps; / snaps inherited from any parents we,
    pub /: *mut *mut u32 num_prior_parent_snaps; / had prior to parent_since,
    pub /: *mut *mut *mut u64 snaps; / snaps specific to this realm,
    pub num_snaps: u32,
    pub parent: *mut ceph_snap_realm,
    pub /: *mut *mut list_head children; / list of child realms,
    pub child_item: list_head,
    pub /: *mut *mut list_head empty_item; / if i have ref==0,
    pub /: *mut *mut list_head dirty_item; / if realm needs new context,
    pub /: *mut *mut list_head rebuild_item; / rebuild snap realms _downward_ in hierarchy,
// the current set of snaps for this realm
    pub cached_context: *mut ceph_snap_context,
    pub inodes_with_caps: list_head,
    pub inodes_with_caps_lock: spinlock_t,
}

//
// Copied from NFS
//
// congestion size, scale with available memory.
//
// 64MB:    8192k
// 128MB:   11585k
// 256MB:   16384k
// 512MB:   23170k
// 1GB:   32768k
// 2GB:   46340k
// 4GB:   65536k
// 8GB:   92681k
// 16GB:  131072k
//
// This allows larger machines to have larger/more transfers.
// Limit the default to 256M
//
// super.c
extern "C" {
    pub fn ceph_force_reconnect(sb: *mut super_block) -> c_int;
}
// snap.c
extern "C" {
    pub fn ceph_change_snap_realm(inode: *mut inode, realm: *mut ceph_snap_realm);
}
extern "C" {
    pub fn ceph_cleanup_global_and_empty_realms(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_trim_snapid_map(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_cleanup_snapid_map(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_umount_begin(sb: *mut super_block);
}
//
// a cap_snap is "pending" if it is still awaiting an in-progress
// sync write (that may/may not still update size, mtime, etc.).
//
// inode.c
extern "C" {
    pub fn ceph_evict_inode(inode: *mut inode);
}
extern "C" {
    pub fn ceph_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn ceph_inode_set_subvolume(inode: *mut inode, subvolume_id: u64);
}
extern "C" {
    pub fn ceph_inode_set_size(inode: *mut inode, size: loff_t) -> bool;
}
extern "C" {
    pub fn __ceph_do_pending_vmtruncate(inode: *mut inode);
}
extern "C" {
    pub fn ceph_queue_inode_work(inode: *mut inode, work_bit: c_int);
}
extern "C" {
    pub fn ceph_try_to_choose_auth_mds(inode: *mut inode, mask: c_int) -> c_int;
}
extern "C" {
    pub fn __ceph_do_getattr(_arg: inode, _arg: NULL, _arg: mask, _arg: force) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_iattr {
    pub fscrypt_auth: *mut ceph_fscrypt_auth,
}

extern "C" {
    pub fn ceph_inode_shutdown(inode: *mut inode);
}
// xattr.c
extern "C" {
    pub fn __ceph_setxattr(: *mut inode, : *const c_char, : *const c_void, _arg: usize, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ceph_do_getvxattr(inode: *mut inode, name: *const c_char, value: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn __ceph_getxattr(: *mut inode, : *const c_char, : *mut c_void, _arg: usize) -> isize;
}
extern "C" {
    pub fn ceph_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn __ceph_destroy_xattrs(ci: *mut ceph_inode_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_acl_sec_ctx {

    pub default_acl: *mut c_void,
    pub acl: *mut c_void,

    pub lsmctx: lsm_context,

    pub fscrypt_auth: *mut ceph_fscrypt_auth,

    pub pagelist: *mut ceph_pagelist,
}

extern "C" {
    pub fn ceph_security_xattr_deadlock(in: *mut inode) -> bool;
}
extern "C" {
    pub fn ceph_security_xattr_wanted(in: *mut inode) -> bool;
}

extern "C" {
    pub fn ceph_release_acl_sec_ctx(as_ctx: *mut ceph_acl_sec_ctx);
}
// acl.c

// caps.c
//
// Determine whether __ceph_remove_cap() has been called on this #cap
// (but the object has not yet been freed because it is protected by
// `ceph_mds_session.s_cap_iterator`).
//
// Caller must lock either `ceph_inode_info.i_ceph_lock` or
// `ceph_mds_session.s_cap_lock`.
//
extern "C" {
    pub fn RB_EMPTY_NODE(_arg: &cap->ci_node) -> return;
}
extern "C" {
    pub fn __ceph_remove_caps(ci: *mut ceph_inode_info);
}
extern "C" {
    pub fn ceph_is_any_caps(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ceph_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ceph_get_cap_refs(ci: *mut ceph_inode_info, caps: c_int);
}
extern "C" {
    pub fn ceph_put_cap_refs(ci: *mut ceph_inode_info, had: c_int);
}
extern "C" {
    pub fn ceph_put_cap_refs_async(ci: *mut ceph_inode_info, had: c_int);
}
extern "C" {
    pub fn __ceph_should_report_size(ci: *mut ceph_inode_info) -> bool;
}
extern "C" {
    pub fn ceph_check_caps(ci: *mut ceph_inode_info, flags: c_int);
}
extern "C" {
    pub fn ceph_check_delayed_caps(mdsc: *mut ceph_mds_client) -> c_ulong;
}
extern "C" {
    pub fn ceph_flush_dirty_caps(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_flush_cap_releases(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_drop_caps_for_unlink(inode: *mut inode) -> c_int;
}
// for counting open files by mode
extern "C" {
    pub fn ceph_get_fmode(ci: *mut ceph_inode_info, mode: c_int, count: c_int);
}
extern "C" {
    pub fn ceph_put_fmode(ci: *mut ceph_inode_info, mode: c_int, count: c_int);
}
// addr.c
extern "C" {
    pub fn ceph_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn ceph_uninline_data(file: *mut file) -> c_int;
}
extern "C" {
    pub fn ceph_pool_perm_check(inode: *mut inode, need: c_int) -> c_int;
}
extern "C" {
    pub fn ceph_pool_perm_destroy(mdsc: *mut *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_purge_inode_cap(inode: *mut inode, cap: *mut ceph_cap, invalidate: *mut bool) -> c_int;
}
// file.c
extern "C" {
    pub fn ceph_renew_caps(inode: *mut inode, fmode: c_int) -> c_int;
}
extern "C" {
    pub fn ceph_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn ceph_release(inode: *mut inode, filp: *mut file) -> c_int;
}
// dir.c
extern "C" {
    pub fn ceph_make_fpos(high: unsigned, off: unsigned, hash_order: bool) -> loff_t;
}
extern "C" {
    pub fn ceph_handle_notrace_create(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn __ceph_dentry_lease_touch(di: *mut ceph_dentry_info);
}
extern "C" {
    pub fn __ceph_dentry_dir_lease_touch(di: *mut ceph_dentry_info);
}
extern "C" {
    pub fn ceph_invalidate_dentry_lease(dentry: *mut dentry);
}
extern "C" {
    pub fn ceph_trim_dentries(mdsc: *mut ceph_mds_client) -> c_int;
}
extern "C" {
    pub fn ceph_dentry_hash(dir: *mut inode, dn: *mut dentry) -> unsigned;
}
extern "C" {
    pub fn ceph_readdir_cache_release(ctl: *mut ceph_readdir_cache_control);
}
// ioctl.c
extern "C" {
    pub fn ceph_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// export.c
// locks.c
extern "C" {
    pub fn ceph_flock_init() -> __init void;
}
extern "C" {
    pub fn ceph_lock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn ceph_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int;
}
extern "C" {
    pub fn ceph_count_locks(inode: *mut inode, p_num: *mut c_int, f_num: *mut c_int);
}
// debugfs.c
extern "C" {
    pub fn ceph_fs_debugfs_init(client: *mut ceph_fs_client);
}
extern "C" {
    pub fn ceph_fs_debugfs_cleanup(client: *mut ceph_fs_client);
}
// quota.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quota_get_realm {
    QUOTA_GET_MAX_FILES,
    QUOTA_GET_MAX_BYTES,
    QUOTA_GET_ANY
}

extern "C" {
    pub fn ceph_adjust_quota_realms_count(inode: *mut inode, inc: bool);
}
extern "C" {
    pub fn ceph_quota_is_max_files_exceeded(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn ceph_quota_is_same_realm(old: *mut inode, new: *mut inode) -> bool;
}
extern "C" {
    pub fn ceph_cleanup_quotarealms_inodes(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_dec_mds_stopping_blocker(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_inc_osd_stopping_blocker(mdsc: *mut ceph_mds_client) -> bool;
}
extern "C" {
    pub fn ceph_dec_osd_stopping_blocker(mdsc: *mut ceph_mds_client);
}
