//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/ct_fs.h
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs {
    pub netdev: *const net_device,
    pub dev: *mut mlx5_core_dev,
// private data
    pub priv_data: [*mut c_void; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_rule {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_ops {
    pub post_ct): *mut *mut mlx5_flow_table ct_nat, mlx5_flow_table,
    pub fs): *mut *mut void (destroy)(struct mlx5_ct_fs,
    pub flow_rule): *mut flow_rule,
    pub fs_rule): *mut *mut *mut void (ct_rule_del)(struct mlx5_ct_fs fs, struct mlx5_ct_fs_rule,
    pub attr): *mut *mut mlx5_flow_spec spec, mlx5_flow_attr,
    pub priv_size: usize,
}

