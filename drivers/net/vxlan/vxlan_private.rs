//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/vxlan/vxlan_private.h
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
// Vxlan private header file
//

pub const PORT_HASH_BITS: c_int = 8;

// per-network namespace private data for this module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_net {
    pub vxlan_list: list_head,
// sock_list is protected by rtnl lock
    pub sock_list: [hlist_head; PORT_HASH_SIZE],
    pub nexthop_notifier_block: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_fdb_key {
    pub eth_addr: [u8; ETH_ALEN],
    pub vni: __be32,
}

// Forwarding table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_fdb {
    pub rhnode: rhash_head,
    pub rcu: rcu_head,
    pub /: *mut *mut unsigned long updated; / jiffies,
    pub used: c_ulong,
    pub remotes: list_head,
    pub key: vxlan_fdb_key,
    pub /: *mut *mut u16 state; / see ndm_state,
    pub /: *mut *mut u16 flags; / see ndm_flags and below,
    pub nh_list: list_head,
    pub fdb_node: hlist_node,
    pub nh: *mut nexthop __rcu,
    pub vdev: *mut vxlan_dev __rcu,
}

pub const NTF_VXLAN_ADDED_BY_USER: c_uint = 0x100;
// Virtual Network hash table head
// Socket hash table head
// First remote destination for a forwarding entry.
extern "C" {
    pub fn list_entry_rcu(_arg: fdb->remotes.next, vxlan_rdst: struct, _arg: list) -> return;
}
extern "C" {
    pub fn list_first_entry(_arg: &fdb->remotes, vxlan_rdst: struct, _arg: list) -> return;
}

extern "C" {
    pub fn ipv6_addr_equal(_arg: &a->sin6.sin6_addr, _arg: &b->sin6.sin6_addr) -> return;
}
extern "C" {
    pub fn nla_put_in6_addr(_arg: skb, _arg: attr, _arg: &ip->sin6.sin6_addr) -> return;
}
extern "C" {
    pub fn nla_put_in_addr(_arg: skb, _arg: attr, _arg: ip->sin.sin_addr.s_addr) -> return;
}
extern "C" {
    pub fn ipv6_addr_is_multicast(_arg: &ip->sin6.sin6_addr) -> return;
}
extern "C" {
    pub fn ipv4_is_multicast(_arg: ip->sin.sin_addr.s_addr) -> return;
}

extern "C" {
    pub fn nla_put_in_addr(_arg: skb, _arg: attr, _arg: ip->sin.sin_addr.s_addr) -> return;
}
extern "C" {
    pub fn ipv4_is_multicast(_arg: ip->sin.sin_addr.s_addr) -> return;
}

extern "C" {
    pub fn sizeof(in6_addr: struct) -> return;
}
extern "C" {
    pub fn sizeof(_arg: __be32) -> return;
}
// vxlan_core.c
// vxlan_vnifilter.c
extern "C" {
    pub fn vxlan_vnigroup_init(vxlan: *mut vxlan_dev) -> c_int;
}
extern "C" {
    pub fn vxlan_vnigroup_uninit(vxlan: *mut vxlan_dev);
}
extern "C" {
    pub fn vxlan_vnifilter_init() -> c_int;
}
extern "C" {
    pub fn vxlan_vnifilter_uninit();
}
extern "C" {
    pub fn vxlan_vs_del_vnigrp(vxlan: *mut vxlan_dev);
}
// vxlan_multicast.c
extern "C" {
    pub fn vxlan_multicast_join(vxlan: *mut vxlan_dev) -> c_int;
}
extern "C" {
    pub fn vxlan_multicast_leave(vxlan: *mut vxlan_dev) -> c_int;
}
// vxlan_mdb.c
extern "C" {
    pub fn vxlan_mdb_init(vxlan: *mut vxlan_dev) -> c_int;
}
extern "C" {
    pub fn vxlan_mdb_fini(vxlan: *mut vxlan_dev);
}
