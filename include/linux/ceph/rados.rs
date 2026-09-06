//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/rados.h
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
// Data types for the Ceph distributed object storage layer RADOS
// (Reliable Autonomic Distributed Object Store).
//

//
// fs id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_fsid {
    pub fsid: [c_uchar; 16],
}

extern "C" {
    pub fn memcmp(_arg: a, _arg: b, _arg: *mut sizeof(a)) -> return;
}
//
// ino, object, etc.
//
pub type ceph_snapid_t = __le64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_timespec {
    pub tv_sec: __le32,
    pub tv_nsec: __le32,
// C attribute field omitted
//
// object layout - how objects are mapped into PGs
//
pub const CEPH_OBJECT_LAYOUT_HASH: c_int = 1;
pub const CEPH_OBJECT_LAYOUT_LINEAR: c_int = 2;
pub const CEPH_OBJECT_LAYOUT_HASHINO: c_int = 3;
//
// pg layout -- how PGs are mapped onto (sets of) OSDs
//
pub const CEPH_PG_LAYOUT_CRUSH: c_int = 0;
pub const CEPH_PG_LAYOUT_HASH: c_int = 1;
pub const CEPH_PG_LAYOUT_LINEAR: c_int = 2;
pub const CEPH_PG_LAYOUT_HYBRID: c_int = 3;

//
// placement group.
// we encode this into one __le64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pg_v1 {
    pub /: *mut *mut __le16 preferred; / preferred primary osd,
    pub /: *mut *mut __le16 ps; / placement seed,
    pub /: *mut *mut __le32 pool; / object pool,
// C attribute field omitted
//
// pg_pool is a set of pgs storing a pool of objects
//
// pg_num -- base number of pseudorandomly placed pgs
//
// pgp_num -- effective number when calculating pg placement.  this
// is used for pg_num increases.  new pgs result in data being "split"
// into new pgs.  for this to proceed smoothly, new pgs are intiially
// colocated with their parents; that is, pgp_num doesn't increase
// until the new pgs have successfully split.  only _then_ are the new
// pgs placed independently.
//
// lpg_num -- localized pg count (per device).  replicas are randomly
// selected.
//
// lpgp_num -- as above.
//

pub const CEPH_POOL_TYPE_REP: c_int = 1;

pub const CEPH_POOL_TYPE_EC: c_int = 3;
//
// stable_mod func is used to control number of placement groups.
// similar to straight-up modulo, but produces a stable mapping as b
// increases over time.  b is the number of bins, and bmask is the
// containing power of 2 minus 1.
//
// b <= bmask and bmask=(2**n)-1
// e.g., b=12 -> bmask=15, b=123 -> bmask=127
//
    pub bmask: return x &,
    pub 1): return x & (bmask >>,
//
// object layout - how a given object should be stored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_object_layout {
    pub /: *mut *mut ceph_pg_v1 ol_pgid; / raw pg, with _full_ ps precision.,
    pub /: *mut *mut __le32 ol_stripe_unit; / for per-object parity, if any,
// C attribute field omitted
//
// compound epoch+version, used by storage layer to serialize mutations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_eversion {
    pub version: __le64,
    pub epoch: __le32,
// C attribute field omitted
//
// osd map bits
//
// status bits

    pub s): *const *const extern char ceph_osd_state_name(int,
// osd weights.  fixed point value: 0x10000 == 1.0 ("in"), 0 == "out"
pub const CEPH_OSD_IN: c_uint = 0x10000;
pub const CEPH_OSD_OUT: c_int = 0;
// osd primary-affinity.  fixed point value: 0x10000 == baseline
pub const CEPH_OSD_MAX_PRIMARY_AFFINITY: c_uint = 0x10000;
pub const CEPH_OSD_DEFAULT_PRIMARY_AFFINITY: c_uint = 0x10000;
//
// osd map flag bits
//

//
// The error code to return when an OSD can't handle a write
// because it is too large.
//

//
// osd ops
//
// WARNING: do not use these op codes directly.  Use the helpers
// defined below instead.  In certain cases, op code behavior was
// redefined, resulting in special-cases in the helpers.
//
pub const CEPH_OSD_OP_MODE: c_uint = 0xf000;
pub const CEPH_OSD_OP_MODE_RD: c_uint = 0x1000;
pub const CEPH_OSD_OP_MODE_WR: c_uint = 0x2000;
pub const CEPH_OSD_OP_MODE_RMW: c_uint = 0x3000;
pub const CEPH_OSD_OP_MODE_SUB: c_uint = 0x4000;
pub const CEPH_OSD_OP_MODE_CACHE: c_uint = 0x8000;
pub const CEPH_OSD_OP_TYPE: c_uint = 0x0f00;
pub const CEPH_OSD_OP_TYPE_LOCK: c_uint = 0x0100;
pub const CEPH_OSD_OP_TYPE_DATA: c_uint = 0x0200;
pub const CEPH_OSD_OP_TYPE_ATTR: c_uint = 0x0300;
pub const CEPH_OSD_OP_TYPE_EXEC: c_uint = 0x0400;
pub const CEPH_OSD_OP_TYPE_PG: c_uint = 0x0500;
pub const CEPH_OSD_OP_TYPE_MULTI: c_uint = 0x0600 /* multiobject */;

// data **/							    \
// read */							    \
// fancy read */						    \
// versioning */						    \
// sync */							    \
// write */							    \
// fancy write */						    \
// omap */							    \
// tiering */							    \
// convert tmap to omap */					    \
// hints */							    \
// multi **/							    \
// attrs **/							    \
// read */							    \
// write */							    \
// subop **/							    \
// lock **/							    \
// exec **/							    \
// note: the RD bit here is wrong; see special-case below in helper */ \
// pg **/							    \

}

