//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/mds_client.h
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

// The first 8 bits are reserved for old ceph releases
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_feature_type {
    CEPHFS_FEATURE_MIMIC = 8,
    CEPHFS_FEATURE_REPLY_ENCODING,
    CEPHFS_FEATURE_RECLAIM_CLIENT,
    CEPHFS_FEATURE_LAZY_CAP_WANTED,
    CEPHFS_FEATURE_MULTI_RECONNECT,
    CEPHFS_FEATURE_DELEG_INO,
    CEPHFS_FEATURE_METRIC_COLLECT,
    CEPHFS_FEATURE_ALTERNATE_NAME,
    CEPHFS_FEATURE_NOTIFY_SESSION_STATE,
    CEPHFS_FEATURE_OP_GETVXATTR,
    CEPHFS_FEATURE_32BITS_RETRY_FWD,
    CEPHFS_FEATURE_NEW_SNAPREALM_INFO,
    CEPHFS_FEATURE_HAS_OWNER_UIDGID,
    CEPHFS_FEATURE_MDS_AUTH_CAPS_CHECK,
    CEPHFS_FEATURE_SUBVOLUME_METRICS,

    CEPHFS_FEATURE_MAX = CEPHFS_FEATURE_SUBVOLUME_METRICS,
}

//
// Some lock dependencies:
//
// session->s_mutex
// mdsc->mutex
//
// mdsc->snap_rwsem
//
// ci->i_ceph_lock
// mdsc->snap_flush_lock
// mdsc->cap_delay_lock
//

