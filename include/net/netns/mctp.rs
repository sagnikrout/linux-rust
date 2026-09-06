//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/mctp.h
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
// MCTP per-net structures
//

pub const MCTP_BINDS_BITS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_mctp {
// Only updated under RTNL, entries freed via RCU
    pub routes: list_head,
// Bound sockets: hash table of sockets, keyed by
// (type, src_eid, dest_eid).
// Specific src_eid/dest_eid entries also have an entry for
// MCTP_ADDR_ANY. This list is updated from non-atomic contexts
// (under bind_lock), and read (under rcu) in packet rx.
//
    pub bind_lock: mutex,
    pub MCTP_BINDS_BITS): DECLARE_HASHTABLE(binds,,
// tag allocations. This list is read and updated from atomic contexts,
// but elements are free()ed after a RCU grace-period
//
    pub keys_lock: spinlock_t,
    pub keys: hlist_head,
// MCTP network
    pub default_net: c_uint,
// neighbour table
    pub neigh_lock: mutex,
    pub neighbours: list_head,
}
