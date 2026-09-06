//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_arg.c
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
// Copyright (c) 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const DR_ICM_MODIFY_HDR_GRANULARITY_4K: c_int = 12;
// modify-header arg pool
    enum dr_arg_chunk_size {
    DR_ARG_CHUNK_SIZE_1,
    DR_ARG_CHUNK_SIZE_MIN = DR_ARG_CHUNK_SIZE_1, /* keep updated when changing */
    DR_ARG_CHUNK_SIZE_2,
    DR_ARG_CHUNK_SIZE_3,
    DR_ARG_CHUNK_SIZE_4,
    DR_ARG_CHUNK_SIZE_MAX,
    };
// argument pool area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dr_arg_pool {
    pub log_chunk_size: enum dr_arg_chunk_size,
    pub dmn: *mut mlx5dr_domain,
    pub free_list: list_head,
    pub /: *mut *mut mutex mutex; / protect arg pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_arg_mgr {
    pub dmn: *mut mlx5dr_domain,
    pub pools: [*mut dr_arg_pool; DR_ARG_CHUNK_SIZE_MAX],
}

#[no_mangle]
unsafe extern "C" fn dr_arg_pool_alloc_objs(pool: *mut dr_arg_pool) -> c_int {
    static int dr_arg_pool_alloc_objs(struct dr_arg_pool *pool)
    {
    struct mlx5dr_arg_obj *arg_obj, *tmp_arg;
    struct list_head cur_list;
    u16 object_range;
    int num_of_objects;
    let mut obj_id: u32 = 0;
    int i, ret;
    INIT_LIST_HEAD(&cur_list);
    object_range =
    pool.dmn.info.caps.log_header_modify_argument_granularity;
    object_range =
    max_t(u32, pool.dmn.info.caps.log_header_modify_argument_granularity,
    DR_ICM_MODIFY_HDR_GRANULARITY_4K);
    object_range =
    min_t(u32, pool.dmn.info.caps.log_header_modify_argument_max_alloc,
    object_range);
    if (pool.log_chunk_size > object_range) {
    mlx5dr_err(pool.dmn, "Required chunk size (%d) is not supported\n",
    pool.log_chunk_size);
    return -ENOMEM;
    }
    num_of_objects = (1 << (object_range - pool.log_chunk_size));
// Only one devx object per range
    ret = mlx5dr_cmd_create_modify_header_arg(pool.dmn.mdev,
    object_range,
    pool.dmn.pdn,
    &obj_id);
    if (ret) {
    mlx5dr_err(pool.dmn, "failed allocating object with range: %d:\n",
    object_range);
    return -EAGAIN;
    }
    for (i = 0; i < num_of_objects; i++) {
    arg_obj = kzalloc_obj(*arg_obj);
    if (!arg_obj) {
    ret = -ENOMEM;
    goto clean_arg_obj;
    }
    arg_obj.log_chunk_size = pool.log_chunk_size;
    list_add_tail(&arg_obj.list_node, &cur_list);
    arg_obj.obj_id = obj_id;
    arg_obj.obj_offset = i * (1 << pool.log_chunk_size);
    }
    list_splice_tail_init(&cur_list, &pool.free_list);
    return 0;
    clean_arg_obj:
    mlx5dr_cmd_destroy_modify_header_arg(pool.dmn.mdev, obj_id);
    list_for_each_entry_safe(arg_obj, tmp_arg, &cur_list, list_node) {
    list_del(&arg_obj.list_node);
    kfree(arg_obj);
    }
    return ret;
    }
    static struct mlx5dr_arg_obj *dr_arg_pool_get_arg_obj(struct dr_arg_pool *pool)
    {
    struct mlx5dr_arg_obj *arg_obj = core::ptr::null_mut();
    int ret;
    mutex_lock(&pool.mutex);
    if (list_empty(&pool.free_list)) {
    ret = dr_arg_pool_alloc_objs(pool);
    if (ret)
    goto out;
    }
    arg_obj = list_first_entry_or_null(&pool.free_list,
    struct mlx5dr_arg_obj,
    list_node);
    WARN(!arg_obj, "couldn't get dr arg obj from pool");
    if (arg_obj)
    list_del_init(&arg_obj.list_node);
    out:
    mutex_unlock(&pool.mutex);
    return arg_obj;
    }
    static void dr_arg_pool_put_arg_obj(struct dr_arg_pool *pool,
    struct mlx5dr_arg_obj *arg_obj)
    {
    mutex_lock(&pool.mutex);
    list_add(&arg_obj.list_node, &pool.free_list);
    mutex_unlock(&pool.mutex);
    }
    static struct dr_arg_pool *dr_arg_pool_create(struct mlx5dr_domain *dmn,
    enum dr_arg_chunk_size chunk_size)
    {
    struct dr_arg_pool *pool;
    pool = kzalloc_obj(*pool);
    if (!pool)
    return core::ptr::null_mut();
    pool.dmn = dmn;
    INIT_LIST_HEAD(&pool.free_list);
    mutex_init(&pool.mutex);
    pool.log_chunk_size = chunk_size;
    if (dr_arg_pool_alloc_objs(pool))
    goto free_pool;
    return pool;
    free_pool:
    kfree(pool);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dr_arg_pool_destroy(pool: *mut dr_arg_pool) {
    static void dr_arg_pool_destroy(struct dr_arg_pool *pool)
    {
    struct mlx5dr_arg_obj *arg_obj, *tmp_arg;
    list_for_each_entry_safe(arg_obj, tmp_arg, &pool.free_list, list_node) {
    list_del(&arg_obj.list_node);
    if (!arg_obj.obj_offset) /* the first in range */
    mlx5dr_cmd_destroy_modify_header_arg(pool.dmn.mdev, arg_obj.obj_id);
    kfree(arg_obj);
    }
    mutex_destroy(&pool.mutex);
    kfree(pool);
    }
#[no_mangle]
unsafe extern "C" fn dr_arg_get_chunk_size(num_of_actions: u16) -> enum dr_arg_chunk_size {
    static enum dr_arg_chunk_size dr_arg_get_chunk_size(u16 num_of_actions)
    {
    if (num_of_actions <= 8)
    return DR_ARG_CHUNK_SIZE_1;
    if (num_of_actions <= 16)
    return DR_ARG_CHUNK_SIZE_2;
    if (num_of_actions <= 32)
    return DR_ARG_CHUNK_SIZE_3;
    if (num_of_actions <= 64)
    return DR_ARG_CHUNK_SIZE_4;
    return DR_ARG_CHUNK_SIZE_MAX;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5dr_arg_get_obj_id(arg_obj: *mut mlx5dr_arg_obj) -> u32 {
    u32 mlx5dr_arg_get_obj_id(struct mlx5dr_arg_obj *arg_obj)
    {
    return (arg_obj.obj_id + arg_obj.obj_offset);
    }
    struct mlx5dr_arg_obj *mlx5dr_arg_get_obj(struct mlx5dr_arg_mgr *mgr,
    u16 num_of_actions,
    u8 *data)
    {
    let mut size: u32 = dr_arg_get_chunk_size(num_of_actions);
    struct mlx5dr_arg_obj *arg_obj;
    int ret;
    if (size >= DR_ARG_CHUNK_SIZE_MAX)
    return core::ptr::null_mut();
    arg_obj = dr_arg_pool_get_arg_obj(mgr.pools[size]);
    if (!arg_obj) {
    mlx5dr_err(mgr.dmn, "Failed allocating args object for modify header\n");
    return core::ptr::null_mut();
    }
// write it into the hw
    ret = mlx5dr_send_postsend_args(mgr.dmn,
    mlx5dr_arg_get_obj_id(arg_obj),
    num_of_actions, data);
    if (ret) {
    mlx5dr_err(mgr.dmn, "Failed writing args object\n");
    goto put_obj;
    }
    return arg_obj;
    put_obj:
    mlx5dr_arg_put_obj(mgr, arg_obj);
    return core::ptr::null_mut();
    }
    void mlx5dr_arg_put_obj(struct mlx5dr_arg_mgr *mgr,
    struct mlx5dr_arg_obj *arg_obj)
    {
    dr_arg_pool_put_arg_obj(mgr.pools[arg_obj.log_chunk_size], arg_obj);
    }
    struct mlx5dr_arg_mgr*
    mlx5dr_arg_mgr_create(struct mlx5dr_domain *dmn)
    {
    struct mlx5dr_arg_mgr *pool_mgr;
    int i;
    if (!mlx5dr_domain_is_support_ptrn_arg(dmn))
    return core::ptr::null_mut();
    pool_mgr = kzalloc_obj(*pool_mgr);
    if (!pool_mgr)
    return core::ptr::null_mut();
    pool_mgr.dmn = dmn;
    for (i = 0; i < DR_ARG_CHUNK_SIZE_MAX; i++) {
    pool_mgr.pools[i] = dr_arg_pool_create(dmn, i);
    if (!pool_mgr.pools[i])
    goto clean_pools;
    }
    return pool_mgr;
    clean_pools:
    for (i--; i >= 0; i--)
    dr_arg_pool_destroy(pool_mgr.pools[i]);
    kfree(pool_mgr);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5dr_arg_mgr_destroy(mgr: *mut mlx5dr_arg_mgr) {
    void mlx5dr_arg_mgr_destroy(struct mlx5dr_arg_mgr *mgr)
    {
    struct dr_arg_pool **pools;
    int i;
    if (!mgr)
    return;
    pools = mgr.pools;
    for (i = 0; i < DR_ARG_CHUNK_SIZE_MAX; i++)
    dr_arg_pool_destroy(pools[i]);
    kfree(mgr);
    }
