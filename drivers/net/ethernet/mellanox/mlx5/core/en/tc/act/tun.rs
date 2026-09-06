//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/act/tun.c
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

    static bool
    tc_act_can_offload_tun_encap(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    int act_index,
    struct mlx5_flow_attr *attr)
    {
    if (!act.tunnel) {
    NL_SET_ERR_MSG_MOD(parse_state.extack,
    "Zero tunnel attributes is not supported");
    return false;
    }
    return true;
    }
    static int
    tc_act_parse_tun_encap(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    struct mlx5e_priv *priv,
    struct mlx5_flow_attr *attr)
    {
    parse_state.tun_info = act.tunnel;
    parse_state.encap = true;
    return 0;
    }
    static int
    tc_act_parse_tun_decap(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    struct mlx5e_priv *priv,
    struct mlx5_flow_attr *attr)
    {
    parse_state.decap = true;
    return 0;
    }
    struct mlx5e_tc_act mlx5e_tc_act_tun_encap = {
    .can_offload = tc_act_can_offload_tun_encap,
    .parse_action = tc_act_parse_tun_encap,
    };
    struct mlx5e_tc_act mlx5e_tc_act_tun_decap = {
    .parse_action = tc_act_parse_tun_decap,
    };
