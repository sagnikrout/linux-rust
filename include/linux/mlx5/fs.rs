//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/fs.h
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

pub const MLX5_FS_DEFAULT_FLOW_TAG: c_uint = 0x0;

pub const MLX5_RDMA_TRANSPORT_BYPASS_PRIO: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_destination_type {
    MLX5_FLOW_DESTINATION_TYPE_NONE,
    MLX5_FLOW_DESTINATION_TYPE_VPORT,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_TABLE,
    MLX5_FLOW_DESTINATION_TYPE_TIR,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_SAMPLER,
    MLX5_FLOW_DESTINATION_TYPE_UPLINK,
    MLX5_FLOW_DESTINATION_TYPE_PORT,
    MLX5_FLOW_DESTINATION_TYPE_COUNTER,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_TABLE_NUM,
    MLX5_FLOW_DESTINATION_TYPE_RANGE,
    MLX5_FLOW_DESTINATION_TYPE_TABLE_TYPE,
    MLX5_FLOW_DESTINATION_TYPE_VHCA_RX,
}

pub const LEFTOVERS_RULE_NUM: c_int = 2;
// priority = 0; /* Priority of leftovers_prio-0
// n_ent = LEFTOVERS_RULE_NUM;
// n_grp = LEFTOVERS_RULE_NUM;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_namespace_type {
    MLX5_FLOW_NAMESPACE_BYPASS,
    MLX5_FLOW_NAMESPACE_KERNEL_RX_MACSEC,
    MLX5_FLOW_NAMESPACE_LAG,
    MLX5_FLOW_NAMESPACE_OFFLOADS,
    MLX5_FLOW_NAMESPACE_ETHTOOL,
    MLX5_FLOW_NAMESPACE_KERNEL,
    MLX5_FLOW_NAMESPACE_LEFTOVERS,
    MLX5_FLOW_NAMESPACE_ANCHOR,
    MLX5_FLOW_NAMESPACE_FDB_BYPASS,
    MLX5_FLOW_NAMESPACE_FDB,
    MLX5_FLOW_NAMESPACE_ESW_EGRESS,
    MLX5_FLOW_NAMESPACE_ESW_INGRESS,
    MLX5_FLOW_NAMESPACE_SNIFFER_RX,
    MLX5_FLOW_NAMESPACE_SNIFFER_TX,
    MLX5_FLOW_NAMESPACE_EGRESS,
    MLX5_FLOW_NAMESPACE_EGRESS_IPSEC,
    MLX5_FLOW_NAMESPACE_EGRESS_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_RX,
    MLX5_FLOW_NAMESPACE_RDMA_RX_KERNEL,
    MLX5_FLOW_NAMESPACE_RDMA_TX,
    MLX5_FLOW_NAMESPACE_PORT_SEL,
    MLX5_FLOW_NAMESPACE_RDMA_RX_COUNTERS,
    MLX5_FLOW_NAMESPACE_RDMA_TX_COUNTERS,
    MLX5_FLOW_NAMESPACE_RDMA_RX_IPSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TX_IPSEC,
    MLX5_FLOW_NAMESPACE_RDMA_RX_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TX_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TRANSPORT_RX,
    MLX5_FLOW_NAMESPACE_RDMA_TRANSPORT_TX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_flow_table_type {
    FS_FT_NIC_RX          = 0x0,
    FS_FT_NIC_TX          = 0x1,
    FS_FT_ESW_EGRESS_ACL  = 0x2,
    FS_FT_ESW_INGRESS_ACL = 0x3,
    FS_FT_FDB             = 0X4,
    FS_FT_SNIFFER_RX	= 0X5,
    FS_FT_SNIFFER_TX	= 0X6,
    FS_FT_RDMA_RX		= 0X7,
    FS_FT_RDMA_TX		= 0X8,
    FS_FT_PORT_SEL		= 0X9,
    FS_FT_FDB_RX		= 0xa,
    FS_FT_FDB_TX		= 0xb,
    FS_FT_RDMA_TRANSPORT_RX	= 0xd,
    FS_FT_RDMA_TRANSPORT_TX	= 0xe,
    FS_FT_MAX_TYPE = FS_FT_RDMA_TRANSPORT_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_context {
    pub flags: u32,
    pub flow_tag: u32,
    pub flow_source: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_spec {
    pub match_criteria_enable: u8,
    pub match_criteria: [u32; MLX5_ST_SZ_DW(fte_match_param)],
    pub match_value: [u32; MLX5_ST_SZ_DW(fte_match_param)],
    pub flow_context: mlx5_flow_context,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_dest_range_field {
    MLX5_FLOW_DEST_RANGE_FIELD_PKT_LEN = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_destination {
    pub type: mlx5_flow_destination_type,
    pub tir_num: u32,
    pub ft_num: u32,
    pub ft: *mut mlx5_flow_table,
    pub counter: *mut mlx5_fc,
    pub id: u16,
    pub vhca: },
    pub num: u16,
    pub vhca_id: u16,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
    pub flags: u8,
    pub vport: },
    pub hit_ft: *mut mlx5_flow_table,
    pub miss_ft: *mut mlx5_flow_table,
    pub field: mlx5_flow_dest_range_field,
    pub min: u32,
    pub max: u32,
    pub range: },
    pub sampler_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdr_tbl {
    pub /: *mut *mut mutex lock; / protects hlist,
    pub 8): DECLARE_HASHTABLE(hlist,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_table_attr {
    pub prio: c_int,
    pub max_fte: c_int,
    pub level: u32,
    pub flags: u32,
    pub uid: u16,
    pub vport: u16,
    pub esw_owner_vhca_id: u16,
    pub next_ft: *mut mlx5_flow_table,
    pub max_num_groups: c_int,
    pub num_reserved_entries: c_int,
    pub autogroup: },
}

extern "C" {
    pub fn mlx5_destroy_flow_table(ft: *mut mlx5_flow_table) -> c_int;
}
// inbox should be set with the following values:
// start_flow_index
// end_flow_index
// match_criteria_enable
// match_criteria
//
extern "C" {
    pub fn mlx5_destroy_flow_group(fg: *mut mlx5_flow_group);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_exe_aso {
    pub object_id: u32,
    pub base_id: c_int,
    pub type: u8,
    pub return_reg_id: u8,
    pub ctrl_data: u32,
    pub meter_idx: u8,
    pub init_color: u8,
    pub flow_meter: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_vlan {
    pub ethtype: u16,
    pub vid: u16,
    pub prio: u8,
}

pub const MLX5_FS_VLAN_DEPTH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_act {
    pub action: u32,
    pub modify_hdr: *mut mlx5_modify_hdr,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_act_crypto_params {
    pub type: u8,
    pub obj_id: u32,
    pub crypto: },
    pub flags: u32,
    pub vlan: [mlx5_fs_vlan; MLX5_FS_VLAN_DEPTH],
    pub counters: *mut ib_counters,
    pub fg: *mut mlx5_flow_group,
    pub exe_aso: mlx5_exe_aso,
}

// Single destination per rule.
// Group ID is implied by the match criteria.
//
extern "C" {
    pub fn mlx5_del_flow_rules(fr: *mut mlx5_flow_handle);
}
extern "C" {
    pub fn mlx5_fc_destroy(dev: *mut mlx5_core_dev, counter: *mut mlx5_fc);
}
extern "C" {
    pub fn mlx5_fc_local_destroy(counter: *mut mlx5_fc);
}
extern "C" {
    pub fn mlx5_fc_local_get(counter: *mut mlx5_fc);
}
extern "C" {
    pub fn mlx5_fc_local_put(counter: *mut mlx5_fc);
}
extern "C" {
    pub fn mlx5_fc_query_lastuse(counter: *mut mlx5_fc) -> u64;
}
extern "C" {
    pub fn mlx5_fc_id(counter: *mut mlx5_fc) -> u32;
}
extern "C" {
    pub fn mlx5_fs_add_rx_underlay_qpn(dev: *mut mlx5_core_dev, underlay_qpn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_remove_rx_underlay_qpn(dev: *mut mlx5_core_dev, underlay_qpn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_get_match_definer_id(definer: *mut mlx5_flow_definer) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_pkt_reformat_params {
    pub type: c_int,
    pub param_0: u8,
    pub param_1: u8,
    pub size: usize,
    pub data: *mut c_void,
}

extern "C" {
    pub fn mlx5_flow_table_id(ft: *mut mlx5_flow_table) -> u32;
}
