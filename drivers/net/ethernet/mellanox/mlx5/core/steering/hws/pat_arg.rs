//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/pat_arg.h
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
// Modify-header arg pool
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_arg_chunk_size {
    MLX5HWS_ARG_CHUNK_SIZE_1,
// Keep MIN updated when changing
    MLX5HWS_ARG_CHUNK_SIZE_MIN = MLX5HWS_ARG_CHUNK_SIZE_1,
    MLX5HWS_ARG_CHUNK_SIZE_2,
    MLX5HWS_ARG_CHUNK_SIZE_3,
    MLX5HWS_ARG_CHUNK_SIZE_4,
    MLX5HWS_ARG_CHUNK_SIZE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pattern_cache {
    pub /: *mut *mut mutex lock; / Protect pattern list,
    pub ptrn_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_pattern_cache_item {
    pub pattern_id: u32,
    pub data: *mut u8,
    pub num_of_actions: u16,
    pub mh_data: },
    pub /: *mut *mut u32 refcount; / protected by pattern_cache lock,
    pub ptrn_list_node: list_head,
}

extern "C" {
    pub fn mlx5hws_arg_get_arg_size(num_of_actions: u16) -> u32;
}
extern "C" {
    pub fn mlx5hws_arg_data_size_to_arg_size(data_size: u16) -> u32;
}
extern "C" {
    pub fn mlx5hws_pat_init_pattern_cache(cache: *mut mlx5hws_pattern_cache) -> c_int;
}
extern "C" {
    pub fn mlx5hws_pat_uninit_pattern_cache(cache: *mut mlx5hws_pattern_cache);
}
extern "C" {
    pub fn mlx5hws_pat_verify_actions(ctx: *mut mlx5hws_context, pattern[]: __be64, sz: usize) -> bool;
}
extern "C" {
    pub fn mlx5hws_arg_destroy(ctx: *mut mlx5hws_context, arg_id: u32);
}
extern "C" {
    pub fn mlx5hws_pat_require_reparse(actions: *mut __be64, num_of_actions: u16) -> bool;
}
