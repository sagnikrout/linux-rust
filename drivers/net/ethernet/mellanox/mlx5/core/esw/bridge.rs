//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/bridge.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_bridge_offloads {
    pub esw: *mut mlx5_eswitch,
    pub bridges: list_head,
    pub ports: xarray,
    pub debugfs_root: *mut dentry,
    pub netdev_nb: notifier_block,
    pub nb_blk: notifier_block,
    pub nb: notifier_block,
    pub wq: *mut workqueue_struct,
    pub update_work: delayed_work,
    pub ingress_ft: *mut mlx5_flow_table,
    pub ingress_igmp_fg: *mut mlx5_flow_group,
    pub ingress_mld_fg: *mut mlx5_flow_group,
    pub ingress_vlan_fg: *mut mlx5_flow_group,
    pub ingress_vlan_filter_fg: *mut mlx5_flow_group,
    pub ingress_qinq_fg: *mut mlx5_flow_group,
    pub ingress_qinq_filter_fg: *mut mlx5_flow_group,
    pub ingress_mac_fg: *mut mlx5_flow_group,
    pub igmp_handle: *mut mlx5_flow_handle,
    pub mld_query_handle: *mut mlx5_flow_handle,
    pub mld_report_handle: *mut mlx5_flow_handle,
    pub mld_done_handle: *mut mlx5_flow_handle,
    pub skip_ft: *mut mlx5_flow_table,
}

extern "C" {
    pub fn mlx5_esw_bridge_cleanup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_bridge_update(br_offloads: *mut mlx5_esw_bridge_offloads);
}
