//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/osd_client.h
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
// completion callback for async writepages
//
extern "C" {
    pub fn void(: *mut *mut ceph_osdc_callback_t)(struct ceph_osd_request) -> typedef;
}

//
// A single extent in a SPARSE_READ reply.
//
// Note that these come from the OSD as little-endian values. On BE arches,
// we convert them in-place after receipt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_sparse_extent {
    pub off: u64,
    pub len: u64,
    pub __packed: },
// Sparse read state machine state values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_sparse_read_state {
    CEPH_SPARSE_READ_HDR	= 0,
    CEPH_SPARSE_READ_EXTENTS,
    CEPH_SPARSE_READ_DATA_LEN,
    CEPH_SPARSE_READ_DATA_PRE,
    CEPH_SPARSE_READ_DATA,
}

//
// A SPARSE_READ reply is a 32-bit count of extents, followed by an array of
// 64-bit offset/length pairs, and then all of the actual file data
// concatenated after it (sans holes).
//
// Unfortunately, we don't know how long the extent array is until we've
// started reading the data section of the reply. The caller should send down
// a destination buffer for the array, but we'll alloc one if it's too small
// or if the caller doesn't.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_sparse_read {
    pub /: *mut *mut ceph_sparse_read_state sr_state; / state machine state,
    pub /: *mut *mut u64 sr_req_off; / orig request offset,
    pub /: *mut *mut u64 sr_req_len; / orig request length,
    pub /: *mut *mut u64 sr_pos; / current pos in buffer,
    pub /: *mut *mut int sr_index; / current extent index,
    pub /: *mut *mut u32 sr_datalen; / length of actual data,
    pub /: *mut *mut u32 sr_count; / extent count in reply,
    pub /: *mut *mut int sr_ext_len; / length of extent array,
    pub /: *mut *mut *mut ceph_sparse_extent sr_extent; / extent array,
}

//
// A given osd we're communicating with.
//
// Note that the o_requests tree can be searched while holding the "lock" mutex
// or the "o_requests_lock" spinlock. Insertion or removal requires both!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd {
    pub o_ref: refcount_t,
    pub o_sparse_op_idx: c_int,
    pub o_osdc: *mut ceph_osd_client,
    pub o_osd: c_int,
    pub o_incarnation: c_int,
    pub o_node: rb_node,
    pub o_con: ceph_connection,
    pub o_requests_lock: spinlock_t,
    pub o_requests: rb_root,
    pub o_linger_requests: rb_root,
    pub o_backoff_mappings: rb_root,
    pub o_backoffs_by_id: rb_root,
    pub o_osd_lru: list_head,
    pub o_auth: ceph_auth_handshake,
    pub lru_ttl: c_ulong,
    pub o_keepalive_item: list_head,
    pub lock: mutex,
    pub o_sparse_read: ceph_sparse_read,
}

pub const CEPH_OSD_SLAB_OPS: c_int = 2;
pub const CEPH_OSD_MAX_OPS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_osd_data_type {
    CEPH_OSD_DATA_TYPE_NONE = 0,
    CEPH_OSD_DATA_TYPE_PAGES,
    CEPH_OSD_DATA_TYPE_PAGELIST,

    CEPH_OSD_DATA_TYPE_BIO,

    CEPH_OSD_DATA_TYPE_BVECS,
    CEPH_OSD_DATA_TYPE_ITER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_data {
    pub type: ceph_osd_data_type,
    pub pages: *mut page,
    pub length: u64,
    pub alignment: u32,
    pub pages_from_pool: bool,
    pub own_pages: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_req_op {
    pub /: *mut *mut *mut u16 op; / CEPH_OSD_OP_,
    pub /: *mut *mut *mut u32 flags; / CEPH_OSD_OP_FLAG_,
    pub /: *mut *mut u32 indata_len; / request,
    pub /: *mut *mut u32 outdata_len; / reply,
    pub rval: i32,
    pub raw_data_in: ceph_osd_data,
    pub length: u64 offset,,
    pub truncate_size: u64,
    pub truncate_seq: u32,
    pub sparse_ext_cnt: c_int,
    pub sparse_ext: *mut ceph_sparse_extent,
    pub osd_data: ceph_osd_data,
    pub extent: },
    pub name_len: u32,
    pub value_len: u32,
    pub /: *mut *mut *mut __u8 cmp_op; / CEPH_OSD_CMPXATTR_OP_,
    pub /: *mut *mut *mut __u8 cmp_mode; / CEPH_OSD_CMPXATTR_MODE_,
    pub osd_data: ceph_osd_data,
    pub xattr: },
    pub class_name: *const c_char,
    pub method_name: *const c_char,
    pub request_info: ceph_osd_data,
    pub request_data: ceph_osd_data,
    pub response_data: ceph_osd_data,
    pub class_len: __u8,
    pub method_len: __u8,
    pub indata_len: u32,
    pub cls: },
    pub cookie: u64,
    pub /: *mut *mut __u8 op; / CEPH_OSD_WATCH_OP_,
    pub gen: u32,
    pub watch: },
    pub request_data: ceph_osd_data,
    pub notify_ack: },
    pub cookie: u64,
    pub request_data: ceph_osd_data,
    pub response_data: ceph_osd_data,
    pub notify: },
    pub response_data: ceph_osd_data,
    pub list_watchers: },
    pub expected_object_size: u64,
    pub expected_write_size: u64,
    pub /: *mut *mut *mut u32 flags; / CEPH_OSD_OP_ALLOC_HINT_FLAG_,
    pub alloc_hint: },
    pub snapid: u64,
    pub src_version: u64,
    pub flags: u8,
    pub src_fadvise_flags: u32,
    pub osd_data: ceph_osd_data,
    pub copy_from: },
    pub ver: u64,
    pub assert_ver: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_request_target {
    pub base_oid: ceph_object_id,
    pub base_oloc: ceph_object_locator,
    pub target_oid: ceph_object_id,
    pub target_oloc: ceph_object_locator,
    pub /: *mut *mut ceph_pg pgid; / last raw pg we mapped to,
    pub /: *mut *mut ceph_spg spgid; / last actual spg we mapped to,
    pub pg_num: u32,
    pub pg_num_mask: u32,
    pub acting: ceph_osds,
    pub up: ceph_osds,
    pub size: c_int,
    pub min_size: c_int,
    pub sort_bitwise: bool,
    pub recovery_deletes: bool,
    pub /: *mut *mut *mut unsigned int flags; / CEPH_OSD_FLAG_,
    pub used_replica: bool,
    pub paused: bool,
    pub epoch: u32,
    pub last_force_resend: u32,
    pub osd: c_int,
}

