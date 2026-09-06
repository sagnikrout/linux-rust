//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_mr.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_mr_route_action {
    MLXSW_SP_MR_ROUTE_ACTION_FORWARD,
    MLXSW_SP_MR_ROUTE_ACTION_TRAP,
    MLXSW_SP_MR_ROUTE_ACTION_TRAP_AND_FORWARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mr_route_key {
    pub vrid: c_int,
    pub proto: mlxsw_sp_l3proto,
    pub group: mlxsw_sp_l3addr,
    pub group_mask: mlxsw_sp_l3addr,
    pub source: mlxsw_sp_l3addr,
    pub source_mask: mlxsw_sp_l3addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mr_route_info {
    pub route_action: mlxsw_sp_mr_route_action,
    pub irif_index: u16,
    pub erif_indices: *mut u16,
    pub erif_num: usize,
    pub min_mtu: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mr_route_params {
    pub key: mlxsw_sp_mr_route_key,
    pub value: mlxsw_sp_mr_route_info,
    pub prio: mlxsw_sp_mr_route_prio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mr_ops {
    pub priv_size: c_int,
    pub route_priv_size: c_int,
    pub priv): *mut *mut *mut int (init)(struct mlxsw_sp mlxsw_sp, void,
    pub route_params): *mut mlxsw_sp_mr_route_params,
    pub route_info): *mut mlxsw_sp_mr_route_info,
    pub bytes): *mut *mut u64 packets, u64,
    pub route_action): mlxsw_sp_mr_route_action,
    pub min_mtu): u16,
    pub irif_index): u16,
    pub erif_index): u16,
    pub erif_index): u16,
    pub route_priv): *mut c_void,
    pub priv): *mut *mut *mut void (fini)(struct mlxsw_sp mlxsw_sp, void,
}

extern "C" {
    pub fn mlxsw_sp_mr_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_mr_vif_del(mr_table: *mut mlxsw_sp_mr_table, vif_index: vifi_t);
}
extern "C" {
    pub fn mlxsw_sp_mr_table_destroy(mr_table: *mut mlxsw_sp_mr_table);
}
extern "C" {
    pub fn mlxsw_sp_mr_table_flush(mr_table: *mut mlxsw_sp_mr_table);
}
extern "C" {
    pub fn mlxsw_sp_mr_table_empty(mr_table: *const mlxsw_sp_mr_table) -> bool;
}
