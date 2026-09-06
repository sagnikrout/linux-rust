//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/acl/egress_lgcy.c
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
// Copyright (c) 2020 Mellanox Technologies Inc. All rights reserved.

#[no_mangle]
unsafe extern "C" fn esw_acl_egress_lgcy_rules_destroy(vport: *mut mlx5_vport) {
    static void esw_acl_egress_lgcy_rules_destroy(struct mlx5_vport *vport)
    {
    esw_acl_egress_vlan_destroy(vport);
    if (!IS_ERR_OR_NULL(vport.egress.legacy.drop_rule)) {
    mlx5_del_flow_rules(vport.egress.legacy.drop_rule);
    vport.egress.legacy.drop_rule = core::ptr::null_mut();
    }
    }
    static int esw_acl_egress_lgcy_groups_create(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    let mut inlen: c_int = MLX5_ST_SZ_BYTES(create_flow_group_in);
    struct mlx5_core_dev *dev = esw.dev;
    struct mlx5_flow_group *drop_grp;
    u32 *flow_group_in;
    let mut err: c_int = 0;
    err = esw_acl_egress_vlan_grp_create(esw, vport);
    if (err)
    return err;
    flow_group_in = kvzalloc(inlen, GFP_KERNEL);
    if (!flow_group_in) {
    err = -ENOMEM;
    goto alloc_err;
    }
    MLX5_SET(create_flow_group_in, flow_group_in, start_flow_index, 1);
    MLX5_SET(create_flow_group_in, flow_group_in, end_flow_index, 1);
    drop_grp = mlx5_create_flow_group(vport.egress.acl, flow_group_in);
    if (IS_ERR(drop_grp)) {
    err = PTR_ERR(drop_grp);
    esw_warn(dev, "Failed to create E-Switch vport[%d] egress drop flow group, err(%d)\n",
    vport.vport, err);
    goto drop_grp_err;
    }
    vport.egress.legacy.drop_grp = drop_grp;
    kvfree(flow_group_in);
    return 0;
    drop_grp_err:
    kvfree(flow_group_in);
    alloc_err:
    esw_acl_egress_vlan_grp_destroy(vport);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn esw_acl_egress_lgcy_groups_destroy(vport: *mut mlx5_vport) {
    static void esw_acl_egress_lgcy_groups_destroy(struct mlx5_vport *vport)
    {
    if (!IS_ERR_OR_NULL(vport.egress.legacy.drop_grp)) {
    mlx5_destroy_flow_group(vport.egress.legacy.drop_grp);
    vport.egress.legacy.drop_grp = core::ptr::null_mut();
    }
    esw_acl_egress_vlan_grp_destroy(vport);
    }
    int esw_acl_egress_lgcy_setup(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    let mut vst_mode_steering: bool = esw_vst_mode_is_steering(esw);
    let mut drop_ctr_dst: mlx5_flow_destination = {};
    struct mlx5_flow_destination *dst = core::ptr::null_mut();
    struct mlx5_fc *drop_counter = core::ptr::null_mut();
    let mut flow_act: mlx5_flow_act = {};
// The egress acl table contains 2 rules:
// 1)Allow traffic with vlan_tag=vst_vlan_id
// 2)Drop all other traffic.
//
    let mut table_size: c_int = 2;
    let mut dest_num: c_int = 0;
    int actions_flag;
    let mut err: c_int = 0;
    if (vport.egress.legacy.drop_counter) {
    drop_counter = vport.egress.legacy.drop_counter;
    } else if (MLX5_CAP_ESW_EGRESS_ACL(esw.dev, flow_counter)) {
    drop_counter = mlx5_fc_create(esw.dev, false);
    if (IS_ERR(drop_counter)) {
    esw_warn(esw.dev,
    "vport[%d] configure egress drop rule counter err(%pe)\n",
    vport.vport, drop_counter);
    drop_counter = core::ptr::null_mut();
    }
    vport.egress.legacy.drop_counter = drop_counter;
    }
    esw_acl_egress_lgcy_rules_destroy(vport);
    if (!vport.info.vlan && !vport.info.qos) {
    esw_acl_egress_lgcy_cleanup(esw, vport);
    return 0;
    }
    if (!vport.egress.acl) {
    vport.egress.acl = esw_acl_table_create(esw, vport,
    MLX5_FLOW_NAMESPACE_ESW_EGRESS,
    table_size);
    if (IS_ERR(vport.egress.acl)) {
    err = PTR_ERR(vport.egress.acl);
    vport.egress.acl = core::ptr::null_mut();
    goto out;
    }
    err = esw_acl_egress_lgcy_groups_create(esw, vport);
    if (err)
    goto out;
    }
    esw_debug(esw.dev,
    "vport[%d] configure egress rules, vlan(%d) qos(%d)\n",
    vport.vport, vport.info.vlan, vport.info.qos);
// Allowed vlan rule
    actions_flag = MLX5_FLOW_CONTEXT_ACTION_ALLOW;
    if (vst_mode_steering)
    actions_flag |= MLX5_FLOW_CONTEXT_ACTION_VLAN_POP;
    err = esw_egress_acl_vlan_create(esw, vport, core::ptr::null_mut(), vport.info.vlan,
    actions_flag);
    if (err)
    goto out;
    flow_act.action = MLX5_FLOW_CONTEXT_ACTION_DROP;
// Attach egress drop flow counter
    if (drop_counter) {
    flow_act.action |= MLX5_FLOW_CONTEXT_ACTION_COUNT;
    drop_ctr_dst.type = MLX5_FLOW_DESTINATION_TYPE_COUNTER;
    drop_ctr_dst.counter = drop_counter;
    dst = &drop_ctr_dst;
    dest_num++;
    }
    vport.egress.legacy.drop_rule =
    mlx5_add_flow_rules(vport.egress.acl, core::ptr::null_mut(),
    &flow_act, dst, dest_num);
    if (IS_ERR(vport.egress.legacy.drop_rule)) {
    err = PTR_ERR(vport.egress.legacy.drop_rule);
    esw_warn(esw.dev,
    "vport[%d] configure egress drop rule failed, err(%d)\n",
    vport.vport, err);
    vport.egress.legacy.drop_rule = core::ptr::null_mut();
    goto out;
    }
    return err;
    out:
    esw_acl_egress_lgcy_cleanup(esw, vport);
    return err;
    }
    void esw_acl_egress_lgcy_cleanup(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    if (IS_ERR_OR_NULL(vport.egress.acl))
    goto clean_drop_counter;
    esw_debug(esw.dev, "Destroy vport[%d] E-Switch egress ACL\n", vport.vport);
    esw_acl_egress_lgcy_rules_destroy(vport);
    esw_acl_egress_lgcy_groups_destroy(vport);
    esw_acl_egress_table_destroy(vport);
    clean_drop_counter:
    if (vport.egress.legacy.drop_counter) {
    mlx5_fc_destroy(esw.dev, vport.egress.legacy.drop_counter);
    vport.egress.legacy.drop_counter = core::ptr::null_mut();
    }
    }
