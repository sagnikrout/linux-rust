//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/int_port.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_tc_int_port_type {
    MLX5E_TC_INT_PORT_INGRESS,
    MLX5E_TC_INT_PORT_EGRESS,
}

extern "C" {
    pub fn mlx5e_tc_int_port_supported(esw: *const mlx5_eswitch) -> bool;
}
extern "C" {
    pub fn mlx5e_tc_int_port_init_rep_rx(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_tc_int_port_cleanup_rep_rx(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_tc_int_port_get_metadata(int_port: *mut mlx5e_tc_int_port) -> u32;
}
extern "C" {
    pub fn mlx5e_tc_int_port_get_metadata_for_match(int_port: *mut mlx5e_tc_int_port) -> u32;
}
extern "C" {
    pub fn mlx5e_tc_int_port_get_flow_source(int_port: *mut mlx5e_tc_int_port) -> c_int;
}

