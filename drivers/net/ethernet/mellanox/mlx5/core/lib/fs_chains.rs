//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/fs_chains.h
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
// Copyright (c) 2020 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_chains_flags {
    MLX5_CHAINS_AND_PRIOS_SUPPORTED = BIT(0),
    MLX5_CHAINS_IGNORE_FLOW_LEVEL_SUPPORTED = BIT(1),
    MLX5_CHAINS_FT_TUNNEL_SUPPORTED = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_chains_attr {
    pub ns: mlx5_flow_namespace_type,
    pub fs_base_prio: c_int,
    pub fs_base_level: c_int,
    pub flags: u32,
    pub max_grp_num: u32,
    pub default_ft: *mut mlx5_flow_table,
    pub mapping: *mut mapping_ctx,
}

extern "C" {
    pub fn mlx5_chains_ignore_flow_level_supported(chains: *mut mlx5_fs_chains) -> bool;
}
extern "C" {
    pub fn mlx5_chains_destroy(chains: *mut mlx5_fs_chains);
}

