//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/bridge_debugfs.c
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
// Copyright (c) 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

    static void *mlx5_esw_bridge_debugfs_start(struct seq_file *seq, loff_t *pos);
    static void *mlx5_esw_bridge_debugfs_next(struct seq_file *seq, void *v, loff_t *pos);
    static void mlx5_esw_bridge_debugfs_stop(struct seq_file *seq, void *v);
    static int mlx5_esw_bridge_debugfs_show(struct seq_file *seq, void *v);
    static const struct seq_operations mlx5_esw_bridge_debugfs_sops = {
    .start	= mlx5_esw_bridge_debugfs_start,
    .next	= mlx5_esw_bridge_debugfs_next,
    .stop	= mlx5_esw_bridge_debugfs_stop,
    .show	= mlx5_esw_bridge_debugfs_show,
    };
    DEFINE_SEQ_ATTRIBUTE(mlx5_esw_bridge_debugfs);
    static void *mlx5_esw_bridge_debugfs_start(struct seq_file *seq, loff_t *pos)
    {
    struct mlx5_esw_bridge *bridge = seq.private;
    rtnl_lock();
    return *pos ? seq_list_start(&bridge.fdb_list, *pos - 1) : SEQ_START_TOKEN;
    }
    static void *mlx5_esw_bridge_debugfs_next(struct seq_file *seq, void *v, loff_t *pos)
    {
    struct mlx5_esw_bridge *bridge = seq.private;
    return seq_list_next(v == SEQ_START_TOKEN ? &bridge.fdb_list : v, &bridge.fdb_list, pos);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_esw_bridge_debugfs_stop(seq: *mut seq_file, v: *mut c_void) {
    static void mlx5_esw_bridge_debugfs_stop(struct seq_file *seq, void *v)
    {
    rtnl_unlock();
    }
#[no_mangle]
unsafe extern "C" fn mlx5_esw_bridge_debugfs_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int mlx5_esw_bridge_debugfs_show(struct seq_file *seq, void *v)
    {
    struct mlx5_esw_bridge_fdb_entry *entry;
    u64 packets, bytes, lastuse;
    if (v == SEQ_START_TOKEN) {
    seq_printf(seq, "%-16s %-17s %4s %20s %20s %20s %5s\n",
    "DEV", "MAC", "VLAN", "PACKETS", "BYTES", "LASTUSE", "FLAGS");
    return 0;
    }
    entry = list_entry(v, struct mlx5_esw_bridge_fdb_entry, list);
    mlx5_fc_query_cached_raw(entry.ingress_counter, &bytes, &packets, &lastuse);
    seq_printf(seq, "%-16s %-17pM %4d %20llu %20llu %20llu %#5x\n",
    entry.dev.name, entry.key.addr, entry.key.vid, packets, bytes, lastuse,
    entry.flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_bridge_debugfs_init(br_netdev: *mut net_device, bridge: *mut mlx5_esw_bridge) {
    void mlx5_esw_bridge_debugfs_init(struct net_device *br_netdev, struct mlx5_esw_bridge *bridge)
    {
    if (!bridge.br_offloads.debugfs_root)
    return;
    bridge.debugfs_dir = debugfs_create_dir(br_netdev.name,
    bridge.br_offloads.debugfs_root);
    debugfs_create_file("fdb", 0400, bridge.debugfs_dir, bridge,
    &mlx5_esw_bridge_debugfs_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_bridge_debugfs_cleanup(bridge: *mut mlx5_esw_bridge) {
    void mlx5_esw_bridge_debugfs_cleanup(struct mlx5_esw_bridge *bridge)
    {
    debugfs_remove_recursive(bridge.debugfs_dir);
    bridge.debugfs_dir = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_bridge_debugfs_offloads_init(br_offloads: *mut mlx5_esw_bridge_offloads) {
    void mlx5_esw_bridge_debugfs_offloads_init(struct mlx5_esw_bridge_offloads *br_offloads)
    {
    if (!br_offloads.esw.debugfs_root)
    return;
    br_offloads.debugfs_root = debugfs_create_dir("bridge", br_offloads.esw.debugfs_root);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_esw_bridge_debugfs_offloads_cleanup(br_offloads: *mut mlx5_esw_bridge_offloads) {
    void mlx5_esw_bridge_debugfs_offloads_cleanup(struct mlx5_esw_bridge_offloads *br_offloads)
    {
    debugfs_remove_recursive(br_offloads.debugfs_root);
    br_offloads.debugfs_root = core::ptr::null_mut();
    }
