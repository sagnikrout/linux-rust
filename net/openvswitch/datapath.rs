//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/datapath.h
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
// Copyright (c) 2007-2014 Nicira, Inc.
//
pub const DATAPATH_H: c_int = 1;

pub const DP_VPORT_HASH_BUCKETS: c_int = 1024;
pub const DP_MASKS_REBALANCE_INTERVAL: c_int = 4000;
//
// struct dp_stats_percpu - per-cpu packet processing statistics for a given
// datapath.
// @n_hit: Number of received packets for which a matching flow was found in
// the flow table.
// @n_missed: Number of received packets that had no matching flow in the flow
// table.  The sum of @n_hit and @n_missed is the number of packets that have
// been received by the datapath.
// @n_lost: Number of received packets that had no matching flow in the flow
// table that could not be sent to userspace (normally due to an overflow in
// one of the datapath's queues).
// @n_mask_hit: Number of masks looked up for flow match.
// @n_mask_hit / (@n_hit + @n_missed)  will be the average masks looked
// up per packet.
// @n_cache_hit: The number of received packets that had their mask found using
// the mask cache.
// @syncp: Synchronization point for 64bit counters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_stats_percpu {
    pub n_hit: u64,
    pub n_missed: u64,
    pub n_lost: u64,
    pub n_mask_hit: u64,
    pub n_cache_hit: u64,
    pub syncp: u64_stats_sync,
}

//
// struct dp_nlsk_pids - array of netlink portids of for a datapath.
// This is used when OVS_DP_F_DISPATCH_UPCALL_PER_CPU
// is enabled and must be protected by rcu.
// @rcu: RCU callback head for deferred destruction.
// @n_pids: Size of @pids array.
// @pids: Array storing the Netlink socket PIDs indexed by CPU ID for packets
// that miss the flow table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_nlsk_pids {
    pub rcu: rcu_head,
    pub n_pids: u32,
    pub pids: [u32; ],
}

//
// struct datapath - datapath for flow-based packet switching
// @rcu: RCU callback head for deferred destruction.
// @list_node: Element in global 'dps' list.
// @table: flow table.
// @ports: Hash table for ports.  %OVSP_LOCAL port always exists.  Protected by
// ovs_mutex and RCU.
// @stats_percpu: Per-CPU datapath statistics.
// @net: Reference to net namespace.
// @user_features: Bitmap of enabled %OVS_DP_F_* features.
// @max_headroom: The maximum headroom of all vports in this datapath; it will
// be used by all the internal vports in this dp.
// @meter_tbl: Meter table.
// @upcall_portids: RCU protected 'struct dp_nlsk_pids'.
//
// Context: See the comment on locking at the top of datapath.c for additional
// locking information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct datapath {
    pub rcu: rcu_head,
    pub list_node: list_head,
// Flow table.
    pub table: flow_table,
// Switch ports.
    pub ports: *mut hlist_head,
// Stats.
    pub stats_percpu: *mut dp_stats_percpu __percpu,
// Network namespace ref.
    pub net: possible_net_t,
    pub user_features: u32,
    pub max_headroom: u32,
// Switch meters.
    pub meter_tbl: dp_meter_table,
    pub upcall_portids: *mut dp_nlsk_pids __rcu,
}

//
// struct ovs_skb_cb - OVS data in skb CB
// @input_vport: The original vport packet came in on. This value is cached
// when a packet is received by OVS.
// @mru: The maximum received fragement size; 0 if the packet is not
// fragmented.
// @acts_origlen: The netlink size of the flow actions applied to this skb.
// @cutlen: The number of bytes in the packet to preserve on output.
// @probability: The sampling probability that was applied to this skb; 0 means
// no sampling has occurred; U32_MAX means 100% probability.
// @upcall_pid: Netlink socket PID to use for sending this packet to userspace;
// 0 means "not set" and default per-CPU or per-vport dispatch should be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_skb_cb {
    pub input_vport: *mut vport,
    pub mru: u16,
    pub acts_origlen: u16,
    pub cutlen: u32,
    pub probability: u32,
    pub upcall_pid: u32,
}

//
// struct dp_upcall_info - metadata to include with a packet sent to userspace
// @cmd: One of %OVS_PACKET_CMD_*.
// @userdata: If nonnull, its variable-length value is passed to userspace as
// %OVS_PACKET_ATTR_USERDATA.
// @actions: If nonnull, its variable-length value is passed to userspace as
// %OVS_PACKET_ATTR_ACTIONS.
// @actions_len: The length of the @actions.
// @portid: Netlink portid to which packet should be sent.  If @portid is 0
// then no packet is sent and the packet is accounted in the datapath's @n_lost
// counter.
// @egress_tun_info: If nonnull, becomes %OVS_PACKET_ATTR_EGRESS_TUN_KEY.
// @mru: If not zero, Maximum received IP fragment size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_upcall_info {
    pub egress_tun_info: *mut ip_tunnel_info,
    pub userdata: *const nlattr,
    pub actions: *const nlattr,
    pub actions_len: c_int,
    pub portid: u32,
    pub cmd: u8,
    pub mru: u16,
}

