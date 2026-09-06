//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/ceph_fs.h
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
//
// ceph_fs.h - Ceph constants and data types to share between kernel and
// user space.
//
// Most types in this file are defined as little-endian, and are
// primarily intended to describe data structures that pass over the
// wire or that are stored on disk.
//
// LGPL2
//

//
// subprotocol versions.  when specific messages types or high-level
// protocols change, bump the affected components.  we keep rev
// internal cluster protocols separately from the public,
// client-facing protocol.
//

pub const CEPH_INO_ROOT: c_int = 1;

//
// name for "old" CephFS file systems,
// see ceph.git e2b151d009640114b2565c901d6f41f6cd5ec652
//

// arbitrary limit on max # of monitors (cluster of 3 is typical)
pub const CEPH_MAX_MON: c_int = 31;
//
// legacy ceph_file_layoute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_file_layout_legacy {
// file -> object mapping
    pub multiple: *mut *mut __le32 fl_stripe_unit; / stripe unit, in bytes. must be,
    pub /: *mut *mut __le32 fl_stripe_count; / over this many objects,
    pub to: *mut *mut __le32 fl_object_size; / until objects are this big, then move,
    pub /: *mut *mut __le32 fl_cas_hash; / UNUSED. 0 = none; 1 = sha256,
// pg -> disk layout
    pub /: *mut *mut __le32 fl_object_stripe_unit; / UNUSED. for per-object parity, if any,
// object -> pg layout
    pub /: *mut *mut __le32 fl_unused; / unused; used to be preferred primary for pg (-1 for none),
    pub /: *mut *mut __le32 fl_pg_pool; / namespace, crush ruleset, rep level,
// C attribute field omitted
    pub ceph_string: struct,
//
// ceph_file_layout - describe data layout for a file/inode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_file_layout {
// file -> object mapping
    pub /: *mut *mut u32 stripe_unit; / stripe unit, in bytes,
    pub /: *mut *mut u32 stripe_count; / over this many objects,
    pub /: *mut *mut u32 object_size; / until objects are this big,
    pub /: *mut *mut s64 pool_id; / rados pool id,
    pub /: *mut *mut *mut ceph_string __rcu pool_ns; / rados pool namespace,
}

