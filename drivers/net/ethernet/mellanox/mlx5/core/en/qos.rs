//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/qos.h
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
// Copyright (c) 2020, Mellanox Technologies inc. All rights reserved.

pub const BYTES_IN_MBIT: c_int = 125000;
extern "C" {
    pub fn mlx5e_qos_bytes_rate_check(mdev: *mut mlx5_core_dev, nbytes: u64) -> c_int;
}
extern "C" {
    pub fn mlx5e_qos_max_leaf_nodes(mdev: *mut mlx5_core_dev) -> c_int;
}
// SQ lifecycle
extern "C" {
    pub fn mlx5e_activate_qos_sq(data: *mut c_void, node_qid: u16, hw_id: u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_deactivate_qos_sq(priv: *mut mlx5e_priv, qid: u16);
}
extern "C" {
    pub fn mlx5e_close_qos_sq(priv: *mut mlx5e_priv, qid: u16);
}
extern "C" {
    pub fn mlx5e_reactivate_qos_sq(priv: *mut mlx5e_priv, qid: u16, txq: *mut netdev_queue);
}
extern "C" {
    pub fn mlx5e_reset_qdisc(dev: *mut net_device, qid: u16);
}
extern "C" {
    pub fn mlx5e_qos_open_queues(priv: *mut mlx5e_priv, chs: *mut mlx5e_channels) -> c_int;
}
extern "C" {
    pub fn mlx5e_qos_activate_queues(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_qos_deactivate_queues(c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_qos_deactivate_all_queues(chs: *mut mlx5e_channels);
}
extern "C" {
    pub fn mlx5e_qos_close_queues(c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_qos_close_all_queues(chs: *mut mlx5e_channels);
}
extern "C" {
    pub fn mlx5e_qos_alloc_queues(priv: *mut mlx5e_priv, chs: *mut mlx5e_channels) -> c_int;
}
// TX datapath API
extern "C" {
    pub fn mlx5e_qid_from_qos(chs: *mut mlx5e_channels, qid: u16) -> u16;
}
// HTB API
extern "C" {
    pub fn mlx5e_htb_setup_tc(priv: *mut mlx5e_priv, htb: *mut tc_htb_qopt_offload) -> c_int;
}
// MQPRIO TX rate limit
extern "C" {
    pub fn mlx5e_mqprio_rl_free(rl: *mut mlx5e_mqprio_rl);
}
extern "C" {
    pub fn mlx5e_mqprio_rl_cleanup(rl: *mut mlx5e_mqprio_rl);
}
extern "C" {
    pub fn mlx5e_mqprio_rl_get_node_hw_id(rl: *mut mlx5e_mqprio_rl, tc: c_int, hw_id: *mut u32) -> c_int;
}
