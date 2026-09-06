//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/act/vlan_mangle.c
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

    struct pedit_headers_action;
    int
    mlx5e_tc_act_vlan_add_rewrite_action(struct mlx5e_priv *priv, int namespace,
    const struct flow_action_entry *act,
    struct mlx5e_tc_flow_parse_attr *parse_attr,
    u32 *action, struct netlink_ext_ack *extack)
    {
    let mut mask16: u16 = VLAN_VID_MASK;
    let mut val16: u16 = act.vlan.vid & VLAN_VID_MASK;
    const struct flow_action_entry pedit_act = {
    .id = FLOW_ACTION_MANGLE,
    .mangle.htype = FLOW_ACT_MANGLE_HDR_TYPE_ETH,
    .mangle.offset = offsetof(struct vlan_ethhdr, h_vlan_TCI),
    .mangle.mask = ~(u32)be16_to_cpu(*(__be16 *)&mask16),
    .mangle.val = (u32)be16_to_cpu(*(__be16 *)&val16),
    };
    u8 match_prio_mask, match_prio_val;
    void *headers_c, *headers_v;
    int err;
    headers_c = mlx5e_get_match_headers_criteria(*action, &parse_attr.spec);
    headers_v = mlx5e_get_match_headers_value(*action, &parse_attr.spec);
    if (!(MLX5_GET(fte_match_set_lyr_2_4, headers_c, cvlan_tag) &&
    MLX5_GET(fte_match_set_lyr_2_4, headers_v, cvlan_tag))) {
    NL_SET_ERR_MSG_MOD(extack, "VLAN rewrite action must have VLAN protocol match");
    return -EOPNOTSUPP;
    }
    match_prio_mask = MLX5_GET(fte_match_set_lyr_2_4, headers_c, first_prio);
    match_prio_val = MLX5_GET(fte_match_set_lyr_2_4, headers_v, first_prio);
    if (act.vlan.prio != (match_prio_val & match_prio_mask)) {
    NL_SET_ERR_MSG_MOD(extack, "Changing VLAN prio is not supported");
    return -EOPNOTSUPP;
    }
    err = mlx5e_tc_act_pedit_parse_action(priv, &pedit_act, namespace, parse_attr.hdrs,
    extack);
// action |= MLX5_FLOW_CONTEXT_ACTION_MOD_HDR;
    return err;
    }
    static int
    tc_act_parse_vlan_mangle(struct mlx5e_tc_act_parse_state *parse_state,
    const struct flow_action_entry *act,
    struct mlx5e_priv *priv,
    struct mlx5_flow_attr *attr)
    {
    enum mlx5_flow_namespace_type ns_type;
    int err;
    ns_type = mlx5e_get_flow_namespace(parse_state.flow);
    err = mlx5e_tc_act_vlan_add_rewrite_action(priv, ns_type, act, attr.parse_attr,
    &attr.action, parse_state.extack);
    if (err)
    return err;
    if (ns_type == MLX5_FLOW_NAMESPACE_FDB) {
    attr.esw_attr.split_count = attr.esw_attr.out_count;
    parse_state.if_count = 0;
    }
    return 0;
    }
    struct mlx5e_tc_act mlx5e_tc_act_vlan_mangle = {
    .parse_action = tc_act_parse_vlan_mangle,
    };
