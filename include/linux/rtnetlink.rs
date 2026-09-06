//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtnetlink.h
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

extern "C" {
    pub fn rtnetlink_send(skb: *mut sk_buff, net: *mut net, pid: u32, group: u32, echo: c_int) -> c_int;
}
extern "C" {
    pub fn rtnl_unicast(skb: *mut sk_buff, net: *mut net, pid: u32) -> c_int;
}
extern "C" {
    pub fn rtnl_set_sk_err(net: *mut net, group: u32, error: c_int);
}
extern "C" {
    pub fn rtnetlink_put_metrics(skb: *mut sk_buff, metrics: *mut u32) -> c_int;
}
// RTNL is used as a global lock for all changes to network configuration
extern "C" {
    pub fn rtnl_lock();
}
extern "C" {
    pub fn rtnl_unlock();
}
extern "C" {
    pub fn rtnl_trylock() -> c_int;
}
extern "C" {
    pub fn rtnl_is_locked() -> c_int;
}
extern "C" {
    pub fn rtnl_lock_interruptible() -> c_int;
}
extern "C" {
    pub fn rtnl_lock_killable() -> c_int;
}
extern "C" {
    pub fn refcount_dec_and_rtnl_lock(r: *mut refcount_t) -> bool;
}

extern "C" {
    pub fn lockdep_rtnl_is_held() -> bool;
}

//
// rcu_dereference_rtnl - rcu_dereference with debug checking
// @p: The pointer to read, prior to dereferencing
//
// Do an rcu_dereference(p), but check caller either holds rcu_read_lock()
// or RTNL. Note : Please prefer rtnl_dereference() or rcu_dereference()
//

//
// rtnl_dereference - fetch RCU pointer when updates are prevented by RTNL
// @p: The pointer to read, prior to dereferencing
//
// Return: the value of the specified RCU-protected pointer, but omit
// the READ_ONCE(), because caller holds RTNL.
//

//
// rcu_replace_pointer_rtnl - replace an RCU pointer under rtnl_lock, returning
// its old value
// @rp: RCU pointer, whose value is returned
// @p: regular pointer
//
// Perform a replacement under rtnl_lock, where @rp is an RCU-annotated
// pointer. The old value of @rp is returned, and @rp is set to @p
//

extern "C" {
    pub fn __rtnl_net_lock(net: *mut net);
}
extern "C" {
    pub fn __rtnl_net_unlock(net: *mut net);
}
extern "C" {
    pub fn rtnl_net_lock(net: *mut net);
}
extern "C" {
    pub fn rtnl_net_unlock(net: *mut net);
}
extern "C" {
    pub fn rtnl_net_trylock(net: *mut net) -> c_int;
}
extern "C" {
    pub fn rtnl_net_lock_killable(net: *mut net) -> c_int;
}
extern "C" {
    pub fn rtnl_net_lock_cmp_fn(a: *const lockdep_map, b: *const lockdep_map) -> c_int;
}
extern "C" {
    pub fn rtnl_net_is_locked(net: *mut net) -> bool;
}

extern "C" {
    pub fn lockdep_rtnl_net_is_held(net: *mut net) -> bool;
}
extern "C" {
    pub fn rtnl_net_queue_work(net: *mut net);
}
extern "C" {
    pub fn rtnl_net_flush_workqueue();
}
extern "C" {
    pub fn rtnl_net_work_func(work: *mut work_struct);
}

extern "C" {
    pub fn rtnl_trylock() -> return;
}
extern "C" {
    pub fn rtnl_lock_killable() -> return;
}

extern "C" {
    pub fn rtnl_dereference(_arg: dev->ingress_queue) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: dev->ingress_queue) -> return;
}

extern "C" {
    pub fn net_inc_ingress_queue();
}
extern "C" {
    pub fn net_dec_ingress_queue();
}

extern "C" {
    pub fn net_inc_egress_queue();
}
extern "C" {
    pub fn net_dec_egress_queue();
}
extern "C" {
    pub fn netdev_xmit_skip_txqueue(skip: bool);
}

extern "C" {
    pub fn rtnetlink_init();
}
extern "C" {
    pub fn __rtnl_unlock();
}
extern "C" {
    pub fn rtnl_kfree_skbs(head: *mut sk_buff, tail: *mut sk_buff);
}
// Shared by rtnl_fdb_dump() and various ndo_fdb_dump() helpers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndo_fdb_dump_context {
    pub ifindex: c_ulong,
    pub fdb_idx: c_ulong,
}

extern "C" {
    pub fn rtnl_offload_xstats_notify(dev: *mut net_device);
}
extern "C" {
    pub fn netlink_has_listeners(_arg: rtnl, _arg: group) -> return;
}
//
// rtnl_notify_needed - check if notification is needed
// @net: Pointer to the net namespace
// @nlflags: netlink ingress message flags
// @group: rtnl group
//
// Based on the ingress message flags and rtnl group, returns true
// if a notification is needed, false otherwise.
//
extern "C" {
    pub fn netif_set_operstate(dev: *mut net_device, newstate: c_int);
}