// an in-flight request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_request {
    pub /: *mut *mut u64 r_tid; / unique for this client,
    pub r_node: rb_node,
    pub /: *mut *mut rb_node r_mc_node; / map check,
    pub r_complete_work: work_struct,
    pub r_osd: *mut ceph_osd,
    pub r_t: ceph_osd_request_target,

    pub r_reply: *mut *mut ceph_msg r_request,,
    pub /: *mut *mut u32 r_sent; / >0 if r_request is sending/sent,
// request osd ops array
    pub r_num_ops: c_uint,
    pub r_result: c_int,
    pub r_osdc: *mut ceph_osd_client,
    pub r_kref: kref,
    pub r_mempool: bool,
    pub /: *mut *mut bool r_linger; / don't resend on failure,
    pub /: *mut *mut completion r_completion; / private to osd_client.c,
    pub r_callback: ceph_osdc_callback_t,
    pub /: *mut *mut *mut inode r_inode; / for use by callbacks,
    pub /: *mut *mut list_head r_private_item; / ditto,
    pub /: *mut *mut *mut void r_priv; / ditto,
// set by submitter
    pub /: *mut *mut u64 r_snapid; / for reads, CEPH_NOSNAP o/w,
    pub /: *mut *mut *mut ceph_snap_context r_snapc; / for writes,
    pub /: *mut *mut timespec64 r_mtime; / ditto,
    pub /: *mut *mut u64 r_data_offset; / ditto,
