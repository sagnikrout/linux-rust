//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/action_ste_pool.h
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
pub const MLX5HWS_ACTION_STE_TABLE_INIT_LOG_SZ: c_int = 10;
pub const MLX5HWS_ACTION_STE_TABLE_STEP_LOG_SZ: c_int = 1;
pub const MLX5HWS_ACTION_STE_TABLE_MAX_LOG_SZ: c_int = 20;
pub const MLX5HWS_ACTION_STE_POOL_CLEANUP_SECONDS: c_int = 300;
pub const MLX5HWS_ACTION_STE_POOL_EXPIRE_SECONDS: c_int = 300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_ste_table {
    pub parent_elem: *mut mlx5hws_action_ste_pool_element,
// Wraps the RTC and STE range for this given action.
    pub pool: *mut mlx5hws_pool,
// Match STEs use this STC to jump to this pool's RTC.
    pub stc: mlx5hws_pool_chunk,
    pub rtc_0_id: u32,
    pub rtc_1_id: u32,
    pub list_node: list_head,
    pub last_used: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_ste_pool_element {
    pub ctx: *mut mlx5hws_context,
    pub parent_pool: *mut mlx5hws_action_ste_pool,
    pub /: *mut *mut size_t log_sz; / Size of the largest table so far.,
    pub opt: mlx5hws_pool_optimize,
    pub available: list_head,
    pub full: list_head,
}

// Central repository of action STEs. The context contains one of these pools
// per queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_ste_pool {
// Protects the entire pool. We have one pool per queue and only one
// operation can be active per rule at a given time. Thus this lock
// protects solely against concurrent garbage collection and we expect
// very little contention.
//
    pub lock: mutex,
    pub elems: [mlx5hws_action_ste_pool_element; MLX5HWS_POOL_OPTIMIZE_MAX],
}

// A chunk of STEs and the table it was allocated from. Used by rules.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_ste_chunk {
    pub action_tbl: *mut mlx5hws_action_ste_table,
    pub ste: mlx5hws_pool_chunk,
}

extern "C" {
    pub fn mlx5hws_action_ste_pool_init(ctx: *mut mlx5hws_context) -> c_int;
}
extern "C" {
    pub fn mlx5hws_action_ste_pool_uninit(ctx: *mut mlx5hws_context);
}
// Callers are expected to fill chunk->ste.order. On success, this function
// populates chunk->tbl and chunk->ste.offset.
//
extern "C" {
    pub fn mlx5hws_action_ste_chunk_free(chunk: *mut mlx5hws_action_ste_chunk);
}
