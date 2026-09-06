//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/hv_vhca.h
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
// Copyright (c) 2019 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_hv_vhca_agent_type {
    MLX5_HV_VHCA_AGENT_CONTROL = 0,
    MLX5_HV_VHCA_AGENT_STATS   = 1,
    MLX5_HV_VHCA_AGENT_MAX = 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_hv_vhca_control_block {
    pub capabilities: u32,
    pub control: u32,
    pub command: u16,
    pub command_ack: u16,
    pub version: u16,
    pub rings: u16,
    pub reserved1: [u32; 28],
}

extern "C" {
    pub fn mlx5_hv_vhca_destroy(hv_vhca: *mut mlx5_hv_vhca);
}
extern "C" {
    pub fn mlx5_hv_vhca_init(hv_vhca: *mut mlx5_hv_vhca);
}
extern "C" {
    pub fn mlx5_hv_vhca_cleanup(hv_vhca: *mut mlx5_hv_vhca);
}
extern "C" {
    pub fn mlx5_hv_vhca_invalidate(context: *mut c_void, block_mask: u64);
}
extern "C" {
    pub fn mlx5_hv_vhca_agent_destroy(agent: *mut mlx5_hv_vhca_agent);
}

