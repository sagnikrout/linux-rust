//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_pgt.c
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_pgt {
    pub pgt_idr: idr,
    pub /: *mut *mut u16 end_index; / Exclusive.,
    pub /: *mut *mut mutex lock; / Protects PGT.,
    pub smpe_index_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_pgt_entry {
    pub ports_list: list_head,
    pub index: u16,
    pub smpe_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_pgt_entry_port {
    pub /: *mut *mut list_head list; / Member of 'ports_list'.,
    pub local_port: u16,
}

#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_pgt_mid_alloc(mlxsw_sp: *mut mlxsw_sp, p_mid: *mut u16) -> c_int {
    int mlxsw_sp_pgt_mid_alloc(struct mlxsw_sp *mlxsw_sp, u16 *p_mid)
    {
    int index, err = 0;
    mutex_lock(&mlxsw_sp.pgt.lock);
    index = idr_alloc(&mlxsw_sp.pgt.pgt_idr, core::ptr::null_mut(), 0,
    mlxsw_sp.pgt.end_index, GFP_KERNEL);
    if (index < 0) {
    err = index;
    goto err_idr_alloc;
    }
// p_mid = index;
    mutex_unlock(&mlxsw_sp.pgt.lock);
    return 0;
    err_idr_alloc:
    mutex_unlock(&mlxsw_sp.pgt.lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_pgt_mid_free(mlxsw_sp: *mut mlxsw_sp, mid_base: u16) {
    void mlxsw_sp_pgt_mid_free(struct mlxsw_sp *mlxsw_sp, u16 mid_base)
    {
    mutex_lock(&mlxsw_sp.pgt.lock);
    WARN_ON(idr_remove(&mlxsw_sp.pgt.pgt_idr, mid_base));
    mutex_unlock(&mlxsw_sp.pgt.lock);
    }
    int mlxsw_sp_pgt_mid_alloc_range(struct mlxsw_sp *mlxsw_sp, u16 *p_mid_base,
    u16 count)
    {
    unsigned int mid_base;
    int i, err;
    mutex_lock(&mlxsw_sp.pgt.lock);
    mid_base = idr_get_cursor(&mlxsw_sp.pgt.pgt_idr);
    for (i = 0; i < count; i++) {
    err = idr_alloc_cyclic(&mlxsw_sp.pgt.pgt_idr, core::ptr::null_mut(),
    mid_base, mid_base + count, GFP_KERNEL);
    if (err < 0)
    goto err_idr_alloc_cyclic;
    }
    mutex_unlock(&mlxsw_sp.pgt.lock);
// p_mid_base = mid_base;
    return 0;
    err_idr_alloc_cyclic:
    for (i--; i >= 0; i--)
    idr_remove(&mlxsw_sp.pgt.pgt_idr, mid_base + i);
    mutex_unlock(&mlxsw_sp.pgt.lock);
    return err;
    }
    void
    mlxsw_sp_pgt_mid_free_range(struct mlxsw_sp *mlxsw_sp, u16 mid_base, u16 count)
    {
    struct idr *pgt_idr = &mlxsw_sp.pgt.pgt_idr;
    int i;
    mutex_lock(&mlxsw_sp.pgt.lock);
    for (i = 0; i < count; i++)
    WARN_ON_ONCE(idr_remove(pgt_idr, mid_base + i));
    mutex_unlock(&mlxsw_sp.pgt.lock);
    }
    static struct mlxsw_sp_pgt_entry_port *
    mlxsw_sp_pgt_entry_port_lookup(struct mlxsw_sp_pgt_entry *pgt_entry,
    u16 local_port)
    {
    struct mlxsw_sp_pgt_entry_port *pgt_entry_port;
    list_for_each_entry(pgt_entry_port, &pgt_entry.ports_list, list) {
    if (pgt_entry_port.local_port == local_port)
    return pgt_entry_port;
    }
    return core::ptr::null_mut();
    }
    static struct mlxsw_sp_pgt_entry *
    mlxsw_sp_pgt_entry_create(struct mlxsw_sp_pgt *pgt, u16 mid, u16 smpe)
    {
    struct mlxsw_sp_pgt_entry *pgt_entry;
    void *ret;
    int err;
    pgt_entry = kzalloc_obj(*pgt_entry);
    if (!pgt_entry)
    return ERR_PTR(-ENOMEM);
    ret = idr_replace(&pgt.pgt_idr, pgt_entry, mid);
    if (IS_ERR(ret)) {
    err = PTR_ERR(ret);
    goto err_idr_replace;
    }
    INIT_LIST_HEAD(&pgt_entry.ports_list);
    pgt_entry.index = mid;
    pgt_entry.smpe_index = smpe;
    return pgt_entry;
    err_idr_replace:
    kfree(pgt_entry);
    return ERR_PTR(err);
    }
    static void mlxsw_sp_pgt_entry_destroy(struct mlxsw_sp_pgt *pgt,
    struct mlxsw_sp_pgt_entry *pgt_entry)
    {
    WARN_ON(!list_empty(&pgt_entry.ports_list));
    pgt_entry = idr_replace(&pgt.pgt_idr, core::ptr::null_mut(), pgt_entry.index);
    if (WARN_ON(IS_ERR(pgt_entry)))
    return;
    kfree(pgt_entry);
    }
    static struct mlxsw_sp_pgt_entry *
    mlxsw_sp_pgt_entry_get(struct mlxsw_sp_pgt *pgt, u16 mid, u16 smpe)
    {
    struct mlxsw_sp_pgt_entry *pgt_entry;
    pgt_entry = idr_find(&pgt.pgt_idr, mid);
    if (pgt_entry)
    return pgt_entry;
    return mlxsw_sp_pgt_entry_create(pgt, mid, smpe);
    }
#[no_mangle]
unsafe extern "C" fn mlxsw_sp_pgt_entry_put(pgt: *mut mlxsw_sp_pgt, mid: u16) {
    static void mlxsw_sp_pgt_entry_put(struct mlxsw_sp_pgt *pgt, u16 mid)
    {
    struct mlxsw_sp_pgt_entry *pgt_entry;
    pgt_entry = idr_find(&pgt.pgt_idr, mid);
    if (WARN_ON(!pgt_entry))
    return;
    if (list_empty(&pgt_entry.ports_list))
    mlxsw_sp_pgt_entry_destroy(pgt, pgt_entry);
    }
    static void mlxsw_sp_pgt_smid2_port_set(char *smid2_pl, u16 local_port,
    bool member)
    {
    mlxsw_reg_smid2_port_set(smid2_pl, local_port, member);
    mlxsw_reg_smid2_port_mask_set(smid2_pl, local_port, 1);
    }
    static int
    mlxsw_sp_pgt_entry_port_write(struct mlxsw_sp *mlxsw_sp,
    const struct mlxsw_sp_pgt_entry *pgt_entry,
    u16 local_port, bool member)
    {
    char *smid2_pl;
    int err;
    smid2_pl = kmalloc(MLXSW_REG_SMID2_LEN, GFP_KERNEL);
    if (!smid2_pl)
    return -ENOMEM;
    mlxsw_reg_smid2_pack(smid2_pl, pgt_entry.index, 0, 0,
    mlxsw_sp.pgt.smpe_index_valid,
    pgt_entry.smpe_index);
    mlxsw_sp_pgt_smid2_port_set(smid2_pl, local_port, member);
    err = mlxsw_reg_write(mlxsw_sp.core, MLXSW_REG(smid2), smid2_pl);
    kfree(smid2_pl);
    return err;
    }
    static struct mlxsw_sp_pgt_entry_port *
    mlxsw_sp_pgt_entry_port_create(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_pgt_entry *pgt_entry,
    u16 local_port)
    {
    struct mlxsw_sp_pgt_entry_port *pgt_entry_port;
    int err;
    pgt_entry_port = kzalloc_obj(*pgt_entry_port);
    if (!pgt_entry_port)
    return ERR_PTR(-ENOMEM);
    err = mlxsw_sp_pgt_entry_port_write(mlxsw_sp, pgt_entry, local_port,
    true);
    if (err)
    goto err_pgt_entry_port_write;
    pgt_entry_port.local_port = local_port;
    list_add(&pgt_entry_port.list, &pgt_entry.ports_list);
    return pgt_entry_port;
    err_pgt_entry_port_write:
    kfree(pgt_entry_port);
    return ERR_PTR(err);
    }
    static void
    mlxsw_sp_pgt_entry_port_destroy(struct mlxsw_sp *mlxsw_sp,
    struct mlxsw_sp_pgt_entry *pgt_entry,
    struct mlxsw_sp_pgt_entry_port *pgt_entry_port)
    {
    list_del(&pgt_entry_port.list);
    mlxsw_sp_pgt_entry_port_write(mlxsw_sp, pgt_entry,
    pgt_entry_port.local_port, false);
    kfree(pgt_entry_port);
    }
    static int mlxsw_sp_pgt_entry_port_add(struct mlxsw_sp *mlxsw_sp, u16 mid,
    u16 smpe, u16 local_port)
    {
    struct mlxsw_sp_pgt_entry_port *pgt_entry_port;
    struct mlxsw_sp_pgt_entry *pgt_entry;
    int err;
    mutex_lock(&mlxsw_sp.pgt.lock);
    pgt_entry = mlxsw_sp_pgt_entry_get(mlxsw_sp.pgt, mid, smpe);
    if (IS_ERR(pgt_entry)) {
    err = PTR_ERR(pgt_entry);
    goto err_pgt_entry_get;
    }
    pgt_entry_port = mlxsw_sp_pgt_entry_port_create(mlxsw_sp, pgt_entry,
    local_port);
    if (IS_ERR(pgt_entry_port)) {
    err = PTR_ERR(pgt_entry_port);
    goto err_pgt_entry_port_get;
    }
    mutex_unlock(&mlxsw_sp.pgt.lock);
    return 0;
    err_pgt_entry_port_get:
    mlxsw_sp_pgt_entry_put(mlxsw_sp.pgt, mid);
    err_pgt_entry_get:
    mutex_unlock(&mlxsw_sp.pgt.lock);
    return err;
    }
    static void mlxsw_sp_pgt_entry_port_del(struct mlxsw_sp *mlxsw_sp,
    u16 mid, u16 smpe, u16 local_port)
    {
    struct mlxsw_sp_pgt_entry_port *pgt_entry_port;
    struct mlxsw_sp_pgt_entry *pgt_entry;
    mutex_lock(&mlxsw_sp.pgt.lock);
    pgt_entry = idr_find(&mlxsw_sp.pgt.pgt_idr, mid);
    if (!pgt_entry)
    goto out;
    pgt_entry_port = mlxsw_sp_pgt_entry_port_lookup(pgt_entry, local_port);
    if (!pgt_entry_port)
    goto out;
    mlxsw_sp_pgt_entry_port_destroy(mlxsw_sp, pgt_entry, pgt_entry_port);
    mlxsw_sp_pgt_entry_put(mlxsw_sp.pgt, mid);
    out:
    mutex_unlock(&mlxsw_sp.pgt.lock);
    }
    int mlxsw_sp_pgt_entry_port_set(struct mlxsw_sp *mlxsw_sp, u16 mid,
    u16 smpe, u16 local_port, bool member)
    {
    if (member)
    return mlxsw_sp_pgt_entry_port_add(mlxsw_sp, mid, smpe,
    local_port);
    mlxsw_sp_pgt_entry_port_del(mlxsw_sp, mid, smpe, local_port);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_pgt_init(mlxsw_sp: *mut mlxsw_sp) -> c_int {
    int mlxsw_sp_pgt_init(struct mlxsw_sp *mlxsw_sp)
    {
    struct mlxsw_sp_pgt *pgt;
    if (!MLXSW_CORE_RES_VALID(mlxsw_sp.core, PGT_SIZE))
    return -EIO;
    pgt = kzalloc_obj(*mlxsw_sp.pgt);
    if (!pgt)
    return -ENOMEM;
    idr_init(&pgt.pgt_idr);
    pgt.end_index = MLXSW_CORE_RES_GET(mlxsw_sp.core, PGT_SIZE);
    mutex_init(&pgt.lock);
    pgt.smpe_index_valid = mlxsw_sp.pgt_smpe_index_valid;
    mlxsw_sp.pgt = pgt;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlxsw_sp_pgt_fini(mlxsw_sp: *mut mlxsw_sp) {
    void mlxsw_sp_pgt_fini(struct mlxsw_sp *mlxsw_sp)
    {
    mutex_destroy(&mlxsw_sp.pgt.lock);
    WARN_ON(!idr_is_empty(&mlxsw_sp.pgt.pgt_idr));
    idr_destroy(&mlxsw_sp.pgt.pgt_idr);
    kfree(mlxsw_sp.pgt);
    }
