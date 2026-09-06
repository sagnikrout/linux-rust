//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/fs_hws.h
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
pub struct mlx5_fs_hws_actions_pool {
    pub tag_action: *mut mlx5hws_action,
    pub pop_vlan_action: *mut mlx5hws_action,
    pub push_vlan_action: *mut mlx5hws_action,
    pub drop_action: *mut mlx5hws_action,
    pub decapl2_action: *mut mlx5hws_action,
    pub remove_hdr_vlan_action: *mut mlx5hws_action,
    pub insert_hdr_pool: mlx5_fs_pool,
    pub dl3tnltol2_pool: mlx5_fs_pool,
    pub el2tol3tnl_pools: xarray,
    pub el2tol2tnl_pools: xarray,
    pub mh_pools: xarray,
    pub table_dests: xarray,
    pub vport_vhca_dests: xarray,
    pub vport_dests: xarray,
    pub aso_meters: xarray,
    pub sample_dests: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_context {
    pub hws_ctx: *mut mlx5hws_context,
    pub hws_pool: mlx5_fs_hws_actions_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_table {
    pub hws_table: *mut mlx5hws_table,
    pub miss_ft_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_action {
    pub hws_action: *mut mlx5hws_action,
    pub fs_pool: *mut mlx5_fs_pool,
    pub pr_data: *mut mlx5_fs_hws_pr,
    pub mh_data: *mut mlx5_fs_hws_mh,
    pub fw_reformat_id: u32,
// Protect `fw_reformat_id` against being initialized from multiple
// threads.
//
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_matcher {
    pub matcher: *mut mlx5hws_bwc_matcher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_rule_action {
    pub action: *mut mlx5hws_action,
    pub counter: *mut mlx5_fc,
    pub exe_aso: *mut mlx5_exe_aso,
    pub sampler_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_rule {
    pub bwc_rule: *mut mlx5hws_bwc_rule,
    pub hws_fs_actions: *mut mlx5_fs_hws_rule_action,
    pub num_fs_actions: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_data {
    pub hws_action: *mut mlx5hws_action,
    pub /: *mut *mut mutex lock; / protects hws_action,
    pub hws_action_refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_fs_hws_create_action_ctx {
    pub actions_type: mlx5hws_action_type,
    pub hws_ctx: *mut mlx5hws_context,
    pub id: u32,
    pub return_reg_id: u8,
}

extern "C" {
    pub fn mlx5_fs_put_hws_action(fs_hws_data: *mut mlx5_fs_hws_data);
}

extern "C" {
    pub fn mlx5_fs_hws_is_supported(dev: *mut mlx5_core_dev) -> bool;
}

