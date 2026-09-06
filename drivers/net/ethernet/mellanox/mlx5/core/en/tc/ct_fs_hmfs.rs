//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/ct_fs_hmfs.c
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
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_hmfs_matcher {
    pub hws_bwc_matcher: *mut mlx5hws_bwc_matcher,
    pub ref: refcount_t,
}

// We need {ipv4, ipv6} x {tcp, udp, gre}  matchers.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_hmfs {
    pub ct_tbl: *mut mlx5hws_table,
    pub ct_nat_tbl: *mut mlx5hws_table,
    pub ct_nat: *mut mlx5_flow_table,
    pub fwd_action: *mut mlx5hws_action,
    pub last_action: *mut mlx5hws_action,
    pub ctx: *mut mlx5hws_context,
    pub /: *mut *mut mutex lock; / Guards matchers,
    pub matchers: [mlx5_ct_fs_hmfs_matcher; NUM_MATCHERS],
    pub matchers_nat: [mlx5_ct_fs_hmfs_matcher; NUM_MATCHERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_hmfs_rule {
    pub fs_rule: mlx5_ct_fs_rule,
    pub hws_bwc_rule: *mut mlx5hws_bwc_rule,
    pub hmfs_matcher: *mut mlx5_ct_fs_hmfs_matcher,
    pub counter: *mut mlx5_fc,
}

#[no_mangle]
unsafe extern "C" fn get_matcher_idx(ipv4: bool, tcp: bool, gre: bool) -> u32 {
    static u32 get_matcher_idx(bool ipv4, bool tcp, bool gre)
    {
    return ipv4 * 3 + tcp * 2 + gre;
    }
    static int mlx5_ct_fs_hmfs_init(struct mlx5_ct_fs *fs, struct mlx5_flow_table *ct,
    struct mlx5_flow_table *ct_nat, struct mlx5_flow_table *post_ct)
    {
    let mut flags: u32 = MLX5HWS_ACTION_FLAG_HWS_FDB | MLX5HWS_ACTION_FLAG_SHARED;
    struct mlx5hws_table *ct_tbl, *ct_nat_tbl, *post_ct_tbl;
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    ct_tbl = ct.fs_hws_table.hws_table;
    ct_nat_tbl = ct_nat.fs_hws_table.hws_table;
    post_ct_tbl = post_ct.fs_hws_table.hws_table;
    fs_hmfs.ct_nat = ct_nat;
    if (!ct_tbl || !ct_nat_tbl || !post_ct_tbl) {
    netdev_warn(fs.netdev, "ct_fs_hmfs: failed to init, missing backing hws tables");
    return -EOPNOTSUPP;
    }
    netdev_dbg(fs.netdev, "using hmfs steering");
    fs_hmfs.ct_tbl = ct_tbl;
    fs_hmfs.ct_nat_tbl = ct_nat_tbl;
    fs_hmfs.ctx = ct_tbl.ctx;
    mutex_init(&fs_hmfs.lock);
    fs_hmfs.fwd_action = mlx5hws_action_create_dest_table(ct_tbl.ctx, post_ct_tbl, flags);
    if (!fs_hmfs.fwd_action) {
    netdev_warn(fs.netdev, "ct_fs_hmfs: failed to create fwd action\n");
    return -EINVAL;
    }
    fs_hmfs.last_action = mlx5hws_action_create_last(ct_tbl.ctx, flags);
    if (!fs_hmfs.last_action) {
    netdev_warn(fs.netdev, "ct_fs_hmfs: failed to create last action\n");
    mlx5hws_action_destroy(fs_hmfs.fwd_action);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_ct_fs_hmfs_destroy(fs: *mut mlx5_ct_fs) {
    static void mlx5_ct_fs_hmfs_destroy(struct mlx5_ct_fs *fs)
    {
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    mlx5hws_action_destroy(fs_hmfs.last_action);
    mlx5hws_action_destroy(fs_hmfs.fwd_action);
    }
    static struct mlx5hws_bwc_matcher *
    mlx5_ct_fs_hmfs_matcher_create(struct mlx5_ct_fs *fs, struct mlx5hws_table *tbl,
    struct mlx5_flow_spec *spec, bool ipv4, bool tcp, bool gre)
    {
    let mut match_criteria_enable: u8 = MLX5_MATCH_MISC_PARAMETERS_2 | MLX5_MATCH_OUTER_HEADERS;
    struct mlx5hws_match_parameters mask = {
    .match_buf = spec.match_criteria,
    .match_sz = sizeof(spec.match_criteria),
    };
    u32 priority = get_matcher_idx(ipv4, tcp, gre);  /* Static priority based on params. */
    struct mlx5hws_bwc_matcher *hws_bwc_matcher;
    hws_bwc_matcher = mlx5hws_bwc_matcher_create(tbl, priority, match_criteria_enable, &mask);
    if (!hws_bwc_matcher)
    return ERR_PTR(-EINVAL);
    return hws_bwc_matcher;
    }
    static struct mlx5_ct_fs_hmfs_matcher *
    mlx5_ct_fs_hmfs_matcher_get(struct mlx5_ct_fs *fs, struct mlx5_flow_spec *spec,
    bool nat, bool ipv4, bool tcp, bool gre)
    {
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    let mut matcher_idx: u32 = get_matcher_idx(ipv4, tcp, gre);
    struct mlx5_ct_fs_hmfs_matcher *hmfs_matcher;
    struct mlx5hws_bwc_matcher *hws_bwc_matcher;
    struct mlx5hws_table *tbl;
    hmfs_matcher = nat ?
    (fs_hmfs.matchers_nat + matcher_idx) :
    (fs_hmfs.matchers + matcher_idx);
    if (refcount_inc_not_zero(&hmfs_matcher.ref))
    return hmfs_matcher;
    mutex_lock(&fs_hmfs.lock);
// Retry with lock, as the matcher might be already created by another cpu.
    if (refcount_inc_not_zero(&hmfs_matcher.ref))
    goto out_unlock;
    tbl = nat ? fs_hmfs.ct_nat_tbl : fs_hmfs.ct_tbl;
    hws_bwc_matcher = mlx5_ct_fs_hmfs_matcher_create(fs, tbl, spec, ipv4, tcp, gre);
    if (IS_ERR(hws_bwc_matcher)) {
    netdev_warn(fs.netdev,
    "ct_fs_hmfs: failed to create bwc matcher (nat %d, ipv4 %d, tcp %d, gre %d), err: %pe\n",
    nat, ipv4, tcp, gre, hws_bwc_matcher);
    hmfs_matcher = ERR_CAST(hws_bwc_matcher);
    goto out_unlock;
    }
    hmfs_matcher.hws_bwc_matcher = hws_bwc_matcher;
    refcount_set(&hmfs_matcher.ref, 1);
    out_unlock:
    mutex_unlock(&fs_hmfs.lock);
    return hmfs_matcher;
    }
    static void
    mlx5_ct_fs_hmfs_matcher_put(struct mlx5_ct_fs *fs, struct mlx5_ct_fs_hmfs_matcher *hmfs_matcher)
    {
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    if (!refcount_dec_and_mutex_lock(&hmfs_matcher.ref, &fs_hmfs.lock))
    return;
    mlx5hws_bwc_matcher_destroy(hmfs_matcher.hws_bwc_matcher);
    mutex_unlock(&fs_hmfs.lock);
    }
pub const NUM_CT_HMFS_RULES: c_int = 4;
    static void mlx5_ct_fs_hmfs_fill_rule_actions(struct mlx5_ct_fs_hmfs *fs_hmfs,
    struct mlx5_flow_attr *attr,
    struct mlx5hws_rule_action *rule_actions)
    {
    struct mlx5_fs_hws_action *mh_action = &attr.modify_hdr.fs_hws_action;
    memset(rule_actions, 0, NUM_CT_HMFS_RULES * sizeof(*rule_actions));
    rule_actions[0].action = mlx5_fc_get_hws_action(fs_hmfs.ctx, attr.counter);
    rule_actions[0].counter.offset =
    attr.counter.id - attr.counter.bulk.base_id;
// Modify header is special, it may require extra arguments outside the action itself.
    if (mh_action.mh_data) {
    rule_actions[1].modify_header.offset = mh_action.mh_data.offset;
    rule_actions[1].modify_header.data = mh_action.mh_data.data;
    }
    rule_actions[1].action = mh_action.hws_action;
    rule_actions[2].action = fs_hmfs.fwd_action;
    rule_actions[3].action = fs_hmfs.last_action;
    }
    static struct mlx5_ct_fs_rule *
    mlx5_ct_fs_hmfs_ct_rule_add(struct mlx5_ct_fs *fs, struct mlx5_flow_spec *spec,
    struct mlx5_flow_attr *attr, struct flow_rule *flow_rule)
    {
    struct mlx5hws_rule_action rule_actions[NUM_CT_HMFS_RULES];
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    struct mlx5hws_match_parameters match_params = {
    .match_buf = spec.match_value,
    .match_sz = ARRAY_SIZE(spec.match_value),
    };
    struct mlx5_ct_fs_hmfs_matcher *hmfs_matcher;
    struct mlx5_ct_fs_hmfs_rule *hmfs_rule;
    bool nat, tcp, ipv4, gre;
    int err;
    if (!mlx5e_tc_ct_is_valid_flow_rule(fs.netdev, flow_rule))
    return ERR_PTR(-EOPNOTSUPP);
    hmfs_rule = kzalloc_obj(*hmfs_rule);
    if (!hmfs_rule)
    return ERR_PTR(-ENOMEM);
    nat = (attr.ft == fs_hmfs.ct_nat);
    ipv4 = mlx5e_tc_get_ip_version(spec, true) == 4;
    tcp = MLX5_GET(fte_match_param, spec.match_value,
    outer_headers.ip_protocol) == IPPROTO_TCP;
    gre = MLX5_GET(fte_match_param, spec.match_value,
    outer_headers.ip_protocol) == IPPROTO_GRE;
    hmfs_matcher = mlx5_ct_fs_hmfs_matcher_get(fs, spec, nat, ipv4, tcp, gre);
    if (IS_ERR(hmfs_matcher)) {
    err = PTR_ERR(hmfs_matcher);
    goto err_free_rule;
    }
    hmfs_rule.hmfs_matcher = hmfs_matcher;
    mlx5_ct_fs_hmfs_fill_rule_actions(fs_hmfs, attr, rule_actions);
    hmfs_rule.counter = attr.counter;
    hmfs_rule.hws_bwc_rule =
    mlx5hws_bwc_rule_create(hmfs_matcher.hws_bwc_matcher, &match_params,
    spec.flow_context.flow_source, rule_actions);
    if (!hmfs_rule.hws_bwc_rule) {
    err = -EINVAL;
    goto err_put_matcher;
    }
    return &hmfs_rule.fs_rule;
    err_put_matcher:
    mlx5_fc_put_hws_action(hmfs_rule.counter);
    mlx5_ct_fs_hmfs_matcher_put(fs, hmfs_matcher);
    err_free_rule:
    kfree(hmfs_rule);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_ct_fs_hmfs_ct_rule_del(fs: *mut mlx5_ct_fs, fs_rule: *mut mlx5_ct_fs_rule) {
    static void mlx5_ct_fs_hmfs_ct_rule_del(struct mlx5_ct_fs *fs, struct mlx5_ct_fs_rule *fs_rule)
    {
    struct mlx5_ct_fs_hmfs_rule *hmfs_rule = container_of(fs_rule,
    struct mlx5_ct_fs_hmfs_rule,
    fs_rule);
    mlx5hws_bwc_rule_destroy(hmfs_rule.hws_bwc_rule);
    mlx5_fc_put_hws_action(hmfs_rule.counter);
    mlx5_ct_fs_hmfs_matcher_put(fs, hmfs_rule.hmfs_matcher);
    kfree(hmfs_rule);
    }
    static int mlx5_ct_fs_hmfs_ct_rule_update(struct mlx5_ct_fs *fs, struct mlx5_ct_fs_rule *fs_rule,
    struct mlx5_flow_spec *spec, struct mlx5_flow_attr *attr)
    {
    struct mlx5_ct_fs_hmfs_rule *hmfs_rule = container_of(fs_rule,
    struct mlx5_ct_fs_hmfs_rule,
    fs_rule);
    struct mlx5hws_rule_action rule_actions[NUM_CT_HMFS_RULES];
    struct mlx5_ct_fs_hmfs *fs_hmfs = mlx5_ct_fs_priv(fs);
    int err;
    mlx5_ct_fs_hmfs_fill_rule_actions(fs_hmfs, attr, rule_actions);
    err = mlx5hws_bwc_rule_action_update(hmfs_rule.hws_bwc_rule, rule_actions);
    if (err) {
    mlx5_fc_put_hws_action(attr.counter);
    return err;
    }
    mlx5_fc_put_hws_action(hmfs_rule.counter);
    hmfs_rule.counter = attr.counter;
    return 0;
    }
    static struct mlx5_ct_fs_ops hmfs_ops = {
    .ct_rule_add = mlx5_ct_fs_hmfs_ct_rule_add,
    .ct_rule_del = mlx5_ct_fs_hmfs_ct_rule_del,
    .ct_rule_update = mlx5_ct_fs_hmfs_ct_rule_update,
    .init = mlx5_ct_fs_hmfs_init,
    .destroy = mlx5_ct_fs_hmfs_destroy,
    .priv_size = sizeof(struct mlx5_ct_fs_hmfs),
    };
    struct mlx5_ct_fs_ops *mlx5_ct_fs_hmfs_ops_get(void)
    {
    return &hmfs_ops;
    }
