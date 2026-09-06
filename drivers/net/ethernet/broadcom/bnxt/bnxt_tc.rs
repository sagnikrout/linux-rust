//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_tc.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2017 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

// Structs used for storing the filter/actions of the TC cmd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_l2_key {
    pub dmac: [u8; ETH_ALEN],
    pub smac: [u8; ETH_ALEN],
    pub inner_vlan_tpid: __be16,
    pub inner_vlan_tci: __be16,
    pub ether_type: __be16,
    pub num_vlans: u8,
    pub dir: u8,
pub const BNXT_DIR_RX: c_int = 1;
pub const BNXT_DIR_TX: c_int = 0;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_l3_key {
    pub daddr: in_addr,
    pub saddr: in_addr,
    pub ipv4: },
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub ipv6: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_l4_key {
    pub ip_proto: u8,
    pub sport: __be16,
    pub dport: __be16,
    pub ports: },
    pub type: u8,
    pub code: u8,
    pub icmp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_tunnel_key {
    pub l2: bnxt_tc_l2_key,
    pub l3: bnxt_tc_l3_key,
    pub l4: bnxt_tc_l4_key,
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_actions {
    pub flags: u32,

    pub dst_fid: u16,
    pub dst_dev: *mut net_device,
    pub push_vlan_tpid: __be16,
    pub push_vlan_tci: __be16,
// tunnel encap
    pub tun_encap_key: ip_tunnel_key,
pub const PEDIT_OFFSET_SMAC_LAST_4_BYTES: c_uint = 0x8;
    pub l2_rewrite_dmac: [__be16; 3],
    pub l2_rewrite_smac: [__be16; 3],
    pub src,: *mut *mut bool src_xlate; / true => translate,
// false => translate dst
// Mutually exclusive, i.e cannot set both
//
    pub /: *mut *mut bool l3_is_ipv4; / false means L3 is ipv6,
    pub l3: bnxt_tc_l3_key,
    pub l4: bnxt_tc_l4_key,
    pub nat: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_flow {
    pub flags: u32,

// flow applicable to pkts ingressing on this fid
    pub src_fid: u16,
    pub l2_key: bnxt_tc_l2_key,
    pub l2_mask: bnxt_tc_l2_key,
    pub l3_key: bnxt_tc_l3_key,
    pub l3_mask: bnxt_tc_l3_key,
    pub l4_key: bnxt_tc_l4_key,
    pub l4_mask: bnxt_tc_l4_key,
    pub tun_key: ip_tunnel_key,
    pub tun_mask: ip_tunnel_key,
    pub actions: bnxt_tc_actions,
// updated stats accounting for hw-counter wrap-around
    pub stats: bnxt_tc_flow_stats,
// previous snap-shot of stats
    pub prev_stats: bnxt_tc_flow_stats,
    pub /: *mut *mut unsigned long lastused; / jiffies,
// for calculating delta from prev_stats and
// updating prev_stats atomically.
//
    pub stats_lock: spinlock_t,
}

// Tunnel encap/decap hash table
// This table is used to maintain a list of flows that use
// the same tunnel encap/decap params (ip_daddrs, vni, udp_dport)
// and the FW returned handle.
// A separate table is maintained for encap and decap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_tunnel_node {
    pub key: ip_tunnel_key,
    pub node: rhash_head,
// tunnel l2 info
    pub l2_info: bnxt_tc_l2_key,

// tunnel handle returned by FW
    pub tunnel_handle: __le32,
    pub refcount: u32,
    pub rcu: rcu_head,
}

// L2 hash table
// The same data-struct is used for L2-flow table and L2-tunnel table.
// The L2 part of a flow or tunnel is stored in a hash table.
// A flow that shares the same L2 key/mask with an
// already existing flow/tunnel must refer to it's flow handle or
// decap_filter_id respectively.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_l2_node {
// hash key: first 16b of key
pub const BNXT_TC_L2_KEY_LEN: c_int = 16;
    pub key: bnxt_tc_l2_key,
    pub node: rhash_head,
// a linked list of flows that share the same l2 key
    pub common_l2_flows: list_head,
// number of flows/tunnels sharing the l2 key
    pub refcount: u16,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_flow_node {
// hash key: provided by TC
    pub cookie: c_ulong,
    pub node: rhash_head,
    pub flow: bnxt_tc_flow,
    pub ext_flow_handle: __le64,
    pub flow_handle: __le16,
    pub flow_id: __le32,
// L2 node in l2 hashtable that shares flow's l2 key
    pub l2_node: *mut bnxt_tc_l2_node,
// for the shared_flows list maintained in l2_node
    pub l2_list_node: list_head,
// tunnel encap related
    pub encap_node: *mut bnxt_tc_tunnel_node,
// tunnel decap related
    pub decap_node: *mut bnxt_tc_tunnel_node,
// L2 node in tunnel-l2 hashtable that shares flow's tunnel l2 key
    pub decap_l2_node: *mut bnxt_tc_l2_node,
// for the shared_flows list maintained in tunnel decap l2_node
    pub decap_l2_list_node: list_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn bnxt_init_tc(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_shutdown_tc(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_tc_flow_stats_work(bp: *mut bnxt);
}

