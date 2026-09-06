//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_bridge.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const BRCTL_VERSION: c_int = 1;
pub const BRCTL_GET_VERSION: c_int = 0;
pub const BRCTL_GET_BRIDGES: c_int = 1;
pub const BRCTL_ADD_BRIDGE: c_int = 2;
pub const BRCTL_DEL_BRIDGE: c_int = 3;
pub const BRCTL_ADD_IF: c_int = 4;
pub const BRCTL_DEL_IF: c_int = 5;
pub const BRCTL_GET_BRIDGE_INFO: c_int = 6;
pub const BRCTL_GET_PORT_LIST: c_int = 7;
pub const BRCTL_SET_BRIDGE_FORWARD_DELAY: c_int = 8;
pub const BRCTL_SET_BRIDGE_HELLO_TIME: c_int = 9;
pub const BRCTL_SET_BRIDGE_MAX_AGE: c_int = 10;
pub const BRCTL_SET_AGEING_TIME: c_int = 11;
pub const BRCTL_SET_GC_INTERVAL: c_int = 12;
pub const BRCTL_GET_PORT_INFO: c_int = 13;
pub const BRCTL_SET_BRIDGE_STP_STATE: c_int = 14;
pub const BRCTL_SET_BRIDGE_PRIORITY: c_int = 15;
pub const BRCTL_SET_PORT_PRIORITY: c_int = 16;
pub const BRCTL_SET_PATH_COST: c_int = 17;
pub const BRCTL_GET_FDB_ENTRIES: c_int = 18;
pub const BR_STATE_DISABLED: c_int = 0;
pub const BR_STATE_LISTENING: c_int = 1;
pub const BR_STATE_LEARNING: c_int = 2;
pub const BR_STATE_FORWARDING: c_int = 3;
pub const BR_STATE_BLOCKING: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bridge_info {
    pub designated_root: __u64,
    pub bridge_id: __u64,
    pub root_path_cost: __u32,
    pub max_age: __u32,
    pub hello_time: __u32,
    pub forward_delay: __u32,
    pub bridge_max_age: __u32,
    pub bridge_hello_time: __u32,
    pub bridge_forward_delay: __u32,
    pub topology_change: __u8,
    pub topology_change_detected: __u8,
    pub root_port: __u8,
    pub stp_enabled: __u8,
    pub ageing_time: __u32,
    pub gc_interval: __u32,
    pub hello_timer_value: __u32,
    pub tcn_timer_value: __u32,
    pub topology_change_timer_value: __u32,
    pub gc_timer_value: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __port_info {
    pub designated_root: __u64,
    pub designated_bridge: __u64,
    pub port_id: __u16,
    pub designated_port: __u16,
    pub path_cost: __u32,
    pub designated_cost: __u32,
    pub state: __u8,
    pub top_change_ack: __u8,
    pub config_pending: __u8,
    pub unused0: __u8,
    pub message_age_timer_value: __u32,
    pub forward_delay_timer_value: __u32,
    pub hold_timer_value: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fdb_entry {
    pub mac_addr: [__u8; ETH_ALEN],
    pub port_no: __u8,
    pub is_local: __u8,
    pub ageing_timer_value: __u32,
    pub port_hi: __u8,
    pub pad0: __u8,
    pub unused: __u16,
}

// Bridge Flags

pub const BRIDGE_MODE_UNDEF: c_uint = 0xFFFF  /* mode undefined */;
// Bridge management nested attributes
// [IFLA_AF_SPEC] = {
// [IFLA_BRIDGE_FLAGS]
// [IFLA_BRIDGE_MODE]
// [IFLA_BRIDGE_VLAN_INFO]
// }
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_vlan_info {
    pub flags: __u16,
    pub vid: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_vlan_xstats {
    pub rx_bytes: __u64,
    pub rx_packets: __u64,
    pub tx_bytes: __u64,
    pub tx_packets: __u64,
    pub vid: __u16,
    pub flags: __u16,
    pub pad2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_instance {
    pub ring_id: __u32,
    pub p_ifindex: __u32,
    pub s_ifindex: __u32,
    pub prio: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_ring_state {
    pub ring_id: __u32,
    pub ring_state: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_ring_role {
    pub ring_id: __u32,
    pub ring_role: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_start_test {
    pub ring_id: __u32,
    pub interval: __u32,
    pub max_miss: __u32,
    pub period: __u32,
    pub monitor: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_in_state {
    pub in_state: __u32,
    pub in_id: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_in_role {
    pub ring_id: __u32,
    pub in_role: __u32,
    pub i_ifindex: __u32,
    pub in_id: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mrp_start_in_test {
    pub interval: __u32,
    pub max_miss: __u32,
    pub period: __u32,
    pub in_id: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_stp_xstats {
    pub transition_blk: __u64,
    pub transition_fwd: __u64,
    pub rx_bpdu: __u64,
    pub tx_bpdu: __u64,
    pub rx_tcn: __u64,
    pub tx_tcn: __u64,
}

// Bridge vlan RTM header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_vlan_msg {
    pub family: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub ifindex: __u32,
}

// flags used in BRIDGE_VLANDB_DUMP_FLAGS attribute to affect dumps

// Bridge vlan RTM attributes
// [BRIDGE_VLANDB_ENTRY] = {
// [BRIDGE_VLANDB_ENTRY_INFO]
// ...
// }
// [BRIDGE_VLANDB_GLOBAL_OPTIONS] = {
// [BRIDGE_VLANDB_GOPTS_ID]
// ...
// }
//

// [BRIDGE_VLANDB_ENTRY] = {
// [BRIDGE_VLANDB_ENTRY_TUNNEL_INFO] = {
// [BRIDGE_VLANDB_TINFO_ID]
// ...
// }
//

// [BRIDGE_VLANDB_ENTRY] = {
// [BRIDGE_VLANDB_ENTRY_STATS] = {
// [BRIDGE_VLANDB_STATS_RX_BYTES]
// ...
// }
// ...
// }
//

// Bridge multicast database attributes
// [MDBA_MDB] = {
// [MDBA_MDB_ENTRY] = {
// [MDBA_MDB_ENTRY_INFO] {
// struct br_mdb_entry
// [MDBA_MDB_EATTR attributes]
// }
// [MDBA_ROUTER] = {
// [MDBA_ROUTER_PORT] = {
// u32 ifindex
// [MDBA_ROUTER_PATTR attributes]
// }
//

// per mdb entry additional attributes

// per mdb entry source

// per mdb entry per source attributes
// these are embedded in MDBA_MDB_SRCLIST_ENTRY
//

// multicast router types

// router port attributes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_port_msg {
    pub family: __u8,
    pub ifindex: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mdb_entry {
    pub ifindex: __u32,
pub const MDB_TEMPORARY: c_int = 0;
pub const MDB_PERMANENT: c_int = 1;
    pub state: __u8,

    pub flags: __u8,
    pub vid: __u16,
    pub ip4: __be32,
    pub ip6: in6_addr,
    pub mac_addr: [c_uchar; ETH_ALEN],
    pub u: },
    pub proto: __be16,
    pub addr: },
}

// [MDBA_GET_ENTRY] = {
// struct br_mdb_entry
// [MDBA_GET_ENTRY_ATTRS] = {
// [MDBE_ATTR_SOURCE]
// struct in_addr / struct in6_addr
// [MDBE_ATTR_SRC_VNI]
// u32
// }
//

// [MDBA_SET_ENTRY_ATTRS] = {
// [MDBE_ATTR_xxx]
// ...
// }
//

// per mdb entry source

// per mdb entry per source attributes
// these are embedded in MDBE_SRC_LIST_ENTRY
//

// Embedded inside LINK_XSTATS_TYPE_BRIDGE

// IGMP/MLD statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mcast_stats {
    pub igmp_v1queries: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_v2queries: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_v3queries: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_leaves: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_v1reports: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_v2reports: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_v3reports: [__u64; BR_MCAST_DIR_SIZE],
    pub igmp_parse_errors: __u64,
    pub mld_v1queries: [__u64; BR_MCAST_DIR_SIZE],
    pub mld_v2queries: [__u64; BR_MCAST_DIR_SIZE],
    pub mld_leaves: [__u64; BR_MCAST_DIR_SIZE],
    pub mld_v1reports: [__u64; BR_MCAST_DIR_SIZE],
    pub mld_v2reports: [__u64; BR_MCAST_DIR_SIZE],
    pub mld_parse_errors: __u64,
    pub mcast_bytes: [__u64; BR_MCAST_DIR_SIZE],
    pub mcast_packets: [__u64; BR_MCAST_DIR_SIZE],
}

// bridge boolean options
// BR_BOOLOPT_NO_LL_LEARN - disable learning from link-local packets
// BR_BOOLOPT_MCAST_VLAN_SNOOPING - control vlan multicast snooping
// BR_BOOLOPT_FDB_LOCAL_VLAN_0 - local FDB entries installed by the bridge
// driver itself should only be added on VLAN 0
//
// IMPORTANT: if adding a new option do not forget to handle
// it in br_boolopt_toggle/get and bridge sysfs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_boolopt_id {
    BR_BOOLOPT_NO_LL_LEARN,
    BR_BOOLOPT_MCAST_VLAN_SNOOPING,
    BR_BOOLOPT_MST_ENABLE,
    BR_BOOLOPT_MDB_OFFLOAD_FAIL_NOTIFICATION,
    BR_BOOLOPT_FDB_LOCAL_VLAN_0,
    BR_BOOLOPT_MAX
}

// struct br_boolopt_multi - change multiple bridge boolean options
//
// @optval: new option values (bit per option)
// @optmask: options to change (bit per option)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_boolopt_multi {
    pub optval: __u32,
    pub optmask: __u32,
}

