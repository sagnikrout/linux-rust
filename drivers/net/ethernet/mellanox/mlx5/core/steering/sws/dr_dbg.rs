//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_dbg.h
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const MLX5DR_DEBUG_DUMP_BUFF_LENGTH: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_dbg_dump_buff {
    pub buff: *mut c_char,
    pub index: u32,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_dbg_dump_data {
    pub buff_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_dbg_dump_info {
    pub /: *mut *mut mutex dbg_mutex; / protect dbg lists,
    pub steering_debugfs: *mut dentry,
    pub fdb_debugfs: *mut dentry,
    pub dump_data: *mut mlx5dr_dbg_dump_data,
    pub state: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn mlx5dr_dbg_init_dump(dmn: *mut mlx5dr_domain);
}
extern "C" {
    pub fn mlx5dr_dbg_uninit_dump(dmn: *mut mlx5dr_domain);
}
extern "C" {
    pub fn mlx5dr_dbg_tbl_add(tbl: *mut mlx5dr_table);
}
extern "C" {
    pub fn mlx5dr_dbg_tbl_del(tbl: *mut mlx5dr_table);
}
extern "C" {
    pub fn mlx5dr_dbg_rule_add(rule: *mut mlx5dr_rule);
}
extern "C" {
    pub fn mlx5dr_dbg_rule_del(rule: *mut mlx5dr_rule);
}
