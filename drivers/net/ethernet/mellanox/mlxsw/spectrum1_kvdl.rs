//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum1_kvdl.c
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
// Copyright (c) 2018 Mellanox Technologies. All rights reserved

pub const MLXSW_SP1_KVDL_SINGLE_BASE: c_int = 0;
pub const MLXSW_SP1_KVDL_SINGLE_SIZE: c_int = 16384;

    (MLXSW_SP1_KVDL_SINGLE_SIZE + MLXSW_SP1_KVDL_SINGLE_BASE - 1)

    (MLXSW_SP1_KVDL_SINGLE_BASE + MLXSW_SP1_KVDL_SINGLE_SIZE)
pub const MLXSW_SP1_KVDL_CHUNKS_SIZE: c_int = 49152;

    (MLXSW_SP1_KVDL_CHUNKS_SIZE + MLXSW_SP1_KVDL_CHUNKS_BASE - 1)

    (MLXSW_SP1_KVDL_CHUNKS_BASE + MLXSW_SP1_KVDL_CHUNKS_SIZE)

    (MLXSW_SP_KVD_LINEAR_SIZE - MLXSW_SP1_KVDL_LARGE_CHUNKS_BASE)

    (MLXSW_SP1_KVDL_LARGE_CHUNKS_SIZE + MLXSW_SP1_KVDL_LARGE_CHUNKS_BASE - 1)
