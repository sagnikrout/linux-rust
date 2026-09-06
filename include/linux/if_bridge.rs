//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_bridge.h
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
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_ip {
    pub ip4: __be32,

    pub ip6: in6_addr,

    pub src: },
    pub ip4: __be32,

    pub ip6: in6_addr,
    pub mac_addr: [c_uchar; ETH_ALEN],
    pub dst: },
    pub proto: __be16,
    pub vid: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_ip_list {
    pub list: list_head,
    pub addr: br_ip,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bridge_flags_bit {
    BR_HAIRPIN_MODE_BIT,
    BR_BPDU_GUARD_BIT,
    BR_ROOT_BLOCK_BIT,
    BR_MULTICAST_FAST_LEAVE_BIT,
    BR_ADMIN_COST_BIT,
    BR_LEARNING_BIT,
    BR_FLOOD_BIT,
    BR_PROMISC_BIT,
    BR_PROXYARP_BIT,
    BR_LEARNING_SYNC_BIT,
    BR_PROXYARP_WIFI_BIT,
    BR_MCAST_FLOOD_BIT,
    BR_MULTICAST_TO_UNICAST_BIT,
    BR_VLAN_TUNNEL_BIT,
    BR_BCAST_FLOOD_BIT,
    BR_NEIGH_SUPPRESS_BIT,
    BR_ISOLATED_BIT,
    BR_MRP_AWARE_BIT,
    BR_MRP_LOST_CONT_BIT,
    BR_MRP_LOST_IN_CONT_BIT,
    BR_TX_FWD_OFFLOAD_BIT,
    BR_PORT_LOCKED_BIT,
    BR_PORT_MAB_BIT,
    BR_NEIGH_VLAN_SUPPRESS_BIT,
    BR_NEIGH_FORWARD_GRAT_BIT,
}

extern "C" {
    pub fn br_ioctl_call(net: *mut net, cmd: c_uint, uarg: *mut void __user) -> c_int;
}

extern "C" {
    pub fn br_multicast_has_querier_anywhere(dev: *mut net_device, proto: c_int) -> bool;
}
extern "C" {
    pub fn br_multicast_has_querier_adjacent(dev: *mut net_device, proto: c_int) -> bool;
}
extern "C" {
    pub fn br_multicast_has_router_adjacent(dev: *mut net_device, proto: c_int) -> bool;
}
extern "C" {
    pub fn br_multicast_enabled(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn br_multicast_router(dev: *const net_device) -> bool;
}

extern "C" {
    pub fn br_vlan_enabled(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn br_vlan_get_pvid(dev: *const net_device, p_pvid: *mut u16) -> c_int;
}
extern "C" {
    pub fn br_vlan_get_pvid_rcu(dev: *const net_device, p_pvid: *mut u16) -> c_int;
}
extern "C" {
    pub fn br_vlan_get_proto(dev: *const net_device, p_proto: *mut u16) -> c_int;
}
extern "C" {
    pub fn br_mst_enabled(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn br_mst_get_info(dev: *const net_device, msti: u16, vids: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn br_mst_get_state(dev: *const net_device, msti: u16, state: *mut u8) -> c_int;
}

extern "C" {
    pub fn br_fdb_clear_offload(dev: *const net_device, vid: u16);
}
extern "C" {
    pub fn br_port_flag_is_set(dev: *const net_device, flag: c_ulong) -> bool;
}
extern "C" {
    pub fn br_port_get_stp_state(dev: *const net_device) -> u8;
}
extern "C" {
    pub fn br_get_ageing_time(br_dev: *const net_device) -> clock_t;
}

