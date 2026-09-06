//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_trap.h
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
// Copyright (c) 2020 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_trap {
    pub policer_items_arr: *mut mlxsw_sp_trap_policer_item,
    pub /: *mut *mut size_t policers_count; / Number of registered policers,
    pub group_items_arr: *mut mlxsw_sp_trap_group_item,
    pub /: *mut *mut size_t groups_count; / Number of registered groups,
    pub trap_items_arr: *mut mlxsw_sp_trap_item,
    pub /: *mut *mut size_t traps_count; / Number of registered traps,
    pub thin_policer_hw_id: u16,
    pub max_policers: u64,
    pub /: *mut *mut unsigned long policers_usage[]; / Usage bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_trap_ops {
    pub p_groups_count): *mut usize,
    pub p_traps_count): *mut usize,
}
