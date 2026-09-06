//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private_mcast_eht.h
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
// Copyright (c) 2020, Nikolay Aleksandrov <nikolay@nvidia.com>
//
pub const BR_MCAST_DEFAULT_EHT_HOSTS_LIMIT: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub union net_bridge_eht_addr {
    pub ip4: __be32,

    pub ip6: in6_addr,

}

// single host's list of set entries and filter_mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_group_eht_host {
    pub rb_node: rb_node,
    pub h_addr: net_bridge_eht_addr,
    pub set_entries: hlist_head,
    pub num_entries: c_uint,
    pub filter_mode: c_uchar,
    pub pg: *mut net_bridge_port_group,
}

// (host, src entry) added to a per-src set and host's list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_group_eht_set_entry {
    pub rb_node: rb_node,
    pub host_list: hlist_node,
    pub h_addr: net_bridge_eht_addr,
    pub timer: timer_list,
    pub br: *mut net_bridge,
    pub eht_set: *mut net_bridge_group_eht_set,
    pub h_parent: *mut net_bridge_group_eht_host,
    pub mcast_gc: net_bridge_mcast_gc,
}

// per-src set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_group_eht_set {
    pub rb_node: rb_node,
    pub src_addr: net_bridge_eht_addr,
    pub entry_tree: rb_root,
    pub timer: timer_list,
    pub pg: *mut net_bridge_port_group,
    pub br: *mut net_bridge,
    pub mcast_gc: net_bridge_mcast_gc,
}

extern "C" {
    pub fn br_multicast_eht_clean_sets(pg: *mut net_bridge_port_group);
}

