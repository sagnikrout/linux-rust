//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc_tun.h
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
// Copyright (c) 2018 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_encap_key {
    pub ip_tun_key: *const ip_tunnel_key,
    pub tc_tunnel: *mut mlx5e_tc_tunnel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_tunnel {
    pub tunnel_type: c_int,
    pub match_level: mlx5_flow_match_level,
    pub priv): *mut *mut bool (can_offload)(struct mlx5e_priv,
    pub e): *mut *mut int (calc_hlen)(struct mlx5e_encap_entry,
    pub extack): *mut netlink_ext_ack,
    pub e): *mut mlx5e_encap_entry,
    pub headers_v): *mut c_void,
    pub headers_v): *mut c_void,
    pub b): *mut mlx5e_encap_key,
    pub mirred_dev): *mut *mut int (get_remote_ifindex)(struct net_device,
}

