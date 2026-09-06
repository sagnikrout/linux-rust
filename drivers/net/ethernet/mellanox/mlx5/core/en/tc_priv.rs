//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc_priv.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2021 Mellanox Technologies.

pub const MLX5E_TC_MAX_SPLITS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_flow_parse_attr {
    pub tun_info: [*const ip_tunnel_info; MLX5_MAX_FLOW_FWD_VPORTS],
    pub mpls_info: [mlx5e_mpls_info; MLX5_MAX_FLOW_FWD_VPORTS],
    pub filter_dev: *mut net_device,
    pub spec: mlx5_flow_spec,
    pub hdrs: [pedit_headers_action; __PEDIT_CMD_MAX],
    pub mod_hdr_acts: mlx5e_tc_mod_hdr_acts,
    pub mirred_ifindex: [c_int; MLX5_MAX_FLOW_FWD_VPORTS],
    pub parse_state: mlx5e_tc_act_parse_state,
}

// Helper struct for accessing a struct containing list_head array.
// Containing struct
// |- Helper array
// [0] Helper item 0
// |- list_head item 0
// |- index (0)
// [1] Helper item 1
// |- list_head item 1
// |- index (1)
// To access the containing struct from one of the list_head items:
// 1. Get the helper item from the list_head item using
// helper item =
// container_of(list_head item, helper struct type, list_head field)
// 2. Get the contining struct from the helper item and its index in the array:
// containing struct =
// container_of(helper item, containing struct type, helper field[index])
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_flow_item {
    pub /: *mut *mut *mut mlx5e_encap_entry e; / attached encap instance,
    pub list: list_head,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encap_route_flow_item {
    pub /: *mut *mut *mut mlx5e_route_entry r; / attached route instance,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_flow {
    pub node: rhash_head,
    pub priv: *mut mlx5e_priv,
    pub cookie: u64,
    pub flags: c_ulong,
    pub 1]: *mut *mut mlx5_flow_handle rule[MLX5E_TC_MAX_SPLITS +,
// flows sharing the same reformat object - currently mpls decap
    pub l3_to_l2_reformat: list_head,
    pub decap_reformat: *mut mlx5e_decap_entry,
// flows sharing same route entry
    pub decap_routes: list_head,
    pub decap_route: *mut mlx5e_route_entry,
    pub encap_routes: [encap_route_flow_item; MLX5_MAX_FLOW_FWD_VPORTS],
// Flow can be associated with multiple encap IDs.
// The number of encaps is bounded by the number of supported
// destinations.
//
    pub encaps: [encap_flow_item; MLX5_MAX_FLOW_FWD_VPORTS],
    pub /: *mut *mut *mut mlx5e_hairpin_entry hpe; / attached hairpin instance,
    pub /: *mut *mut list_head hairpin; / flows sharing the same hairpin,
    pub /: *mut *mut list_head peer[MLX5_MAX_PORTS]; / flows with peer flow,
    pub peer: *mut *mut DECLARE_BITMAP(peer_used, MLX5_MAX_PORTS); / tracks populated,
// slots
//
    pub (e.g: *mut *mut list_head unready; / flows not ready to be offloaded,
// due to missing route)
//
    pub /: *mut *mut list_head peer_flows; / flows on peer,
    pub del: *mut *mut int peer_index; / peer-flow index pinned at add time, used at,
// time so removal is independent of LAG state
// changes between add and del.
//
    pub /: *mut *mut *mut net_device orig_dev; / netdev adding flow first,
    pub tmp_entry_index: c_int,
    pub /: *mut *mut list_head tmp_list; / temporary flow list used by neigh update,
    pub refcnt: refcount_t,
    pub rcu_head: rcu_head,
    pub init_done: completion,
    pub del_hw_done: completion,
    pub attr: *mut mlx5_flow_attr,
    pub extra_split_attr: *mut mlx5_flow_attr,
    pub attrs: list_head,
    pub chain_mapping: u32,
}

extern "C" {
    pub fn mlx5e_tc_get_ip_version(spec: *mut mlx5_flow_spec, outer: bool) -> u8;
}
extern "C" {
    pub fn mlx5e_tc_unoffload_flow_post_acts(flow: *mut mlx5e_tc_flow);
}
extern "C" {
    pub fn mlx5e_tc_offload_flow_post_acts(flow: *mut mlx5e_tc_flow) -> c_int;
}
extern "C" {
    pub fn mlx5e_is_eswitch_flow(flow: *mut mlx5e_tc_flow) -> bool;
}
extern "C" {
    pub fn mlx5e_is_ft_flow(flow: *mut mlx5e_tc_flow) -> bool;
}
extern "C" {
    pub fn mlx5e_is_offloaded_flow(flow: *mut mlx5e_tc_flow) -> bool;
}
extern "C" {
    pub fn mlx5e_get_flow_namespace(flow: *mut mlx5e_tc_flow) -> c_int;
}
extern "C" {
    pub fn mlx5e_same_hw_devs(priv: *mut mlx5e_priv, peer_priv: *mut mlx5e_priv) -> bool;
}
// Complete all memory stores before setting bit.

// test_and_set_bit() provides all necessary barriers
extern "C" {
    pub fn test_and_set_bit(_arg: flag, _arg: &flow->flags) -> return;
}

// Complete all memory stores before clearing bit.

// Read fields of flow structure only after checking flags.

extern "C" {
    pub fn mlx5e_flow_put(priv: *mut mlx5e_priv, flow: *mut mlx5e_tc_flow);
}
