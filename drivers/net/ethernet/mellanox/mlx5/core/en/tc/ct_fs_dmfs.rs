//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc/ct_fs_dmfs.c
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES.

    netdev_dbg(fs.netdev, "ct_fs_dmfs debug: " fmt "\n", ##args)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_fs_dmfs_rule {
    pub fs_rule: mlx5_ct_fs_rule,
    pub rule: *mut mlx5_flow_handle,
    pub attr: *mut mlx5_flow_attr,
}

    static int
    mlx5_ct_fs_dmfs_init(struct mlx5_ct_fs *fs, struct mlx5_flow_table *ct,
    struct mlx5_flow_table *ct_nat, struct mlx5_flow_table *post_ct)
    {
    return 0;
    }
    static void
    mlx5_ct_fs_dmfs_destroy(struct mlx5_ct_fs *fs)
    {
    }
    static struct mlx5_ct_fs_rule *
    mlx5_ct_fs_dmfs_ct_rule_add(struct mlx5_ct_fs *fs, struct mlx5_flow_spec *spec,
    struct mlx5_flow_attr *attr, struct flow_rule *flow_rule)
    {
    struct mlx5e_priv *priv = netdev_priv(fs.netdev);
    struct mlx5_ct_fs_dmfs_rule *dmfs_rule;
    int err;
    dmfs_rule = kzalloc_obj(*dmfs_rule);
    if (!dmfs_rule)
    return ERR_PTR(-ENOMEM);
    dmfs_rule.rule = mlx5_tc_rule_insert(priv, spec, attr);
    if (IS_ERR(dmfs_rule.rule)) {
    err = PTR_ERR(dmfs_rule.rule);
    ct_dbg("Failed to add ct entry fs rule");
    goto err_insert;
    }
    dmfs_rule.attr = attr;
    return &dmfs_rule.fs_rule;
    err_insert:
    kfree(dmfs_rule);
    return ERR_PTR(err);
    }
    static void
    mlx5_ct_fs_dmfs_ct_rule_del(struct mlx5_ct_fs *fs, struct mlx5_ct_fs_rule *fs_rule)
    {
    struct mlx5_ct_fs_dmfs_rule *dmfs_rule = container_of(fs_rule,
    struct mlx5_ct_fs_dmfs_rule,
    fs_rule);
    mlx5_tc_rule_delete(netdev_priv(fs.netdev), dmfs_rule.rule, dmfs_rule.attr);
    kfree(dmfs_rule);
    }
    static int mlx5_ct_fs_dmfs_ct_rule_update(struct mlx5_ct_fs *fs, struct mlx5_ct_fs_rule *fs_rule,
    struct mlx5_flow_spec *spec, struct mlx5_flow_attr *attr)
    {
    struct mlx5_ct_fs_dmfs_rule *dmfs_rule = container_of(fs_rule,
    struct mlx5_ct_fs_dmfs_rule,
    fs_rule);
    struct mlx5e_priv *priv = netdev_priv(fs.netdev);
    struct mlx5_flow_handle *rule;
    rule = mlx5_tc_rule_insert(priv, spec, attr);
    if (IS_ERR(rule))
    return PTR_ERR(rule);
    mlx5_tc_rule_delete(priv, dmfs_rule.rule, dmfs_rule.attr);
    dmfs_rule.rule = rule;
    dmfs_rule.attr = attr;
    return 0;
    }
    static struct mlx5_ct_fs_ops dmfs_ops = {
    .ct_rule_add = mlx5_ct_fs_dmfs_ct_rule_add,
    .ct_rule_del = mlx5_ct_fs_dmfs_ct_rule_del,
    .ct_rule_update = mlx5_ct_fs_dmfs_ct_rule_update,
    .init = mlx5_ct_fs_dmfs_init,
    .destroy = mlx5_ct_fs_dmfs_destroy,
    };
    struct mlx5_ct_fs_ops *mlx5_ct_fs_dmfs_ops_get(void)
    {
    return &dmfs_ops;
    }
