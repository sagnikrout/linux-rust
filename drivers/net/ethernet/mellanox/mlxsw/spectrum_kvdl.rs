//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_kvdl.c
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
// Copyright (c) 2016-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_kvdl {
    pub kvdl_ops: *const mlxsw_sp_kvdl_ops,
    pub /: *mut *mut mutex kvdl_lock; / Protects kvdl allocations,
    pub priv: [c_ulong; ],
// priv has to be always the last item
}

#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_kvdl_init(mlxsw_sp: *mut mlxsw_sp) -> c_int {
    int mlxsw_sp_kvdl_init(struct mlxsw_sp *mlxsw_sp)
    {
    const struct mlxsw_sp_kvdl_ops *kvdl_ops = mlxsw_sp.kvdl_ops;
    struct mlxsw_sp_kvdl *kvdl;
    int err;
    kvdl = kzalloc(sizeof(*mlxsw_sp.kvdl) + kvdl_ops.priv_size,
    GFP_KERNEL);
    if (!kvdl)
    return -ENOMEM;
    mutex_init(&kvdl.kvdl_lock);
    kvdl.kvdl_ops = kvdl_ops;
    mlxsw_sp.kvdl = kvdl;
    err = kvdl_ops.init(mlxsw_sp, kvdl.priv);
    if (err)
    goto err_init;
    return 0;
    err_init:
    mutex_destroy(&kvdl.kvdl_lock);
    kfree(kvdl);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_kvdl_fini(mlxsw_sp: *mut mlxsw_sp) {
    void mlxsw_sp_kvdl_fini(struct mlxsw_sp *mlxsw_sp)
    {
    struct mlxsw_sp_kvdl *kvdl = mlxsw_sp.kvdl;
    kvdl.kvdl_ops.fini(mlxsw_sp, kvdl.priv);
    mutex_destroy(&kvdl.kvdl_lock);
    kfree(kvdl);
    }
    int mlxsw_sp_kvdl_alloc(struct mlxsw_sp *mlxsw_sp,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count, u32 *p_entry_index)
    {
    struct mlxsw_sp_kvdl *kvdl = mlxsw_sp.kvdl;
    int err;
    mutex_lock(&kvdl.kvdl_lock);
    err = kvdl.kvdl_ops.alloc(mlxsw_sp, kvdl.priv, type,
    entry_count, p_entry_index);
    mutex_unlock(&kvdl.kvdl_lock);
    return err;
    }
    void mlxsw_sp_kvdl_free(struct mlxsw_sp *mlxsw_sp,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count, int entry_index)
    {
    struct mlxsw_sp_kvdl *kvdl = mlxsw_sp.kvdl;
    mutex_lock(&kvdl.kvdl_lock);
    kvdl.kvdl_ops.free(mlxsw_sp, kvdl.priv, type,
    entry_count, entry_index);
    mutex_unlock(&kvdl.kvdl_lock);
    }
    int mlxsw_sp_kvdl_alloc_count_query(struct mlxsw_sp *mlxsw_sp,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count,
    unsigned int *p_alloc_count)
    {
    struct mlxsw_sp_kvdl *kvdl = mlxsw_sp.kvdl;
    return kvdl.kvdl_ops.alloc_size_query(mlxsw_sp, kvdl.priv, type,
    entry_count, p_alloc_count);
    }
