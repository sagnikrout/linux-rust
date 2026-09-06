//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx4/pd.c
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


//
// Copyright (c) 2006, 2007 Cisco Systems, Inc.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    enum {
    MLX4_NUM_RESERVED_UARS = 8
    };
#[no_mangle]
pub unsafe extern "C" fn mlx4_pd_alloc(dev: *mut mlx4_dev, pdn: *mut u32) -> c_int {
    int mlx4_pd_alloc(struct mlx4_dev *dev, u32 *pdn)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
// pdn = mlx4_bitmap_alloc(&priv->pd_bitmap);
    if (*pdn == -1)
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx4_pd_alloc);
#[no_mangle]
pub unsafe extern "C" fn mlx4_pd_free(dev: *mut mlx4_dev, pdn: u32) {
    void mlx4_pd_free(struct mlx4_dev *dev, u32 pdn)
    {
    mlx4_bitmap_free(&mlx4_priv(dev).pd_bitmap, pdn, MLX4_USE_RR);
    }
    EXPORT_SYMBOL_GPL(mlx4_pd_free);
#[no_mangle]
pub unsafe extern "C" fn __mlx4_xrcd_alloc(dev: *mut mlx4_dev, xrcdn: *mut u32) -> c_int {
    int __mlx4_xrcd_alloc(struct mlx4_dev *dev, u32 *xrcdn)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
// xrcdn = mlx4_bitmap_alloc(&priv->xrcd_bitmap);
    if (*xrcdn == -1)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_xrcd_alloc(dev: *mut mlx4_dev, xrcdn: *mut u32) -> c_int {
    int mlx4_xrcd_alloc(struct mlx4_dev *dev, u32 *xrcdn)
    {
    u64 out_param;
    int err;
    if (mlx4_is_mfunc(dev)) {
    err = mlx4_cmd_imm(dev, 0, &out_param,
    RES_XRCD, RES_OP_RESERVE,
    MLX4_CMD_ALLOC_RES,
    MLX4_CMD_TIME_CLASS_A, MLX4_CMD_WRAPPED);
    if (err)
    return err;
// xrcdn = get_param_l(&out_param);
    return 0;
    }
    return __mlx4_xrcd_alloc(dev, xrcdn);
    }
    EXPORT_SYMBOL_GPL(mlx4_xrcd_alloc);
#[no_mangle]
pub unsafe extern "C" fn __mlx4_xrcd_free(dev: *mut mlx4_dev, xrcdn: u32) {
    void __mlx4_xrcd_free(struct mlx4_dev *dev, u32 xrcdn)
    {
    mlx4_bitmap_free(&mlx4_priv(dev).xrcd_bitmap, xrcdn, MLX4_USE_RR);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_xrcd_free(dev: *mut mlx4_dev, xrcdn: u32) {
    void mlx4_xrcd_free(struct mlx4_dev *dev, u32 xrcdn)
    {
    let mut in_param: u64 = 0;
    int err;
    if (mlx4_is_mfunc(dev)) {
    set_param_l(&in_param, xrcdn);
    err = mlx4_cmd(dev, in_param, RES_XRCD,
    RES_OP_RESERVE, MLX4_CMD_FREE_RES,
    MLX4_CMD_TIME_CLASS_A, MLX4_CMD_WRAPPED);
    if (err)
    mlx4_warn(dev, "Failed to release xrcdn %d\n", xrcdn);
    } else
    __mlx4_xrcd_free(dev, xrcdn);
    }
    EXPORT_SYMBOL_GPL(mlx4_xrcd_free);
#[no_mangle]
pub unsafe extern "C" fn mlx4_init_pd_table(dev: *mut mlx4_dev) -> c_int {
    int mlx4_init_pd_table(struct mlx4_dev *dev)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    return mlx4_bitmap_init(&priv.pd_bitmap, dev.caps.num_pds,
    (1 << NOT_MASKED_PD_BITS) - 1,
    dev.caps.reserved_pds, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_cleanup_pd_table(dev: *mut mlx4_dev) {
    void mlx4_cleanup_pd_table(struct mlx4_dev *dev)
    {
    mlx4_bitmap_cleanup(&mlx4_priv(dev).pd_bitmap);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_init_xrcd_table(dev: *mut mlx4_dev) -> c_int {
    int mlx4_init_xrcd_table(struct mlx4_dev *dev)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    return mlx4_bitmap_init(&priv.xrcd_bitmap, (1 << 16),
    (1 << 16) - 1, dev.caps.reserved_xrcds + 1, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_cleanup_xrcd_table(dev: *mut mlx4_dev) {
    void mlx4_cleanup_xrcd_table(struct mlx4_dev *dev)
    {
    mlx4_bitmap_cleanup(&mlx4_priv(dev).xrcd_bitmap);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_uar_alloc(dev: *mut mlx4_dev, uar: *mut mlx4_uar) -> c_int {
    int mlx4_uar_alloc(struct mlx4_dev *dev, struct mlx4_uar *uar)
    {
    int offset;
    uar.index = mlx4_bitmap_alloc(&mlx4_priv(dev).uar_table.bitmap);
    if (uar.index == -1)
    return -ENOMEM;
    if (mlx4_is_slave(dev))
    offset = uar.index % ((int)pci_resource_len(dev.persist.pdev,
    2) /
    dev.caps.uar_page_size);
    else
    offset = uar.index;
    uar.pfn = (pci_resource_start(dev.persist.pdev, 2) >> PAGE_SHIFT)
    + offset;
    uar.map = core::ptr::null_mut();
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx4_uar_alloc);
#[no_mangle]
pub unsafe extern "C" fn mlx4_uar_free(dev: *mut mlx4_dev, uar: *mut mlx4_uar) {
    void mlx4_uar_free(struct mlx4_dev *dev, struct mlx4_uar *uar)
    {
    mlx4_bitmap_free(&mlx4_priv(dev).uar_table.bitmap, uar.index, MLX4_USE_RR);
    }
    EXPORT_SYMBOL_GPL(mlx4_uar_free);
#[no_mangle]
pub unsafe extern "C" fn mlx4_bf_alloc(dev: *mut mlx4_dev, bf: *mut mlx4_bf, node: c_int) -> c_int {
    int mlx4_bf_alloc(struct mlx4_dev *dev, struct mlx4_bf *bf, int node)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    struct mlx4_uar *uar;
    let mut err: c_int = 0;
    int idx;
    if (!priv.bf_mapping)
    return -ENOMEM;
    mutex_lock(&priv.bf_mutex);
    if (!list_empty(&priv.bf_list))
    uar = list_entry(priv.bf_list.next, struct mlx4_uar, bf_list);
    else {
    if (mlx4_bitmap_avail(&priv.uar_table.bitmap) < MLX4_NUM_RESERVED_UARS) {
    err = -ENOMEM;
    goto out;
    }
    uar = kmalloc_node(sizeof(*uar), GFP_KERNEL, node);
    if (!uar) {
    uar = kmalloc_obj(*uar);
    if (!uar) {
    err = -ENOMEM;
    goto out;
    }
    }
    err = mlx4_uar_alloc(dev, uar);
    if (err)
    goto free_kmalloc;
    uar.map = ioremap(uar.pfn << PAGE_SHIFT, PAGE_SIZE);
    if (!uar.map) {
    err = -ENOMEM;
    goto free_uar;
    }
    uar.bf_map = io_mapping_map_wc(priv.bf_mapping,
    uar.index << PAGE_SHIFT,
    PAGE_SIZE);
    if (!uar.bf_map) {
    err = -ENOMEM;
    goto unamp_uar;
    }
    uar.free_bf_bmap = 0;
    list_add(&uar.bf_list, &priv.bf_list);
    }
    idx = ffz(uar.free_bf_bmap);
    uar.free_bf_bmap |= 1 << idx;
    bf.uar = uar;
    bf.offset = 0;
    bf.buf_size = dev.caps.bf_reg_size / 2;
    bf.reg = uar.bf_map + idx * dev.caps.bf_reg_size;
    if (uar.free_bf_bmap == (1 << dev.caps.bf_regs_per_page) - 1)
    list_del_init(&uar.bf_list);
    goto out;
    unamp_uar:
    bf.uar = core::ptr::null_mut();
    iounmap(uar.map);
    free_uar:
    mlx4_uar_free(dev, uar);
    free_kmalloc:
    kfree(uar);
    out:
    mutex_unlock(&priv.bf_mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(mlx4_bf_alloc);
#[no_mangle]
pub unsafe extern "C" fn mlx4_bf_free(dev: *mut mlx4_dev, bf: *mut mlx4_bf) {
    void mlx4_bf_free(struct mlx4_dev *dev, struct mlx4_bf *bf)
    {
    struct mlx4_priv *priv = mlx4_priv(dev);
    int idx;
    if (!bf.uar || !bf.uar.bf_map)
    return;
    mutex_lock(&priv.bf_mutex);
    idx = (bf.reg - bf.uar.bf_map) / dev.caps.bf_reg_size;
    bf.uar.free_bf_bmap &= ~(1 << idx);
    if (!bf.uar.free_bf_bmap) {
    if (!list_empty(&bf.uar.bf_list))
    list_del(&bf.uar.bf_list);
    io_mapping_unmap(bf.uar.bf_map);
    iounmap(bf.uar.map);
    mlx4_uar_free(dev, bf.uar);
    kfree(bf.uar);
    } else if (list_empty(&bf.uar.bf_list))
    list_add(&bf.uar.bf_list, &priv.bf_list);
    mutex_unlock(&priv.bf_mutex);
    }
    EXPORT_SYMBOL_GPL(mlx4_bf_free);
#[no_mangle]
pub unsafe extern "C" fn mlx4_init_uar_table(dev: *mut mlx4_dev) -> c_int {
    int mlx4_init_uar_table(struct mlx4_dev *dev)
    {
    let mut num_reserved_uar: c_int = mlx4_get_num_reserved_uar(dev);
    mlx4_dbg(dev, "uar_page_shift = %d", dev.uar_page_shift);
    mlx4_dbg(dev, "Effective reserved_uars=%d", dev.caps.reserved_uars);
    if (dev.caps.num_uars <= num_reserved_uar) {
    mlx4_err(
    dev, "Only %d UAR pages (need more than %d)\n",
    dev.caps.num_uars, num_reserved_uar);
    mlx4_err(dev, "Increase firmware log2_uar_bar_megabytes?\n");
    return -ENODEV;
    }
    return mlx4_bitmap_init(&mlx4_priv(dev).uar_table.bitmap,
    dev.caps.num_uars, dev.caps.num_uars - 1,
    dev.caps.reserved_uars, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_cleanup_uar_table(dev: *mut mlx4_dev) {
    void mlx4_cleanup_uar_table(struct mlx4_dev *dev)
    {
    mlx4_bitmap_cleanup(&mlx4_priv(dev).uar_table.bitmap);
    }
