//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/eswitch_offloads_termtbl.c
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
// Copyright (c) 2019 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_termtbl_handle {
    pub termtbl_hlist: hlist_node,
    pub termtbl: *mut mlx5_flow_table,
    pub flow_act: mlx5_flow_act,
    pub dest: mlx5_flow_destination,
    pub rule: *mut mlx5_flow_handle,
    pub ref_count: c_int,
}

    static u32
    mlx5_eswitch_termtbl_hash(struct mlx5_flow_act *flow_act,
    struct mlx5_flow_destination *dest)
    {
    u32 hash;
    hash = jhash_1word(flow_act.action, 0);
    hash = jhash((const void *)&flow_act.vlan,
    sizeof(flow_act.vlan), hash);
    hash = jhash((const void *)&dest.vport.num,
    sizeof(dest.vport.num), hash);
    hash = jhash((const void *)&dest.vport.vhca_id,
    sizeof(dest.vport.num), hash);
    if (flow_act.pkt_reformat)
    hash = jhash(flow_act.pkt_reformat,
    sizeof(*flow_act.pkt_reformat),
    hash);
    return hash;
    }
    static int
    mlx5_eswitch_termtbl_cmp(struct mlx5_flow_act *flow_act1,
    struct mlx5_flow_destination *dest1,
    struct mlx5_flow_act *flow_act2,
    struct mlx5_flow_destination *dest2)
    {
    int ret;
    ret = flow_act1.action != flow_act2.action ||
    dest1.vport.num != dest2.vport.num ||
    dest1.vport.vhca_id != dest2.vport.vhca_id ||
    memcmp(&flow_act1.vlan, &flow_act2.vlan,
    sizeof(flow_act1.vlan));
    if (ret)
    return ret;
    if (flow_act1.pkt_reformat && flow_act2.pkt_reformat)
    return memcmp(flow_act1.pkt_reformat, flow_act2.pkt_reformat,
    sizeof(*flow_act1.pkt_reformat));
    return !(flow_act1.pkt_reformat == flow_act2.pkt_reformat);
    }
    static int
    mlx5_eswitch_termtbl_create(struct mlx5_core_dev *dev,
    struct mlx5_termtbl_handle *tt,
    struct mlx5_flow_act *flow_act)
    {
    let mut ft_attr: mlx5_flow_table_attr = {};
    struct mlx5_flow_namespace *root_ns;
    int err, err2;
    root_ns = mlx5_get_flow_namespace(dev, MLX5_FLOW_NAMESPACE_FDB);
    if (!root_ns) {
    esw_warn(dev, "Failed to get FDB flow namespace\n");
    return -EOPNOTSUPP;
    }
// As this is the terminating action then the termination table is the
// same prio as the slow path
//
    ft_attr.flags = MLX5_FLOW_TABLE_TERMINATION | MLX5_FLOW_TABLE_UNMANAGED |
    MLX5_FLOW_TABLE_TUNNEL_EN_REFORMAT;
    ft_attr.prio = FDB_TC_OFFLOAD;
    ft_attr.max_fte = 1;
    ft_attr.level = 1;
    ft_attr.autogroup.max_num_groups = 1;
    tt.termtbl = mlx5_create_auto_grouped_flow_table(root_ns, &ft_attr);
    if (IS_ERR(tt.termtbl)) {
    err = PTR_ERR(tt.termtbl);
    esw_warn(dev, "Failed to create termination table, err %pe\n", tt.termtbl);
    return err;
    }
    tt.rule = mlx5_add_flow_rules(tt.termtbl, core::ptr::null_mut(), flow_act,
    &tt.dest, 1);
    if (IS_ERR(tt.rule)) {
    err = PTR_ERR(tt.rule);
    esw_warn(dev, "Failed to create termination table rule, err %pe\n", tt.rule);
    goto add_flow_err;
    }
    return 0;
    add_flow_err:
    err2 = mlx5_destroy_flow_table(tt.termtbl);
    if (err2)
    esw_warn(dev, "Failed to destroy termination table, err %d\n", err2);
    return err;
    }
    static struct mlx5_termtbl_handle *
    mlx5_eswitch_termtbl_get_create(struct mlx5_eswitch *esw,
    struct mlx5_flow_act *flow_act,
    struct mlx5_flow_destination *dest,
    struct mlx5_esw_flow_attr *attr)
    {
    struct mlx5_termtbl_handle *tt;
    let mut found: bool = false;
    u32 hash_key;
    int err;
    mutex_lock(&esw.offloads.termtbl_mutex);
    hash_key = mlx5_eswitch_termtbl_hash(flow_act, dest);
    hash_for_each_possible(esw.offloads.termtbl_tbl, tt,
    termtbl_hlist, hash_key) {
    if (!mlx5_eswitch_termtbl_cmp(&tt.flow_act, &tt.dest,
    flow_act, dest)) {
    found = true;
    break;
    }
    }
    if (found)
    goto tt_add_ref;
    tt = kzalloc_obj(*tt);
    if (!tt) {
    err = -ENOMEM;
    goto tt_create_err;
    }
    tt.dest.type = MLX5_FLOW_DESTINATION_TYPE_VPORT;
    tt.dest.vport.num = dest.vport.num;
    tt.dest.vport.vhca_id = dest.vport.vhca_id;
    tt.dest.vport.flags = dest.vport.flags;
    memcpy(&tt.flow_act, flow_act, sizeof(*flow_act));
    err = mlx5_eswitch_termtbl_create(esw.dev, tt, flow_act);
    if (err)
    goto tt_create_err;
    hash_add(esw.offloads.termtbl_tbl, &tt.termtbl_hlist, hash_key);
    tt_add_ref:
    tt.ref_count++;
    mutex_unlock(&esw.offloads.termtbl_mutex);
    return tt;
    tt_create_err:
    kfree(tt);
    mutex_unlock(&esw.offloads.termtbl_mutex);
    return ERR_PTR(err);
    }
    void
    mlx5_eswitch_termtbl_put(struct mlx5_eswitch *esw,
    struct mlx5_termtbl_handle *tt)
    {
    mutex_lock(&esw.offloads.termtbl_mutex);
    if (--tt.ref_count == 0)
    hash_del(&tt.termtbl_hlist);
    mutex_unlock(&esw.offloads.termtbl_mutex);
    if (!tt.ref_count) {
    mlx5_del_flow_rules(tt.rule);
    mlx5_destroy_flow_table(tt.termtbl);
    kfree(tt);
    }
    }
    static void
    mlx5_eswitch_termtbl_actions_move(struct mlx5_flow_act *src,
    struct mlx5_flow_act *dst)
    {
    if (src.action & MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH) {
    src.action &= ~MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH;
    dst.action |= MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH;
    memcpy(&dst.vlan[0], &src.vlan[0], sizeof(src.vlan[0]));
    memset(&src.vlan[0], 0, sizeof(src.vlan[0]));
    if (src.action & MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH_2) {
    src.action &= ~MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH_2;
    dst.action |= MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH_2;
    memcpy(&dst.vlan[1], &src.vlan[1], sizeof(src.vlan[1]));
    memset(&src.vlan[1], 0, sizeof(src.vlan[1]));
    }
    }
    }
    static bool mlx5_eswitch_offload_is_uplink_port(const struct mlx5_eswitch *esw,
    const struct mlx5_flow_spec *spec)
    {
    u16 port_mask, port_value;
    if (MLX5_CAP_ESW_FLOWTABLE(esw.dev, flow_source))
    return spec.flow_context.flow_source ==
    MLX5_FLOW_CONTEXT_FLOW_SOURCE_UPLINK;
    port_mask = MLX5_GET(fte_match_param, spec.match_criteria,
    misc_parameters.source_port);
    port_value = MLX5_GET(fte_match_param, spec.match_value,
    misc_parameters.source_port);
    return (port_mask & port_value) == MLX5_VPORT_UPLINK;
    }
    bool
    mlx5_eswitch_termtbl_required(struct mlx5_eswitch *esw,
    struct mlx5_flow_attr *attr,
    struct mlx5_flow_act *flow_act,
    struct mlx5_flow_spec *spec)
    {
    struct mlx5_esw_flow_attr *esw_attr = attr.esw_attr;
    int i;
    if (!MLX5_CAP_ESW_FLOWTABLE_FDB(esw.dev, termination_table) ||
    !MLX5_CAP_ESW_FLOWTABLE_FDB(esw.dev, ignore_flow_level) ||
    mlx5e_tc_attr_flags_skip(attr.flags) ||
    (!mlx5_eswitch_offload_is_uplink_port(esw, spec) && !esw_attr.int_port))
    return false;
// push vlan on RX
    if (flow_act.action & MLX5_FLOW_CONTEXT_ACTION_VLAN_PUSH &&
    !(mlx5_fs_get_capabilities(esw.dev, MLX5_FLOW_NAMESPACE_FDB) &
    MLX5_FLOW_STEERING_CAP_VLAN_PUSH_ON_RX))
    return true;
// hairpin
    for (i = esw_attr.split_count; i < esw_attr.out_count; i++)
    if (!esw_attr.dest_int_port && esw_attr.dests[i].vport_valid &&
    esw_attr.dests[i].vport == MLX5_VPORT_UPLINK)
    return true;
    return false;
    }
    struct mlx5_flow_handle *
    mlx5_eswitch_add_termtbl_rule(struct mlx5_eswitch *esw,
    struct mlx5_flow_table *fdb,
    struct mlx5_flow_spec *spec,
    struct mlx5_esw_flow_attr *attr,
    struct mlx5_flow_act *flow_act,
    struct mlx5_flow_destination *dest,
    int num_dest)
    {
    let mut term_tbl_act: mlx5_flow_act = {};
    struct mlx5_flow_handle *rule = core::ptr::null_mut();
    let mut term_table_created: bool = false;
    let mut num_vport_dests: c_int = 0;
    int i, curr_dest;
    mlx5_eswitch_termtbl_actions_move(flow_act, &term_tbl_act);
    term_tbl_act.action |= MLX5_FLOW_CONTEXT_ACTION_FWD_DEST;
    for (i = 0; i < num_dest; i++) {
    struct mlx5_termtbl_handle *tt;
// only vport destinations can be terminated
    if (dest[i].type != MLX5_FLOW_DESTINATION_TYPE_VPORT)
    continue;
    if (attr.dests[num_vport_dests].flags & MLX5_ESW_DEST_ENCAP) {
    term_tbl_act.action |= MLX5_FLOW_CONTEXT_ACTION_PACKET_REFORMAT;
    term_tbl_act.pkt_reformat = attr.dests[num_vport_dests].pkt_reformat;
    } else {
    term_tbl_act.action &= ~MLX5_FLOW_CONTEXT_ACTION_PACKET_REFORMAT;
    term_tbl_act.pkt_reformat = core::ptr::null_mut();
    }
// get the terminating table for the action list
    tt = mlx5_eswitch_termtbl_get_create(esw, &term_tbl_act,
    &dest[i], attr);
    if (IS_ERR(tt)) {
    esw_warn(esw.dev, "Failed to get termination table, err %pe\n", tt);
    goto revert_changes;
    }
    attr.dests[num_vport_dests].termtbl = tt;
    num_vport_dests++;
// link the destination with the termination table
    dest[i].type = MLX5_FLOW_DESTINATION_TYPE_FLOW_TABLE;
    dest[i].ft = tt.termtbl;
    term_table_created = true;
    }
// at least one destination should reference a termination table
    if (!term_table_created)
    goto revert_changes;
// create the FTE
    flow_act.action &= ~MLX5_FLOW_CONTEXT_ACTION_PACKET_REFORMAT;
    flow_act.pkt_reformat = core::ptr::null_mut();
    flow_act.flags |= FLOW_ACT_IGNORE_FLOW_LEVEL;
    rule = mlx5_add_flow_rules(fdb, spec, flow_act, dest, num_dest);
    if (IS_ERR(rule))
    goto revert_changes;
    goto out;
    revert_changes:
// revert the changes that were made to the original flow_act
// and fall-back to the original rule actions
//
    mlx5_eswitch_termtbl_actions_move(&term_tbl_act, flow_act);
    for (curr_dest = 0; curr_dest < num_vport_dests; curr_dest++) {
    struct mlx5_termtbl_handle *tt = attr.dests[curr_dest].termtbl;
    attr.dests[curr_dest].termtbl = core::ptr::null_mut();
// search for the destination associated with the
// current term table
//
    for (i = 0; i < num_dest; i++) {
    if (dest[i].ft != tt.termtbl)
    continue;
    memset(&dest[i], 0, sizeof(dest[i]));
    dest[i].type = MLX5_FLOW_DESTINATION_TYPE_VPORT;
    dest[i].vport.num = tt.dest.vport.num;
    dest[i].vport.vhca_id = tt.dest.vport.vhca_id;
    mlx5_eswitch_termtbl_put(esw, tt);
    break;
    }
    }
    rule = mlx5_add_flow_rules(fdb, spec, flow_act, dest, num_dest);
    out:
    return rule;
    }
