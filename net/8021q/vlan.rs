//! Automatically rewritten from C Header to Rust Module
//! Source: net/8021q/vlan.h
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

// if this changes, algorithm will have to be reworked because this
// depends on completely exhausting the VLAN identifier space.  Thus
// it gives constant time look-up, but in many cases it wastes memory.
//
pub const VLAN_GROUP_ARRAY_SPLIT_PARTS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlan_protos {
    VLAN_PROTO_8021Q	= 0,
    VLAN_PROTO_8021AD,
    VLAN_PROTO_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_group {
    pub nr_vlan_devs: c_uint,
    pub /: *mut *mut hlist_node hlist; / linked list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_info {
    pub device: *mut *mut *mut net_device real_dev; / The ethernet(like),
// the vlan is attached to.
//
    pub grp: vlan_group,
    pub vid_list: list_head,
    pub nr_vids: c_uint,
    pub auto_vid0: bool,
    pub rcu: rcu_head,
}

// paired with smp_wmb() in vlan_group_prealloc_vid()
extern "C" {
    pub fn __vlan_group_get_device(_arg: vg, _arg: pidx, _arg: vlan_id) -> return;
}
// Must be invoked with rcu_read_lock or with RTNL.

extern "C" {
    pub fn vlan_filter_push_vids(vlan_info: *mut vlan_info, proto: __be16) -> c_int;
}
extern "C" {
    pub fn vlan_filter_drop_vids(vlan_info: *mut vlan_info, proto: __be16);
}
// netdev_work events propagated from the real device, see vlan_dev_work().
// found in vlan_dev.c
extern "C" {
    pub fn vlan_dev_free_egress_priority(dev: *const net_device);
}
extern "C" {
    pub fn vlan_dev_change_flags(dev: *const net_device, flag: u32, mask: u32) -> c_int;
}
extern "C" {
    pub fn vlan_setup(dev: *mut net_device);
}
extern "C" {
    pub fn register_vlan_dev(dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn unregister_vlan_dev(dev: *mut net_device, head: *mut list_head);
}

extern "C" {
    pub fn vlan_gvrp_request_join(dev: *const net_device) -> c_int;
}
extern "C" {
    pub fn vlan_gvrp_request_leave(dev: *const net_device);
}
extern "C" {
    pub fn vlan_gvrp_init_applicant(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn vlan_gvrp_uninit_applicant(dev: *mut net_device);
}
extern "C" {
    pub fn vlan_gvrp_init() -> c_int;
}
extern "C" {
    pub fn vlan_gvrp_uninit();
}

extern "C" {
    pub fn vlan_mvrp_request_join(dev: *const net_device) -> c_int;
}
extern "C" {
    pub fn vlan_mvrp_request_leave(dev: *const net_device);
}
extern "C" {
    pub fn vlan_mvrp_init_applicant(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn vlan_mvrp_uninit_applicant(dev: *mut net_device);
}
extern "C" {
    pub fn vlan_mvrp_init() -> c_int;
}
extern "C" {
    pub fn vlan_mvrp_uninit();
}

extern "C" {
    pub fn vlan_netlink_init() -> c_int;
}
extern "C" {
    pub fn vlan_netlink_fini();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_net {
// /proc/net/vlan
    pub proc_vlan_dir: *mut proc_dir_entry,
// /proc/net/vlan/config
    pub proc_vlan_conf: *mut proc_dir_entry,
// Determines interface naming scheme.
    pub name_type: c_ushort,
}