//
// note that the following tmap stuff is also defined in the ceph librados.h
// any modification here needs to be updated there
//

//
// osd op flags
//
// An op may be READ, WRITE, or READ|WRITE.
//

// xattr comparison
// cloneid
// note: use only ODD ids to prevent pre-giant code from
//
// an individual object operation.  each may be accompanied by some data
// payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osd_op {
    pub /: *mut *mut *mut __le16 op; / CEPH_OSD_OP_,
    pub /: *mut *mut *mut __le32 flags; / CEPH_OSD_OP_FLAG_,
    pub length: __le64 offset,,
    pub truncate_size: __le64,
    pub truncate_seq: __le32,
// C attribute field omitted
    pub name_len: __le32,
    pub value_len: __le32,
    pub /: *mut *mut *mut __u8 cmp_op; / CEPH_OSD_CMPXATTR_OP_,
    pub /: *mut *mut *mut __u8 cmp_mode; / CEPH_OSD_CMPXATTR_MODE_,
// C attribute field omitted
    pub class_len: __u8,
    pub method_len: __u8,
    pub argc: __u8,
    pub indata_len: __le32,
// C attribute field omitted
    pub count: __le64 cookie,,
// C attribute field omitted
    pub snapid: __le64,
// C attribute field omitted
    pub cookie: __le64,
    pub /: *mut *mut __le64 ver; / no longer used,
    pub /: *mut *mut *mut __u8 op; / CEPH_OSD_WATCH_OP_,
    pub /: *mut *mut __le32 gen; / registration generation,
// C attribute field omitted
    pub cookie: __le64,
// C attribute field omitted
    pub unused: __le64,
    pub ver: __le64,
// C attribute field omitted
    pub length: __le64 offset,,
    pub src_offset: __le64,
// C attribute field omitted
    pub expected_object_size: __le64,
    pub expected_write_size: __le64,
    pub /: *mut *mut *mut __le32 flags; / CEPH_OSD_OP_ALLOC_HINT_FLAG_,
// C attribute field omitted
    pub snapid: __le64,
    pub src_version: __le64,
    pub /: *mut *mut *mut __u8 flags; / CEPH_OSD_COPY_FROM_FLAG_,
//
// CEPH_OSD_OP_FLAG_FADVISE_*: fadvise flags
// for src object, flags for dest object are in
// ceph_osd_op::flags.
//
    pub src_fadvise_flags: __le32,
// C attribute field omitted
}
