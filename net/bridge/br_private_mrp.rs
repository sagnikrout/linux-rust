//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private_mrp.h
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

pub const MRP_OPT_PADDING: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp {
// list of mrp instances
    pub list: hlist_node,
    pub p_port: *mut net_bridge_port __rcu,
    pub s_port: *mut net_bridge_port __rcu,
    pub i_port: *mut net_bridge_port __rcu,
    pub ring_id: u32,
    pub in_id: u16,
    pub prio: u16,
    pub ring_role: br_mrp_ring_role_type,
    pub ring_role_offloaded: u8,
    pub ring_state: br_mrp_ring_state_type,
    pub ring_transitions: u32,
    pub in_role: br_mrp_in_role_type,
    pub in_role_offloaded: u8,
    pub in_state: br_mrp_in_state_type,
    pub in_transitions: u32,
    pub test_work: delayed_work,
    pub test_interval: u32,
    pub test_end: c_ulong,
    pub test_count_miss: u32,
    pub test_max_miss: u32,
    pub test_monitor: bool,
    pub in_test_work: delayed_work,
    pub in_test_interval: u32,
    pub in_test_end: c_ulong,
    pub in_test_count_miss: u32,
    pub in_test_max_miss: u32,
    pub seq_id: u32,
    pub rcu: rcu_head,
}

// This type is returned by br_mrp_switchdev functions that allow to have a SW
// backup in case the HW can't implement completely the protocol.
// BR_MRP_NONE - means the HW can't run at all the protocol, so the SW stops
// configuring the node anymore.
// BR_MRP_SW - the HW can help the SW to run the protocol, by redirecting MRP
// frames to CPU.
// BR_MRP_HW - the HW can implement completely the protocol.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_hw_support {
    BR_MRP_NONE,
    BR_MRP_SW,
    BR_MRP_HW,
}

// br_mrp.c
extern "C" {
    pub fn br_mrp_add(br: *mut net_bridge, instance: *mut br_mrp_instance) -> c_int;
}
extern "C" {
    pub fn br_mrp_del(br: *mut net_bridge, instance: *mut br_mrp_instance) -> c_int;
}
extern "C" {
    pub fn br_mrp_set_ring_role(br: *mut net_bridge, role: *mut br_mrp_ring_role) -> c_int;
}
extern "C" {
    pub fn br_mrp_start_test(br: *mut net_bridge, test: *mut br_mrp_start_test) -> c_int;
}
extern "C" {
    pub fn br_mrp_set_in_state(br: *mut net_bridge, state: *mut br_mrp_in_state) -> c_int;
}
extern "C" {
    pub fn br_mrp_set_in_role(br: *mut net_bridge, role: *mut br_mrp_in_role) -> c_int;
}
// br_mrp_switchdev.c
extern "C" {
    pub fn br_mrp_switchdev_add(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int;
}
extern "C" {
    pub fn br_mrp_switchdev_del(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int;
}
extern "C" {
    pub fn br_mrp_port_switchdev_set_state(p: *mut net_bridge_port, state: u32) -> c_int;
}
// br_mrp_netlink.c
extern "C" {
    pub fn br_mrp_ring_port_open(dev: *mut net_device, loc: u8) -> c_int;
}
extern "C" {
    pub fn br_mrp_in_port_open(dev: *mut net_device, loc: u8) -> c_int;
}
// MRP protocol data units
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_tlv_hdr {
    pub type: __u8,
    pub length: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_common_hdr {
    pub seq_id: __be16,
    pub domain: [__u8; MRP_DOMAIN_UUID_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_ring_test_hdr {
    pub prio: __be16,
    pub sa: [__u8; ETH_ALEN],
    pub port_role: __be16,
    pub state: __be16,
    pub transitions: __be16,
    pub timestamp: __be32,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_in_test_hdr {
    pub id: __be16,
    pub sa: [__u8; ETH_ALEN],
    pub port_role: __be16,
    pub state: __be16,
    pub transitions: __be16,
    pub timestamp: __be32,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_oui_hdr {
    pub oui: [__u8; MRP_OUI_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_sub_option1_hdr {
    pub type: __u8,
    pub data: [__u8; MRP_MANUFACTURE_DATA_LENGTH],
}
