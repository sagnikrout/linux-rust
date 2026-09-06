//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/st.c
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
//
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_st_idx_data {
    pub usecount: refcount_t,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_st {
// serialize access upon alloc/free flows
    pub lock: mutex,
    pub index_limit: xa_limit,
    pub /: *mut *mut xarray idx_xa; / key == index, value == mlx5_st_idx_data,
    pub 1: u8 direct_mode :,
}

    struct mlx5_st *mlx5_st_create(struct mlx5_core_dev *dev)
    {
    struct pci_dev *pdev = dev.pdev;
    struct mlx5_st *st;
    let mut direct_mode: u8 = 0;
    u16 num_entries;
    u32 tbl_loc;
    int ret;
    if (!MLX5_CAP_GEN(dev, mkey_pcie_tph))
    return core::ptr::null_mut();

    if (mlx5_core_is_sf(dev))
    return dev.priv.parent_mdev.st;

// Checking whether the device is capable
    if (!pdev.tph_cap)
    return core::ptr::null_mut();
    tbl_loc = pcie_tph_get_st_table_loc(pdev);
    if (tbl_loc == PCI_TPH_LOC_NONE)
    direct_mode = 1;
    if (!direct_mode) {
    num_entries = pcie_tph_get_st_table_size(pdev);
// We need a reserved entry for non TPH cases
    if (num_entries < 2)
    return core::ptr::null_mut();
    }
// The OS doesn't support ST
    ret = pcie_enable_tph(pdev, PCI_TPH_ST_DS_MODE);
    if (ret)
    return core::ptr::null_mut();
    st = kzalloc_obj(*st);
    if (!st)
    goto end;
    mutex_init(&st.lock);
    xa_init_flags(&st.idx_xa, XA_FLAGS_ALLOC);
    st.direct_mode = direct_mode;
    if (st.direct_mode)
    return st;
// entry 0 is reserved for non TPH cases
    st.index_limit.min = MLX5_MKC_PCIE_TPH_NO_STEERING_TAG_INDEX + 1;
    st.index_limit.max = num_entries - 1;
    return st;
    end:
    pcie_disable_tph(dev.pdev);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_st_destroy(dev: *mut mlx5_core_dev) {
    void mlx5_st_destroy(struct mlx5_core_dev *dev)
    {
    struct mlx5_st *st = dev.st;
    if (mlx5_core_is_sf(dev) || !st)
    return;
    pcie_disable_tph(dev.pdev);
    WARN_ON_ONCE(!xa_empty(&st.idx_xa));
    kfree(st);
    }
    int mlx5_st_alloc_index(struct mlx5_core_dev *dev, enum tph_mem_type mem_type,
    unsigned int cpu_uid, u16 *st_index)
    {
    struct mlx5_st_idx_data *idx_data;
    struct mlx5_st *st = dev.st;
    unsigned long index;
    u32 xa_id;
    u16 tag;
    int ret;
    if (!st)
    return -EOPNOTSUPP;
    ret = pcie_tph_get_cpu_st(dev.pdev, mem_type, cpu_uid, &tag);
    if (ret)
    return ret;
    if (st.direct_mode) {
// st_index = tag;
    return 0;
    }
    mutex_lock(&st.lock);
    xa_for_each(&st.idx_xa, index, idx_data) {
    if (tag == idx_data.tag) {
    refcount_inc(&idx_data.usecount);
// st_index = index;
    goto end;
    }
    }
    idx_data = kzalloc_obj(*idx_data);
    if (!idx_data) {
    ret = -ENOMEM;
    goto end;
    }
    refcount_set(&idx_data.usecount, 1);
    idx_data.tag = tag;
    ret = xa_alloc(&st.idx_xa, &xa_id, idx_data, st.index_limit, GFP_KERNEL);
    if (ret)
    goto clean_idx_data;
    ret = pcie_tph_set_st_entry(dev.pdev, xa_id, tag);
    if (ret)
    goto clean_idx_xa;
// st_index = xa_id;
    goto end;
    clean_idx_xa:
    xa_erase(&st.idx_xa, xa_id);
    clean_idx_data:
    kfree(idx_data);
    end:
    mutex_unlock(&st.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mlx5_st_alloc_index);
#[no_mangle]
pub unsafe extern "C" fn mlx5_st_dealloc_index(dev: *mut mlx5_core_dev, st_index: u16) -> c_int {
    int mlx5_st_dealloc_index(struct mlx5_core_dev *dev, u16 st_index)
    {
    struct mlx5_st_idx_data *idx_data;
    struct mlx5_st *st = dev.st;
    let mut ret: c_int = 0;
    if (!st)
    return -EOPNOTSUPP;
    if (st.direct_mode)
    return 0;
    mutex_lock(&st.lock);
    idx_data = xa_load(&st.idx_xa, st_index);
    if (WARN_ON_ONCE(!idx_data)) {
    ret = -EINVAL;
    goto end;
    }
    if (refcount_dec_and_test(&idx_data.usecount)) {
    xa_erase(&st.idx_xa, st_index);
    kfree(idx_data);
// We leave PCI config space as was before, no mkey will refer to it
    }
    end:
    mutex_unlock(&st.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mlx5_st_dealloc_index);
