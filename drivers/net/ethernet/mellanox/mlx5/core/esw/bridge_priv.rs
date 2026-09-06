//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/bridge_priv.h
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

pub const MLX5_ESW_BRIDGE_INGRESS_TABLE_IGMP_GRP_SIZE: c_int = 1;
pub const MLX5_ESW_BRIDGE_INGRESS_TABLE_MLD_GRP_SIZE: c_int = 3;
pub const MLX5_ESW_BRIDGE_INGRESS_TABLE_VLAN_GRP_SIZE: c_int = 131072;

pub const MLX5_ESW_BRIDGE_INGRESS_TABLE_IGMP_GRP_IDX_FROM: c_int = 0;

pub const MLX5_ESW_BRIDGE_EGRESS_TABLE_VLAN_GRP_SIZE: c_int = 131072;

pub const MLX5_ESW_BRIDGE_EGRESS_TABLE_VLAN_GRP_IDX_FROM: c_int = 0;

pub const MLX5_ESW_BRIDGE_SKIP_TABLE_SIZE: c_int = 0;
pub const MLX5_ESW_BRIDGE_MCAST_TABLE_FILTER_GRP_SIZE: c_int = 1;
pub const MLX5_ESW_BRIDGE_MCAST_TABLE_FWD_GRP_SIZE: c_int = 1;
pub const MLX5_ESW_BRIDGE_MCAST_TABLE_VLAN_GRP_SIZE: c_int = 4095;

pub const MLX5_ESW_BRIDGE_MCAST_TABLE_FILTER_GRP_IDX_FROM: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_fdb_key {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_mdb_key {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_fdb_entry {
    pub key: mlx5_esw_bridge_fdb_key,
    pub ht_node: rhash_head,
    pub dev: *mut net_device,
    pub list: list_head,
    pub vlan_list: list_head,
    pub vport_num: u16,
    pub esw_owner_vhca_id: u16,
    pub flags: u16,
    pub ingress_handle: *mut mlx5_flow_handle,
    pub ingress_counter: *mut mlx5_fc,
    pub lastuse: c_ulong,
    pub egress_handle: *mut mlx5_flow_handle,
    pub filter_handle: *mut mlx5_flow_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_mdb_entry {
    pub key: mlx5_esw_bridge_mdb_key,
    pub ht_node: rhash_head,
    pub list: list_head,
    pub ports: xarray,
    pub num_ports: c_int,
    pub egress_handle: *mut mlx5_flow_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_vlan {
    pub vid: u16,
    pub flags: u16,
    pub fdb_list: list_head,
    pub pkt_reformat_push: *mut mlx5_pkt_reformat,
    pub pkt_reformat_pop: *mut mlx5_pkt_reformat,
    pub pkt_mod_hdr_push_mark: *mut mlx5_modify_hdr,
    pub mcast_handle: *mut mlx5_flow_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_port {
    pub vport_num: u16,
    pub esw_owner_vhca_id: u16,
    pub flags: u16,
    pub bridge: *mut mlx5_esw_bridge,
    pub vlans: xarray,
    pub ft: *mut mlx5_flow_table,
    pub filter_fg: *mut mlx5_flow_group,
    pub vlan_fg: *mut mlx5_flow_group,
    pub qinq_fg: *mut mlx5_flow_group,
    pub fwd_fg: *mut mlx5_flow_group,
    pub filter_handle: *mut mlx5_flow_handle,
    pub fwd_handle: *mut mlx5_flow_handle,
    pub mcast: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge {
    pub ifindex: c_int,
    pub refcnt: c_int,
    pub list: list_head,
    pub br_offloads: *mut mlx5_esw_bridge_offloads,
    pub debugfs_dir: *mut dentry,
    pub fdb_list: list_head,
    pub fdb_ht: rhashtable,
    pub mdb_list: list_head,
    pub mdb_ht: rhashtable,
    pub egress_ft: *mut mlx5_flow_table,
    pub egress_vlan_fg: *mut mlx5_flow_group,
    pub egress_qinq_fg: *mut mlx5_flow_group,
    pub egress_mac_fg: *mut mlx5_flow_group,
    pub egress_miss_fg: *mut mlx5_flow_group,
    pub egress_miss_pkt_reformat: *mut mlx5_pkt_reformat,
    pub egress_miss_handle: *mut mlx5_flow_handle,
    pub ageing_time: c_ulong,
    pub flags: u32,
    pub vlan_proto: u16,
}

extern "C" {
    pub fn mlx5_esw_bridge_port_key(port: *mut mlx5_esw_bridge_port) -> c_ulong;
}
extern "C" {
    pub fn mlx5_esw_bridge_port_mcast_init(port: *mut mlx5_esw_bridge_port) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_bridge_port_mcast_cleanup(port: *mut mlx5_esw_bridge_port);
}
extern "C" {
    pub fn mlx5_esw_bridge_vlan_mcast_cleanup(vlan: *mut mlx5_esw_bridge_vlan);
}
extern "C" {
    pub fn mlx5_esw_bridge_mcast_enable(bridge: *mut mlx5_esw_bridge) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_bridge_mcast_disable(bridge: *mut mlx5_esw_bridge);
}
extern "C" {
    pub fn mlx5_esw_bridge_mdb_init(bridge: *mut mlx5_esw_bridge) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_bridge_mdb_cleanup(bridge: *mut mlx5_esw_bridge);
}
extern "C" {
    pub fn mlx5_esw_bridge_mdb_flush(bridge: *mut mlx5_esw_bridge);
}
extern "C" {
    pub fn mlx5_esw_bridge_debugfs_offloads_init(br_offloads: *mut mlx5_esw_bridge_offloads);
}
extern "C" {
    pub fn mlx5_esw_bridge_debugfs_offloads_cleanup(br_offloads: *mut mlx5_esw_bridge_offloads);
}
extern "C" {
    pub fn mlx5_esw_bridge_debugfs_init(br_netdev: *mut net_device, bridge: *mut mlx5_esw_bridge);
}
extern "C" {
    pub fn mlx5_esw_bridge_debugfs_cleanup(bridge: *mut mlx5_esw_bridge);
}
