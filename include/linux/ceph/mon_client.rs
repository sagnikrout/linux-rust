//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/mon_client.h
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
// The monitor map enumerates the set of all monitors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_monmap {
    pub fsid: ceph_fsid,
    pub epoch: u32,
    pub num_mon: u32,
    pub __counted_by(num_mon): ceph_entity_inst mon_inst[],
}

//
// Generic mechanism for resending monitor requests.
//
// a pending monitor request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_request {
    pub monc: *mut ceph_mon_client,
    pub delayed_work: delayed_work,
    pub delay: c_ulong,
    pub do_request: ceph_monc_request_func_t,
}

extern "C" {
    pub fn void(: *mut *mut ceph_monc_callback_t)(struct ceph_mon_generic_request) -> typedef;
}
//
// ceph_mon_generic_request is being used for the statfs and
// mon_get_version requests which are being done a bit differently
// because we need to get data back to the caller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_generic_request {
    pub monc: *mut ceph_mon_client,
    pub kref: kref,
    pub tid: u64,
    pub node: rb_node,
    pub result: c_int,
    pub completion: completion,
    pub complete_cb: ceph_monc_callback_t,
    pub /: *mut *mut u64 private_data; / r_tid/linger_id,
    pub /: *mut *mut *mut ceph_msg request; / original request,
    pub /: *mut *mut *mut ceph_msg reply; / and reply,
    pub st: *mut ceph_statfs,
    pub newest: u64,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_mon_client {
    pub client: *mut ceph_client,
    pub monmap: *mut ceph_monmap,
    pub mutex: mutex,
    pub delayed_work: delayed_work,
    pub auth: *mut ceph_auth_client,
    pub m_subscribe_ack: *mut *mut *mut *mut ceph_msg m_auth, m_auth_reply, m_subscribe,,
    pub pending_auth: c_int,
    pub hunting: bool,
    pub /: *mut *mut int cur_mon; / last monitor i contacted,
    pub sub_renew_after: c_ulong,
    pub sub_renew_sent: c_ulong,
    pub con: ceph_connection,
    pub had_a_connection: bool,
    pub /: *mut *mut int hunt_mult; / [1..CEPH_MONC_HUNT_MAX_MULT],
// pending generic requests
    pub generic_request_tree: rb_root,
    pub last_tid: u64,
// subs, indexed with CEPH_SUB_*
    pub item: ceph_mon_subscribe_item,
    pub want: bool,
    pub /: *mut *mut u32 have; / epoch,
    pub subs: [}; 4],
    pub /: *mut *mut int fs_cluster_id; / "mdsmap.<id>" sub,

    pub debugfs_file: *mut dentry,

}

extern "C" {
    pub fn ceph_monc_init(monc: *mut ceph_mon_client, cl: *mut ceph_client) -> c_int;
}
extern "C" {
    pub fn ceph_monc_stop(monc: *mut ceph_mon_client);
}
extern "C" {
    pub fn ceph_monc_reopen_session(monc: *mut ceph_mon_client);
}
//
// The model here is to indicate that we need a new map of at least
// epoch @epoch, and also call in when we receive a map.  We will
// periodically rerequest the map from the monitor cluster until we
// get what we want.
//
extern "C" {
    pub fn ceph_monc_got_map(monc: *mut ceph_mon_client, sub: c_int, epoch: u32);
}
extern "C" {
    pub fn ceph_monc_renew_subs(monc: *mut ceph_mon_client);
}
extern "C" {
    pub fn ceph_monc_open_session(monc: *mut ceph_mon_client) -> c_int;
}
extern "C" {
    pub fn ceph_monc_validate_auth(monc: *mut ceph_mon_client) -> c_int;
}