pub const MLXSW_SP1_KVDL_SINGLE_ALLOC_SIZE: c_int = 1;
pub const MLXSW_SP1_KVDL_CHUNKS_ALLOC_SIZE: c_int = 32;
pub const MLXSW_SP1_KVDL_LARGE_CHUNKS_ALLOC_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_kvdl_part_info {
    pub part_index: c_uint,
    pub start_index: c_uint,
    pub end_index: c_uint,
    pub alloc_size: c_uint,
    pub resource_id: enum mlxsw_sp_resource_id,
}

    enum mlxsw_sp1_kvdl_part_id {
    MLXSW_SP1_KVDL_PART_ID_SINGLE,
    MLXSW_SP1_KVDL_PART_ID_CHUNKS,
    MLXSW_SP1_KVDL_PART_ID_LARGE_CHUNKS,
    };

    [MLXSW_SP1_KVDL_PART_ID_##id] = {				\
    .start_index = MLXSW_SP1_KVDL_##id##_BASE,		\
    .end_index = MLXSW_SP1_KVDL_##id##_END,			\
    .alloc_size = MLXSW_SP1_KVDL_##id##_ALLOC_SIZE,		\
    .resource_id = MLXSW_SP_RESOURCE_KVD_LINEAR_##id,	\
    }
    static const struct mlxsw_sp1_kvdl_part_info mlxsw_sp1_kvdl_parts_info[] = {
    MLXSW_SP1_KVDL_PART_INFO(SINGLE),
    MLXSW_SP1_KVDL_PART_INFO(CHUNKS),
    MLXSW_SP1_KVDL_PART_INFO(LARGE_CHUNKS),
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_kvdl_part {
    pub info: mlxsw_sp1_kvdl_part_info,
    pub /: *mut *mut unsigned long usage[]; / Entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp1_kvdl {
    pub parts: [*mut mlxsw_sp1_kvdl_part; MLXSW_SP1_KVDL_PARTS_INFO_LEN],
}

    static struct mlxsw_sp1_kvdl_part *
    mlxsw_sp1_kvdl_alloc_size_part(struct mlxsw_sp1_kvdl *kvdl,
    unsigned int alloc_size)
    {
    struct mlxsw_sp1_kvdl_part *part, *min_part = core::ptr::null_mut();
    int i;
    for (i = 0; i < MLXSW_SP1_KVDL_PARTS_INFO_LEN; i++) {
    part = kvdl.parts[i];
    if (alloc_size <= part.info.alloc_size &&
    (!min_part ||
    part.info.alloc_size <= min_part.info.alloc_size))
    min_part = part;
    }
    return min_part ?: ERR_PTR(-ENOBUFS);
    }
    static struct mlxsw_sp1_kvdl_part *
    mlxsw_sp1_kvdl_index_part(struct mlxsw_sp1_kvdl *kvdl, u32 kvdl_index)
    {
    struct mlxsw_sp1_kvdl_part *part;
    int i;
    for (i = 0; i < MLXSW_SP1_KVDL_PARTS_INFO_LEN; i++) {
    part = kvdl.parts[i];
    if (kvdl_index >= part.info.start_index &&
    kvdl_index <= part.info.end_index)
    return part;
    }
    return ERR_PTR(-EINVAL);
    }
    static u32
    mlxsw_sp1_kvdl_to_kvdl_index(const struct mlxsw_sp1_kvdl_part_info *info,
    unsigned int entry_index)
    {
    return info.start_index + entry_index * info.alloc_size;
    }
    static unsigned int
    mlxsw_sp1_kvdl_to_entry_index(const struct mlxsw_sp1_kvdl_part_info *info,
    u32 kvdl_index)
    {
    return (kvdl_index - info.start_index) / info.alloc_size;
    }
    static int mlxsw_sp1_kvdl_part_alloc(struct mlxsw_sp1_kvdl_part *part,
    u32 *p_kvdl_index)
    {
    const struct mlxsw_sp1_kvdl_part_info *info = &part.info;
    unsigned int entry_index, nr_entries;
    nr_entries = (info.end_index - info.start_index + 1) /
    info.alloc_size;
    entry_index = find_first_zero_bit(part.usage, nr_entries);
    if (entry_index == nr_entries)
    return -ENOBUFS;
    __set_bit(entry_index, part.usage);
// p_kvdl_index = mlxsw_sp1_kvdl_to_kvdl_index(info, entry_index);
    return 0;
    }
    static void mlxsw_sp1_kvdl_part_free(struct mlxsw_sp1_kvdl_part *part,
    u32 kvdl_index)
    {
    const struct mlxsw_sp1_kvdl_part_info *info = &part.info;
    unsigned int entry_index;
    entry_index = mlxsw_sp1_kvdl_to_entry_index(info, kvdl_index);
    __clear_bit(entry_index, part.usage);
    }
    static int mlxsw_sp1_kvdl_alloc(struct mlxsw_sp *mlxsw_sp, void *priv,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count,
    u32 *p_entry_index)
    {
    struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
// Find partition with smallest allocation size satisfying the
// requested size.
//
    part = mlxsw_sp1_kvdl_alloc_size_part(kvdl, entry_count);
    if (IS_ERR(part))
    return PTR_ERR(part);
    return mlxsw_sp1_kvdl_part_alloc(part, p_entry_index);
    }
    static void mlxsw_sp1_kvdl_free(struct mlxsw_sp *mlxsw_sp, void *priv,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count, int entry_index)
    {
    struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
    part = mlxsw_sp1_kvdl_index_part(kvdl, entry_index);
    if (IS_ERR(part))
    return;
    mlxsw_sp1_kvdl_part_free(part, entry_index);
    }
    static int mlxsw_sp1_kvdl_alloc_size_query(struct mlxsw_sp *mlxsw_sp,
    void *priv,
    enum mlxsw_sp_kvdl_entry_type type,
    unsigned int entry_count,
    unsigned int *p_alloc_size)
    {
    struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
    part = mlxsw_sp1_kvdl_alloc_size_part(kvdl, entry_count);
    if (IS_ERR(part))
    return PTR_ERR(part);
// p_alloc_size = part->info.alloc_size;
    return 0;
    }
    static void mlxsw_sp1_kvdl_part_update(struct mlxsw_sp1_kvdl_part *part,
    struct mlxsw_sp1_kvdl_part *part_prev,
    unsigned int size)
    {
    if (!part_prev) {
    part.info.end_index = size - 1;
    } else {
    part.info.start_index = part_prev.info.end_index + 1;
    part.info.end_index = part.info.start_index + size - 1;
    }
    }
    static struct mlxsw_sp1_kvdl_part *
    mlxsw_sp1_kvdl_part_init(struct mlxsw_sp *mlxsw_sp,
    const struct mlxsw_sp1_kvdl_part_info *info,
    struct mlxsw_sp1_kvdl_part *part_prev)
    {
    struct devlink *devlink = priv_to_devlink(mlxsw_sp.core);
    struct mlxsw_sp1_kvdl_part *part;
    let mut need_update: bool = true;
    unsigned int nr_entries;
    u64 resource_size;
    int err;
    err = devl_resource_size_get(devlink, info.resource_id,
    &resource_size);
    if (err) {
    need_update = false;
    resource_size = info.end_index - info.start_index + 1;
    }
    nr_entries = div_u64(resource_size, info.alloc_size);
    part = kzalloc_flex(*part, usage, BITS_TO_LONGS(nr_entries));
    if (!part)
    return ERR_PTR(-ENOMEM);
    memcpy(&part.info, info, sizeof(part.info));
    if (need_update)
    mlxsw_sp1_kvdl_part_update(part, part_prev, resource_size);
    return part;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_part_fini(part: *mut mlxsw_sp1_kvdl_part) {
    static void mlxsw_sp1_kvdl_part_fini(struct mlxsw_sp1_kvdl_part *part)
    {
    kfree(part);
    }
    static int mlxsw_sp1_kvdl_parts_init(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp1_kvdl *kvdl)
    {
    const struct mlxsw_sp1_kvdl_part_info *info;
    struct mlxsw_sp1_kvdl_part *part_prev = core::ptr::null_mut();
    int err, i;
    for (i = 0; i < MLXSW_SP1_KVDL_PARTS_INFO_LEN; i++) {
    info = &mlxsw_sp1_kvdl_parts_info[i];
    kvdl.parts[i] = mlxsw_sp1_kvdl_part_init(mlxsw_sp, info,
    part_prev);
    if (IS_ERR(kvdl.parts[i])) {
    err = PTR_ERR(kvdl.parts[i]);
    goto err_kvdl_part_init;
    }
    part_prev = kvdl.parts[i];
    }
    return 0;
    err_kvdl_part_init:
    for (i--; i >= 0; i--)
    mlxsw_sp1_kvdl_part_fini(kvdl.parts[i]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_parts_fini(kvdl: *mut mlxsw_sp1_kvdl) {
    static void mlxsw_sp1_kvdl_parts_fini(struct mlxsw_sp1_kvdl *kvdl)
    {
    int i;
    for (i = 0; i < MLXSW_SP1_KVDL_PARTS_INFO_LEN; i++)
    mlxsw_sp1_kvdl_part_fini(kvdl.parts[i]);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_part_occ(part: *mut mlxsw_sp1_kvdl_part) -> u64 {
    static u64 mlxsw_sp1_kvdl_part_occ(struct mlxsw_sp1_kvdl_part *part)
    {
    const struct mlxsw_sp1_kvdl_part_info *info = &part.info;
    unsigned int nr_entries;
    let mut bit: c_int = -1;
    let mut occ: u64 = 0;
    nr_entries = (info.end_index -
    info.start_index + 1) /
    info.alloc_size;
    while ((bit = find_next_bit(part.usage, nr_entries, bit + 1))
    < nr_entries)
    occ += info.alloc_size;
    return occ;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_occ_get(priv: *mut c_void) -> u64 {
    static u64 mlxsw_sp1_kvdl_occ_get(void *priv)
    {
    const struct mlxsw_sp1_kvdl *kvdl = priv;
    let mut occ: u64 = 0;
    int i;
    for (i = 0; i < MLXSW_SP1_KVDL_PARTS_INFO_LEN; i++)
    occ += mlxsw_sp1_kvdl_part_occ(kvdl.parts[i]);
    return occ;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_single_occ_get(priv: *mut c_void) -> u64 {
    static u64 mlxsw_sp1_kvdl_single_occ_get(void *priv)
    {
    const struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
    part = kvdl.parts[MLXSW_SP1_KVDL_PART_ID_SINGLE];
    return mlxsw_sp1_kvdl_part_occ(part);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_chunks_occ_get(priv: *mut c_void) -> u64 {
    static u64 mlxsw_sp1_kvdl_chunks_occ_get(void *priv)
    {
    const struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
    part = kvdl.parts[MLXSW_SP1_KVDL_PART_ID_CHUNKS];
    return mlxsw_sp1_kvdl_part_occ(part);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_large_chunks_occ_get(priv: *mut c_void) -> u64 {
    static u64 mlxsw_sp1_kvdl_large_chunks_occ_get(void *priv)
    {
    const struct mlxsw_sp1_kvdl *kvdl = priv;
    struct mlxsw_sp1_kvdl_part *part;
    part = kvdl.parts[MLXSW_SP1_KVDL_PART_ID_LARGE_CHUNKS];
    return mlxsw_sp1_kvdl_part_occ(part);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_init(mlxsw_sp: *mut mlxsw_sp, priv: *mut c_void) -> c_int {
    static int mlxsw_sp1_kvdl_init(struct mlxsw_sp *mlxsw_sp, void *priv)
    {
    struct devlink *devlink = priv_to_devlink(mlxsw_sp.core);
    struct mlxsw_sp1_kvdl *kvdl = priv;
    int err;
    err = mlxsw_sp1_kvdl_parts_init(mlxsw_sp, kvdl);
    if (err)
    return err;
    devl_resource_occ_get_register(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR,
    mlxsw_sp1_kvdl_occ_get,
    kvdl);
    devl_resource_occ_get_register(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_SINGLE,
    mlxsw_sp1_kvdl_single_occ_get,
    kvdl);
    devl_resource_occ_get_register(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_CHUNKS,
    mlxsw_sp1_kvdl_chunks_occ_get,
    kvdl);
    devl_resource_occ_get_register(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_LARGE_CHUNKS,
    mlxsw_sp1_kvdl_large_chunks_occ_get,
    kvdl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp1_kvdl_fini(mlxsw_sp: *mut mlxsw_sp, priv: *mut c_void) {
    static void mlxsw_sp1_kvdl_fini(struct mlxsw_sp *mlxsw_sp, void *priv)
    {
    struct devlink *devlink = priv_to_devlink(mlxsw_sp.core);
    struct mlxsw_sp1_kvdl *kvdl = priv;
    devl_resource_occ_get_unregister(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_LARGE_CHUNKS);
    devl_resource_occ_get_unregister(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_CHUNKS);
    devl_resource_occ_get_unregister(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR_SINGLE);
    devl_resource_occ_get_unregister(devlink,
    MLXSW_SP_RESOURCE_KVD_LINEAR);
    mlxsw_sp1_kvdl_parts_fini(kvdl);
    }
    const struct mlxsw_sp_kvdl_ops mlxsw_sp1_kvdl_ops = {
    .priv_size = sizeof(struct mlxsw_sp1_kvdl),
    .init = mlxsw_sp1_kvdl_init,
    .fini = mlxsw_sp1_kvdl_fini,
    .alloc = mlxsw_sp1_kvdl_alloc,
    .free = mlxsw_sp1_kvdl_free,
    .alloc_size_query = mlxsw_sp1_kvdl_alloc_size_query,
    };
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp1_kvdl_resources_register(mlxsw_core: *mut mlxsw_core) -> c_int {
    int mlxsw_sp1_kvdl_resources_register(struct mlxsw_core *mlxsw_core)
    {
    struct devlink *devlink = priv_to_devlink(mlxsw_core);
    static struct devlink_resource_size_params size_params;
    u32 kvdl_max_size;
    int err;
    kvdl_max_size = MLXSW_CORE_RES_GET(mlxsw_core, KVD_SIZE) -
    MLXSW_CORE_RES_GET(mlxsw_core, KVD_SINGLE_MIN_SIZE) -
    MLXSW_CORE_RES_GET(mlxsw_core, KVD_DOUBLE_MIN_SIZE);
    devlink_resource_size_params_init(&size_params, 0, kvdl_max_size,
    MLXSW_SP1_KVDL_SINGLE_ALLOC_SIZE,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    err = devl_resource_register(devlink, MLXSW_SP_RESOURCE_NAME_KVD_LINEAR_SINGLES,
    MLXSW_SP1_KVDL_SINGLE_SIZE,
    MLXSW_SP_RESOURCE_KVD_LINEAR_SINGLE,
    MLXSW_SP_RESOURCE_KVD_LINEAR,
    &size_params);
    if (err)
    return err;
    devlink_resource_size_params_init(&size_params, 0, kvdl_max_size,
    MLXSW_SP1_KVDL_CHUNKS_ALLOC_SIZE,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    err = devl_resource_register(devlink, MLXSW_SP_RESOURCE_NAME_KVD_LINEAR_CHUNKS,
    MLXSW_SP1_KVDL_CHUNKS_SIZE,
    MLXSW_SP_RESOURCE_KVD_LINEAR_CHUNKS,
    MLXSW_SP_RESOURCE_KVD_LINEAR,
    &size_params);
    if (err)
    return err;
    devlink_resource_size_params_init(&size_params, 0, kvdl_max_size,
    MLXSW_SP1_KVDL_LARGE_CHUNKS_ALLOC_SIZE,
    DEVLINK_RESOURCE_UNIT_ENTRY);
    err = devl_resource_register(devlink, MLXSW_SP_RESOURCE_NAME_KVD_LINEAR_LARGE_CHUNKS,
    MLXSW_SP1_KVDL_LARGE_CHUNKS_SIZE,
    MLXSW_SP_RESOURCE_KVD_LINEAR_LARGE_CHUNKS,
    MLXSW_SP_RESOURCE_KVD_LINEAR,
    &size_params);
    return err;
    }
