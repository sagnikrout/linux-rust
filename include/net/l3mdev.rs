//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/l3mdev.h
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
// include/net/l3mdev.h - L3 master device API
// Copyright (c) 2015 Cumulus Networks
// Copyright (c) 2015 David Ahern <dsa@cumulusnetworks.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l3mdev_type {
    L3MDEV_TYPE_UNSPEC,
    L3MDEV_TYPE_VRF,
    __L3MDEV_TYPE_MAX
}

extern "C" {
    pub fn int(net: *mut *mut lookup_by_table_id_t)(struct net, table_d: u32) -> typedef;
}
//
// struct l3mdev_ops - l3mdev operations
//
// @l3mdev_fib_table: Get FIB table id to use for lookups
//
// @l3mdev_l3_rcv:    Hook in L3 receive path
//
// @l3mdev_l3_out:    Hook in L3 output path
//
// @l3mdev_link_scope_lookup: IPv6 lookup for linklocal and mcast destinations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l3mdev_ops {
    pub dev): *const *const u32 (l3mdev_fib_table)(struct net_device,
    pub proto): *mut *mut sk_buff skb, u16,
    pub proto): u16,
// IPv6 ops
    pub fl6): *mut flowi6,
}

extern "C" {
    pub fn l3mdev_update_flow(net: *mut net, fl: *mut flowi);
}
extern "C" {
    pub fn l3mdev_master_ifindex_rcu(dev: *const net_device) -> c_int;
}
// netdev_master_upper_dev_get_rcu calls
// list_first_or_null_rcu to walk the upper dev list.
// list_first_or_null_rcu does not handle a const arg. We aren't
// making changes, just want the master device from that list so
// typecast to remove the const
//
extern "C" {
    pub fn l3mdev_master_upper_ifindex_by_index_rcu(net: *mut net, ifindex: c_int) -> c_int;
}
extern "C" {
    pub fn l3mdev_fib_table_rcu(dev: *const net_device) -> u32;
}
extern "C" {
    pub fn l3mdev_fib_table_by_index(net: *mut net, ifindex: c_int) -> u32;
}
extern "C" {
    pub fn l3mdev_l3_rcv(_arg: skb, _arg: AF_INET) -> return;
}
extern "C" {
    pub fn l3mdev_l3_rcv(_arg: skb, _arg: AF_INET6) -> return;
}
extern "C" {
    pub fn l3mdev_l3_out(_arg: sk, _arg: skb, _arg: AF_INET) -> return;
}
extern "C" {
    pub fn l3mdev_l3_out(_arg: sk, _arg: skb, _arg: AF_INET6) -> return;
}

