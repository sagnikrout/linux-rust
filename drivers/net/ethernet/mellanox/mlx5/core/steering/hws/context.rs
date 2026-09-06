//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/context.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_context_flags {
    MLX5HWS_CONTEXT_FLAG_HWS_SUPPORT = 1 << 0,
    MLX5HWS_CONTEXT_FLAG_PRIVATE_PD = 1 << 1,
    MLX5HWS_CONTEXT_FLAG_BWC_SUPPORT = 1 << 2,
    MLX5HWS_CONTEXT_FLAG_NATIVE_SUPPORT = 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_context_shared_stc_type {
    MLX5HWS_CONTEXT_SHARED_STC_DECAP_L3 = 0,
    MLX5HWS_CONTEXT_SHARED_STC_DOUBLE_POP = 1,
    MLX5HWS_CONTEXT_SHARED_STC_MAX = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_context_common_res {
    pub default_stc: *mut mlx5hws_action_default_stc,
    pub shared_stc: [*mut mlx5hws_action_shared_stc; MLX5HWS_CONTEXT_SHARED_STC_MAX],
    pub default_miss: *mut mlx5hws_cmd_forward_tbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_context_debug_info {
    pub steering_debugfs: *mut dentry,
    pub fdb_debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_context_vports {
    pub esw_manager_gvmi: u16,
    pub uplink_gvmi: u16,
    pub vport_gvmi_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_context {
    pub mdev: *mut mlx5_core_dev,
    pub caps: *mut mlx5hws_cmd_query_caps,
    pub pd_num: u32,
    pub stc_pool: *mut mlx5hws_pool,
    pub /: *mut *mut *mut mlx5hws_action_ste_pool action_ste_pool; / One per queue,
    pub action_ste_cleanup: delayed_work,
    pub common_res: mlx5hws_context_common_res,
    pub pattern_cache: *mut mlx5hws_pattern_cache,
    pub definer_cache: *mut mlx5hws_definer_cache,
    pub /: *mut *mut mutex ctrl_lock; / control lock to protect the whole context,
    pub flags: mlx5hws_context_flags,
    pub send_queue: *mut mlx5hws_send_engine,
    pub queues: usize,
    pub /: *mut *mut *mut mutex bwc_send_queue_locks; / protect BWC queues,
    pub bwc_lock_class_keys: *mut lock_class_key,
    pub tbl_list: list_head,
    pub debug_info: mlx5hws_context_debug_info,
    pub peer_ctx_xa: xarray,
    pub vports: mlx5hws_context_vports,
}

extern "C" {
    pub fn mlx5hws_context_cap_dynamic_reparse(ctx: *mut mlx5hws_context) -> bool;
}
extern "C" {
    pub fn mlx5hws_context_get_reparse_mode(ctx: *mut mlx5hws_context) -> u8;
}
