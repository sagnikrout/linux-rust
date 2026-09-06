//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum1_acl_tcam.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_acl_tcam_region {
    pub cregion: mlxsw_sp_acl_ctcam_region,
    pub region: *mut mlxsw_sp_acl_tcam_region,
    struct {
    pub cchunk: mlxsw_sp_acl_ctcam_chunk,
    pub centry: mlxsw_sp_acl_ctcam_entry,
    pub rulei: *mut mlxsw_sp_acl_rule_info,
    pub catchall: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_acl_tcam_chunk {
    pub cchunk: mlxsw_sp_acl_ctcam_chunk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_acl_tcam_entry {
    pub centry: mlxsw_sp_acl_ctcam_entry,
}

    static int
    mlxsw_sp1_acl_ctcam_region_entry_insert(struct mlxsw_sp_acl_ctcam_region *cregion,
    struct mlxsw_sp_acl_ctcam_entry *centry,
    const char *mask)
    {
    return 0;
    }
    static void
    mlxsw_sp1_acl_ctcam_region_entry_remove(struct mlxsw_sp_acl_ctcam_region *cregion,
    struct mlxsw_sp_acl_ctcam_entry *centry)
    {
    }
    static const struct mlxsw_sp_acl_ctcam_region_ops
    mlxsw_sp1_acl_ctcam_region_ops = {
    .entry_insert = mlxsw_sp1_acl_ctcam_region_entry_insert,
    .entry_remove = mlxsw_sp1_acl_ctcam_region_entry_remove,
    };
    static int mlxsw_sp1_acl_tcam_init(struct mlxsw_sp *mlxsw_sp, void *priv,
    struct mlxsw_sp_acl_tcam *tcam)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_acl_tcam_fini(mlxsw_sp: *mut mlxsw_sp, priv: *mut c_void) {
    static void mlxsw_sp1_acl_tcam_fini(struct mlxsw_sp *mlxsw_sp, void *priv)
    {
    }
    static int
    mlxsw_sp1_acl_ctcam_region_catchall_add(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp1_acl_tcam_region *region)
    {
    struct mlxsw_sp_acl_rule_info *rulei;
    int err;
    mlxsw_sp_acl_ctcam_chunk_init(&region.cregion,
    &region.catchall.cchunk,
    MLXSW_SP_ACL_TCAM_CATCHALL_PRIO);
    rulei = mlxsw_sp_acl_rulei_create(mlxsw_sp.acl, core::ptr::null_mut());
    if (IS_ERR(rulei)) {
    err = PTR_ERR(rulei);
    goto err_rulei_create;
    }
    err = mlxsw_sp_acl_rulei_act_continue(rulei);
    if (WARN_ON(err))
    goto err_rulei_act_continue;
    err = mlxsw_sp_acl_rulei_commit(rulei);
    if (err)
    goto err_rulei_commit;
    err = mlxsw_sp_acl_ctcam_entry_add(mlxsw_sp, &region.cregion,
    &region.catchall.cchunk,
    &region.catchall.centry,
    rulei, false);
    if (err)
    goto err_entry_add;
    region.catchall.rulei = rulei;
    return 0;
    err_entry_add:
    err_rulei_commit:
    err_rulei_act_continue:
    mlxsw_sp_acl_rulei_destroy(mlxsw_sp, rulei);
    err_rulei_create:
    mlxsw_sp_acl_ctcam_chunk_fini(&region.catchall.cchunk);
    return err;
    }
    static void
    mlxsw_sp1_acl_ctcam_region_catchall_del(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp1_acl_tcam_region *region)
    {
    struct mlxsw_sp_acl_rule_info *rulei = region.catchall.rulei;
    mlxsw_sp_acl_ctcam_entry_del(mlxsw_sp, &region.cregion,
    &region.catchall.cchunk,
    &region.catchall.centry);
    mlxsw_sp_acl_rulei_destroy(mlxsw_sp, rulei);
    mlxsw_sp_acl_ctcam_chunk_fini(&region.catchall.cchunk);
    }
    static int
    mlxsw_sp1_acl_tcam_region_init(struct mlxsw_sp *mlxsw_sp, void *region_priv,
    void *tcam_priv,
    struct mlxsw_sp_acl_tcam_region *_region,
    void *hints_priv)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    int err;
    err = mlxsw_sp_acl_ctcam_region_init(mlxsw_sp, &region.cregion,
    _region,
    &mlxsw_sp1_acl_ctcam_region_ops);
    if (err)
    return err;
    err = mlxsw_sp1_acl_ctcam_region_catchall_add(mlxsw_sp, region);
    if (err)
    goto err_catchall_add;
    region.region = _region;
    return 0;
    err_catchall_add:
    mlxsw_sp_acl_ctcam_region_fini(&region.cregion);
    return err;
    }
    static void
    mlxsw_sp1_acl_tcam_region_fini(struct mlxsw_sp *mlxsw_sp, void *region_priv)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    mlxsw_sp1_acl_ctcam_region_catchall_del(mlxsw_sp, region);
    mlxsw_sp_acl_ctcam_region_fini(&region.cregion);
    }
    static int
    mlxsw_sp1_acl_tcam_region_associate(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_acl_tcam_region *region)
    {
    return 0;
    }
    static void mlxsw_sp1_acl_tcam_chunk_init(void *region_priv, void *chunk_priv,
    unsigned int priority)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    struct mlxsw_sp1_acl_tcam_chunk *chunk = chunk_priv;
    mlxsw_sp_acl_ctcam_chunk_init(&region.cregion, &chunk.cchunk,
    priority);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_acl_tcam_chunk_fini(chunk_priv: *mut c_void) {
    static void mlxsw_sp1_acl_tcam_chunk_fini(void *chunk_priv)
    {
    struct mlxsw_sp1_acl_tcam_chunk *chunk = chunk_priv;
    mlxsw_sp_acl_ctcam_chunk_fini(&chunk.cchunk);
    }
    static int mlxsw_sp1_acl_tcam_entry_add(struct mlxsw_sp *mlxsw_sp,
    void *region_priv, void *chunk_priv,
    void *entry_priv,
    struct mlxsw_sp_acl_rule_info *rulei)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    struct mlxsw_sp1_acl_tcam_chunk *chunk = chunk_priv;
    struct mlxsw_sp1_acl_tcam_entry *entry = entry_priv;
    return mlxsw_sp_acl_ctcam_entry_add(mlxsw_sp, &region.cregion,
    &chunk.cchunk, &entry.centry,
    rulei, false);
    }
    static void mlxsw_sp1_acl_tcam_entry_del(struct mlxsw_sp *mlxsw_sp,
    void *region_priv, void *chunk_priv,
    void *entry_priv)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    struct mlxsw_sp1_acl_tcam_chunk *chunk = chunk_priv;
    struct mlxsw_sp1_acl_tcam_entry *entry = entry_priv;
    mlxsw_sp_acl_ctcam_entry_del(mlxsw_sp, &region.cregion,
    &chunk.cchunk, &entry.centry);
    }
    static int
    mlxsw_sp1_acl_tcam_entry_action_replace(struct mlxsw_sp *mlxsw_sp,
    void *region_priv, void *entry_priv,
    struct mlxsw_sp_acl_rule_info *rulei)
    {
    return -EOPNOTSUPP;
    }
    static int
    mlxsw_sp1_acl_tcam_region_entry_activity_get(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_acl_tcam_region *_region,
    unsigned int offset,
    bool *activity)
    {
    char ptce2_pl[MLXSW_REG_PTCE2_LEN];
    int err;
    mlxsw_reg_ptce2_pack(ptce2_pl, true, MLXSW_REG_PTCE2_OP_QUERY_CLEAR_ON_READ,
    _region.tcam_region_info, offset, 0);
    err = mlxsw_reg_query(mlxsw_sp.core, MLXSW_REG(ptce2), ptce2_pl);
    if (err)
    return err;
// activity = mlxsw_reg_ptce2_a_get(ptce2_pl);
    return 0;
    }
    static int
    mlxsw_sp1_acl_tcam_entry_activity_get(struct mlxsw_sp *mlxsw_sp,
    void *region_priv, void *entry_priv,
    bool *activity)
    {
    struct mlxsw_sp1_acl_tcam_region *region = region_priv;
    struct mlxsw_sp1_acl_tcam_entry *entry = entry_priv;
    unsigned int offset;
    offset = mlxsw_sp_acl_ctcam_entry_offset(&entry.centry);
    return mlxsw_sp1_acl_tcam_region_entry_activity_get(mlxsw_sp,
    region.region,
    offset, activity);
    }
    const struct mlxsw_sp_acl_tcam_ops mlxsw_sp1_acl_tcam_ops = {
    .key_type		= MLXSW_REG_PTAR_KEY_TYPE_FLEX,
    .priv_size		= 0,
    .init			= mlxsw_sp1_acl_tcam_init,
    .fini			= mlxsw_sp1_acl_tcam_fini,
    .region_priv_size	= sizeof(struct mlxsw_sp1_acl_tcam_region),
    .region_init		= mlxsw_sp1_acl_tcam_region_init,
    .region_fini		= mlxsw_sp1_acl_tcam_region_fini,
    .region_associate	= mlxsw_sp1_acl_tcam_region_associate,
    .chunk_priv_size	= sizeof(struct mlxsw_sp1_acl_tcam_chunk),
    .chunk_init		= mlxsw_sp1_acl_tcam_chunk_init,
    .chunk_fini		= mlxsw_sp1_acl_tcam_chunk_fini,
    .entry_priv_size	= sizeof(struct mlxsw_sp1_acl_tcam_entry),
    .entry_add		= mlxsw_sp1_acl_tcam_entry_add,
    .entry_del		= mlxsw_sp1_acl_tcam_entry_del,
    .entry_action_replace	= mlxsw_sp1_acl_tcam_entry_action_replace,
    .entry_activity_get	= mlxsw_sp1_acl_tcam_entry_activity_get,
    };
