//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lag/lag.h
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
// Copyright (c) 2019 Mellanox Technologies.

pub const MLX5_LAG_MAX_HASH_BUCKETS: c_int = 16;
// XArray mark for the LAG master device
// (device with lowest mlx5_get_dev_index).
// Note: XA_MARK_0 is reserved by XA_FLAGS_ALLOC for free-slot tracking.
//

// XArray mark for port-level entries (excludes SD secondaries)

// Like xa_for_each_marked but starting from a given index

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_lag_mode {
    MLX5_LAG_MODE_NONE,
    MLX5_LAG_MODE_ROCE,
    MLX5_LAG_MODE_SRIOV,
    MLX5_LAG_MODE_MULTIPATH,
    MLX5_LAG_MODE_MPESW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lag_func {
    pub dev: *mut mlx5_core_dev,
    pub netdev: *mut net_device,
    pub has_drop: bool,
    pub /: *mut *mut unsigned int idx; / xarray index assigned by LAG,
    pub port_change_nb: mlx5_nb,
    pub /: *mut *mut u32 group_id; / SD group ID, 0 = not SD,
    pub /: *mut *mut bool sd_fdb_active; / set on all SD group members,
// Lag demux resources - only populated on master devices
    pub lag_demux_ft: *mut mlx5_flow_table,
    pub lag_demux_fg: *mut mlx5_flow_group,
    pub lag_demux_rules: xarray,
}

// Used for collection of netdev event info.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lag_tracker {
    pub tx_type: netdev_lag_tx_type,
    pub netdev_state: [netdev_lag_lower_state_info; MLX5_MAX_PORTS],
    pub is_bonded:1: c_uint,
    pub has_inactive:1: c_uint,
    pub hash_type: netdev_lag_hash,
    pub bond_speed_mbps: u32,
}

// LAG data of a ConnectX card.
// It serves both its phys functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_lag {
    pub mode: mlx5_lag_mode,
    pub mode_flags: c_ulong,
    pub state_flags: c_ulong,
    pub ports: u8,
    pub buckets: u8,
    pub mode_changes_in_progress: c_int,
    pub MLX5_LAG_MAX_HASH_BUCKETS]: *mut *mut u8 v2p_map[MLX5_MAX_PORTS,
    pub ref: kref,
    pub pfs: xarray,
    pub tracker: lag_tracker,
    pub wq: *mut workqueue_struct,
    pub bond_work: delayed_work,
    pub speed_update_work: work_struct,
    pub nb: notifier_block,
    pub net: possible_net_t,
    pub lag_mp: lag_mp,
    pub port_sel: mlx5_lag_port_sel,
// Protect lag fields/state changes
    pub lock: mutex,
    pub lag_mpesw: lag_mpesw,
}

extern "C" {
    pub fn xa_load(_arg: &ldev->pfs, _arg: idx) -> return;
}
// Get device index (mlx5_get_dev_index) from xarray index
// Find lag_func by device index (reverse lookup from mlx5_get_dev_index)
// Find lag_func by mlx5_core_dev pointer
extern "C" {
    pub fn test_bit(_arg: MLX5_LAG_FLAG_NDEVS_READY, _arg: &ldev->state_flags) -> return;
}

extern "C" {
    pub fn mlx5_lag_shared_fdb_destroy(ldev: *mut mlx5_lag, group_id: u32);
}
extern "C" {
    pub fn mlx5_lag_create_vport_lag(ldev: *mut mlx5_lag, group_id: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_destroy_vport_lag(ldev: *mut mlx5_lag, group_id: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_create_single_fdb(ldev: *mut mlx5_lag) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_destroy_single_fdb(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_shared_fdb_supported(ldev: *mut mlx5_lag) -> bool;
}
extern "C" {
    pub fn mlx5_lag_shared_fdb_supported_filter(ldev: *mut mlx5_lag, filter: u32) -> bool;
}

extern "C" {
    pub fn mlx5_lag_check_prereq(ldev: *mut mlx5_lag) -> bool;
}
extern "C" {
    pub fn mlx5_lag_is_sd(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_lag_demux_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_lag_demux_rule_del(dev: *mut mlx5_core_dev, vport_index: c_int);
}
extern "C" {
    pub fn mlx5_ldev_add_debugfs(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_ldev_remove_debugfs(dbg: *mut dentry);
}
extern "C" {
    pub fn mlx5_disable_lag(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_remove_devices(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_remove_devices_filter(ldev: *mut mlx5_lag, filter: u32);
}
extern "C" {
    pub fn mlx5_deactivate_lag(ldev: *mut mlx5_lag) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_add_devices(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_add_devices_filter(ldev: *mut mlx5_lag, filter: u32);
}

extern "C" {
    pub fn mlx5_lag_set_vports_agg_speed(ldev: *mut mlx5_lag);
}
extern "C" {
    pub fn mlx5_lag_reset_vports_speed(ldev: *mut mlx5_lag);
}

// Iterator filter constants for mlx5_lag_for_each()

// any other value = iterate devices with that specific group_id

// Convenience wrappers - keeps existing behavior

extern "C" {
    pub fn mlx5_get_next_lag_func(ldev: *mut mlx5_lag, start_idx: c_int, filter: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_get_dev_index_by_seq(ldev: *mut mlx5_lag, seq: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_num_devs(ldev: *mut mlx5_lag) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_num_netdevs(ldev: *mut mlx5_lag) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_unload_reps_from_locked(ldev: *mut mlx5_lag, filter: u32);
}
extern "C" {
    pub fn mlx5_ldev_remove_mdev(ldev: *mut mlx5_lag, dev: *mut mlx5_core_dev);
}
