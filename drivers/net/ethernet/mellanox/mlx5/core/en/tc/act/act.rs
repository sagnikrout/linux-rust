//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/act/act.h
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
#[derive(Copy, Clone)]
pub struct mlx5e_tc_act_parse_state {
    pub flow_action: *mut flow_action,
    pub flow: *mut mlx5e_tc_flow,
    pub extack: *mut netlink_ext_ack,
    pub actions: u32,
    pub encap: bool,
    pub decap: bool,
    pub mpls_push: bool,
    pub eth_push: bool,
    pub eth_pop: bool,
    pub ptype_host: bool,
    pub tun_info: *const ip_tunnel_info,
    pub mpls_info: mlx5e_mpls_info,
    pub ifindexes: [c_int; MLX5_MAX_FLOW_FWD_VPORTS],
    pub if_count: c_int,
    pub ct_priv: *mut mlx5_tc_ct_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_act_branch_ctrl {
    pub act_id: flow_action_id,
    pub extval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_act {
    pub attr): *mut mlx5_flow_attr,
    pub attr): *mut mlx5_flow_attr,
    pub attr): *mut mlx5_flow_attr,
    pub attr): *mut mlx5_flow_attr,
    pub act): *const *const bool (is_missable)(struct flow_action_entry,
    pub act): *mut flow_action_entry,
    pub fl_act): *mut flow_offload_action,
    pub fl_act): *mut flow_offload_action,
    pub cond_false): *mut mlx5e_tc_act_branch_ctrl,
    pub is_terminating_action: bool,
}
