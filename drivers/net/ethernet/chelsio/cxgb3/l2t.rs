//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/l2t.h
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
// Copyright (c) 2003-2008 Chelsio, Inc. All rights reserved.
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
    pub /: *mut *mut u16 idx; / entry index,
    pub /: *mut *mut u32 addr; / dest IP address,
    pub /: *mut *mut int ifindex; / neighbor's net_device's ifindex,
    pub /: *mut *mut u16 smt_idx; / SMT index,
    pub /: *mut *mut u16 vlan; / VLAN TCI (id: bits 0-11, prio: 13-15,
    pub /: *mut *mut *mut neighbour neigh; / associated neighbour,
    pub /: *mut *mut *mut l2t_entry first; / start of hash chain,
    pub /: *mut *mut *mut l2t_entry next; / next l2t_entry on chain,
    pub /: *mut *mut sk_buff_head arpq; / queue of packets awaiting resolution,
    pub lock: spinlock_t,
    pub /: *mut *mut atomic_t refcnt; / entry reference count,
    pub /: *mut *mut u8 dmac[6]; / neighbour's MAC address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2t_data {
    pub /: *mut *mut unsigned int nentries; / number of entries,
    pub /: *mut *mut *mut l2t_entry rover; / starting point for next allocation,
    pub /: *mut *mut atomic_t nfree; / number of free entries,
    pub lock: rwlock_t,
    pub /: *mut *mut rcu_head rcu_head; / to handle rcu cleanup,
    pub __counted_by(nentries): l2t_entry l2tab[],
}

//
// Callback stored in an skb to handle address resolution failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2t_skb_cb {
    pub arp_failure_handler: arp_failure_handler_func,
}

//
// Getting to the L2 data from an offload device.
//

pub const W_TCB_L2T_IX: c_int = 0;
pub const S_TCB_L2T_IX: c_int = 7;
pub const M_TCB_L2T_IX: c_uint = 0x7ffULL;

extern "C" {
    pub fn t3_l2e_free(d: *mut l2t_data, e: *mut l2t_entry);
}
extern "C" {
    pub fn t3_l2t_update(dev: *mut t3cdev, neigh: *mut neighbour);
}
extern "C" {
    pub fn cxgb3_ofld_send(dev: *mut t3cdev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cxgb3_ofld_send(_arg: dev, _arg: skb) -> return;
}
extern "C" {
    pub fn t3_l2t_send_slow(_arg: dev, _arg: skb, _arg: e) -> return;
}
