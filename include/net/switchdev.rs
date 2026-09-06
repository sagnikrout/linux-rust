//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/switchdev.h
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
// include/net/switchdev.h - Switch device API
// Copyright (c) 2014-2015 Jiri Pirko <jiri@resnulli.us>
// Copyright (c) 2014-2015 Scott Feldman <sfeldma@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum switchdev_attr_id {
    SWITCHDEV_ATTR_ID_UNDEFINED,
    SWITCHDEV_ATTR_ID_PORT_STP_STATE,
    SWITCHDEV_ATTR_ID_PORT_MST_STATE,
    SWITCHDEV_ATTR_ID_PORT_BRIDGE_FLAGS,
    SWITCHDEV_ATTR_ID_PORT_PRE_BRIDGE_FLAGS,
    SWITCHDEV_ATTR_ID_PORT_MROUTER,
    SWITCHDEV_ATTR_ID_BRIDGE_AGEING_TIME,
    SWITCHDEV_ATTR_ID_BRIDGE_VLAN_FILTERING,
    SWITCHDEV_ATTR_ID_BRIDGE_VLAN_PROTOCOL,
    SWITCHDEV_ATTR_ID_BRIDGE_MC_DISABLED,
    SWITCHDEV_ATTR_ID_BRIDGE_MROUTER,
    SWITCHDEV_ATTR_ID_BRIDGE_MST,
    SWITCHDEV_ATTR_ID_MRP_PORT_ROLE,
    SWITCHDEV_ATTR_ID_VLAN_MSTI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_mst_state {
    pub msti: u16,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_brport_flags {
    pub val: c_ulong,
    pub mask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_vlan_msti {
    pub vid: u16,
    pub msti: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_attr {
    pub orig_dev: *mut net_device,
    pub id: switchdev_attr_id,
    pub flags: u32,
    pub complete_priv: *mut c_void,
    pub priv): *mut *mut *mut void (complete)(struct net_device dev, int err, void,
    pub /: *mut *mut u8 stp_state; / PORT_STP_STATE,
    pub /: *mut *mut switchdev_mst_state mst_state; / PORT_MST_STATE,
    pub /: *mut *mut switchdev_brport_flags brport_flags; / PORT_BRIDGE_FLAGS,
    pub /: *mut *mut bool mrouter; / PORT_MROUTER,
    pub /: *mut *mut clock_t ageing_time; / BRIDGE_AGEING_TIME,
    pub /: *mut *mut bool vlan_filtering; / BRIDGE_VLAN_FILTERING,
    pub /: *mut *mut u16 vlan_protocol; / BRIDGE_VLAN_PROTOCOL,
    pub /: *mut *mut bool mst; / BRIDGE_MST,
    pub /: *mut *mut bool mc_disabled; / MC_DISABLED,
    pub /: *mut *mut u8 mrp_port_role; / MRP_PORT_ROLE,
    pub /: *mut *mut switchdev_vlan_msti vlan_msti; / VLAN_MSTI,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum switchdev_obj_id {
    SWITCHDEV_OBJ_ID_UNDEFINED,
    SWITCHDEV_OBJ_ID_PORT_VLAN,
    SWITCHDEV_OBJ_ID_PORT_MDB,
    SWITCHDEV_OBJ_ID_HOST_MDB,
    SWITCHDEV_OBJ_ID_MRP,
    SWITCHDEV_OBJ_ID_RING_TEST_MRP,
    SWITCHDEV_OBJ_ID_RING_ROLE_MRP,
    SWITCHDEV_OBJ_ID_RING_STATE_MRP,
    SWITCHDEV_OBJ_ID_IN_TEST_MRP,
    SWITCHDEV_OBJ_ID_IN_ROLE_MRP,
    SWITCHDEV_OBJ_ID_IN_STATE_MRP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj {
    pub list: list_head,
    pub orig_dev: *mut net_device,
    pub id: switchdev_obj_id,
    pub flags: u32,
    pub complete_priv: *mut c_void,
    pub priv): *mut *mut *mut void (complete)(struct net_device dev, int err, void,
}

// SWITCHDEV_OBJ_ID_PORT_VLAN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_port_vlan {
    pub obj: switchdev_obj,
    pub flags: u16,
    pub vid: u16,
// If set, the notifier signifies a change of one of the following
// flags for a VLAN that already exists:
// - BRIDGE_VLAN_INFO_PVID
// - BRIDGE_VLAN_INFO_UNTAGGED
// Entries with BRIDGE_VLAN_INFO_BRENTRY unset are not notified at all.
//
    pub changed: bool,
}

// SWITCHDEV_OBJ_ID_PORT_MDB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_port_mdb {
    pub obj: switchdev_obj,
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
}

// SWITCHDEV_OBJ_ID_MRP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_mrp {
    pub obj: switchdev_obj,
    pub p_port: *mut net_device,
    pub s_port: *mut net_device,
    pub ring_id: u32,
    pub prio: u16,
}

// SWITCHDEV_OBJ_ID_RING_TEST_MRP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_ring_test_mrp {
    pub obj: switchdev_obj,
// The value is in us and a value of 0 represents to stop
    pub interval: u32,
    pub max_miss: u8,
    pub ring_id: u32,
    pub period: u32,
    pub monitor: bool,
}

// SWICHDEV_OBJ_ID_RING_ROLE_MRP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_ring_role_mrp {
    pub obj: switchdev_obj,
    pub ring_role: u8,
    pub ring_id: u32,
    pub sw_backup: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_ring_state_mrp {
    pub obj: switchdev_obj,
    pub ring_state: u8,
    pub ring_id: u32,
}

// SWITCHDEV_OBJ_ID_IN_TEST_MRP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_in_test_mrp {
    pub obj: switchdev_obj,
// The value is in us and a value of 0 represents to stop
    pub interval: u32,
    pub in_id: u32,
    pub period: u32,
    pub max_miss: u8,
}

