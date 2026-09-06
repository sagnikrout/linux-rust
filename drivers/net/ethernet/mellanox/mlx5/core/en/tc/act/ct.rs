//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/act/ct.c
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
    tc_act_can_offload_ct(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    int act_index,
    struct mlx5_flow_attr *attr)
    {
    return !((act.ct.action & TCA_CT_ACT_COMMIT) &&
    flow_action_is_last_entry(parse_state.flow_action, act));
    }
    static int
    tc_act_parse_ct(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    struct mlx5e_priv *priv,
    struct mlx5_flow_attr *attr)
    {
    int err;
    err = mlx5_tc_ct_parse_action(parse_state.ct_priv, attr, act, parse_state.extack);
    if (err)
    return err;
    if (mlx5e_is_eswitch_flow(parse_state.flow)) {
    attr.esw_attr.split_count = attr.esw_attr.out_count;
    parse_state.if_count = 0;
    }
    attr.flags |= MLX5_ATTR_FLAG_CT;
    return 0;
    }
    static int
    tc_act_post_parse_ct(struct mlx5e_tc_act_parse_state *parse_state,
    struct mlx5e_priv *priv,
    struct mlx5_flow_attr *attr)
    {
    if (!(attr.flags & MLX5_ATTR_FLAG_CT))
    return 0;
    return mlx5_tc_ct_flow_offload(parse_state.ct_priv, attr);
    }
    static bool
    tc_act_is_multi_table_act_ct(struct mlx5e_priv *priv,
    const struct flow_action_entry *act,
    struct mlx5_flow_attr *attr)
    {
    if (act.ct.action & TCA_CT_ACT_CLEAR)
    return false;
    return true;
    }
    static bool
    tc_act_is_missable_ct(const struct flow_action_entry *act)
    {
    return !(act.ct.action & TCA_CT_ACT_CLEAR);
    }
    struct mlx5e_tc_act mlx5e_tc_act_ct = {
    .can_offload = tc_act_can_offload_ct,
    .parse_action = tc_act_parse_ct,
    .post_parse = tc_act_post_parse_ct,
    .is_multi_table_act = tc_act_is_multi_table_act_ct,
    .is_missable = tc_act_is_missable_ct,
    };
