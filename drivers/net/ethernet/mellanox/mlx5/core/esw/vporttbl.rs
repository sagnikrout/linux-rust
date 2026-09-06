//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/vporttbl.c
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
// Copyright (c) 2021 Mellanox Technologies.

// This struct is used as a key to the hash table and we need it to be packed
// so hash result is consistent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport_key {
    pub chain: u32,
    pub prio: u16,
    pub vport: u16,
    pub vhca_id: u16,
    pub vport_ns: *mut esw_vport_tbl_namespace,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport_table {
    pub hlist: hlist_node,
    pub fdb: *mut mlx5_flow_table,
    pub num_rules: u32,
    pub key: mlx5_vport_key,
}

    static void
    esw_vport_tbl_init(struct mlx5_eswitch *esw, struct esw_vport_tbl_namespace *ns)
    {
    if (esw.offloads.encap != DEVLINK_ESWITCH_ENCAP_MODE_NONE)
    ns.flags |= (MLX5_FLOW_TABLE_TUNNEL_EN_REFORMAT |
    MLX5_FLOW_TABLE_TUNNEL_EN_DECAP);
    }
    static struct mlx5_flow_table *
    esw_vport_tbl_create(struct mlx5_eswitch *esw, struct mlx5_flow_namespace *ns,
    const struct esw_vport_tbl_namespace *vport_ns)
    {
    let mut ft_attr: mlx5_flow_table_attr = {};
    struct mlx5_flow_table *fdb;
    if (vport_ns.max_num_groups)
    ft_attr.autogroup.max_num_groups = vport_ns.max_num_groups;
    else
    ft_attr.autogroup.max_num_groups = esw.params.large_group_num;
    ft_attr.max_fte = vport_ns.max_fte;
    ft_attr.prio = FDB_PER_VPORT;
    ft_attr.flags = vport_ns.flags;
    fdb = mlx5_create_auto_grouped_flow_table(ns, &ft_attr);
    if (IS_ERR(fdb)) {
    esw_warn(esw.dev, "Failed to create per vport FDB Table err %pe\n",
    fdb);
    }
    return fdb;
    }
    static u32 flow_attr_to_vport_key(struct mlx5_eswitch *esw,
    struct mlx5_vport_tbl_attr *attr,
    struct mlx5_vport_key *key)
    {
    key.vport = attr.vport;
    key.chain = attr.chain;
    key.prio = attr.prio;
    key.vhca_id = MLX5_CAP_GEN(esw.dev, vhca_id);
    key.vport_ns  = attr.vport_ns;
    return jhash(key, sizeof(*key), 0);
    }
// caller must hold vports.lock
    static struct mlx5_vport_table *
    esw_vport_tbl_lookup(struct mlx5_eswitch *esw, struct mlx5_vport_key *skey, u32 key)
    {
    struct mlx5_vport_table *e;
    hash_for_each_possible(esw.fdb_table.offloads.vports.table, e, hlist, key)
    if (!memcmp(&e.key, skey, sizeof(*skey)))
    return e;
    return core::ptr::null_mut();
    }
    struct mlx5_flow_table *
    mlx5_esw_vporttbl_get(struct mlx5_eswitch *esw, struct mlx5_vport_tbl_attr *attr)
    {
    struct mlx5_core_dev *dev = esw.dev;
    struct mlx5_flow_namespace *ns;
    struct mlx5_flow_table *fdb;
    struct mlx5_vport_table *e;
    struct mlx5_vport_key skey;
    u32 hkey;
    mutex_lock(&esw.fdb_table.offloads.vports.lock);
    esw_vport_tbl_init(esw, attr.vport_ns);
    hkey = flow_attr_to_vport_key(esw, attr, &skey);
    e = esw_vport_tbl_lookup(esw, &skey, hkey);
    if (e) {
    e.num_rules++;
    goto out;
    }
    e = kzalloc_obj(*e);
    if (!e) {
    fdb = ERR_PTR(-ENOMEM);
    goto err_alloc;
    }
    ns = mlx5_get_flow_namespace(dev, MLX5_FLOW_NAMESPACE_FDB);
    if (!ns) {
    esw_warn(dev, "Failed to get FDB namespace\n");
    fdb = ERR_PTR(-ENOENT);
    goto err_ns;
    }
    fdb = esw_vport_tbl_create(esw, ns, attr.vport_ns);
    if (IS_ERR(fdb))
    goto err_ns;
    e.fdb = fdb;
    e.num_rules = 1;
    e.key = skey;
    hash_add(esw.fdb_table.offloads.vports.table, &e.hlist, hkey);
    out:
    mutex_unlock(&esw.fdb_table.offloads.vports.lock);
    return e.fdb;
    err_ns:
    kfree(e);
    err_alloc:
    mutex_unlock(&esw.fdb_table.offloads.vports.lock);
    return fdb;
    }
    void
    mlx5_esw_vporttbl_put(struct mlx5_eswitch *esw, struct mlx5_vport_tbl_attr *attr)
    {
    struct mlx5_vport_table *e;
    struct mlx5_vport_key key;
    u32 hkey;
    mutex_lock(&esw.fdb_table.offloads.vports.lock);
    esw_vport_tbl_init(esw, attr.vport_ns);
    hkey = flow_attr_to_vport_key(esw, attr, &key);
    e = esw_vport_tbl_lookup(esw, &key, hkey);
    if (!e || --e.num_rules)
    goto out;
    hash_del(&e.hlist);
    mlx5_destroy_flow_table(e.fdb);
    kfree(e);
    out:
    mutex_unlock(&esw.fdb_table.offloads.vports.lock);
    }
