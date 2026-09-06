//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/fs_dr.h
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
// Copyright (c) 2019 Mellanox Technologies
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_dr_action {
    pub dr_action: *mut mlx5dr_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_dr_rule {
    pub dr_rule: *mut mlx5dr_rule,
// Only actions created by fs_dr
    pub dr_actions: *mut mlx5dr_action,
    pub num_actions: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_dr_domain {
    pub dr_domain: *mut mlx5dr_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_dr_matcher {
    pub dr_matcher: *mut mlx5dr_matcher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_dr_table {
    pub dr_table: *mut mlx5dr_table,
    pub miss_action: *mut mlx5dr_action,
}

extern "C" {
    pub fn mlx5_fs_dr_is_supported(dev: *mut mlx5_core_dev) -> bool;
}