// internal
    pub /: *mut *mut u64 r_version; / data version sent in reply,
    pub /: *mut *mut unsigned long r_stamp; / jiffies, send or check time,
    pub /: *mut *mut unsigned long r_start_stamp; / jiffies,
    pub /: *mut *mut ktime_t r_start_latency; / ktime_t,
    pub /: *mut *mut ktime_t r_end_latency; / ktime_t,
    pub r_attempts: c_int,
    pub r_map_dne_bound: u32,
    pub __counted_by(r_num_ops): ceph_osd_req_op r_ops[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_request_redirect {
    pub oloc: ceph_object_locator,
}

//
// osd request identifier
//
// caller name + incarnation# + tid to unique identify this request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_reqid {
    pub name: ceph_entity_name,
    pub tid: __le64,
    pub inc: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_blkin_trace_info {
    pub trace_id: __le64,
    pub span_id: __le64,
    pub parent_span_id: __le64,
    pub __packed: },
    pub data_len): *mut *mut u64 notifier_id, void data, size_t,
    pub err): *mut *mut *mut typedef void (rados_watcherrcb_t)(void arg, u64 cookie, int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_linger_request {
    pub osdc: *mut ceph_osd_client,
    pub linger_id: u64,
    pub committed: bool,
    pub /: *mut *mut bool is_watch; / watch or notify,
    pub osd: *mut ceph_osd,
    pub reg_req: *mut ceph_osd_request,
    pub ping_req: *mut ceph_osd_request,
    pub ping_sent: c_ulong,
    pub watch_valid_thru: c_ulong,
    pub pending_lworks: list_head,
    pub t: ceph_osd_request_target,
    pub map_dne_bound: u32,
    pub mtime: timespec64,
    pub kref: kref,
    pub lock: mutex,
    pub /: *mut *mut rb_node node; / osd,
    pub /: *mut *mut rb_node osdc_node; / osdc,
    pub /: *mut *mut rb_node mc_node; / map check,
    pub scan_item: list_head,
    pub reg_commit_wait: completion,
    pub notify_finish_wait: completion,
    pub reg_commit_error: c_int,
    pub notify_finish_error: c_int,
    pub last_error: c_int,
    pub register_gen: u32,
    pub notify_id: u64,
    pub wcb: rados_watchcb2_t,
    pub errcb: rados_watcherrcb_t,
    pub data: *mut c_void,
    pub request_pl: *mut ceph_pagelist,
    pub notify_id_pages: *mut page,
    pub preply_pages: *mut page,
    pub preply_len: *mut usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_watch_item {
    pub name: ceph_entity_name,
    pub cookie: u64,
    pub addr: ceph_entity_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_spg_mapping {
    pub node: rb_node,
    pub spgid: ceph_spg,
    pub backoffs: rb_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_hobject_id {
    pub key: *mut c_void,
    pub key_len: usize,
    pub oid: *mut c_void,
    pub oid_len: usize,
    pub snapid: u64,
    pub hash: u32,
    pub is_max: u8,
    pub nspace: *mut c_void,
    pub nspace_len: usize,
    pub pool: i64,
// cache
    pub hash_reverse_bits: u32,
}

//
// PG-wide backoff: [begin, end)
// per-object backoff: begin == end
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_backoff {
    pub spg_node: rb_node,
    pub id_node: rb_node,
    pub spgid: ceph_spg,
    pub id: u64,
    pub begin: *mut ceph_hobject_id,
    pub end: *mut ceph_hobject_id,
}

pub const CEPH_LINGER_ID_START: c_uint = 0xffff000000000000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_client {
    pub client: *mut ceph_client,
    pub /: *mut *mut *mut ceph_osdmap osdmap; / current map,
    pub lock: rw_semaphore,
    pub /: *mut *mut rb_root osds; / osds,
    pub /: *mut *mut list_head osd_lru; / idle osds,
    pub osd_lru_lock: spinlock_t,
    pub epoch_barrier: u32,
    pub homeless_osd: ceph_osd,
    pub /: *mut *mut atomic64_t last_tid; / tid of last request,
    pub last_linger_id: u64,
    pub /: *mut *mut rb_root linger_requests; / lingering requests,
    pub map_checks: rb_root,
    pub linger_map_checks: rb_root,
    pub num_requests: core::sync::atomic::AtomicI32,
    pub num_homeless: core::sync::atomic::AtomicI32,
    pub abort_err: c_int,
    pub timeout_work: delayed_work,
    pub osds_timeout_work: delayed_work,

    pub debugfs_file: *mut dentry,

    pub req_mempool: *mut mempool_t,
    pub msgpool_op: ceph_msgpool,
    pub msgpool_op_reply: ceph_msgpool,
    pub notify_wq: *mut workqueue_struct,
    pub completion_wq: *mut workqueue_struct,
}

extern "C" {
    pub fn ceph_osdc_setup() -> c_int;
}
extern "C" {
    pub fn ceph_osdc_cleanup();
}
extern "C" {
    pub fn ceph_osdc_stop(osdc: *mut ceph_osd_client);
}
extern "C" {
    pub fn ceph_osdc_reopen_osds(osdc: *mut ceph_osd_client);
}
extern "C" {
    pub fn ceph_osdc_update_epoch_barrier(osdc: *mut ceph_osd_client, eb: u32);
}
extern "C" {
    pub fn ceph_osdc_abort_requests(osdc: *mut ceph_osd_client, err: c_int);
}
extern "C" {
    pub fn ceph_osdc_clear_abort_err(osdc: *mut ceph_osd_client);
}

extern "C" {
    pub fn ceph_osdc_alloc_messages(req: *mut ceph_osd_request, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn __ceph_alloc_sparse_ext_map(op: *mut ceph_osd_req_op, cnt: c_int) -> c_int;
}
//
// How big an extent array should we preallocate for a sparse read? This is
// just a starting value.  If we get more than this back from the OSD, the
// receiver will reallocate.
//
pub const CEPH_SPARSE_EXT_ARRAY_INITIAL: c_int = 16;
extern "C" {
    pub fn __ceph_alloc_sparse_ext_map(_arg: op, _arg: cnt) -> return;
}
extern "C" {
    pub fn ceph_osdc_get_request(req: *mut ceph_osd_request);
}
extern "C" {
    pub fn ceph_osdc_put_request(req: *mut ceph_osd_request);
}
extern "C" {
    pub fn ceph_osdc_cancel_request(req: *mut ceph_osd_request);
}
extern "C" {
    pub fn ceph_osdc_sync(osdc: *mut ceph_osd_client);
}
extern "C" {
    pub fn ceph_osdc_flush_notifies(osdc: *mut ceph_osd_client);
}
extern "C" {
    pub fn ceph_osdc_maybe_request_map(osdc: *mut ceph_osd_client);
}
// watch/notify
// Find offset into the buffer of the end of the extent map
// No extents? No data