//
// struct ovs_net - Per net-namespace data for ovs.
// @dps: List of datapaths to enable dumping them all out.
// Protected by genl_mutex.
// @dp_notify_work: A work notifier to handle port unregistering.
// @masks_rebalance: A work to periodically optimize flow table caches.
// @ct_limit_info: Hash table of conntrack zone connection limits. Protected
// by RCU; updates and teardown are serialized by ovs_mutex. May be NULL during
// netns teardown.
// @ct_limit_exit_data: CT limit state detached at .pre_exit, freed at .exit.
// @xt_label: Whether connlables are configured for the network or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_net {
    pub dps: list_head,
    pub dp_notify_work: work_struct,
    pub masks_rebalance: delayed_work,

    pub ct_limit_info: *mut ovs_ct_limit_info __rcu,
    pub ct_limit_exit_data: *mut ovs_ct_limit_info,

    pub xt_label: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_frag_data {
    pub dst: c_ulong,
    pub vport: *mut vport,
    pub cb: ovs_skb_cb,
    pub inner_protocol: __be16,
    pub /: *mut *mut u16 network_offset; / valid only for MPLS,
    pub vlan_tci: u16,
    pub vlan_proto: __be16,
    pub l2_len: c_uint,
    pub mac_proto: u8,
    pub l2_data: [u8; MAX_L2_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deferred_action {
    pub skb: *mut sk_buff,
    pub actions: *const nlattr,
    pub actions_len: c_int,
// Store pkt_key clone when creating deferred action.
    pub pkt_key: sw_flow_key,
}

pub const DEFERRED_ACTION_FIFO_SIZE: c_int = 10;
pub const OVS_RECURSION_LIMIT: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_fifo {
    pub head: c_int,
    pub tail: c_int,
// Deferred action fifo queue storage.
    pub fifo: [deferred_action; DEFERRED_ACTION_FIFO_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action_flow_keys {
    pub key: [sw_flow_key; OVS_DEFERRED_ACTION_THRESHOLD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovs_pcpu_storage {
    pub action_fifos: action_fifo,
    pub flow_keys: action_flow_keys,
    pub frag_data: ovs_frag_data,
    pub exec_level: c_int,
    pub owner: *mut task_struct,
    pub bh_lock: local_lock_t,
}

//
// enum ovs_pkt_hash_types - hash info to include with a packet
// to send to userspace.
// @OVS_PACKET_HASH_SW_BIT: indicates hash was computed in software stack.
// @OVS_PACKET_HASH_L4_BIT: indicates hash is a canonical 4-tuple hash
// over transport ports.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovs_pkt_hash_types {
    OVS_PACKET_HASH_SW_BIT = (1ULL << 32),
    OVS_PACKET_HASH_L4_BIT = (1ULL << 33),
}

extern "C" {
    pub fn ovs_lock();
}
extern "C" {
    pub fn ovs_unlock();
}

extern "C" {
    pub fn lockdep_ovsl_is_held() -> c_int;
}

pub const lockdep_ovsl_is_held(): c_int = 1;

extern "C" {
    pub fn read_pnet(_arg: &dp->net) -> return;
}
extern "C" {
    pub fn ovs_lookup_vport(_arg: dp, _arg: port_no) -> return;
}
extern "C" {
    pub fn ovs_lookup_vport(_arg: dp, _arg: port_no) -> return;
}
extern "C" {
    pub fn ovs_lookup_vport(_arg: dp, _arg: port_no) -> return;
}
// Must be called with rcu_read_lock.
// The caller must hold either ovs_mutex or rcu_read_lock to keep the
// returned dp pointer valid.
//
extern "C" {
    pub fn ovs_dp_process_packet(skb: *mut sk_buff, key: *mut sw_flow_key);
}
extern "C" {
    pub fn ovs_dp_detach_port(: *mut vport);
}
extern "C" {
    pub fn ovs_dp_get_upcall_portid(dp: *const datapath, cpu_id: u32) -> u32;
}
extern "C" {
    pub fn ovs_dp_notify_wq(work: *mut work_struct);
}
// 'KEY' must not have any bits set outside of the 'MASK'

