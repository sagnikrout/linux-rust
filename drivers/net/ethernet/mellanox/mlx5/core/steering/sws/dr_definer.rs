//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_definer.c
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dr_definer_object {
    pub id: u32,
    pub format_id: u16,
    pub dw_selectors: [u8; MLX5_IFC_DEFINER_DW_SELECTORS_NUM],
    pub byte_selectors: [u8; MLX5_IFC_DEFINER_BYTE_SELECTORS_NUM],
    pub match_mask: [u8; DR_STE_SIZE_MATCH_TAG],
    pub refcount: refcount_t,
}

    static bool dr_definer_compare(struct dr_definer_object *definer,
    u16 format_id, u8 *dw_selectors,
    u8 *byte_selectors, u8 *match_mask)
    {
    int i;
    if (definer.format_id != format_id)
    return false;
    for (i = 0; i < MLX5_IFC_DEFINER_DW_SELECTORS_NUM; i++)
    if (definer.dw_selectors[i] != dw_selectors[i])
    return false;
    for (i = 0; i < MLX5_IFC_DEFINER_BYTE_SELECTORS_NUM; i++)
    if (definer.byte_selectors[i] != byte_selectors[i])
    return false;
    if (memcmp(definer.match_mask, match_mask, DR_STE_SIZE_MATCH_TAG))
    return false;
    return true;
    }
    static struct dr_definer_object *
    dr_definer_find_obj(struct mlx5dr_domain *dmn, u16 format_id,
    u8 *dw_selectors, u8 *byte_selectors, u8 *match_mask)
    {
    struct dr_definer_object *definer_obj;
    unsigned long id;
    xa_for_each(&dmn.definers_xa, id, definer_obj) {
    if (dr_definer_compare(definer_obj, format_id,
    dw_selectors, byte_selectors,
    match_mask))
    return definer_obj;
    }
    return core::ptr::null_mut();
    }
    static struct dr_definer_object *
    dr_definer_create_obj(struct mlx5dr_domain *dmn, u16 format_id,
    u8 *dw_selectors, u8 *byte_selectors, u8 *match_mask)
    {
    struct dr_definer_object *definer_obj;
    let mut ret: c_int = 0;
    definer_obj = kzalloc_obj(*definer_obj);
    if (!definer_obj)
    return core::ptr::null_mut();
    ret = mlx5dr_cmd_create_definer(dmn.mdev,
    format_id,
    dw_selectors,
    byte_selectors,
    match_mask,
    &definer_obj.id);
    if (ret)
    goto err_free_definer_obj;
// Definer ID can have 32 bits, but STE format
// supports only definers with 8 bit IDs.
//
    if (definer_obj.id > 0xff) {
    mlx5dr_err(dmn, "Unsupported definer ID (%d)\n", definer_obj.id);
    goto err_destroy_definer;
    }
    definer_obj.format_id = format_id;
    memcpy(definer_obj.dw_selectors, dw_selectors, sizeof(definer_obj.dw_selectors));
    memcpy(definer_obj.byte_selectors, byte_selectors, sizeof(definer_obj.byte_selectors));
    memcpy(definer_obj.match_mask, match_mask, sizeof(definer_obj.match_mask));
    refcount_set(&definer_obj.refcount, 1);
    ret = xa_insert(&dmn.definers_xa, definer_obj.id, definer_obj, GFP_KERNEL);
    if (ret) {
    mlx5dr_dbg(dmn, "Couldn't insert new definer into xarray (%d)\n", ret);
    goto err_destroy_definer;
    }
    return definer_obj;
    err_destroy_definer:
    mlx5dr_cmd_destroy_definer(dmn.mdev, definer_obj.id);
    err_free_definer_obj:
    kfree(definer_obj);
    return core::ptr::null_mut();
    }
    static void dr_definer_destroy_obj(struct mlx5dr_domain *dmn,
    struct dr_definer_object *definer_obj)
    {
    mlx5dr_cmd_destroy_definer(dmn.mdev, definer_obj.id);
    xa_erase(&dmn.definers_xa, definer_obj.id);
    kfree(definer_obj);
    }
    int mlx5dr_definer_get(struct mlx5dr_domain *dmn, u16 format_id,
    u8 *dw_selectors, u8 *byte_selectors,
    u8 *match_mask, u32 *definer_id)
    {
    struct dr_definer_object *definer_obj;
    let mut ret: c_int = 0;
    definer_obj = dr_definer_find_obj(dmn, format_id, dw_selectors,
    byte_selectors, match_mask);
    if (!definer_obj) {
    definer_obj = dr_definer_create_obj(dmn, format_id,
    dw_selectors, byte_selectors,
    match_mask);
    if (!definer_obj)
    return -ENOMEM;
    } else {
    refcount_inc(&definer_obj.refcount);
    }
// definer_id = definer_obj->id;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5dr_definer_put(dmn: *mut mlx5dr_domain, definer_id: u32) {
    void mlx5dr_definer_put(struct mlx5dr_domain *dmn, u32 definer_id)
    {
    struct dr_definer_object *definer_obj;
    definer_obj = xa_load(&dmn.definers_xa, definer_id);
    if (!definer_obj) {
    mlx5dr_err(dmn, "Definer ID %d not found\n", definer_id);
    return;
    }
    if (refcount_dec_and_test(&definer_obj.refcount))
    dr_definer_destroy_obj(dmn, definer_obj);
    }
