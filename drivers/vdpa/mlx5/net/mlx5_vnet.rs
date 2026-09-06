//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/mlx5/net/mlx5_vnet.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_net_resources {
    pub tisn: u32,
    pub tdn: u32,
    pub tirn: u32,
    pub rqtn: u32,
    pub valid: bool,
    pub tirn_dent: *mut dentry,
}

pub const MLX5V_MACVLAN_SIZE: c_int = 256;
pub const MLX5_VDPA_IRQ_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_irq_pool_entry {
    pub map: msi_map,
    pub used: bool,
    pub name: [c_char; MLX5_VDPA_IRQ_NAME_LEN],
    pub dev_id: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_irq_pool {
    pub num_ent: c_int,
    pub entries: *mut mlx5_vdpa_irq_pool_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_net {
    pub mvdev: mlx5_vdpa_dev,
    pub res: mlx5_vdpa_net_resources,
    pub config: virtio_net_config,
    pub vqs: *mut mlx5_vdpa_virtqueue,
    pub event_cbs: *mut vdpa_callback,
// Serialize vq resources creation and destruction. This is required
// since memory map might change and we need to destroy and create
// resources while driver in operational.
//
    pub reslock: rw_semaphore,
    pub rxft: *mut mlx5_flow_table,
    pub rx_dent: *mut dentry,
    pub rx_table_dent: *mut dentry,
    pub setup: bool,
    pub needs_teardown: bool,
    pub cur_num_vqs: u32,
    pub rqt_size: u32,
    pub nb_registered: bool,
    pub nb: notifier_block,
    pub config_cb: vdpa_callback,
    pub cvq_ent: mlx5_vdpa_wq_ent,
    pub macvlan_hash: [hlist_head; MLX5V_MACVLAN_SIZE],
    pub irqp: mlx5_vdpa_irq_pool,
    pub debugfs: *mut dentry,
    pub umem_1_buffer_param_a: u32,
    pub umem_1_buffer_param_b: u32,
    pub umem_2_buffer_param_a: u32,
    pub umem_2_buffer_param_b: u32,
    pub umem_3_buffer_param_a: u32,
    pub umem_3_buffer_param_b: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vdpa_counter {
    pub counter: *mut mlx5_fc,
    pub dent: *mut dentry,
    pub mdev: *mut mlx5_core_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macvlan_node {
    pub hlist: hlist_node,
    pub ucast_rule: *mut mlx5_flow_handle,
    pub mcast_rule: *mut mlx5_flow_handle,
    pub macvlan: u64,
    pub ndev: *mut mlx5_vdpa_net,
    pub tagged: bool,

    pub dent: *mut dentry,
    pub ucast_counter: mlx5_vdpa_counter,
    pub mcast_counter: mlx5_vdpa_counter,

}

extern "C" {
    pub fn mlx5_vdpa_add_debugfs(ndev: *mut mlx5_vdpa_net);
}
extern "C" {
    pub fn mlx5_vdpa_remove_debugfs(ndev: *mut mlx5_vdpa_net);
}
extern "C" {
    pub fn mlx5_vdpa_add_rx_flow_table(ndev: *mut mlx5_vdpa_net);
}
extern "C" {
    pub fn mlx5_vdpa_remove_rx_flow_table(ndev: *mut mlx5_vdpa_net);
}
extern "C" {
    pub fn mlx5_vdpa_add_tirn(ndev: *mut mlx5_vdpa_net);
}
extern "C" {
    pub fn mlx5_vdpa_remove_tirn(ndev: *mut mlx5_vdpa_net);
}

