//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/fs_core.h
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
// Copyright (c) 2015, Mellanox Technologies. All rights reserved.
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

pub const FDB_TC_MAX_CHAIN: c_int = 3;

// The index of the last real chain (FT) + 1 as chain zero is valid as well

pub const FDB_TC_MAX_PRIO: c_int = 16;
pub const FDB_TC_LEVELS_PER_PRIO: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_definer {
    pub ns_type: mlx5_flow_namespace_type,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_resource_owner {
    MLX5_FLOW_RESOURCE_OWNER_FW,
    MLX5_FLOW_RESOURCE_OWNER_SW,
    MLX5_FLOW_RESOURCE_OWNER_HWS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_modify_hdr {
    pub ns_type: mlx5_flow_namespace_type,
    pub owner: mlx5_flow_resource_owner,
    pub fs_dr_action: mlx5_fs_dr_action,
    pub fs_hws_action: mlx5_fs_hws_action,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_pkt_reformat {
    pub ns_type: mlx5_flow_namespace_type,
    pub /: *mut *mut int reformat_type; / from mlx5_ifc,
    pub owner: mlx5_flow_resource_owner,
    pub fs_dr_action: mlx5_fs_dr_action,
    pub fs_hws_action: mlx5_fs_hws_action,
    pub id: u32,
}

// FS_TYPE_PRIO_CHAINS is a PRIO that will have namespaces only,
// and those are in parallel to one another when going over them to connect
// a new flow table. Meaning the last flow table in a TYPE_PRIO prio in one
// parallel namespace will not automatically connect to the first flow table
// found in any prio in any next namespace, but skip the entire containing
// TYPE_PRIO_CHAINS prio.
//
// This is used to implement tc chains, each chain of prios is a different
// namespace inside a containing TYPE_PRIO_CHAINS prio.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_node_type {
    FS_TYPE_NAMESPACE,
    FS_TYPE_PRIO,
    FS_TYPE_PRIO_CHAINS,
    FS_TYPE_FLOW_TABLE,
    FS_TYPE_FLOW_GROUP,
    FS_TYPE_FLOW_ENTRY,
    FS_TYPE_FLOW_DEST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_flow_table_op_mod {
    FS_FT_OP_MOD_NORMAL,
    FS_FT_OP_MOD_LAG_DEMUX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_fte_status {
    FS_FTE_STATUS_EXISTING = 1UL << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_steering_mode {
    MLX5_FLOW_STEERING_MODE_DMFS,
    MLX5_FLOW_STEERING_MODE_SMFS,
    MLX5_FLOW_STEERING_MODE_HMFS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_steering_capabilty {
    MLX5_FLOW_STEERING_CAP_VLAN_PUSH_ON_RX = 1UL << 0,
    MLX5_FLOW_STEERING_CAP_VLAN_POP_ON_TX = 1UL << 1,
    MLX5_FLOW_STEERING_CAP_MATCH_RANGES = 1UL << 2,
    MLX5_FLOW_STEERING_CAP_DUPLICATE_MATCH = 1UL << 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_steering {
    pub dev: *mut mlx5_core_dev,
    pub mode: mlx5_flow_steering_mode,
    pub fgs_cache: *mut kmem_cache,
    pub ftes_cache: *mut kmem_cache,
    pub root_ns: *mut mlx5_flow_root_namespace,
    pub fdb_root_ns: *mut mlx5_flow_root_namespace,
    pub fdb_sub_ns: *mut mlx5_flow_namespace,
    pub esw_egress_root_ns: xarray,
    pub esw_ingress_root_ns: xarray,
    pub sniffer_tx_root_ns: *mut mlx5_flow_root_namespace,
    pub sniffer_rx_root_ns: *mut mlx5_flow_root_namespace,
    pub rdma_rx_root_ns: *mut mlx5_flow_root_namespace,
    pub rdma_tx_root_ns: *mut mlx5_flow_root_namespace,
    pub egress_root_ns: *mut mlx5_flow_root_namespace,
    pub port_sel_root_ns: *mut mlx5_flow_root_namespace,
    pub rdma_transport_rx_root_ns: *mut mlx5_flow_root_namespace,
    pub rdma_transport_tx_root_ns: *mut mlx5_flow_root_namespace,
    pub rdma_transport_rx_vports: c_int,
    pub rdma_transport_tx_vports: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_node {
    pub list: list_head,
    pub children: list_head,
    pub type: fs_node_type,
    pub parent: *mut fs_node,
    pub root: *mut fs_node,
// lock the node for writing and traversing
    pub lock: rw_semaphore,
    pub refcount: refcount_t,
    pub active: bool,
    pub ): *mut *mut void (del_hw_func)(struct fs_node,
    pub ): *mut *mut void (del_sw_func)(struct fs_node,
    pub version: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_rule {
    pub node: fs_node,
    pub ft: *mut mlx5_flow_table,
    pub dest_attr: mlx5_flow_destination,
// next_ft should be accessed under chain_lock and only of
// destination type is FWD_NEXT_fT.
//
    pub next_ft: list_head,
    pub sw_action: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_handle {
    pub num_rules: c_int,
    pub __counted_by(num_rules): *mut *mut mlx5_flow_rule rule[],
}

// Type of children is mlx5_flow_group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_table {
    pub node: fs_node,
    pub fs_dr_table: mlx5_fs_dr_table,
    pub fs_hws_table: mlx5_fs_hws_table,
}

// Protect fwd_rules
// FWD rules that point on this flow table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ft_underlay_qp {
    pub list: list_head,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fte_action {
    pub modify_mask: c_int,
    pub dests_size: u32,
    pub fwd_dests: u32,
    pub flow_context: mlx5_flow_context,
    pub action: mlx5_flow_act,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fte_dup {
    pub children: list_head,
    pub act_dests: fs_fte_action,
}

// Type of children is mlx5_flow_rule
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fte {
    pub node: fs_node,
    pub fs_dr_rule: mlx5_fs_dr_rule,
    pub fs_hws_rule: mlx5_fs_hws_rule,
}

// Type of children is mlx5_flow_table/namespace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_prio {
    pub node: fs_node,
    pub num_levels: c_uint,
    pub start_level: c_uint,
    pub prio: c_uint,
    pub num_ft: c_uint,
}

// Type of children is fs_prio
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_namespace {
// parent == NULL => root ns
    pub node: fs_node,
    pub def_miss_action: mlx5_flow_table_miss_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_group_mask {
    pub match_criteria_enable: u8,
    pub match_criteria: [u32; MLX5_ST_SZ_DW_MATCH_PARAM],
}

// Type of children is fs_fte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_group {
    pub node: fs_node,
    pub fs_dr_matcher: mlx5_fs_dr_matcher,
    pub fs_hws_matcher: mlx5_fs_hws_matcher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_root_namespace {
    pub ns: mlx5_flow_namespace,
    pub mode: mlx5_flow_steering_mode,
    pub fs_dr_domain: mlx5_fs_dr_domain,
    pub fs_hws_context: mlx5_fs_hws_context,
}

// Should be held when chaining flow tables
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fc_type {
    MLX5_FC_TYPE_POOL_ACQUIRED = 0,
    MLX5_FC_TYPE_SINGLE,
    MLX5_FC_TYPE_LOCAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fc_cache {
    pub packets: u64,
    pub bytes: u64,
    pub lastuse: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fc {
    pub id: u32,
    pub aging: bool,
    pub type: mlx5_fc_type,
    pub bulk: *mut mlx5_fc_bulk,
    pub cache: mlx5_fc_cache,
    pub fc_local_refcount: refcount_t,
// last{packets,bytes} are used for calculating deltas since last reading.
    pub lastpackets: u64,
    pub lastbytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fc_bulk {
    pub fs_bulk: mlx5_fs_bulk,
    pub base_id: u32,
    pub hws_data: mlx5_fs_hws_data,
    pub fcs: [mlx5_fc; ],
}

extern "C" {
    pub fn mlx5_fc_get_base_id(counter: *mut mlx5_fc) -> u32;
}
extern "C" {
    pub fn mlx5_init_fc_stats(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cleanup_fc_stats(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_fs_core_alloc(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_core_free(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_fs_core_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_core_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_fs_get_capabilities(dev: *mut mlx5_core_dev, type: mlx5_flow_namespace_type) -> u32;
}

