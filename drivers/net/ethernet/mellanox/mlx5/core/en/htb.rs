//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/htb.h
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

pub const MLX5E_QOS_MAX_LEAF_NODES: c_int = 256;
extern "C" {
    pub fn int(data: *mut *mut mlx5e_fp_htb_enumerate)(void, qid: u16, hw_id: u32) -> typedef;
}
extern "C" {
    pub fn mlx5e_htb_enumerate_leaves(htb: *mut mlx5e_htb, callback: mlx5e_fp_htb_enumerate, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5e_htb_cur_leaf_nodes(htb: *mut mlx5e_htb) -> c_int;
}
// TX datapath API
extern "C" {
    pub fn mlx5e_htb_get_txq_by_classid(htb: *mut mlx5e_htb, classid: u16) -> c_int;
}
// HTB TC handlers
extern "C" {
    pub fn mlx5e_htb_free(htb: *mut mlx5e_htb);
}
extern "C" {
    pub fn mlx5e_htb_cleanup(htb: *mut mlx5e_htb);
}
