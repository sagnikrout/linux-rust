//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/acl/ingress_lgcy.c
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
unsafe extern "C" fn esw_acl_ingress_lgcy_rules_destroy(vport: *mut mlx5_vport) {
    static void esw_acl_ingress_lgcy_rules_destroy(struct mlx5_vport *vport)
    {
    if (vport.ingress.legacy.drop_rule) {
    mlx5_del_flow_rules(vport.ingress.legacy.drop_rule);
    vport.ingress.legacy.drop_rule = core::ptr::null_mut();
    }
    esw_acl_ingress_allow_rule_destroy(vport);
    }
    static int esw_acl_ingress_lgcy_groups_create(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    let mut inlen: c_int = MLX5_ST_SZ_BYTES(create_flow_group_in);
    struct mlx5_core_dev *dev = esw.dev;
    struct mlx5_flow_group *g;
    void *match_criteria;
    u32 *flow_group_in;
    int err;
    flow_group_in = kvzalloc(inlen, GFP_KERNEL);
    if (!flow_group_in)
    return -ENOMEM;
    match_criteria = MLX5_ADDR_OF(create_flow_group_in, flow_group_in, match_criteria);
    MLX5_SET(create_flow_group_in, flow_group_in, match_criteria_enable,
    MLX5_MATCH_OUTER_HEADERS);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.cvlan_tag);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.smac_47_16);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.smac_15_0);
    MLX5_SET(create_flow_group_in, flow_group_in, start_flow_index, 0);
    MLX5_SET(create_flow_group_in, flow_group_in, end_flow_index, 0);
    g = mlx5_create_flow_group(vport.ingress.acl, flow_group_in);
    if (IS_ERR(g)) {
    err = PTR_ERR(g);
    esw_warn(dev, "vport[%d] ingress create untagged spoofchk flow group, err(%d)\n",
    vport.vport, err);
    goto spoof_err;
    }
    vport.ingress.legacy.allow_untagged_spoofchk_grp = g;
    memset(flow_group_in, 0, inlen);
    MLX5_SET(create_flow_group_in, flow_group_in, match_criteria_enable,
    MLX5_MATCH_OUTER_HEADERS);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.cvlan_tag);
    MLX5_SET(create_flow_group_in, flow_group_in, start_flow_index, 1);
    MLX5_SET(create_flow_group_in, flow_group_in, end_flow_index, 1);
    g = mlx5_create_flow_group(vport.ingress.acl, flow_group_in);
    if (IS_ERR(g)) {
    err = PTR_ERR(g);
    esw_warn(dev, "vport[%d] ingress create untagged flow group, err(%d)\n",
    vport.vport, err);
    goto untagged_err;
    }
    vport.ingress.legacy.allow_untagged_only_grp = g;
    memset(flow_group_in, 0, inlen);
    MLX5_SET(create_flow_group_in, flow_group_in, match_criteria_enable,
    MLX5_MATCH_OUTER_HEADERS);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.smac_47_16);
    MLX5_SET_TO_ONES(fte_match_param, match_criteria, outer_headers.smac_15_0);
    MLX5_SET(create_flow_group_in, flow_group_in, start_flow_index, 2);
    MLX5_SET(create_flow_group_in, flow_group_in, end_flow_index, 2);
    g = mlx5_create_flow_group(vport.ingress.acl, flow_group_in);
    if (IS_ERR(g)) {
    err = PTR_ERR(g);
    esw_warn(dev, "vport[%d] ingress create spoofchk flow group, err(%d)\n",
    vport.vport, err);
    goto allow_spoof_err;
    }
    vport.ingress.legacy.allow_spoofchk_only_grp = g;
    memset(flow_group_in, 0, inlen);
    MLX5_SET(create_flow_group_in, flow_group_in, start_flow_index, 3);
    MLX5_SET(create_flow_group_in, flow_group_in, end_flow_index, 3);
    g = mlx5_create_flow_group(vport.ingress.acl, flow_group_in);
    if (IS_ERR(g)) {
    err = PTR_ERR(g);
    esw_warn(dev, "vport[%d] ingress create drop flow group, err(%d)\n",
    vport.vport, err);
    goto drop_err;
    }
    vport.ingress.legacy.drop_grp = g;
    kvfree(flow_group_in);
    return 0;
    drop_err:
    if (!IS_ERR_OR_NULL(vport.ingress.legacy.allow_spoofchk_only_grp)) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_spoofchk_only_grp);
    vport.ingress.legacy.allow_spoofchk_only_grp = core::ptr::null_mut();
    }
    allow_spoof_err:
    if (!IS_ERR_OR_NULL(vport.ingress.legacy.allow_untagged_only_grp)) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_untagged_only_grp);
    vport.ingress.legacy.allow_untagged_only_grp = core::ptr::null_mut();
    }
    untagged_err:
    if (!IS_ERR_OR_NULL(vport.ingress.legacy.allow_untagged_spoofchk_grp)) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_untagged_spoofchk_grp);
    vport.ingress.legacy.allow_untagged_spoofchk_grp = core::ptr::null_mut();
    }
    spoof_err:
    kvfree(flow_group_in);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn esw_acl_ingress_lgcy_groups_destroy(vport: *mut mlx5_vport) {
    static void esw_acl_ingress_lgcy_groups_destroy(struct mlx5_vport *vport)
    {
    if (vport.ingress.legacy.allow_spoofchk_only_grp) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_spoofchk_only_grp);
    vport.ingress.legacy.allow_spoofchk_only_grp = core::ptr::null_mut();
    }
    if (vport.ingress.legacy.allow_untagged_only_grp) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_untagged_only_grp);
    vport.ingress.legacy.allow_untagged_only_grp = core::ptr::null_mut();
    }
    if (vport.ingress.legacy.allow_untagged_spoofchk_grp) {
    mlx5_destroy_flow_group(vport.ingress.legacy.allow_untagged_spoofchk_grp);
    vport.ingress.legacy.allow_untagged_spoofchk_grp = core::ptr::null_mut();
    }
    if (vport.ingress.legacy.drop_grp) {
    mlx5_destroy_flow_group(vport.ingress.legacy.drop_grp);
    vport.ingress.legacy.drop_grp = core::ptr::null_mut();
    }
    }
    int esw_acl_ingress_lgcy_setup(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    let mut vst_mode_steering: bool = esw_vst_mode_is_steering(esw);
    let mut drop_ctr_dst: mlx5_flow_destination = {};
    struct mlx5_flow_destination *dst = core::ptr::null_mut();
    let mut flow_act: mlx5_flow_act = {};
    struct mlx5_flow_spec *spec = core::ptr::null_mut();
    struct mlx5_fc *counter = core::ptr::null_mut();
    let mut vst_check_cvlan: bool = false;
    let mut vst_push_cvlan: bool = false;
// The ingress acl table contains 4 groups
// (2 active rules at the same time -
// 1 allow rule from one of the first 3 groups.
// 1 drop rule from the last group):
// 1)Allow untagged traffic with smac=original mac.
// 2)Allow untagged traffic.
// 3)Allow traffic with smac=original mac.
// 4)Drop all other traffic.
//
    let mut table_size: c_int = 4;
    let mut dest_num: c_int = 0;
    let mut err: c_int = 0;
    u8 *smac_v;
    esw_acl_ingress_lgcy_rules_destroy(vport);
    if (vport.ingress.legacy.drop_counter) {
    counter = vport.ingress.legacy.drop_counter;
    } else if (MLX5_CAP_ESW_INGRESS_ACL(esw.dev, flow_counter)) {
    counter = mlx5_fc_create(esw.dev, false);
    if (IS_ERR(counter)) {
    esw_warn(esw.dev,
    "vport[%d] configure ingress drop rule counter failed\n",
    vport.vport);
    counter = core::ptr::null_mut();
    }
    vport.ingress.legacy.drop_counter = counter;
    }
    if (!vport.info.vlan && !vport.info.qos && !vport.info.spoofchk) {
    esw_acl_ingress_lgcy_cleanup(esw, vport);
    return 0;
    }
    if (!vport.ingress.acl) {
    vport.ingress.acl = esw_acl_table_create(esw, vport,
    MLX5_FLOW_NAMESPACE_ESW_INGRESS,
    table_size);
    if (IS_ERR(vport.ingress.acl)) {
    err = PTR_ERR(vport.ingress.acl);
    vport.ingress.acl = core::ptr::null_mut();
    goto out;
    }
    err = esw_acl_ingress_lgcy_groups_create(esw, vport);
    if (err)
    goto out;
    }
    esw_debug(esw.dev,
    "vport[%d] configure ingress rules, vlan(%d) qos(%d)\n",
    vport.vport, vport.info.vlan, vport.info.qos);
    spec = kvzalloc_obj(*spec);
    if (!spec) {
    err = -ENOMEM;
    goto out;
    }
    if ((vport.info.vlan || vport.info.qos)) {
    if (vst_mode_steering)
    vst_push_cvlan = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !MLX5_CAP_ESW(esw->dev, _arg: vport_cvlan_insert_always)) -> else {
    else if (!MLX5_CAP_ESW(esw.dev, vport_cvlan_insert_always))
    vst_check_cvlan = true;
    }
    if (vst_check_cvlan || vport.info.spoofchk)
    spec.match_criteria_enable = MLX5_MATCH_OUTER_HEADERS;
// Create ingress allow rule
    flow_act.action = MLX5_FLOW_CONTEXT_ACTION_ALLOW;
    if (vst_push_cvlan) {
    flow_act.action |= MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH;
    flow_act.vlan[0].prio = vport.info.qos;
    flow_act.vlan[0].vid = vport.info.vlan;
    flow_act.vlan[0].ethtype = ETH_P_8021Q;
    }
    if (vst_check_cvlan)
    MLX5_SET_TO_ONES(fte_match_param, spec.match_criteria,
    outer_headers.cvlan_tag);
    if (vport.info.spoofchk) {
    MLX5_SET_TO_ONES(fte_match_param, spec.match_criteria,
    outer_headers.smac_47_16);
    MLX5_SET_TO_ONES(fte_match_param, spec.match_criteria,
    outer_headers.smac_15_0);
    smac_v = MLX5_ADDR_OF(fte_match_param,
    spec.match_value,
    outer_headers.smac_47_16);
    ether_addr_copy(smac_v, vport.info.mac);
    }
    vport.ingress.allow_rule = mlx5_add_flow_rules(vport.ingress.acl, spec,
    &flow_act, core::ptr::null_mut(), 0);
    if (IS_ERR(vport.ingress.allow_rule)) {
    err = PTR_ERR(vport.ingress.allow_rule);
    esw_warn(esw.dev,
    "vport[%d] configure ingress allow rule, err(%d)\n",
    vport.vport, err);
    vport.ingress.allow_rule = core::ptr::null_mut();
    goto out;
    }
    if (!vst_check_cvlan && !vport.info.spoofchk)
    goto out;
    memset(&flow_act, 0, sizeof(flow_act));
    flow_act.action = MLX5_FLOW_CONTEXT_ACTION_DROP;
// Attach drop flow counter
    if (counter) {
    flow_act.action |= MLX5_FLOW_CONTEXT_ACTION_COUNT;
    drop_ctr_dst.type = MLX5_FLOW_DESTINATION_TYPE_COUNTER;
    drop_ctr_dst.counter = counter;
    dst = &drop_ctr_dst;
    dest_num++;
    }
    vport.ingress.legacy.drop_rule =
    mlx5_add_flow_rules(vport.ingress.acl, core::ptr::null_mut(),
    &flow_act, dst, dest_num);
    if (IS_ERR(vport.ingress.legacy.drop_rule)) {
    err = PTR_ERR(vport.ingress.legacy.drop_rule);
    esw_warn(esw.dev,
    "vport[%d] configure ingress drop rule, err(%d)\n",
    vport.vport, err);
    vport.ingress.legacy.drop_rule = core::ptr::null_mut();
    goto out;
    }
    kvfree(spec);
    return 0;
    out:
    if (err)
    esw_acl_ingress_lgcy_cleanup(esw, vport);
    kvfree(spec);
    return err;
    }
    void esw_acl_ingress_lgcy_cleanup(struct mlx5_eswitch *esw,
    struct mlx5_vport *vport)
    {
    if (IS_ERR_OR_NULL(vport.ingress.acl))
    goto clean_drop_counter;
    esw_debug(esw.dev, "Destroy vport[%d] E-Switch ingress ACL\n", vport.vport);
    esw_acl_ingress_lgcy_rules_destroy(vport);
    esw_acl_ingress_lgcy_groups_destroy(vport);
    esw_acl_ingress_table_destroy(vport);
    clean_drop_counter:
    if (vport.ingress.legacy.drop_counter) {
    mlx5_fc_destroy(esw.dev, vport.ingress.legacy.drop_counter);
    vport.ingress.legacy.drop_counter = core::ptr::null_mut();
    }
    }
