//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/fs_hws_pools.h
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
// Copyright (c) 2025 NVIDIA Corporation & Affiliates

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_pr {
    pub bulk: *mut mlx5_fs_hws_pr_bulk,
    pub offset: u32,
    pub hdr_idx: u8,
    pub data: *mut u8,
    pub data_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_pr_bulk {
    pub fs_bulk: mlx5_fs_bulk,
    pub hws_action: *mut mlx5hws_action,
    pub prs_data: [mlx5_fs_hws_pr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_pr_pool_ctx {
    pub reformat_type: mlx5hws_action_type,
    pub encap_data_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_mh {
    pub bulk: *mut mlx5_fs_hws_mh_bulk,
    pub offset: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_mh_bulk {
    pub fs_bulk: mlx5_fs_bulk,
    pub mh_pool: *mut mlx5_fs_pool,
    pub hws_action: *mut mlx5hws_action,
    pub mhs_data: [mlx5_fs_hws_mh; ],
}

extern "C" {
    pub fn mlx5_fs_hws_pr_pool_cleanup(pr_pool: *mut mlx5_fs_pool);
}
extern "C" {
    pub fn mlx5_fs_hws_mh_pool_cleanup(fs_hws_mh_pool: *mut mlx5_fs_pool);
}
extern "C" {
    pub fn mlx5_fc_put_hws_action(counter: *mut mlx5_fc);
}
