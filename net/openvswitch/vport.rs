//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/vport.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2007-2012 Nicira, Inc.
//
pub const VPORT_H: c_int = 1;

// The following definitions are for users of the vport subsystem:
extern "C" {
    pub fn ovs_vport_init() -> c_int;
}
extern "C" {
    pub fn ovs_vport_exit();
}
extern "C" {
    pub fn ovs_vport_del(: *mut vport);
}
extern "C" {
    pub fn ovs_vport_get_stats(: *mut vport, : *mut ovs_vport_stats);
}
extern "C" {
    pub fn ovs_vport_get_upcall_stats(vport: *mut vport, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_vport_set_upcall_portids(: *mut vport, pids: *const nlattr) -> c_int;
}
extern "C" {
    pub fn ovs_vport_get_upcall_portids(: *const vport, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ovs_vport_find_upcall_portid(: *const vport, : *mut sk_buff) -> u32;
}
//
// struct vport_portids - array of netlink portids of a vport.
// must be protected by rcu.
// @rn_ids: The reciprocal value of @n_ids.
// @rcu: RCU callback head for deferred destruction.
// @n_ids: Size of @ids array.
// @ids: Array storing the Netlink socket pids to be used for packets received
// on this port that miss the flow table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_portids {
    pub rn_ids: reciprocal_value,
    pub rcu: rcu_head,
    pub n_ids: u32,
    pub ids: [u32; ],
}

//
// struct vport - one port within a datapath
// @dev: Pointer to net_device.
// @dev_tracker: refcount tracker for @dev reference
// @dp: Datapath to which this port belongs.
// @upcall_portids: RCU protected 'struct vport_portids'.
// @port_no: Index into @dp's @ports array.
// @hash_node: Element in @dev_table hash table in vport.c.
// @dp_hash_node: Element in @datapath->ports hash table in datapath.c.
// @ops: Class structure.
// @upcall_stats: Upcall stats of every ports.
// @detach_list: list used for detaching vport in net-exit call.
// @rcu: RCU callback head for deferred destruction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub dp: *mut datapath,
    pub upcall_portids: *mut vport_portids __rcu,
    pub port_no: u16,
    pub hash_node: hlist_node,
    pub dp_hash_node: hlist_node,
    pub ops: *const vport_ops,
    pub upcall_stats: *mut vport_upcall_stats_percpu __percpu,
    pub detach_list: list_head,
    pub rcu: rcu_head,
}

//
// struct vport_parms - parameters for creating a new vport
//
// @name: New vport's name.
// @type: New vport's type.
// @desired_ifindex: New vport's ifindex.
// @dp: New vport's datapath.
// @port_no: New vport's port number.
// @upcall_portids: %OVS_VPORT_ATTR_UPCALL_PID attribute from Netlink message,
// %NULL if none was supplied.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_parms {
    pub name: *const c_char,
    pub type: ovs_vport_type,
    pub desired_ifindex: c_int,
// For ovs_vport_alloc().
    pub dp: *mut datapath,
    pub port_no: u16,
    pub upcall_portids: *mut nlattr,
}

//
// struct vport_ops - definition of a type of virtual port
//
// @type: %OVS_VPORT_TYPE_* value for this type of virtual port.
// @create: Create a new vport configured as specified.  On success returns
// a new vport allocated with ovs_vport_alloc(), otherwise an ERR_PTR() value.
// @destroy: Destroys a vport.  Must call vport_free() on the vport but not
// before an RCU grace period has elapsed.
// @send: Send a packet on the device.
// zero for dropped packets or negative for error.
// @list: List entry in the global list of vport types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_ops {
    pub type: ovs_vport_type,
// Called with ovs_mutex.
    pub ): *const *const *const vport (create)(vport_parms,
    pub ): *mut *mut void (destroy)(struct vport,
    pub skb): *mut *mut int (send)(struct sk_buff,
    pub list: list_head,
}

//
// struct vport_upcall_stats_percpu - per-cpu packet upcall statistics for
// a given vport.
// @syncp:     Synchronization point for 64bit counters.
// @n_success: Number of packets that upcall to userspace succeed.
// @n_fail:    Number of packets that upcall to userspace failed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_upcall_stats_percpu {
    pub syncp: u64_stats_sync,
    pub n_success: u64_stats_t,
    pub n_fail: u64_stats_t,
}

extern "C" {
    pub fn ovs_vport_free(: *mut vport);
}
pub const VPORT_ALIGN: c_int = 8;
//
// vport_priv - access private data area of vport
//
// @vport: vport to access
//
// Returns: A void pointer to a private data allocated in the @vport.
//
// If a nonzero size was passed in priv_size of vport_alloc() a private data
// area was allocated on creation.  This allows that area to be accessed and
// used for any purpose needed by the vport implementer.
//
// vport_from_priv - lookup vport from private data pointer
//
// @priv: Start of private data area.
//
// Returns: A reference to a vport structure that contains @priv.
//
// It is sometimes useful to translate from a pointer to the private data
// area to the vport, such as in the case where the private data pointer is
// the result of a hash table lookup.  @priv must point to the start of the
// private data area.
//
extern "C" {
    pub fn ovs_vport_ops_register(ops: *mut vport_ops) -> c_int;
}
extern "C" {
    pub fn ovs_vport_ops_unregister(ops: *mut vport_ops);
}
extern "C" {
    pub fn ovs_vport_send(vport: *mut vport, skb: *mut sk_buff, mac_proto: u8);
}
