//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/nodemanager.h
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
// nodemanager.h
//
// Function prototypes
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//

// This totally doesn't belong here.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum o2nm_fence_method {
    O2NM_FENCE_RESET	= 0,
    O2NM_FENCE_PANIC,
    O2NM_FENCE_METHODS,	/* Number of fence methods */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2nm_node {
    pub nd_lock: spinlock_t,
    pub nd_item: config_item,
    pub /: *mut *mut char nd_name[O2NM_MAX_NAME_LEN+1]; / replace?,
    pub nd_num: __u8,
// only one address per node, as attributes, for now.
    pub nd_ipv4_address: __be32,
    pub nd_ipv4_port: __be16,
    pub nd_ip_node: rb_node,
// there can be only one local node for now
    pub nd_local: c_int,
    pub nd_set_attributes: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2nm_cluster {
    pub cl_group: config_group,
    pub cl_has_local:1: unsigned,
    pub cl_local_node: u8,
    pub cl_nodes_lock: rwlock_t,
    pub cl_nodes: [*mut o2nm_node; O2NM_MAX_NODES],
    pub cl_node_ip_tree: rb_root,
    pub cl_idle_timeout_ms: c_uint,
    pub cl_keepalive_delay_ms: c_uint,
    pub cl_reconnect_delay_ms: c_uint,
    pub cl_fence_method: o2nm_fence_method,
// this bitmap is part of a hack for disk bitmap.. will go eventually. - zab
    pub cl_nodes_bitmap: [c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES)],
}

extern "C" {
    pub fn o2nm_this_node() -> u8;
}
extern "C" {
    pub fn o2nm_configured_node_map(map: *mut c_ulong, bytes: unsigned) -> c_int;
}
extern "C" {
    pub fn o2nm_node_get(node: *mut o2nm_node);
}
extern "C" {
    pub fn o2nm_node_put(node: *mut o2nm_node);
}
extern "C" {
    pub fn o2nm_depend_item(item: *mut config_item) -> c_int;
}
extern "C" {
    pub fn o2nm_depend_item_unlocked(item: *mut config_item) -> c_int;
}
extern "C" {
    pub fn o2nm_undepend_item(item: *mut config_item);
}
extern "C" {
    pub fn o2nm_depend_node(node_num: u8) -> c_int;
}
extern "C" {
    pub fn o2nm_undepend_node(node_num: u8);
}
extern "C" {
    pub fn o2nm_depend_this_node() -> c_int;
}
extern "C" {
    pub fn o2nm_undepend_this_node();
}
