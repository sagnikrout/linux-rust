//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera_router_hw.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2019-2021 Marvell International Ltd. All rights reserved.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_vr {
    pub router_node: list_head,
    pub refcount: refcount_t,
    pub /: *mut *mut u32 tb_id; / key (kernel fib table id),
    pub /: *mut *mut u16 hw_vr_id; / virtual router ID,
    pub __pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_rif_entry {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_rif_entry_key {
    pub iface: prestera_iface,
    pub key: },
    pub vr: *mut prestera_vr,
    pub addr: [c_uchar; ETH_ALEN],
    pub /: *mut *mut u16 hw_id; / rif_id,
    pub /: *mut *mut list_head router_node; / ht,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_ip_addr {
    pub ipv4: __be32,
    pub ipv6: in6_addr,
    pub u: },
    pub v: },

// (V) == PRESTERA_IPV6 ? */ 128 /* : 0 */)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_nh_neigh_key {
    pub addr: prestera_ip_addr,
// Seems like rif is obsolete, because there is iface in info ?
// Key can contain functional fields, or fields, which is used to
// filter duplicate objects on logical level (before you pass it to
// HW)... also key can be used to cover hardware restrictions.
// In our case rif - is logical interface (even can be VLAN), which
// is used in combination with IP address (which is also not related to
// hardware nexthop) to provide logical compression of created nexthops.
// You even can imagine, that rif+IPaddr is just cookie.
//
// struct prestera_rif *rif;
// Use just as cookie, to divide ARP domains (in order with addr)
    pub rif: *mut c_void,
}

// Used for hw call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_neigh_info {
    pub iface: prestera_iface,
    pub ha: [c_uchar; ETH_ALEN],
    pub /: *mut *mut u8 connected; / bool. indicate, if mac/oif valid,
    pub __pad: [u8; 1],
}

// Used to notify nh about neigh change
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_nh_neigh {
    pub key: prestera_nh_neigh_key,
    pub info: prestera_neigh_info,
    pub /: *mut *mut rhash_head ht_node; / node of prestera_vr,
    pub nexthop_group_list: list_head,
}

pub const PRESTERA_NHGR_SIZE_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_nexthop_group {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_nexthop_group_key {
    pub neigh: [prestera_nh_neigh_key; PRESTERA_NHGR_SIZE_MAX],
    pub key: },
// Store intermediate object here.
// This prevent overhead kzalloc call.
//
// nh_neigh is used only to notify nexthop_group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_nh_neigh_head {
    pub this: *mut prestera_nexthop_group,
    pub head: list_head,
// ptr to neigh is not necessary.
// It used to prevent lookup of nh_neigh by key (n) on destroy
//
    pub neigh: *mut prestera_nh_neigh,
    pub nh_neigh_head: [}; PRESTERA_NHGR_SIZE_MAX],
    pub /: *mut *mut rhash_head ht_node; / node of prestera_vr,
    pub refcount: refcount_t,
    pub /: *mut *mut u32 grp_id; / hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fib_key {
    pub addr: prestera_ip_addr,
    pub prefix_len: u32,
    pub tb_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fib_info {
    pub vr: *mut prestera_vr,
    pub vr_node: list_head,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_fib_type {
    PRESTERA_FIB_TYPE_INVALID = 0,
// must be pointer to nh_grp id
    PRESTERA_FIB_TYPE_UC_NH,
// It can be connected route
// and will be overlapped with neighbours
//
    PRESTERA_FIB_TYPE_TRAP,
    PRESTERA_FIB_TYPE_DROP
    } type;
// Valid only if type = UC_NH
    struct prestera_nexthop_group *nh_grp;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fib_node {
    pub /: *mut *mut rhash_head ht_node; / node of prestera_vr,
    pub key: prestera_fib_key,
    pub /: *mut *mut prestera_fib_info info; / action related info,
}

extern "C" {
    pub fn prestera_router_hw_init(sw: *mut prestera_switch) -> c_int;
}
extern "C" {
    pub fn prestera_router_hw_fini(sw: *mut prestera_switch);
}
