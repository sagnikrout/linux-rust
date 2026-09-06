//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/osdmap.h
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
// The osd map describes the current membership of the osd cluster and
// specifies the mapping of objects to placement groups and placement
// groups to (sets of) osds.  That is, it completely specifies the
// (desired) distribution of all data objects in the system at some
// point in time.
//
// Each map version is identified by an epoch, which increases monotonically.
//
// The map can be updated either via an incremental map (diff) describing
// the change between two successive epochs, or as a fully encoded map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pg {
    pub pool: u64,
    pub seed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_spg {
    pub pgid: ceph_pg,
    pub shard: i8,
}

extern "C" {
    pub fn ceph_pg_compare(lhs: *const ceph_pg, rhs: *const ceph_pg) -> c_int;
}
extern "C" {
    pub fn ceph_spg_compare(lhs: *const ceph_spg, rhs: *const ceph_spg) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pg_pool_info {
    pub node: rb_node,
    pub id: i64,
    pub /: *mut *mut *mut u8 type; / CEPH_POOL_TYPE_,
    pub size: u8,
    pub min_size: u8,
    pub crush_ruleset: u8,
    pub object_hash: u8,
    pub last_force_request_resend: u32,
    pub pgp_num: u32 pg_num,,
    pub pgp_num_mask: int pg_num_mask,,
    pub read_tier: i64,
    pub /: *mut *mut s64 write_tier; / wins for read+write ops,
    pub /: *mut *mut *mut u64 flags; / CEPH_POOL_FLAG_,
    pub name: *mut c_char,
    pub /: *mut *mut bool was_full; / for handle_one_map(),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_object_locator {
    pub pool: i64,
    pub pool_ns: *mut ceph_string,
}

extern "C" {
    pub fn ceph_oloc_destroy(oloc: *mut ceph_object_locator);
}
//
// 51-char inline_name is long enough for all cephfs and all but one
// rbd requests: <imgname> in "<imgname>.rbd"/"rbd_id.<imgname>" can be
// arbitrarily long (~PAGE_SIZE).  It's done once during rbd map; all
// other rbd requests fit into inline_name.
//
// Makes ceph_object_id 64 bytes on 64-bit.
//
pub const CEPH_OID_INLINE_LEN: c_int = 52;
//
// Both inline and external buffers have space for a NUL-terminator,
// which is carried around.  It's not required though - RADOS object
// names don't have to be NUL-terminated and may contain NULs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_object_id {
    pub name: *mut c_char,
    pub inline_name: [c_char; CEPH_OID_INLINE_LEN],
    pub name_len: c_int,
}

// oid = (struct ceph_object_id) __CEPH_OID_INITIALIZER(*oid);
extern "C" {
    pub fn ceph_oid_printf(oid: *mut ceph_object_id, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ceph_oid_destroy(oid: *mut ceph_object_id);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct workspace_manager {
    pub idle_ws: list_head,
    pub ws_lock: spinlock_t,
// Number of free workspaces
    pub free_ws: c_int,
// Total number of allocated workspaces
    pub total_ws: core::sync::atomic::AtomicI32,
// Waiters for a free workspace
    pub ws_wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pg_mapping {
    pub node: rb_node,
    pub pgid: ceph_pg,
    pub len: c_int,
    pub osds: [c_int; ],
    pub pg_upmap: } pg_temp,,
    pub osd: c_int,
    pub primary_temp: },
    pub len: c_int,
    pub from_to: [c_int; ][2],
    pub pg_upmap_items: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osdmap {
    pub fsid: ceph_fsid,
    pub epoch: u32,
    pub modified: ceph_timespec created,,
    pub /: *mut *mut *mut u32 flags; / CEPH_OSDMAP_,
    pub /: *mut *mut u32 max_osd; / size of osd_state, _offload, _addr arrays,
    pub /: *mut *mut *mut *mut u32 osd_state; / CEPH_OSD_,
    pub /: *mut *mut *mut u32 osd_weight; / 0 = failed, 0x10000 = 100% normal,
    pub osd_addr: *mut ceph_entity_addr,
    pub pg_temp: rb_root,
    pub primary_temp: rb_root,
// remap (post-CRUSH, pre-up)
    pub /: *mut *mut rb_root pg_upmap; / PG := raw set,
    pub /: *mut *mut rb_root pg_upmap_items; / from -> to within raw set,
    pub osd_primary_affinity: *mut u32,
    pub pg_pools: rb_root,
    pub pool_max: u32,
// the CRUSH map specifies the mapping of placement groups to
// the list of osds that store+replicate them.
    pub crush: *mut crush_map,
    pub crush_wsm: workspace_manager,
}

extern "C" {
    pub fn ceph_get_primary_affinity(map: *mut ceph_osdmap, osd: c_int) -> u32;
}

// p += 4;	/* skip deprecated preferred value
extern "C" {
    pub fn ceph_osdmap_destroy(map: *mut ceph_osdmap);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_osds {
    pub osds: [c_int; CEPH_PG_MAX_SIZE],
    pub size: c_int,
    pub /: *mut *mut int primary; / id, NOT index,
}

extern "C" {
    pub fn ceph_osds_copy(dest: *mut ceph_osds, src: *const ceph_osds);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_loc {
    pub cl_type_name: *mut c_char,
    pub cl_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crush_loc_node {
    pub cl_node: rb_node,
    pub /: *mut *mut crush_loc cl_loc; / pointers into cl_data,
    pub cl_data: [c_char; ],
}

extern "C" {
    pub fn ceph_parse_crush_location(crush_location: *mut c_char, locs: *mut rb_root) -> c_int;
}
extern "C" {
    pub fn ceph_compare_crush_locs(locs1: *mut rb_root, locs2: *mut rb_root) -> c_int;
}
extern "C" {
    pub fn ceph_clear_crush_locs(locs: *mut rb_root);
}
extern "C" {
    pub fn ceph_pg_poolid_by_name(map: *mut ceph_osdmap, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn ceph_pg_pool_flags(map: *mut ceph_osdmap, id: u64) -> u64;
}
