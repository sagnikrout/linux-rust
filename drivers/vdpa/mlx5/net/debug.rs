//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/mlx5/net/debug.c
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

#[no_mangle]
unsafe extern "C" fn tirn_show(file: *mut seq_file, priv: *mut c_void) -> c_int {
    static int tirn_show(struct seq_file *file, void *priv)
    {
    struct mlx5_vdpa_net *ndev = file.private;
    seq_printf(file, "0x%x\n", ndev.res.tirn);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tirn);
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_remove_tirn(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_remove_tirn(struct mlx5_vdpa_net *ndev)
    {
    if (ndev.debugfs)
    debugfs_remove(ndev.res.tirn_dent);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_add_tirn(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_add_tirn(struct mlx5_vdpa_net *ndev)
    {
    ndev.res.tirn_dent = debugfs_create_file("tirn", 0444, ndev.rx_dent,
    ndev, &tirn_fops);
    }
#[no_mangle]
unsafe extern "C" fn rx_flow_table_show(file: *mut seq_file, priv: *mut c_void) -> c_int {
    static int rx_flow_table_show(struct seq_file *file, void *priv)
    {
    struct mlx5_vdpa_net *ndev = file.private;
    seq_printf(file, "0x%x\n", mlx5_flow_table_id(ndev.rxft));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(rx_flow_table);
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_remove_rx_flow_table(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_remove_rx_flow_table(struct mlx5_vdpa_net *ndev)
    {
    if (ndev.debugfs)
    debugfs_remove(ndev.rx_table_dent);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_add_rx_flow_table(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_add_rx_flow_table(struct mlx5_vdpa_net *ndev)
    {
    ndev.rx_table_dent = debugfs_create_file("table_id", 0444, ndev.rx_dent,
    ndev, &rx_flow_table_fops);
    }

#[no_mangle]
unsafe extern "C" fn packets_show(file: *mut seq_file, priv: *mut c_void) -> c_int {
    static int packets_show(struct seq_file *file, void *priv)
    {
    struct mlx5_vdpa_counter *counter = file.private;
    u64 packets;
    u64 bytes;
    int err;
    err = mlx5_fc_query(counter.mdev, counter.counter, &packets, &bytes);
    if (err)
    return err;
    seq_printf(file, "0x%llx\n", packets);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bytes_show(file: *mut seq_file, priv: *mut c_void) -> c_int {
    static int bytes_show(struct seq_file *file, void *priv)
    {
    struct mlx5_vdpa_counter *counter = file.private;
    u64 packets;
    u64 bytes;
    int err;
    err = mlx5_fc_query(counter.mdev, counter.counter, &packets, &bytes);
    if (err)
    return err;
    seq_printf(file, "0x%llx\n", bytes);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(packets);
    DEFINE_SHOW_ATTRIBUTE(bytes);
    static void add_counter_node(struct mlx5_vdpa_counter *counter,
    struct dentry *parent)
    {
    debugfs_create_file("packets", 0444, parent, counter,
    &packets_fops);
    debugfs_create_file("bytes", 0444, parent, counter,
    &bytes_fops);
    }
    void mlx5_vdpa_add_rx_counters(struct mlx5_vdpa_net *ndev,
    struct macvlan_node *node)
    {
    static const char *ut = "untagged";
    char vidstr[9];
    u16 vid;
    node.ucast_counter.mdev = ndev.mvdev.mdev;
    node.mcast_counter.mdev = ndev.mvdev.mdev;
    if (node.tagged) {
    vid = key2vid(node.macvlan);
    snprintf(vidstr, sizeof(vidstr), "0x%x", vid);
    } else {
    strcpy(vidstr, ut);
    }
    node.dent = debugfs_create_dir(vidstr, ndev.rx_dent);
    if (IS_ERR(node.dent)) {
    node.dent = core::ptr::null_mut();
    return;
    }
    node.ucast_counter.dent = debugfs_create_dir("ucast", node.dent);
    if (IS_ERR(node.ucast_counter.dent))
    return;
    add_counter_node(&node.ucast_counter, node.ucast_counter.dent);
    node.mcast_counter.dent = debugfs_create_dir("mcast", node.dent);
    if (IS_ERR(node.mcast_counter.dent))
    return;
    add_counter_node(&node.mcast_counter, node.mcast_counter.dent);
    }
    void mlx5_vdpa_remove_rx_counters(struct mlx5_vdpa_net *ndev,
    struct macvlan_node *node)
    {
    if (node.dent && ndev.debugfs)
    debugfs_remove_recursive(node.dent);
    }

#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_add_debugfs(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_add_debugfs(struct mlx5_vdpa_net *ndev)
    {
    struct mlx5_core_dev *mdev;
    mdev = ndev.mvdev.mdev;
    ndev.debugfs = debugfs_create_dir(dev_name(&ndev.mvdev.vdev.dev),
    mlx5_debugfs_get_dev_root(mdev));
    if (!IS_ERR(ndev.debugfs))
    ndev.rx_dent = debugfs_create_dir("rx", ndev.debugfs);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_remove_debugfs(ndev: *mut mlx5_vdpa_net) {
    void mlx5_vdpa_remove_debugfs(struct mlx5_vdpa_net *ndev)
    {
    debugfs_remove_recursive(ndev.debugfs);
    ndev.debugfs = core::ptr::null_mut();
    }
