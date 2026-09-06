//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_common.c
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
// Copyright (c) 2016, Mellanox Technologies. All rights reserved.
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

// mlx5e global resources should be placed in this file.
// Global resources are common to all the netdevices created on the same nic.
//
#[no_mangle]
pub unsafe extern "C" fn mlx5e_mkey_set_relaxed_ordering(mdev: *mut mlx5_core_dev, mkc: *mut c_void) {
    void mlx5e_mkey_set_relaxed_ordering(struct mlx5_core_dev *mdev, void *mkc)
    {
    let mut ro_write: bool = MLX5_CAP_GEN(mdev, relaxed_ordering_write);
    bool ro_read = MLX5_CAP_GEN(mdev, relaxed_ordering_read) ||
    (pcie_relaxed_ordering_enabled(mdev.pdev) &&
    MLX5_CAP_GEN(mdev, relaxed_ordering_read_pci_enabled));
    MLX5_SET(mkc, mkc, relaxed_ordering_read, ro_read);
    MLX5_SET(mkc, mkc, relaxed_ordering_write, ro_write);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_create_mkey(mdev: *mut mlx5_core_dev, pdn: u32, mkey: *mut u32) -> c_int {
    int mlx5e_create_mkey(struct mlx5_core_dev *mdev, u32 pdn, u32 *mkey)
    {
    let mut inlen: c_int = MLX5_ST_SZ_BYTES(create_mkey_in);
    void *mkc;
    u32 *in;
    int err;
    in = kvzalloc(inlen, GFP_KERNEL);
    if (!in)
    return -ENOMEM;
    mkc = MLX5_ADDR_OF(create_mkey_in, in, memory_key_mkey_entry);
    MLX5_SET(mkc, mkc, access_mode_1_0, MLX5_MKC_ACCESS_MODE_PA);
    MLX5_SET(mkc, mkc, lw, 1);
    MLX5_SET(mkc, mkc, lr, 1);
    mlx5e_mkey_set_relaxed_ordering(mdev, mkc);
    MLX5_SET(mkc, mkc, pd, pdn);
    MLX5_SET(mkc, mkc, length64, 1);
    MLX5_SET(mkc, mkc, qpn, 0xffffff);
    err = mlx5_core_create_mkey(mdev, mkey, in, inlen);
    kvfree(in);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_create_tis(mdev: *mut mlx5_core_dev, in: *mut c_void, tisn: *mut u32) -> c_int {
    int mlx5e_create_tis(struct mlx5_core_dev *mdev, void *in, u32 *tisn)
    {
    void *tisc = MLX5_ADDR_OF(create_tis_in, in, ctx);
    MLX5_SET(tisc, tisc, transport_domain, mdev.mlx5e_res.hw_objs.td.tdn);
    if (mlx5_lag_is_lacp_owner(mdev))
    MLX5_SET(tisc, tisc, strict_lag_tx_port_affinity, 1);
    return mlx5_core_create_tis(mdev, in, tisn);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_destroy_tis(mdev: *mut mlx5_core_dev, tisn: u32) {
    void mlx5e_destroy_tis(struct mlx5_core_dev *mdev, u32 tisn)
    {
    mlx5_core_destroy_tis(mdev, tisn);
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_destroy_tises(mdev: *mut mlx5_core_dev, tisn[MLX5_MAX_PORTS][MLX5_MAX_NUM_TC]: u32) {
    static void mlx5e_destroy_tises(struct mlx5_core_dev *mdev, u32 tisn[MLX5_MAX_PORTS][MLX5_MAX_NUM_TC])
    {
    int tc, i;
    for (i = 0; i < mlx5e_get_num_lag_ports(mdev); i++)
    for (tc = 0; tc < MLX5_MAX_NUM_TC; tc++)
    mlx5e_destroy_tis(mdev, tisn[i][tc]);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_lag_should_assign_affinity(mdev: *mut mlx5_core_dev) -> bool {
    static bool mlx5_lag_should_assign_affinity(struct mlx5_core_dev *mdev)
    {
    return MLX5_CAP_GEN(mdev, lag_tx_port_affinity) && mlx5e_get_num_lag_ports(mdev) > 1;
    }
#[no_mangle]
unsafe extern "C" fn mlx5e_create_tises(mdev: *mut mlx5_core_dev, tisn[MLX5_MAX_PORTS][MLX5_MAX_NUM_TC]: u32) -> c_int {
    static int mlx5e_create_tises(struct mlx5_core_dev *mdev, u32 tisn[MLX5_MAX_PORTS][MLX5_MAX_NUM_TC])
    {
    int tc, i;
    int err;
    for (i = 0; i < mlx5e_get_num_lag_ports(mdev); i++) {
    for (tc = 0; tc < MLX5_MAX_NUM_TC; tc++) {
    u32 in[MLX5_ST_SZ_DW(create_tis_in)] = {};
    void *tisc;
    tisc = MLX5_ADDR_OF(create_tis_in, in, ctx);
    MLX5_SET(tisc, tisc, prio, tc << 1);
    if (mlx5_lag_should_assign_affinity(mdev))
    MLX5_SET(tisc, tisc, lag_tx_port_affinity, i + 1);
    err = mlx5e_create_tis(mdev, in, &tisn[i][tc]);
    if (err)
    goto err_close_tises;
    }
    }
    return 0;
    err_close_tises:
    for (; i >= 0; i--) {
    for (tc--; tc >= 0; tc--)
    mlx5e_destroy_tis(mdev, tisn[i][tc]);
    tc = MLX5_MAX_NUM_TC;
    }
    return err;
    }
    static unsigned int
    mlx5e_get_devlink_param_num_doorbells(struct mlx5_core_dev *dev)
    {
    let mut param_id: u32 = DEVLINK_PARAM_GENERIC_ID_NUM_DOORBELLS;
    struct devlink *devlink = priv_to_devlink(dev);
    union devlink_param_value val;
    int err;
    err = devl_param_driverinit_value_get(devlink, param_id, &val);
    return err ? MLX5_DEFAULT_NUM_DOORBELLS : val.vu32;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_create_mdev_resources(mdev: *mut mlx5_core_dev, create_tises: bool) -> c_int {
    int mlx5e_create_mdev_resources(struct mlx5_core_dev *mdev, bool create_tises)
    {
    struct mlx5e_hw_objs *res = &mdev.mlx5e_res.hw_objs;
    unsigned int num_doorbells, i;
    int err;
    err = mlx5_core_alloc_pd(mdev, &res.pdn);
    if (err) {
    mlx5_core_err(mdev, "alloc pd failed, %d\n", err);
    return err;
    }
    err = mlx5_core_alloc_transport_domain(mdev, &res.td.tdn);
    if (err) {
    mlx5_core_err(mdev, "alloc td failed, %d\n", err);
    goto err_dealloc_pd;
    }
    err = mlx5e_create_mkey(mdev, res.pdn, &res.mkey);
    if (err) {
    mlx5_core_err(mdev, "create mkey failed, %d\n", err);
    goto err_dealloc_transport_domain;
    }
    num_doorbells = min(mlx5e_get_devlink_param_num_doorbells(mdev),
    mlx5e_get_max_num_channels(mdev));
    res.bfregs = kzalloc_objs(*res.bfregs, num_doorbells);
    if (!res.bfregs) {
    err = -ENOMEM;
    goto err_destroy_mkey;
    }
    for (i = 0; i < num_doorbells; i++) {
    err = mlx5_alloc_bfreg(mdev, res.bfregs + i, false, false);
    if (err) {
    mlx5_core_warn(mdev,
    "could only allocate %d/%d doorbells, err %d.\n",
    i, num_doorbells, err);
    break;
    }
    }
    res.num_bfregs = i;
    if (create_tises) {
    err = mlx5e_create_tises(mdev, res.tisn);
    if (err) {
    mlx5_core_err(mdev, "alloc tises failed, %d\n", err);
    goto err_destroy_bfregs;
    }
    res.tisn_valid = true;
    }
    INIT_LIST_HEAD(&res.td.tirs_list);
    mutex_init(&res.td.list_lock);
    mdev.mlx5e_res.dek_priv = mlx5_crypto_dek_init(mdev);
    if (IS_ERR(mdev.mlx5e_res.dek_priv)) {
    mlx5_core_err(mdev, "crypto dek init failed, %pe\n",
    mdev.mlx5e_res.dek_priv);
    mdev.mlx5e_res.dek_priv = core::ptr::null_mut();
    }
    return 0;
    err_destroy_bfregs:
    for (i = 0; i < res.num_bfregs; i++)
    mlx5_free_bfreg(mdev, res.bfregs + i);
    kfree(res.bfregs);
    err_destroy_mkey:
    mlx5_core_destroy_mkey(mdev, res.mkey);
    err_dealloc_transport_domain:
    mlx5_core_dealloc_transport_domain(mdev, res.td.tdn);
    err_dealloc_pd:
    mlx5_core_dealloc_pd(mdev, res.pdn);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5e_destroy_mdev_resources(mdev: *mut mlx5_core_dev) {
    void mlx5e_destroy_mdev_resources(struct mlx5_core_dev *mdev)
    {
    struct mlx5e_hw_objs *res = &mdev.mlx5e_res.hw_objs;
    mlx5_crypto_dek_cleanup(mdev.mlx5e_res.dek_priv);
    mdev.mlx5e_res.dek_priv = core::ptr::null_mut();
    if (res.tisn_valid)
    mlx5e_destroy_tises(mdev, res.tisn);
    for (unsigned int i = 0; i < res.num_bfregs; i++)
    mlx5_free_bfreg(mdev, res.bfregs + i);
    kfree(res.bfregs);
    mlx5_core_destroy_mkey(mdev, res.mkey);
    mlx5_core_dealloc_transport_domain(mdev, res.td.tdn);
    mlx5_core_dealloc_pd(mdev, res.pdn);
    memset(res, 0, sizeof(*res));
    }
    int mlx5e_modify_tirs_lb(struct mlx5_core_dev *mdev, bool enable_uc_lb,
    bool enable_mc_lb)
    {
    struct mlx5e_tir_builder *builder;
    struct mlx5e_tir *tir;
    let mut err: c_int = 0;
    builder = mlx5e_tir_builder_alloc(true);
    if (!builder)
    return -ENOMEM;
    mlx5e_tir_builder_build_self_lb_block(builder, enable_uc_lb,
    enable_mc_lb);
    mutex_lock(&mdev.mlx5e_res.hw_objs.td.list_lock);
    list_for_each_entry(tir, &mdev.mlx5e_res.hw_objs.td.tirs_list, list) {
    err = mlx5e_tir_modify(tir, builder);
    if (err) {
    mlx5_core_err(mdev,
    "modify tir(0x%x) enable_lb uc(%d) mc(%d) failed, %d\n",
    mlx5e_tir_get_tirn(tir),
    enable_uc_lb, enable_mc_lb, err);
    break;
    }
    }
    mutex_unlock(&mdev.mlx5e_res.hw_objs.td.list_lock);
    mlx5e_tir_builder_free(builder);
    return err;
    }
    int mlx5e_refresh_tirs(struct mlx5_core_dev *mdev, bool enable_uc_lb,
    bool enable_mc_lb)
    {
    if (MLX5_CAP_GEN(mdev, tis_tir_td_order))
    return 0; /* refresh not needed */
    return mlx5e_modify_tirs_lb(mdev, enable_uc_lb, enable_mc_lb);
    }
