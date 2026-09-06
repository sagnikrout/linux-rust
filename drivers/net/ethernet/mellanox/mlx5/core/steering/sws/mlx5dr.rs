//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/mlx5dr.h
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
// Copyright (c) 2019, Mellanox Technologies
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_domain_type {
    MLX5DR_DOMAIN_TYPE_NIC_RX,
    MLX5DR_DOMAIN_TYPE_NIC_TX,
    MLX5DR_DOMAIN_TYPE_FDB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_domain_sync_flags {
    MLX5DR_DOMAIN_SYNC_FLAGS_SW = 1 << 0,
    MLX5DR_DOMAIN_SYNC_FLAGS_HW = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_action_reformat_type {
    DR_ACTION_REFORMAT_TYP_TNL_L2_TO_L2,
    DR_ACTION_REFORMAT_TYP_L2_TO_TNL_L2,
    DR_ACTION_REFORMAT_TYP_TNL_L3_TO_L2,
    DR_ACTION_REFORMAT_TYP_L2_TO_TNL_L3,
    DR_ACTION_REFORMAT_TYP_INSERT_HDR,
    DR_ACTION_REFORMAT_TYP_REMOVE_HDR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_parameters {
    pub match_sz: usize,
    pub /: *mut *mut *mut u64 match_buf; / Device spec format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_dest {
    pub dest: *mut mlx5dr_action,
    pub reformat: *mut mlx5dr_action,
}

extern "C" {
    pub fn mlx5dr_domain_destroy(domain: *mut mlx5dr_domain) -> c_int;
}
extern "C" {
    pub fn mlx5dr_table_destroy(table: *mut mlx5dr_table) -> c_int;
}
extern "C" {
    pub fn mlx5dr_table_get_id(table: *mut mlx5dr_table) -> u32;
}
extern "C" {
    pub fn mlx5dr_matcher_destroy(matcher: *mut mlx5dr_matcher) -> c_int;
}
extern "C" {
    pub fn mlx5dr_rule_destroy(rule: *mut mlx5dr_rule) -> c_int;
}
extern "C" {
    pub fn mlx5dr_action_destroy(action: *mut mlx5dr_action) -> c_int;
}
extern "C" {
    pub fn mlx5dr_action_get_pkt_reformat_id(action: *mut mlx5dr_action) -> u32;
}
// buddy functions & structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_icm_buddy_mem {
    pub bitmap: *mut c_ulong,
    pub num_free: *mut c_uint,
    pub max_order: u32,
    pub list_node: list_head,
    pub icm_mr: *mut mlx5dr_icm_mr,
    pub pool: *mut mlx5dr_icm_pool,
// Amount of memory in used chunks - HW may be accessing this memory
    pub used_memory: u64,
// Memory optimisation
    pub ste_arr: *mut mlx5dr_ste,
    pub miss_list: *mut list_head,
    pub hw_ste_arr: *mut u8,
}

extern "C" {
    pub fn mlx5dr_buddy_cleanup(buddy: *mut mlx5dr_icm_buddy_mem);
}
