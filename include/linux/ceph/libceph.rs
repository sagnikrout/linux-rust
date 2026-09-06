//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/libceph.h
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
// mount options
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_options {
    pub flags: c_int,
    pub fsid: ceph_fsid,
    pub my_addr: ceph_entity_addr,
    pub /: *mut *mut unsigned long mount_timeout; / jiffies,
    pub /: *mut *mut unsigned long osd_idle_ttl; / jiffies,
    pub /: *mut *mut unsigned long osd_keepalive_timeout; / jiffies,
    pub /: *mut *mut unsigned long osd_request_timeout; / jiffies,
    pub /: *mut *mut u32 read_from_replica; / CEPH_OSD_FLAG_BALANCE/LOCALIZE_READS,
    pub /: *mut *mut *mut int con_modes[2]; / CEPH_CON_MODE_,
//
// any type that can't be simply compared or doesn't need
// to be compared should go beyond this point,
// ceph_compare_options() should be updated accordingly
//
    pub first: *mut *mut *mut ceph_entity_addr mon_addr; / should be the,
    pub num_mon: c_int,
    pub name: *mut c_char,
    pub key: *mut ceph_crypto_key,
    pub crush_locs: rb_root,
}

//
// defaults
//

pub const CEPH_MONC_HUNT_BACKOFF: c_int = 2;
pub const CEPH_MONC_HUNT_MAX_MULT: c_int = 10;

//
// The largest possible rbd data object is 32M.
// The largest possible rbd object map object is 64M.
//
// There is no limit on the size of cephfs objects, but it has to obey
// rsize and wsize mount options anyway.
//

//
// per client state
//
// possibly shared by multiple mount points, if they are
// mounting the same ceph filesystem/cluster.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_client {
    pub fsid: ceph_fsid,
    pub have_fsid: bool,
    pub private: *mut c_void,
    pub options: *mut ceph_options,
    pub /: *mut *mut mutex mount_mutex; / serialize mount attempts,
    pub auth_wq: wait_queue_head_t,
    pub auth_err: c_int,
    pub ): *mut *mut *mut int (extra_mon_dispatch)(struct ceph_client , struct ceph_msg,
    pub supported_features: u64,
    pub required_features: u64,
    pub /: *mut *mut ceph_messenger msgr; / messenger instance,
    pub monc: ceph_mon_client,
    pub osdc: ceph_osd_client,

    pub debugfs_dir: *mut dentry,
    pub debugfs_monmap: *mut dentry,
    pub debugfs_osdmap: *mut dentry,
    pub debugfs_options: *mut dentry,

}

//
// snapshots
//
// A "snap context" is the set of existing snapshots when we
// write data.  It is used by the OSD to guide its COW behavior.
//
// The ceph_snap_context is refcounted, and attached to each dirty
// page, indicating which context the dirty data belonged when it was
// dirtied.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_snap_context {
    pub nref: refcount_t,
    pub seq: u64,
    pub num_snaps: u32,
    pub snaps: [u64; ],
}

extern "C" {
    pub fn ceph_put_snap_context(sc: *mut ceph_snap_context);
}
//
// calculate the number of pages a given length and offset map onto,
// if we align the data.
//

//
// @lookup_param_type is a parameter and not constructed from (@type,
// @keyfld) with typeof() because adding const is too unwieldy.
//

//
// Shorthands for integer keys.
//

// ceph_common.c
extern "C" {
    pub fn libceph_compatible(data: *mut c_void) -> bool;
}
extern "C" {
    pub fn ceph_check_fsid(client: *mut ceph_client, fsid: *mut ceph_fsid) -> c_int;
}
extern "C" {
    pub fn ceph_parse_fsid(str: *const c_char, fsid: *mut ceph_fsid) -> c_int;
}
extern "C" {
    pub fn ceph_destroy_options(opt: *mut ceph_options);
}
extern "C" {
    pub fn ceph_client_gid(client: *mut ceph_client) -> u64;
}
extern "C" {
    pub fn ceph_destroy_client(client: *mut ceph_client);
}
extern "C" {
    pub fn ceph_reset_client_addr(client: *mut ceph_client);
}
extern "C" {
    pub fn __ceph_open_session(client: *mut ceph_client) -> c_int;
}
extern "C" {
    pub fn ceph_open_session(client: *mut ceph_client) -> c_int;
}
// pagevec.c
extern "C" {
    pub fn ceph_release_page_vector(pages: *mut page, num_pages: c_int);
}
extern "C" {
    pub fn ceph_zero_page_vector_range(off: c_int, len: c_int, pages: *mut page);
}
