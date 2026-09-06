//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/table.h
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
#[derive(Copy, Clone)]
pub struct mlx5hws_default_miss {
// My miss table
    pub miss_tbl: *mut mlx5hws_table,
    pub next: list_head,
// Tables missing to my table
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_table {
    pub ctx: *mut mlx5hws_context,
    pub ft_id: u32,
    pub type: mlx5hws_table_type,
    pub fw_ft_type: u32,
    pub level: u32,
    pub uid: u16,
    pub matchers_list: list_head,
    pub tbl_list_node: list_head,
    pub default_miss: mlx5hws_default_miss,
}

// ret_type = FS_FT_FDB;
extern "C" {
    pub fn mlx5hws_table_update_connected_miss_tables(dst_tbl: *mut mlx5hws_table) -> c_int;
}
extern "C" {
    pub fn mlx5hws_table_ft_set_default_next_ft(tbl: *mut mlx5hws_table, ft_id: u32) -> c_int;
}