extern "C" {
    pub fn ceph_file_layout_is_valid(layout: *const ceph_file_layout) -> c_int;
}
pub const CEPH_MIN_STRIPE_UNIT: c_int = 65536;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_dir_layout {
    pub /: *mut *mut __u8 dl_dir_hash; / see ceph_hash.h for ids,
    pub dl_unused1: __u8,
    pub dl_unused2: __u16,
    pub dl_unused3: __u32,
// C attribute field omitted
// crypto algorithms
pub const CEPH_CRYPTO_NONE: c_uint = 0x0;
pub const CEPH_CRYPTO_AES: c_uint = 0x1;
pub const CEPH_CRYPTO_AES256KRB5: c_uint = 0x2 /* AES256-CTS-HMAC384-192 */;

// security/authentication protocols
pub const CEPH_AUTH_UNKNOWN: c_uint = 0x0;
pub const CEPH_AUTH_NONE: c_uint = 0x1;
pub const CEPH_AUTH_CEPHX: c_uint = 0x2;
pub const CEPH_AUTH_MODE_NONE: c_int = 0;
pub const CEPH_AUTH_MODE_AUTHORIZER: c_int = 1;
pub const CEPH_AUTH_MODE_MON: c_int = 10;
// msgr2 protocol modes
pub const CEPH_CON_MODE_UNKNOWN: c_uint = 0x0;
pub const CEPH_CON_MODE_CRC: c_uint = 0x1;
pub const CEPH_CON_MODE_SECURE: c_uint = 0x2;

    pub proto): *const *const char ceph_auth_proto_name(int,
    pub mode): *const *const char ceph_con_mode_name(int,
//
// message layer
//
// message types
//
// misc
pub const CEPH_MSG_SHUTDOWN: c_int = 1;
pub const CEPH_MSG_PING: c_int = 2;
// client <-> monitor
pub const CEPH_MSG_MON_MAP: c_int = 4;
pub const CEPH_MSG_MON_GET_MAP: c_int = 5;
pub const CEPH_MSG_STATFS: c_int = 13;
pub const CEPH_MSG_STATFS_REPLY: c_int = 14;
pub const CEPH_MSG_MON_SUBSCRIBE: c_int = 15;
pub const CEPH_MSG_MON_SUBSCRIBE_ACK: c_int = 16;
pub const CEPH_MSG_AUTH: c_int = 17;
pub const CEPH_MSG_AUTH_REPLY: c_int = 18;
pub const CEPH_MSG_MON_GET_VERSION: c_int = 19;
pub const CEPH_MSG_MON_GET_VERSION_REPLY: c_int = 20;
// client <-> mds
pub const CEPH_MSG_MDS_MAP: c_int = 21;
pub const CEPH_MSG_FS_MAP_USER: c_int = 103;
pub const CEPH_MSG_CLIENT_SESSION: c_int = 22;
pub const CEPH_MSG_CLIENT_RECONNECT: c_int = 23;
pub const CEPH_MSG_CLIENT_REQUEST: c_int = 24;
pub const CEPH_MSG_CLIENT_REQUEST_FORWARD: c_int = 25;
pub const CEPH_MSG_CLIENT_REPLY: c_int = 26;
pub const CEPH_MSG_CLIENT_METRICS: c_int = 29;
pub const CEPH_MSG_CLIENT_CAPS: c_uint = 0x310;
pub const CEPH_MSG_CLIENT_LEASE: c_uint = 0x311;
pub const CEPH_MSG_CLIENT_SNAP: c_uint = 0x312;
pub const CEPH_MSG_CLIENT_CAPRELEASE: c_uint = 0x313;
pub const CEPH_MSG_CLIENT_QUOTA: c_uint = 0x314;
// pool ops
pub const CEPH_MSG_POOLOP_REPLY: c_int = 48;
pub const CEPH_MSG_POOLOP: c_int = 49;
// mon commands
pub const CEPH_MSG_MON_COMMAND: c_int = 50;
pub const CEPH_MSG_MON_COMMAND_ACK: c_int = 51;
// osd
pub const CEPH_MSG_OSD_MAP: c_int = 41;
pub const CEPH_MSG_OSD_OP: c_int = 42;
pub const CEPH_MSG_OSD_OPREPLY: c_int = 43;
pub const CEPH_MSG_WATCH_NOTIFY: c_int = 44;
pub const CEPH_MSG_OSD_BACKOFF: c_int = 61;
// watch-notify operations
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_request_header {
    pub have_version: __le64,
    pub session_mon: __le16,
    pub session_mon_tid: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_statfs {
    pub monhdr: ceph_mon_request_header,
    pub fsid: ceph_fsid,
    pub contains_data_pool: __u8,
    pub data_pool: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_statfs {
    pub kb_avail: __le64 kb, kb_used,,
    pub num_objects: __le64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_statfs_reply {
    pub fsid: ceph_fsid,
    pub version: __le64,
    pub st: ceph_statfs,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_command {
    pub monhdr: ceph_mon_request_header,
    pub fsid: ceph_fsid,
    pub /: *mut *mut __le32 num_strs; / always 1,
    pub str_len: __le32,
    pub str: [c_char; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_getmap {
    pub monhdr: ceph_mon_request_header,
    pub fsid: ceph_fsid,
    pub start: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_getmap {
    pub monhdr: ceph_mon_request_header,
    pub fsid: ceph_fsid,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_client_mount {
    pub monhdr: ceph_mon_request_header,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_subscribe_item {
    pub start: __le64,
    pub flags: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_subscribe_ack {
    pub /: *mut *mut __le32 duration; / seconds,
    pub fsid: ceph_fsid,
// C attribute field omitted

//
// mdsmap flags
//

//
// mds states
// > 0 -> in
// <= 0 -> out
//

    pub s): *const *const extern char ceph_mds_state_name(int,
//
// metadata lock types.
// - these are bitmasks.. we can compose them
// - they also define the lock ordering by the MDS
// - a few of these are internal to the mds
//
pub const CEPH_LOCK_DVERSION: c_int = 1;
pub const CEPH_LOCK_DN: c_int = 2;
pub const CEPH_LOCK_ISNAP: c_int = 16;

pub const CEPH_LOCK_IFILE: c_int = 64;
pub const CEPH_LOCK_IAUTH: c_int = 128;
pub const CEPH_LOCK_ILINK: c_int = 256;

pub const CEPH_LOCK_IXATTR: c_int = 2048;

// client_session ops
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_session_head {
    pub op: __le32,
    pub seq: __le64,
    pub stamp: ceph_timespec,
    pub max_leases: __le32 max_caps,,
// C attribute field omitted
// client_request
//
// metadata ops.
// & 0x001000 -> write op
// & 0x010000 -> follow symlink (e.g. stat(), not lstat()).
//
pub const CEPH_MDS_OP_WRITE: c_uint = 0x001000;
}

//
// Ceph setxattr request flags.
//

//
// readdir request flags;
//

//
// readdir reply flags.
//

//
// open request flags
//
pub const CEPH_O_RDONLY: c_int = 00000000;
pub const CEPH_O_WRONLY: c_int = 00000001;
pub const CEPH_O_RDWR: c_int = 00000002;
pub const CEPH_O_CREAT: c_int = 00000100;
pub const CEPH_O_EXCL: c_int = 00000200;
pub const CEPH_O_TRUNC: c_int = 00001000;
pub const CEPH_O_DIRECTORY: c_int = 00200000;
pub const CEPH_O_NOFOLLOW: c_int = 00400000;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ceph_mds_request_args {
    pub /: *mut *mut *mut __le32 mask; / CEPH_CAP_,
// C attribute field omitted
    pub mode: __le32,
    pub uid: __le32,
    pub gid: __le32,
    pub mtime: ceph_timespec,
    pub atime: ceph_timespec,
    pub /: *mut *mut __le64 size, old_size; / old_size needed by truncate,
    pub /: *mut *mut *mut __le32 mask; / CEPH_SETATTR_,
// C attribute field omitted
    pub /: *mut *mut __le32 frag; / which dir fragment,
    pub /: *mut *mut __le32 max_entries; / how many dentries to grab,
    pub max_bytes: __le32,
    pub flags: __le16,
    pub offset_hash: __le32,
// C attribute field omitted
    pub mode: __le32,
    pub rdev: __le32,
// C attribute field omitted
    pub mode: __le32,
// C attribute field omitted
    pub flags: __le32,
    pub mode: __le32,
    pub /: *mut *mut __le32 stripe_unit; / layout for newly created file,
    pub /: *mut *mut __le32 stripe_count; / ...,
    pub object_size: __le32,
    pub pool: __le32,
    pub /: *mut *mut *mut __le32 mask; / CEPH_CAP_,
    pub old_size: __le64,
// C attribute field omitted
    pub flags: __le32,
    pub /: *mut *mut __le32 osdmap_epoch; / used for setting file/dir layouts,
// C attribute field omitted
    pub layout: ceph_file_layout_legacy,
// C attribute field omitted
    pub /: *mut *mut __u8 rule; / currently fcntl or flock,
    pub remove*/: *mut *mut __u8 type; / shared, exclusive,,
    pub /: *mut *mut __le64 owner; / owner of the lock,
    pub /: *mut *mut __le64 pid; / process id requesting the lock,
    pub /: *mut *mut __le64 start; / initial location to lock,
    pub /: *mut *mut __le64 length; / num bytes to lock from start,
    pub /: *mut *mut __u8 wait; / will caller wait for lock to become available?,
// C attribute field omitted
    pub /: *mut *mut *mut __le32 mask; / CEPH_CAP_,
    pub snapid: __le64,
    pub parent: __le64,
    pub hash: __le32,
// C attribute field omitted
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union ceph_mds_request_args_ext {
    pub old: ceph_mds_request_args,
    pub mode: __le32,
    pub uid: __le32,
    pub gid: __le32,
    pub mtime: ceph_timespec,
    pub atime: ceph_timespec,
    pub /: *mut *mut __le64 size, old_size; / old_size needed by truncate,
    pub /: *mut *mut *mut __le32 mask; / CEPH_SETATTR_,
    pub btime: ceph_timespec,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_request_head_legacy {
    pub oldest_client_tid: __le64,
    pub /: *mut *mut __le32 mdsmap_epoch; / on client,
    pub /: *mut *mut *mut __le32 flags; / CEPH_MDS_FLAG_,
    pub /: *mut *mut __u8 num_retry, num_fwd; / count retry, fwd attempts,
    pub /: *mut *mut __le16 num_releases; / # include cap/lease release records,
    pub /: *mut *mut __le32 op; / mds op code,
    pub caller_gid: __le32 caller_uid,,
    pub mknod,: *mut *mut __le64 ino; / use this ino for openc, mkdir,,
    pub args: ceph_mds_request_args,
// C attribute field omitted
pub const CEPH_MDS_REQUEST_HEAD_VERSION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_request_head {
    pub /: *mut *mut __le16 version; / struct version,
    pub oldest_client_tid: __le64,
    pub /: *mut *mut __le32 mdsmap_epoch; / on client,
    pub /: *mut *mut *mut __le32 flags; / CEPH_MDS_FLAG_,
    pub /: *mut *mut __u8 num_retry, num_fwd; / legacy count retry and fwd attempts,
    pub /: *mut *mut __le16 num_releases; / # include cap/lease release records,
    pub /: *mut *mut __le32 op; / mds op code,
    pub caller_gid: __le32 caller_uid,,
    pub mknod,: *mut *mut __le64 ino; / use this ino for openc, mkdir,,
    pub args: ceph_mds_request_args_ext,
    pub /: *mut *mut __le32 ext_num_retry; / new count retry attempts,
    pub /: *mut *mut __le32 ext_num_fwd; / new count fwd attempts,
    pub /: *mut *mut __le32 struct_len; / to store size of struct ceph_mds_request_head,
    pub /: *mut *mut __le32 owner_uid, owner_gid; / used for OPs which create inodes,
// C attribute field omitted
// cap/lease release record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_request_release {
    pub /: *mut *mut __le64 ino, cap_id; / ino and unique cap id,
    pub /: *mut *mut __le32 caps, wanted; / new issued, wanted,
    pub mseq: __le32 seq, issue_seq,,
    pub /: *mut *mut __le32 dname_seq; / if releasing a dentry lease, a,
    pub /: *mut *mut __le32 dname_len; / string follows.,
// C attribute field omitted
// client reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_head {
    pub op: __le32,
    pub result: __le32,
    pub mdsmap_epoch: __le32,
    pub /: *mut *mut __u8 safe; / true if committed to disk,
    pub records: *mut *mut __u8 is_dentry, is_target; / true if dentry, target inode,
// C attribute field omitted
// one for each node split
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_frag_tree_split {
    pub /: *mut *mut __le32 frag; / this frag splits...,
    pub /: *mut *mut __le32 by; / ...by this many bits,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_frag_tree_head {
    pub /: *mut *mut __le32 nsplits; / num ceph_frag_tree_split records,
    pub splits: [ceph_frag_tree_split; ],
// C attribute field omitted
// capability issue, for bundling with mds reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_cap {
    pub /: *mut *mut __le32 caps, wanted; / caps issued, wanted,
    pub cap_id: __le64,
    pub mseq: __le32 seq,,
    pub /: *mut *mut __le64 realm; / snap realm,
    pub /: *mut *mut *mut __u8 flags; / CEPH_CAP_FLAG_,
// C attribute field omitted

// inode record, for bundling with mds reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_inode {
    pub ino: __le64,
    pub snapid: __le64,
    pub rdev: __le32,
    pub /: *mut *mut __le64 version; / inode version,
    pub /: *mut *mut __le64 xattr_version; / version for xattr blob,
    pub /: *mut *mut ceph_mds_reply_cap cap; / caps issued for this inode,
    pub layout: ceph_file_layout_legacy,
    pub atime: ceph_timespec ctime, mtime,,
    pub time_warp_seq: __le32,
    pub truncate_size: __le64 size, max_size,,
    pub truncate_seq: __le32,
    pub gid: __le32 mode, uid,,
    pub nlink: __le32,
    pub /: *mut *mut __le64 files, subdirs, rbytes, rfiles, rsubdirs; / dir stats,
    pub rctime: ceph_timespec,
    pub /: *mut *mut ceph_frag_tree_head fragtree; / (must be at end of struct),
// C attribute field omitted
// followed by frag array, symlink string, dir layout, xattr blob
// reply_lease follows dname, and reply_inode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_lease {
    pub /: *mut *mut __le16 mask; / lease type(s),
    pub /: *mut *mut __le32 duration_ms; / lease duration,
    pub seq: __le32,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_reply_dirfrag {
    pub /: *mut *mut __le32 frag; / fragment,
    pub /: *mut *mut __le32 auth; / auth mds, if this is a delegation point,
    pub /: *mut *mut __le32 ndist; / number of mds' this is replicated on,
    pub dist: [__le32; ],
// C attribute field omitted
pub const CEPH_LOCK_FCNTL: c_int = 1;
pub const CEPH_LOCK_FLOCK: c_int = 2;
pub const CEPH_LOCK_FCNTL_INTR: c_int = 3;
pub const CEPH_LOCK_FLOCK_INTR: c_int = 4;
pub const CEPH_LOCK_SHARED: c_int = 1;
pub const CEPH_LOCK_EXCL: c_int = 2;
pub const CEPH_LOCK_UNLOCK: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_filelock {
    pub /: *mut *mut __le64 start;/ file offset to start lock at,
    pub /: *mut *mut __le64 length; / num bytes to lock; 0 for all following start,
    pub /: *mut *mut __le64 client; / which client holds the lock,
    pub /: *mut *mut __le64 owner; / owner the lock,
    pub /: *mut *mut __le64 pid; / process id holding the lock on the client,
    pub /: *mut *mut __u8 type; / shared lock, exclusive lock, or unlock,
// C attribute field omitted
// file access modes
pub const CEPH_FILE_MODE_PIN: c_int = 0;
pub const CEPH_FILE_MODE_RD: c_int = 1;
pub const CEPH_FILE_MODE_WR: c_int = 2;

pub const CEPH_FILE_MODE_BITS: c_int = 4;

    pub flags): int ceph_flags_to_mode(int,

// capability bits

// generic cap bits

pub const CEPH_CAP_SIMPLE_BITS: c_int = 2;
pub const CEPH_CAP_FILE_BITS: c_int = 8;
// per-lock shift
pub const CEPH_CAP_SAUTH: c_int = 2;
pub const CEPH_CAP_SLINK: c_int = 4;
pub const CEPH_CAP_SXATTR: c_int = 6;
pub const CEPH_CAP_SFILE: c_int = 8;
pub const CEPH_CAP_SFLOCK: c_int = 20;
pub const CEPH_CAP_BITS: c_int = 22;
// composed values

// cap masks (for getattr)

// cap masks async dir operations

    pub mode): int ceph_caps_for_mode(int,
}

// flags field in client cap messages (version >= 10)

//
// caps message, used for capability callbacks, acks, requests, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_caps {
    pub /: *mut *mut *mut __le32 op; / CEPH_CAP_OP_,
    pub realm: __le64 ino,,
    pub cap_id: __le64,
    pub issue_seq: __le32 seq,,
    pub /: *mut *mut __le32 caps, wanted, dirty; / latest issued/wanted/dirty,
    pub migrate_seq: __le32,
    pub snap_follows: __le64,
    pub snap_trace_len: __le32,
// authlock
    pub mode: __le32 uid, gid,,
// linklock
    pub nlink: __le32,
// xattrlock
    pub xattr_len: __le32,
    pub xattr_version: __le64,
// a union of non-export and export bodies.
    pub truncate_size: __le64 size, max_size,,
    pub truncate_seq: __le32,
    pub ctime: ceph_timespec mtime, atime,,
    pub layout: ceph_file_layout_legacy,
    pub time_warp_seq: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_peer {
    pub cap_id: __le64,
    pub issue_seq: __le32,
    pub mseq: __le32,
    pub mds: __le32,
    pub flags: __u8,
// C attribute field omitted
// cap release msg head
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_release {
    pub /: *mut *mut __le32 num; / number of cap_items that follow,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_item {
    pub ino: __le64,
    pub cap_id: __le64,
    pub issue_seq: __le32 migrate_seq,,
// C attribute field omitted

    pub o): *const *const extern char ceph_lease_op_name(int,
// lease msg header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_lease {
    pub /: *mut *mut *mut __u8 action; / CEPH_MDS_LEASE_,
    pub /: *mut *mut __le16 mask; / which lease,
    pub ino: __le64,
    pub /: *mut *mut __le64 first, last; / snap range,
    pub seq: __le32,
    pub /: *mut *mut __le32 duration_ms; / duration of renewal,
// C attribute field omitted
// followed by a __le32+string for dname
// client reconnect
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_reconnect {
    pub cap_id: __le64,
    pub wanted: __le32,
    pub issued: __le32,
    pub snaprealm: __le64,
    pub /: *mut *mut __le64 pathbase; / base ino for our path to this ino,
    pub /: *mut *mut __le32 flock_len; / size of flock state blob, if any,
// C attribute field omitted
// followed by flock blob
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_cap_reconnect_v1 {
    pub cap_id: __le64,
    pub wanted: __le32,
    pub issued: __le32,
    pub size: __le64,
    pub atime: ceph_timespec mtime,,
    pub snaprealm: __le64,
    pub /: *mut *mut __le64 pathbase; / base ino for our path to this ino,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_snaprealm_reconnect {
    pub /: *mut *mut __le64 ino; / snap realm base,
    pub /: *mut *mut __le64 seq; / snap seq for this snap realm,
    pub /: *mut *mut __le64 parent; / parent realm,
// C attribute field omitted
//
// snaps
//
}

// snap msg header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_snap_head {
    pub /: *mut *mut *mut __le32 op; / CEPH_SNAP_OP_,
    pub /: *mut *mut __le64 split; / ino to split off, if any,
    pub /: *mut *mut __le32 num_split_inos; / # inos belonging to new child realm,
    pub /: *mut *mut __le32 num_split_realms; / # child realms udner new child realm,
    pub /: *mut *mut __le32 trace_len; / size of snap trace blob,
// C attribute field omitted
// followed by split ino list, then split realms, then the trace blob
//
// encode info about a snaprealm, as viewed by a client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_snap_realm {
    pub /: *mut *mut __le64 ino; / ino,
    pub /: *mut *mut __le64 created; / snap: when created,
    pub /: *mut *mut __le64 parent; / ino: parent realm,
    pub /: *mut *mut __le64 parent_since; / snap: same parent since,
    pub /: *mut *mut __le64 seq; / snap: version,
    pub num_snaps: __le32,
    pub num_prior_parent_snaps: __le32,
// C attribute field omitted
// followed by my snap list, then prior parent snap list
//
// quotas
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mds_quota {
    pub /: *mut *mut __le64 ino; / ino,
    pub rctime: ceph_timespec,
    pub /: *mut *mut __le64 rbytes; / dir stats,
    pub rfiles: __le64,
    pub rsubdirs: __le64,
    pub /: *mut *mut __u8 struct_v; / compat,
    pub struct_compat: __u8,
    pub struct_len: __le32,
    pub /: *mut *mut __le64 max_bytes; / quota max. bytes,
    pub /: *mut *mut __le64 max_files; / quota max. files,
// C attribute field omitted
