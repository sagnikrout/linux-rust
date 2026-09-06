//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/rss.h
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_rss_init_type {
    MLX5E_RSS_INIT_NO_TIRS = 0,
    MLX5E_RSS_INIT_TIRS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rss_init_params {
    pub type: mlx5e_rss_init_type,
    pub pkt_merge_param: *const mlx5e_packet_merge_param,
    pub nch: c_uint,
    pub max_nch: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rss_params {
    pub inner_ft_support: bool,
    pub drop_rqn: u32,
    pub self_lb_blk: bool,
}

extern "C" {
    pub fn mlx5e_rss_params_indir_cleanup(indir: *mut mlx5e_rss_params_indir);
}
extern "C" {
    pub fn mlx5e_rss_ctx_resize(rss: *mut mlx5e_rss, new_size: u32);
}
extern "C" {
    pub fn mlx5e_rss_cleanup(rss: *mut mlx5e_rss) -> c_int;
}
extern "C" {
    pub fn mlx5e_rss_refcnt_inc(rss: *mut mlx5e_rss);
}
extern "C" {
    pub fn mlx5e_rss_refcnt_dec(rss: *mut mlx5e_rss);
}
extern "C" {
    pub fn mlx5e_rss_refcnt_read(rss: *mut mlx5e_rss) -> c_uint;
}
extern "C" {
    pub fn mlx5e_rss_get_inner_ft_support(rss: *mut mlx5e_rss) -> bool;
}
extern "C" {
    pub fn mlx5e_rss_set_indir_actual_size(rss: *mut mlx5e_rss, size: u32);
}
extern "C" {
    pub fn mlx5e_rss_valid_tir(rss: *mut mlx5e_rss, tt: mlx5_traffic_types, inner: bool) -> bool;
}
extern "C" {
    pub fn mlx5e_rss_get_rqtn(rss: *mut mlx5e_rss) -> u32;
}
extern "C" {
    pub fn mlx5e_rss_enable(rss: *mut mlx5e_rss, rqns: *mut u32, vhca_ids: *mut u32, num_rqns: c_uint);
}
extern "C" {
    pub fn mlx5e_rss_disable(rss: *mut mlx5e_rss);
}
extern "C" {
    pub fn mlx5e_rss_get_hash(rss: *mut mlx5e_rss) -> mlx5e_rss_params_hash;
}
extern "C" {
    pub fn mlx5e_rss_get_hash_fields(rss: *mut mlx5e_rss, tt: mlx5_traffic_types) -> u8;
}
extern "C" {
    pub fn mlx5e_rss_set_indir_uniform(rss: *mut mlx5e_rss, nch: c_uint);
}
