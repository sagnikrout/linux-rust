//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tir.h
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
// Copyright (c) 2021, Mellanox Technologies inc. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rss_params_hash {
    pub hfunc: u8,
    pub toeplitz_hash_key: [u8; 40],
    pub symmetric: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rss_params_traffic_type {
    pub l3_prot_type: u8,
    pub l4_prot_type: u8,
    pub rx_hash_fields: u32,
}

extern "C" {
    pub fn mlx5e_tir_builder_free(builder: *mut mlx5e_tir_builder);
}
extern "C" {
    pub fn mlx5e_tir_builder_clear(builder: *mut mlx5e_tir_builder);
}
extern "C" {
    pub fn mlx5e_tir_builder_build_inline(builder: *mut mlx5e_tir_builder, tdn: u32, rqn: u32);
}
extern "C" {
    pub fn mlx5e_tir_builder_build_direct(builder: *mut mlx5e_tir_builder);
}
extern "C" {
    pub fn mlx5e_tir_builder_build_tls(builder: *mut mlx5e_tir_builder);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tir {
    pub mdev: *mut mlx5_core_dev,
    pub tirn: u32,
    pub list: list_head,
}

extern "C" {
    pub fn mlx5e_tir_destroy(tir: *mut mlx5e_tir);
}
extern "C" {
    pub fn mlx5e_tir_modify(tir: *mut mlx5e_tir, builder: *mut mlx5e_tir_builder) -> c_int;
}
