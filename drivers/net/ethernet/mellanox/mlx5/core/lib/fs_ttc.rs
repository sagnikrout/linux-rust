//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/fs_ttc.h
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
// Copyright (c) 2020 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_traffic_types {
    MLX5_TT_IPV4_TCP,
    MLX5_TT_IPV6_TCP,
    MLX5_TT_IPV4_UDP,
    MLX5_TT_IPV6_UDP,
    MLX5_TT_IPV4_IPSEC_AH,
    MLX5_TT_IPV6_IPSEC_AH,
    MLX5_TT_IPV4_IPSEC_ESP,
    MLX5_TT_IPV6_IPSEC_ESP,
    MLX5_TT_IPV4,
    MLX5_TT_IPV6,
    MLX5_TT_ANY,
    MLX5_TT_DECRYPTED_ESP_OUTER_IPV4_TCP,
    MLX5_TT_DECRYPTED_ESP_OUTER_IPV6_TCP,
    MLX5_TT_DECRYPTED_ESP_OUTER_IPV4_UDP,
    MLX5_TT_DECRYPTED_ESP_OUTER_IPV6_UDP,
    MLX5_TT_DECRYPTED_ESP_INNER_IPV4_TCP,
    MLX5_TT_DECRYPTED_ESP_INNER_IPV6_TCP,
    MLX5_TT_DECRYPTED_ESP_INNER_IPV4_UDP,
    MLX5_TT_DECRYPTED_ESP_INNER_IPV6_UDP,
    MLX5_NUM_TT,
    MLX5_NUM_INDIR_TIRS = MLX5_TT_ANY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_tunnel_types {
    MLX5_TT_IPV4_GRE,
    MLX5_TT_IPV6_GRE,
    MLX5_TT_IPV4_IPIP,
    MLX5_TT_IPV6_IPIP,
    MLX5_TT_IPV4_IPV6,
    MLX5_TT_IPV6_IPV6,
    MLX5_NUM_TUNNEL_TT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ttc_rule {
    pub rule: *mut mlx5_flow_handle,
    pub default_dest: mlx5_flow_destination,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttc_params {
    pub ns_type: mlx5_flow_namespace_type,
    pub ft_attr: mlx5_flow_table_attr,
    pub dests: [mlx5_flow_destination; MLX5_NUM_TT],
    pub MLX5_NUM_TT): DECLARE_BITMAP(ignore_dests,,
    pub inner_ttc: bool,
    pub MLX5_NUM_TUNNEL_TT): DECLARE_BITMAP(ignore_tunnel_dests,,
    pub tunnel_dests: [mlx5_flow_destination; MLX5_NUM_TUNNEL_TT],
    pub ipsec_rss: bool,
}

extern "C" {
    pub fn mlx5_destroy_ttc_table(ttc: *mut mlx5_ttc_table);
}
extern "C" {
    pub fn mlx5_tunnel_inner_ft_supported(mdev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_get_proto_by_tunnel_type(tt: mlx5_tunnel_types) -> u8;
}
extern "C" {
    pub fn mlx5_ttc_has_esp_flow_group(ttc: *mut mlx5_ttc_table) -> bool;
}
extern "C" {
    pub fn mlx5_ttc_destroy_ipsec_rules(ttc: *mut mlx5_ttc_table);
}
