//! Automatically rewritten from C Header to Rust Module
//! Source: net/dsa/switch.h
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

// DSA_NOTIFIER_AGEING_TIME
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_ageing_time_info {
    pub ageing_time: c_uint,
}

// DSA_NOTIFIER_BRIDGE_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_bridge_info {
    pub dp: *const dsa_port,
    pub bridge: dsa_bridge,
    pub tx_fwd_offload: bool,
    pub extack: *mut netlink_ext_ack,
}

// DSA_NOTIFIER_FDB_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_fdb_info {
    pub dp: *const dsa_port,
    pub addr: *const c_uchar,
    pub vid: u16,
    pub db: dsa_db,
}

// DSA_NOTIFIER_LAG_FDB_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_lag_fdb_info {
    pub lag: *mut dsa_lag,
    pub addr: *const c_uchar,
    pub vid: u16,
    pub db: dsa_db,
}

// DSA_NOTIFIER_MDB_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_mdb_info {
    pub dp: *const dsa_port,
    pub mdb: *const switchdev_obj_port_mdb,
    pub db: dsa_db,
}

// DSA_NOTIFIER_LAG_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_lag_info {
    pub dp: *const dsa_port,
    pub lag: dsa_lag,
    pub info: *mut netdev_lag_upper_info,
    pub extack: *mut netlink_ext_ack,
}

// DSA_NOTIFIER_VLAN_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_vlan_info {
    pub dp: *const dsa_port,
    pub vlan: *const switchdev_obj_port_vlan,
    pub extack: *mut netlink_ext_ack,
}

// DSA_NOTIFIER_MTU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_mtu_info {
    pub dp: *const dsa_port,
    pub mtu: c_int,
}

// DSA_NOTIFIER_TAG_PROTO_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_tag_proto_info {
    pub tag_ops: *const dsa_device_ops,
}

// DSA_NOTIFIER_TAG_8021Q_VLAN_*
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_tag_8021q_vlan_info {
    pub dp: *const dsa_port,
    pub vid: u16,
}

// DSA_NOTIFIER_CONDUIT_STATE_CHANGE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_notifier_conduit_state_info {
    pub conduit: *const net_device,
    pub operational: bool,
}

extern "C" {
    pub fn dsa_tree_notify(dst: *mut dsa_switch_tree, e: c_ulong, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dsa_broadcast(e: c_ulong, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dsa_switch_register_notifier(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn dsa_switch_unregister_notifier(ds: *mut dsa_switch);
}