// SWICHDEV_OBJ_ID_IN_ROLE_MRP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_in_role_mrp {
    pub obj: switchdev_obj,
    pub i_port: *mut net_device,
    pub ring_id: u32,
    pub in_id: u16,
    pub in_role: u8,
    pub sw_backup: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_obj_in_state_mrp {
    pub obj: switchdev_obj,
    pub in_id: u32,
    pub in_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_brport {
    pub dev: *mut net_device,
    pub ctx: *const c_void,
    pub atomic_nb: *mut notifier_block,
    pub blocking_nb: *mut notifier_block,
    pub tx_fwd_offload: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum switchdev_notifier_type {
    SWITCHDEV_FDB_ADD_TO_BRIDGE = 1,
    SWITCHDEV_FDB_DEL_TO_BRIDGE,
    SWITCHDEV_FDB_ADD_TO_DEVICE,
    SWITCHDEV_FDB_DEL_TO_DEVICE,
    SWITCHDEV_FDB_OFFLOADED,
    SWITCHDEV_FDB_FLUSH_TO_BRIDGE,

    SWITCHDEV_PORT_OBJ_ADD, /* Blocking. */
    SWITCHDEV_PORT_OBJ_DEL, /* Blocking. */
    SWITCHDEV_PORT_ATTR_SET, /* May be blocking . */

    SWITCHDEV_VXLAN_FDB_ADD_TO_BRIDGE,
    SWITCHDEV_VXLAN_FDB_DEL_TO_BRIDGE,
    SWITCHDEV_VXLAN_FDB_ADD_TO_DEVICE,
    SWITCHDEV_VXLAN_FDB_DEL_TO_DEVICE,
    SWITCHDEV_VXLAN_FDB_OFFLOADED,

    SWITCHDEV_BRPORT_OFFLOADED,
    SWITCHDEV_BRPORT_UNOFFLOADED,
    SWITCHDEV_BRPORT_REPLAY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_info {
    pub dev: *mut net_device,
    pub extack: *mut netlink_ext_ack,
    pub ctx: *const c_void,
}

// Remember to update br_switchdev_fdb_populate() when adding
// new members to this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_fdb_info {
    pub /: *mut *mut switchdev_notifier_info info; / must be first,
    pub addr: *const c_uchar,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_port_obj_info {
    pub /: *mut *mut switchdev_notifier_info info; / must be first,
    pub obj: *const switchdev_obj,
    pub handled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_port_attr_info {
    pub /: *mut *mut switchdev_notifier_info info; / must be first,
    pub attr: *const switchdev_attr,
    pub handled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchdev_notifier_brport_info {
    pub /: *mut *mut switchdev_notifier_info info; / must be first,
    pub brport: switchdev_brport,
}

extern "C" {
    pub fn switchdev_deferred_process();
}
extern "C" {
    pub fn register_switchdev_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_switchdev_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn register_switchdev_blocking_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_switchdev_blocking_notifier(nb: *mut notifier_block) -> c_int;
}

