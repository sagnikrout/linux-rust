//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/fs.h
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
// Copyright (c) 2018 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_flow_table {
    pub num_groups: c_int,
    pub t: *mut mlx5_flow_table,
    pub g: *mut mlx5_flow_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_l2_rule {
    pub 2]: u8 addr[ETH_ALEN +,
    pub rule: *mut mlx5_flow_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_promisc_table {
    pub ft: mlx5e_flow_table,
    pub rule: *mut mlx5_flow_handle,
}

// Forward declaration and APIs to get private fields of vlan_table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_l2_table {
    pub ft: mlx5e_flow_table,
    pub netdev_uc: [hlist_head; MLX5E_L2_ADDR_HASH_SIZE],
    pub netdev_mc: [hlist_head; MLX5E_L2_ADDR_HASH_SIZE],
    pub broadcast: mlx5e_l2_rule,
    pub allmulti: mlx5e_l2_rule,
    pub trap_rule: *mut mlx5_flow_handle,
    pub broadcast_enabled: bool,
    pub allmulti_enabled: bool,
    pub promisc_enabled: bool,
}

// NIC promisc FT level
// NIC prio FTS

extern "C" {
    pub fn mlx5e_arfs_destroy_tables(fs: *mut mlx5e_flow_steering, ntuple: bool);
}
extern "C" {
    pub fn mlx5e_arfs_enable(fs: *mut mlx5e_flow_steering) -> c_int;
}
extern "C" {
    pub fn mlx5e_arfs_disable(fs: *mut mlx5e_flow_steering) -> c_int;
}

extern "C" {
    pub fn mlx5e_destroy_ttc_table(fs: *mut mlx5e_flow_steering);
}
extern "C" {
    pub fn mlx5e_destroy_flow_table(ft: *mut mlx5e_flow_table);
}
extern "C" {
    pub fn mlx5e_enable_cvlan_filter(fs: *mut mlx5e_flow_steering, promisc: bool);
}
extern "C" {
    pub fn mlx5e_disable_cvlan_filter(fs: *mut mlx5e_flow_steering, promisc: bool);
}
extern "C" {
    pub fn mlx5e_fs_cleanup(fs: *mut mlx5e_flow_steering);
}
extern "C" {
    pub fn mlx5e_fs_set_ns(fs: *mut mlx5e_flow_steering, ns: *mut mlx5_flow_namespace, egress: bool);
}

extern "C" {
    pub fn mlx5e_fs_set_ttc(fs: *mut mlx5e_flow_steering, ttc: *mut mlx5_ttc_table, inner: bool);
}

extern "C" {
    pub fn mlx5e_fs_set_arfs(fs: *mut mlx5e_flow_steering, arfs: *mut mlx5e_arfs_tables);
}

extern "C" {
    pub fn mlx5e_fs_set_ptp(fs: *mut mlx5e_flow_steering, ptp_fs: *mut mlx5e_ptp_fs);
}
extern "C" {
    pub fn mlx5e_fs_set_any(fs: *mut mlx5e_flow_steering, any: *mut mlx5e_fs_any);
}
extern "C" {
    pub fn mlx5e_fs_set_udp(fs: *mut mlx5e_flow_steering, udp: *mut mlx5e_fs_udp);
}

extern "C" {
    pub fn mlx5e_fs_set_accel_tcp(fs: *mut mlx5e_flow_steering, accel_tcp: *mut mlx5e_accel_fs_tcp);
}

extern "C" {
    pub fn mlx5e_fs_set_state_destroy(fs: *mut mlx5e_flow_steering, state_destroy: bool);
}
extern "C" {
    pub fn mlx5e_fs_set_vlan_strip_disable(fs: *mut mlx5e_flow_steering, vlan_strip_disable: bool);
}
extern "C" {
    pub fn mlx5e_add_vlan_trap(fs: *mut mlx5e_flow_steering, trap_id: c_int, tir_num: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_remove_vlan_trap(fs: *mut mlx5e_flow_steering);
}
extern "C" {
    pub fn mlx5e_add_mac_trap(fs: *mut mlx5e_flow_steering, trap_id: c_int, tir_num: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5e_remove_mac_trap(fs: *mut mlx5e_flow_steering);
}
extern "C" {
    pub fn mlx5e_fs_init_l2_addr(fs: *mut mlx5e_flow_steering, netdev: *mut net_device);
}