pub const CEPH_CAP_FLUSH_WAIT_TIMEOUT_SEC: c_int = 60;
pub const CEPH_CAP_FLUSH_MAX_DUMP_ENTRIES: c_int = 5;
pub const CEPH_CAP_FLUSH_MAX_DUMP_ITERS: c_int = 5;
pub const CEPH_CLIENT_RESET_REASON_LEN: c_int = 64;
pub const CEPH_CLIENT_RESET_DRAIN_SEC: c_int = 30;
pub const CEPH_CLIENT_RESET_CLOSE_GRACE_MS: c_int = 100;
pub const CEPH_CLIENT_RESET_WAIT_TIMEOUT_SEC: c_int = 120;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_client_reset_phase {
    CEPH_CLIENT_RESET_IDLE = 0,
//
// QUIESCING is set synchronously by schedule_reset() before the
// workqueue item is dispatched.  It gates new requests (any
// phase != IDLE blocks callers) during the window between
// scheduling and the work function's transition to DRAINING.
//
    CEPH_CLIENT_RESET_QUIESCING,
    CEPH_CLIENT_RESET_DRAINING,
    CEPH_CLIENT_RESET_TEARDOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_client_reset_state {
    pub /: *mut *mut spinlock_t lock; / protects all fields below,
    pub /: *mut *mut u64 trigger_count; / number of resets triggered,
    pub /: *mut *mut u64 success_count; / number of successful resets,
    pub /: *mut *mut u64 failure_count; / number of failed resets,
    pub /: *mut *mut unsigned long last_start; / jiffies when last reset started,
    pub /: *mut *mut unsigned long last_finish; / jiffies when last reset finished,
    pub /: *mut *mut int last_errno; / result of most recent reset,
    pub /: *mut *mut ceph_client_reset_phase phase; / current reset phase,
    pub /: *mut *mut bool drain_timed_out; / drain exceeded timeout,
    pub /: *mut *mut bool shutdown; / destroy in progress,
    pub /: *mut *mut int sessions_reset; / sessions torn down in last reset,
    pub /: *mut *mut char last_reason[CEPH_CLIENT_RESET_REASON_LEN]; / operator-supplied reason,
// Request blocking during reset
    pub /: *mut *mut wait_queue_head_t blocked_wq; / waitqueue for blocked callers,
    pub /: *mut *mut atomic_t blocked_requests; / count of blocked callers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_match {
    pub /: *mut *mut s64 uid; / default to MDS_AUTH_UID_ANY,
    pub num_gids: u32,
    pub /: *mut *mut *mut u32 gids; / use these GIDs,
    pub this: *mut *mut *mut char path; / require path to be child of,
    pub fs_name: *mut c_char,
    pub /: *mut *mut bool root_squash; / default to false,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_auth {
    pub match: ceph_mds_cap_match,
    pub readable: bool,
    pub writeable: bool,
}

//
// parsed info about a single inode.  pointers are into the encoded
// on-wire structures within the mds reply message payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_info_in {
    pub in: *mut ceph_mds_reply_inode,
    pub dir_layout: ceph_dir_layout,
    pub symlink_len: u32,
    pub symlink: *mut c_char,
    pub xattr_len: u32,
    pub xattr_data: *mut c_char,
    pub inline_version: u64,
    pub inline_len: u32,
    pub inline_data: *mut c_char,
    pub pool_ns_len: u32,
    pub pool_ns_data: *mut c_char,
    pub max_bytes: u64,
    pub max_files: u64,
    pub dir_pin: i32,
    pub btime: ceph_timespec,
    pub snap_btime: ceph_timespec,
    pub fscrypt_auth: *mut u8,
    pub fscrypt_file: *mut u8,
    pub fscrypt_auth_len: u32,
    pub fscrypt_file_len: u32,
    pub rsnaps: u64,
    pub change_attr: u64,
    pub subvolume_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_dir_entry {
    pub is_nokey: bool,
    pub name: *mut c_char,
    pub name_len: u32,
    pub raw_hash: u32,
    pub lease: *mut ceph_mds_reply_lease,
    pub inode: ceph_mds_reply_info_in,
    pub offset: loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_xattr {
    pub xattr_value: *mut c_char,
    pub xattr_value_len: usize,
}

//
// parsed info about an mds reply, including information about
// either: 1) the target inode and/or its parent directory and dentry,
// and directory contents (for readdir results), or
// 2) the file range lock info (for fcntl F_GETLK results).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_info_parsed {
    pub head: *mut ceph_mds_reply_head,
// trace
    pub targeti: ceph_mds_reply_info_in diri,,
    pub dirfrag: *mut ceph_mds_reply_dirfrag,
    pub dname: *mut c_char,
    pub altname: *mut u8,
    pub dname_len: u32,
    pub altname_len: u32,
    pub dlease: *mut ceph_mds_reply_lease,
    pub xattr_info: ceph_mds_reply_xattr,
// extra
// for fcntl F_GETLK results
    pub filelock_reply: *mut ceph_filelock,
// for readdir results
    pub dir_dir: *mut ceph_mds_reply_dirfrag,
    pub dir_buf_size: usize,
    pub dir_nr: c_int,
    pub dir_end: bool,
    pub dir_complete: bool,
    pub hash_order: bool,
    pub offset_hash: bool,
    pub dir_entries: *mut ceph_mds_reply_dir_entry,
}

// for create results
// encoded blob describing snapshot contexts for certain
//
// cap releases are batched and sent to the MDS en masse.
//
// Account for per-message overhead of mds_cap_release header
// and __le32 for osd epoch barrier trailing field.
//

//
// state associated with each MDS<->client session
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_session {
    pub s_mdsc: *mut ceph_mds_client,
    pub s_mds: c_int,
    pub s_state: c_int,
    pub /: *mut *mut unsigned long s_ttl; / time until mds kills us,
    pub s_features: c_ulong,
    pub /: *mut *mut u64 s_seq; / incoming msg seq #,
    pub /: *mut *mut mutex s_mutex; / serialize session messages,
    pub s_con: ceph_connection,
    pub s_auth: ceph_auth_handshake,
    pub /: *mut *mut atomic_t s_cap_gen; / inc each time we get mds stale msg,
    pub /: *mut *mut unsigned long s_cap_ttl; / when session caps expire. protected by s_mutex,
// protected by s_cap_lock
    pub s_cap_lock: spinlock_t,
    pub s_ref: refcount_t,
    pub /: *mut *mut list_head s_caps; / all caps issued by this session,
    pub s_cap_iterator: *mut ceph_cap,
    pub s_nr_caps: c_int,
    pub s_num_cap_releases: c_int,
    pub s_cap_reconnect: c_int,
    pub s_readonly: c_int,
    pub /: *mut *mut list_head s_cap_releases; / waiting cap_release messages,
    pub s_cap_release_work: work_struct,
// See ceph_inode_info->i_dirty_item.
    pub /: *mut *mut list_head s_cap_dirty; / inodes w/ dirty caps,
// See ceph_inode_info->i_flushing_item.
    pub /: *mut *mut list_head s_cap_flushing; / inodes w/ flushing caps,
    pub /: *mut *mut unsigned long s_renew_requested; / last time we sent a renew req,
    pub s_renew_seq: u64,
    pub /: *mut *mut list_head s_waiting; / waiting requests,
    pub /: *mut *mut list_head s_unsafe; / unsafe requests,
    pub s_delegated_inos: xarray,
    pub s_num_deleg_inos: core::sync::atomic::AtomicI32,
}

//
// modes of choosing which MDS to send a request to
//
// request completion callback
//
// wait for request completion callback
//
// an in-flight mds request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_request {
    pub /: *mut *mut u64 r_tid; / transaction id,
    pub r_node: rb_node,
    pub r_mdsc: *mut ceph_mds_client,
    pub r_kref: kref,
    pub /: *mut *mut int r_op; / mds op code,
// operation on what?
    pub /: *mut *mut *mut inode r_inode; / arg1,
    pub /: *mut *mut *mut dentry r_dentry; / arg1,
    pub /: *mut *mut *mut dentry r_old_dentry; / arg2: rename from or link from,
    pub /: *mut *mut *mut inode r_old_dentry_dir; / arg2: old dentry's parent dir,
    pub r_path2: *mut *mut char r_path1,,
    pub r_ino2: ceph_vino r_ino1,,
    pub /: *mut *mut *mut inode r_parent; / parent dir inode,
    pub /: *mut *mut *mut inode r_target_inode; / resulting inode,
    pub /: *mut *mut *mut inode r_new_inode; / new inode (for creates),
    pub /: *const *const *const qstr r_dname; / stable name (for ->d_revalidate),

    pub r_req_flags: c_ulong,
    pub r_fill_mutex: mutex,
    pub r_args: ceph_mds_request_args,
    pub r_fscrypt_auth: *mut ceph_fscrypt_auth,
    pub r_fscrypt_file: u64,
    pub /: *mut *mut *mut u8 r_altname; / fscrypt binary crypttext for long filenames,
    pub /: *mut *mut u32 r_altname_len; / length of r_altname,
    pub /: *mut *mut int r_fmode; / file mode, if expecting cap,
    pub r_request_release_offset: c_int,
    pub r_cred: *const cred,
    pub r_mnt_idmap: *mut mnt_idmap,
    pub r_stamp: timespec64,
// for choosing which mds to send this request to
    pub r_direct_mode: c_int,
    pub /: *mut *mut u32 r_direct_hash; / choose dir frag based on this dentry hash,
// data payload is used for xattr ops
    pub r_pagelist: *mut ceph_pagelist,
// what caps shall we drop?
    pub r_inode_unless: int r_inode_drop,,
    pub r_dentry_unless: int r_dentry_drop,,
    pub r_old_dentry_unless: int r_old_dentry_drop,,
    pub r_old_inode: *mut inode,
    pub r_old_inode_unless: int r_old_inode_drop,,
    pub /: *mut *mut *mut ceph_msg r_request; / original request,
    pub r_reply: *mut ceph_msg,
    pub r_reply_info: ceph_mds_reply_info_parsed,
    pub r_err: c_int,
    pub r_readdir_offset: u32,
    pub r_locked_page: *mut page,
    pub r_dir_caps: c_int,
    pub r_num_caps: c_int,
    pub /: *mut *mut unsigned long r_timeout; / optional. jiffies, 0 is "wait forever",
    pub /: *mut *mut unsigned long r_started; / start time to measure timeout against,
    pub /: *mut *mut unsigned long r_start_latency; / start time to measure latency,
    pub /: *mut *mut unsigned long r_end_latency; / finish time to measure latency,
    pub only,: *mut *mut unsigned long r_request_started; / start time for mds request,
// link unsafe requests to parent directory, for fsync
    pub r_unsafe_dir: *mut inode,
    pub r_unsafe_dir_item: list_head,
// unsafe requests that modify the target inode
    pub r_unsafe_target_item: list_head,
    pub r_session: *mut ceph_mds_session,
    pub /: *mut *mut int r_attempts; / resend attempts,
    pub /: *mut *mut int r_num_fwd; / number of forward attempts,
    pub any*/: *mut *mut int r_resend_mds; / mds to resend to next, if,
    pub at*/: *mut *mut u32 r_sent_on_mseq; / cap mseq request was sent,
    pub r_deleg_ino: u64,
    pub r_wait: list_head,
    pub r_completion: completion,
    pub r_safe_completion: completion,
    pub r_callback: ceph_mds_request_callback_t,
    pub /: *mut *mut list_head r_unsafe_item; / per-session unsafe list item,
    pub r_dir_release_cnt: c_longlong,
    pub r_dir_ordered_cnt: c_longlong,
    pub r_readdir_cache_idx: c_int,
    pub r_feature_needed: c_int,
    pub r_caps_reservation: ceph_cap_reservation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pool_perm {
    pub node: rb_node,
    pub perm: c_int,
    pub pool: i64,
    pub pool_ns_len: usize,
    pub pool_ns: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_snapid_map {
    pub node: rb_node,
    pub lru: list_head,
    pub ref: core::sync::atomic::AtomicI32,
    pub dev: dev_t,
    pub snap: u64,
    pub last_used: c_ulong,
}

//
// node for list of quotarealm inodes that are not visible from the filesystem
// mountpoint, but required to handle, e.g. quotas.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_quotarealm_inode {
    pub node: rb_node,
    pub ino: u64,
    pub /: *mut *mut unsigned long timeout; / last time a lookup failed for this inode,
    pub mutex: mutex,
    pub inode: *mut inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cap_wait {
    pub list: list_head,
    pub ino: u64,
    pub tgid: pid_t,
    pub need: c_int,
    pub want: c_int,
}

//
// mds client state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_client {
    pub fsc: *mut ceph_fs_client,
    pub /: *mut *mut mutex mutex; / all nested structures,
    pub mdsmap: *mut ceph_mdsmap,
    pub safe_umount_waiters: completion,
    pub session_close_wq: wait_queue_head_t,
    pub waiting_for_map: list_head,
    pub mdsmap_err: c_int,
    pub /: *mut *mut *mut *mut ceph_mds_session sessions; / NULL for mds if no session,
    pub num_sessions: core::sync::atomic::AtomicI32,
    pub /: *mut *mut int max_sessions; / len of sessions array,
    pub /: *mut *mut spinlock_t stopping_lock; / protect snap_empty,
    pub /: *mut *mut int stopping; / the stage of shutting down,
    pub stopping_blockers: core::sync::atomic::AtomicI32,
    pub stopping_waiter: completion,
    pub dirty_folios: core::sync::atomic::AtomicI64,
    pub flush_end_wq: wait_queue_head_t,
    pub /: *mut *mut atomic64_t quotarealms_count; / # realms with quota,
//
// We keep a list of inodes we don't see in the mountpoint but that we
// need to track quota realms.
//
    pub quotarealms_inodes: rb_root,
    pub quotarealms_inodes_mutex: mutex,
//
// snap_rwsem will cover cap linkage into snaprealms, and
// realm snap contexts.  (later, we can do per-realm snap
// contexts locks..)  the empty list contains realms with no
// references (implying they contain no inodes with caps) that
// should be destroyed.
//
    pub last_snap_seq: u64,
    pub snap_rwsem: rw_semaphore,
    pub snap_realms: rb_root,
    pub snap_empty: list_head,
    pub num_snap_realms: c_int,
    pub /: *mut *mut spinlock_t snap_empty_lock; / protect snap_empty,
    pub /: *mut *mut u64 last_tid; / most recent mds request,
    pub request,: *mut *mut u64 oldest_tid; / oldest incomplete mds,
    pub /: *mut *mut rb_root request_tree; / pending mds requests,
    pub /: *mut *mut delayed_work delayed_work; / delayed work,
    pub /: *mut *mut unsigned long last_renew_caps; / last time we renewed our caps,
    pub /: *mut *mut list_head cap_delay_list; / caps with delayed release,
    pub /: *mut *mut list_head cap_unlink_delay_list; / caps with delayed release for unlink,
    pub /: *mut *mut spinlock_t cap_delay_lock; / protects cap_delay_list and cap_unlink_delay_list,
    pub /: *mut *mut list_head snap_flush_list; / cap_snaps ready to flush,
    pub snap_flush_lock: spinlock_t,
    pub last_cap_flush_tid: u64,
    pub cap_flush_list: list_head,
    pub /: *mut *mut list_head cap_dirty_migrating; / ...that are migration...,
    pub /: *mut *mut int num_cap_flushing; / # caps we are flushing,
    pub /: *mut *mut spinlock_t cap_dirty_lock; / protects above items,
    pub cap_flushing_wq: wait_queue_head_t,
    pub cap_reclaim_work: work_struct,
    pub cap_reclaim_pending: core::sync::atomic::AtomicI32,
    pub cap_unlink_work: work_struct,
//
// Cap reservations
//
// Maintain a global pool of preallocated struct ceph_caps, referenced
// by struct ceph_caps_reservations.  This ensures that we preallocate
// memory needed to successfully process an MDS response.  (If an MDS
// sends us cap information and we fail to process it, we will have
// problems due to the client and MDS being out of sync.)
//
// Reservations are 'owned' by a ceph_cap_reservation context.
//
    pub caps_list_lock: spinlock_t,
    pub or: *mut *mut list_head caps_list; / unused (reserved,

    pub cap_wait_list: list_head,

    pub /: *mut *mut int caps_total_count; / total caps allocated,
    pub /: *mut *mut int caps_use_count; / in use,
    pub /: *mut *mut int caps_use_max; / max used caps,
    pub /: *mut *mut int caps_reserve_count; / unused, reserved,
    pub /: *mut *mut int caps_avail_count; / unused, unreserved,
    pub many: *mut *mut int caps_min_count; / keep at least this,
    pub dentry_list_lock: spinlock_t,
    pub /: *mut *mut list_head dentry_leases; / fifo list,
    pub /: *mut *mut list_head dentry_dir_leases; / lru list,
    pub metric: ceph_client_metric,
    pub reset_work: work_struct,
    pub reset_state: ceph_client_reset_state,
    pub subvol_metrics: ceph_subvolume_metrics_tracker,
// Subvolume metrics send tracking
    pub subvol_metrics_last_mutex: mutex,
    pub subvol_metrics_last: *mut ceph_subvol_metric_snapshot,
    pub subvol_metrics_last_nr: u32,
    pub subvol_metrics_sent: u64,
    pub subvol_metrics_nonzero_sends: u64,
    pub snapid_map_lock: spinlock_t,
    pub snapid_map_tree: rb_root,
    pub snapid_map_lru: list_head,
    pub pool_perm_rwsem: rw_semaphore,
    pub pool_perm_tree: rb_root,
// protected by mutex
    pub s_cap_auths_num: u32,
    pub s_cap_auths: *mut ceph_mds_cap_auth,
    pub 1]: char nodename[__NEW_UTS_LEN +,
}

extern "C" {
    pub fn check_session_state(s: *mut ceph_mds_session) -> bool;
}
extern "C" {
    pub fn inc_session_sequence(s: *mut ceph_mds_session);
}
extern "C" {
    pub fn ceph_put_mds_session(s: *mut ceph_mds_session);
}
extern "C" {
    pub fn ceph_mdsc_wait_for_reset(mdsc: *mut ceph_mds_client) -> c_int;
}
extern "C" {
    pub fn ceph_mdsc_init(fsc: *mut ceph_fs_client) -> c_int;
}
extern "C" {
    pub fn ceph_mdsc_close_sessions(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_mdsc_force_umount(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_mdsc_destroy(fsc: *mut ceph_fs_client);
}
extern "C" {
    pub fn ceph_mdsc_sync(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_invalidate_dir_request(req: *mut ceph_mds_request);
}
extern "C" {
    pub fn ceph_mdsc_release_dir_caps(req: *mut ceph_mds_request);
}
extern "C" {
    pub fn ceph_mdsc_release_dir_caps_async(req: *mut ceph_mds_request);
}
extern "C" {
    pub fn ceph_mdsc_release_request(kref: *mut kref);
}
extern "C" {
    pub fn send_flush_mdlog(s: *mut ceph_mds_session);
}
extern "C" {
    pub fn ceph_queue_cap_reclaim_work(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_reclaim_caps_nr(mdsc: *mut ceph_mds_client, nr: c_int);
}
extern "C" {
    pub fn ceph_queue_cap_unlink_work(mdsc: *mut ceph_mds_client);
}
extern "C" {
    pub fn ceph_mdsc_pre_umount(mdsc: *mut ceph_mds_client);
}
//
// Structure to group path-related output parameters for build_*_path functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_path_info {
    pub path: *const c_char,
    pub pathlen: c_int,
    pub vino: ceph_vino,
    pub freepath: bool,
}

extern "C" {
    pub fn __ceph_mdsc_drop_dentry_lease(dentry: *mut dentry);
}
extern "C" {
    pub fn ceph_wait_on_conflict_unlink(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ceph_get_deleg_ino(session: *mut ceph_mds_session) -> u64;
}
extern "C" {
    pub fn ceph_restore_deleg_ino(session: *mut ceph_mds_session, ino: u64) -> c_int;
}
