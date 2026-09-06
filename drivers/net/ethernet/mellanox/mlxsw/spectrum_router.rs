//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_router.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_router_nve_decap {
    pub ul_tb_id: u32,
    pub tunnel_index: u32,
    pub ul_proto: mlxsw_sp_l3proto,
    pub ul_sip: mlxsw_sp_l3addr,
    pub valid:1: u8,
}

// gen_pool_alloc() returns 0 when allocation fails, so use an offset
pub const MLXSW_SP_ROUTER_GENALLOC_OFFSET: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_router {
    pub mlxsw_sp: *mut mlxsw_sp,
    pub crif_ht: rhashtable,
    pub rifs_table: *mut gen_pool,
    pub rifs: *mut mlxsw_sp_rif,
    pub rif_mac_profiles_idr: idr,
    pub rif_mac_profiles_count: core::sync::atomic::AtomicI32,
    pub rifs_count: core::sync::atomic::AtomicI32,
    pub max_rif_mac_profile: u8,
    pub vrs: *mut mlxsw_sp_vr,
    pub neigh_ht: rhashtable,
    pub nexthop_group_ht: rhashtable,
    pub nexthop_ht: rhashtable,
    pub nexthop_list: list_head,
// One tree for each protocol: IPv4 and IPv6
    pub proto_trees: [*mut mlxsw_sp_lpm_tree; 2],
    pub trees: *mut mlxsw_sp_lpm_tree,
    pub tree_count: c_uint,
    pub lpm: },
    pub dw: delayed_work,
    pub /: *mut *mut unsigned long interval; / ms,
    pub neigh_count: core::sync::atomic::AtomicI32,
    pub neighs_update: },
    pub nexthop_probe_dw: delayed_work,

    pub nexthop_neighs_list: list_head,
    pub ipip_list: list_head,
    pub nexthop_nb: notifier_block,
    pub fib_nb: notifier_block,
    pub netevent_nb: notifier_block,
    pub inetaddr_nb: notifier_block,
    pub inet6addr_nb: notifier_block,
    pub netdevice_nb: notifier_block,
    pub inetaddr_valid_nb: notifier_block,
    pub inet6addr_valid_nb: notifier_block,
    pub rif_ops_arr: *const mlxsw_sp_rif_ops,
    pub ipip_ops_arr: *const mlxsw_sp_ipip_ops,
    pub nve_decap_config: mlxsw_sp_router_nve_decap,
    pub /: *mut *mut mutex lock; / Protects shared router resources,
    pub ll_op_ctx: *mut mlxsw_sp_fib_entry_op_ctx,
    pub lb_crif: *mut mlxsw_sp_crif,
    pub adj_grp_size_ranges: *const mlxsw_sp_adj_grp_size_range,
    pub adj_grp_size_ranges_count: usize,
    pub nh_grp_activity_dw: delayed_work,
    pub nh_res_grp_list: list_head,
    pub inc_parsing_depth: bool,
    pub num_groups: refcount_t,
    pub adj_trap_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_rif_ipip_lb_config {
    pub lb_ipipt: mlxsw_reg_ritr_loopback_ipip_type,
    pub okey: u32,
    pub /: *mut *mut mlxsw_sp_l3proto ul_protocol; / Underlay.,
    pub saddr: mlxsw_sp_l3addr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_rif_counter_dir {
    MLXSW_SP_RIF_COUNTER_INGRESS,
    MLXSW_SP_RIF_COUNTER_EGRESS,
}

extern "C" {
    pub fn mlxsw_sp_ipip_lb_rif_index(rif: *const mlxsw_sp_rif_ipip_lb) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_ipip_lb_ul_rif_id(lb_rif: *const mlxsw_sp_rif_ipip_lb) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_ipip_dev_ul_tb_id(ol_dev: *const net_device) -> u32;
}
extern "C" {
    pub fn mlxsw_sp_rif_dev_ifindex(rif: *const mlxsw_sp_rif) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_rif_has_dev(rif: *const mlxsw_sp_rif) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_neigh_entry_type(neigh_entry: *mut mlxsw_sp_neigh_entry) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_neigh4_entry_dip(neigh_entry: *mut mlxsw_sp_neigh_entry) -> u32;
}

extern "C" {
    pub fn mlxsw_sp_neigh_ipv6_ignore(neigh_entry: *mut mlxsw_sp_neigh_entry) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_nexthop_is_forward(nh: *const mlxsw_sp_nexthop) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_nexthop_group_has_ipip(nh: *mut mlxsw_sp_nexthop) -> bool;
}

extern "C" {
    pub fn mlxsw_sp_ipip_ecn_encap_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_ipip_ecn_decap_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
