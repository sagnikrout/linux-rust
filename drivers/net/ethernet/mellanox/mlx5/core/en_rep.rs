//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_rep.h
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


//
// Copyright (c) 2017, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_neigh_update_table {
    pub neigh_ht: rhashtable,
// Save the neigh hash entries in a list in addition to the hash table
// (neigh_ht). In order to iterate easily over the neigh entries.
// Used for stats query.
//
    pub neigh_list: list_head,
// protect lookup/remove operations
    pub encap_lock: mutex,
    pub netevent_nb: notifier_block,
    pub neigh_stats_work: delayed_work,
    pub /: *mut *mut unsigned long min_interval; / jiffies,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rep_uplink_priv {
// indirect block callbacks are invoked on bind/unbind events
// on registered higher level devices (e.g. tunnel devices)
//
// tc_indr_block_cb_priv_list is used to lookup indirect callback
// private data
//
    pub tc_indr_block_priv_list: list_head,
    pub tun_entropy: mlx5_tun_entropy,
// protects unready_flows
    pub unready_flows_lock: mutex,
    pub unready_flows: list_head,
    pub reoffload_flows_work: work_struct,
// maps tun_info to a unique id
    pub tunnel_mapping: *mut mapping_ctx,
// maps tun_enc_opts to a unique id
    pub tunnel_enc_opts_mapping: *mut mapping_ctx,
    pub post_act: *mut mlx5e_post_act,
    pub ct_priv: *mut mlx5_tc_ct_priv,
    pub tc_psample: *mut mlx5e_tc_psample,
// support eswitch vports bonding
    pub bond: *mut mlx5e_rep_bond,
// tc tunneling encapsulation private data
    pub encap: *mut mlx5e_tc_tun_encap,
// OVS internal port support
    pub int_port_priv: *mut mlx5e_tc_int_port_priv,
    pub flow_meters: *mut mlx5e_flow_meters,
// tc action stats
    pub action_stats_handle: *mut mlx5e_tc_act_stats_handle,
    pub mpesw_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_priv {
    pub rep: *mut mlx5_eswitch_rep,
    pub neigh_update: mlx5e_neigh_update_table,
    pub netdev: *mut net_device,
    pub root_ft: *mut mlx5_flow_table,
    pub vport_rx_rule: *mut mlx5_flow_handle,
    pub vport_sqs_list: list_head,
    pub /: *mut *mut mlx5_rep_uplink_priv uplink_priv; / valid for uplink rep,
    pub prev_vf_vport_stats: rtnl_link_stats64,
    pub send_to_vport_meta_rule: *mut mlx5_flow_handle,
    pub tc_ht: rhashtable,
    pub rep_vnic_reporter: *mut devlink_health_reporter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_neigh {
    pub v4: __be32,
    pub v6: in6_addr,
    pub dst_ip: },
    pub family: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_neigh_hash_entry {
    pub rhash_node: rhash_head,
    pub m_neigh: mlx5e_neigh,
    pub priv: *mut mlx5e_priv,
    pub neigh_dev: *mut net_device,
// Save the neigh hash entry in a list on the representor in
// addition to the hash table. In order to iterate easily over the
// neighbour entries. Used for stats query.
//
    pub neigh_list: list_head,
// protects encap list
    pub encap_list_lock: spinlock_t,
// encap list sharing the same neigh
    pub encap_list: list_head,
// neigh hash entry can be deleted only when the refcount is zero.
// refcount is needed to avoid neigh hash entry removal by TC, while
// it's used by the neigh notification call.
//
    pub refcnt: refcount_t,
// Save the last reported time offloaded traffic pass over one of the
// neigh hash entry flows. Use it to periodically update the neigh
// 'used' value and avoid neigh deleting by the kernel.
//
    pub reported_lastuse: c_ulong,
    pub rcu: rcu_head,
}

// set when the encap entry is successfully offloaded into HW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_decap_key {
    pub key: ethhdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_decap_entry {
    pub key: mlx5e_decap_key,
    pub flows: list_head,
    pub hlist: hlist_node,
    pub refcnt: refcount_t,
    pub res_ready: completion,
    pub compl_result: c_int,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_mpls_info {
    pub label: u32,
    pub tc: u8,
    pub bos: u8,
    pub ttl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_encap_entry {
// attached neigh hash entry
    pub nhe: *mut mlx5e_neigh_hash_entry,
// neigh hash entry list of encaps sharing the same neigh
    pub encap_list: list_head,
// a node of the eswitch encap hash table which keeping all the encap
// entries
//
    pub encap_hlist: hlist_node,
    pub flows: list_head,
    pub route_list: list_head,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
    pub tun_info: *const ip_tunnel_info,
    pub mpls_info: mlx5e_mpls_info,
    pub /: *mut *mut unsigned char h_dest[ETH_ALEN]; / destination eth addr,
    pub out_dev: *mut net_device,
    pub route_dev_ifindex: c_int,
    pub tunnel: *mut mlx5e_tc_tunnel,
    pub reformat_type: c_int,
    pub flags: u8,
    pub encap_header: *mut c_char,
    pub encap_size: c_int,
    pub refcnt: refcount_t,
    pub res_ready: completion,
    pub compl_result: c_int,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_sq_peer {
    pub rule: *mut mlx5_flow_handle,
    pub peer: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rep_sq {
    pub send_to_vport_rule: *mut mlx5_flow_handle,
    pub sq_peer: xarray,
    pub sqn: u32,
    pub list: list_head,
}

extern "C" {
    pub fn mlx5e_rep_init() -> c_int;
}
extern "C" {
    pub fn mlx5e_rep_cleanup();
}
extern "C" {
    pub fn mlx5e_rep_bond_init(rpriv: *mut mlx5e_rep_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_rep_bond_cleanup(rpriv: *mut mlx5e_rep_priv);
}
extern "C" {
    pub fn mlx5e_rep_bond_update(priv: *mut mlx5e_priv, cleanup: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_rep_has_offload_stats(dev: *const net_device, attr_id: c_int) -> bool;
}
extern "C" {
    pub fn mlx5e_is_uplink_rep(priv: *mut mlx5e_priv) -> bool;
}
extern "C" {
    pub fn mlx5e_rep_activate_channels(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_rep_deactivate_channels(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_rep_queue_neigh_stats_work(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_eswitch_vf_rep(netdev: *const net_device) -> bool;
}
extern "C" {
    pub fn mlx5e_eswitch_uplink_rep(netdev: *const net_device) -> bool;
}

