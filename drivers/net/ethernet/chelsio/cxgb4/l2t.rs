//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/l2t.h
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


//
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2014 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const VLAN_NONE: c_uint = 0xfff;
// when state is one of the below the entry is not hashed
//
// Each L2T entry plays multiple roles.  First of all, it keeps state for the
// corresponding entry of the HW L2 table and maintains a queue of offload
// packets awaiting address resolution.  Second, it is a node of a hash table
// chain, where the nodes of the chain are linked together through their next
// pointer.  Finally, each node is a bucket of a hash table, pointing to the
// first element in its chain through its first pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2t_entry {
    pub /: *mut *mut u16 state; / entry state,
    pub /: *mut *mut u16 idx; / entry index within in-memory table,
    pub /: *mut *mut u32 addr[4]; / next hop IP or IPv6 address,
    pub /: *mut *mut int ifindex; / neighbor's net_device's ifindex,
    pub /: *mut *mut *mut neighbour neigh; / associated neighbour,
    pub /: *mut *mut *mut l2t_entry first; / start of hash chain,
    pub /: *mut *mut *mut l2t_entry next; / next l2t_entry on chain,
    pub /: *mut *mut sk_buff_head arpq; / packet queue awaiting resolution,
    pub lock: spinlock_t,
    pub /: *mut *mut atomic_t refcnt; / entry reference count,
    pub /: *mut *mut u16 hash; / hash bucket the entry is on,
    pub /: *mut *mut u16 vlan; / VLAN TCI (id: bits 0-11, prio: 13-15,
    pub /: *mut *mut u8 v6; / whether entry is for IPv6,
    pub /: *mut *mut u8 lport; / associated offload logical interface,
    pub /: *mut *mut u8 dmac[ETH_ALEN]; / neighbour's MAC address,
}

extern "C" {
    pub fn void(handle: *mut *mut arp_err_handler_t)(void, skb: *mut sk_buff) -> typedef;
}
//
// Callback stored in an skb to handle address resolution failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2t_skb_cb {
    pub handle: *mut c_void,
    pub arp_err_handler: arp_err_handler_t,
}

extern "C" {
    pub fn cxgb4_l2t_release(e: *mut l2t_entry);
}
extern "C" {
    pub fn t4_l2t_update(adap: *mut adapter, neigh: *mut neighbour);
}
extern "C" {
    pub fn do_l2t_write_rpl(p: *mut adapter, rpl: *const cpl_l2t_write_rpl);
}
extern "C" {
    pub fn cxgb4_check_l2t_valid(e: *mut l2t_entry) -> bool;
}
