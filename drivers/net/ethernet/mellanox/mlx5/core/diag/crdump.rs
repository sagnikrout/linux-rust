//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/diag/crdump.c
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
// Copyright (c) 2019 Mellanox Technologies

pub const BAD_ACCESS: c_uint = 0xBADACCE5;
pub const MLX5_PROTECTED_CR_SCAN_CRSPACE: c_uint = 0x7;
#[no_mangle]
unsafe extern "C" fn mlx5_crdump_enabled(dev: *mut mlx5_core_dev) -> bool {
    static bool mlx5_crdump_enabled(struct mlx5_core_dev *dev)
    {
    return !!dev.priv.health.crdump_size;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_crdump_fill(dev: *mut mlx5_core_dev, cr_data: *mut u32) -> c_int {
    static int mlx5_crdump_fill(struct mlx5_core_dev *dev, u32 *cr_data)
    {
    let mut crdump_size: u32 = dev.priv.health.crdump_size;
    int i, ret;
    for (i = 0; i < (crdump_size / 4); i++)
    cr_data[i] = BAD_ACCESS;
    ret = mlx5_vsc_gw_read_block_fast(dev, cr_data, crdump_size);
    if (ret <= 0) {
    if (ret == 0)
    return -EIO;
    return ret;
    }
    if (crdump_size != ret) {
    mlx5_core_warn(dev, "failed to read full dump, read %d out of %u\n",
    ret, crdump_size);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_crdump_collect(dev: *mut mlx5_core_dev, cr_data: *mut u32) -> c_int {
    int mlx5_crdump_collect(struct mlx5_core_dev *dev, u32 *cr_data)
    {
    int ret;
    if (!mlx5_crdump_enabled(dev))
    return -ENODEV;
    ret = mlx5_vsc_gw_lock(dev);
    if (ret) {
    mlx5_core_warn(dev, "crdump: failed to lock vsc gw err %d\n",
    ret);
    return ret;
    }
// Verify no other PF is running cr-dump or sw reset
    ret = mlx5_vsc_sem_set_space(dev, MLX5_SEMAPHORE_SW_RESET,
    MLX5_VSC_LOCK);
    if (ret) {
    if (ret == -EBUSY)
    mlx5_core_info(dev, "SW reset semaphore is already in use\n");
    else
    mlx5_core_warn(dev, "Failed to lock SW reset semaphore\n");
    goto unlock_gw;
    }
    ret = mlx5_vsc_gw_set_space(dev, MLX5_VSC_SPACE_SCAN_CRSPACE, core::ptr::null_mut());
    if (ret)
    goto unlock_sem;
    ret = mlx5_crdump_fill(dev, cr_data);
    unlock_sem:
    mlx5_vsc_sem_set_space(dev, MLX5_SEMAPHORE_SW_RESET, MLX5_VSC_UNLOCK);
    unlock_gw:
    mlx5_vsc_gw_unlock(dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_crdump_enable(dev: *mut mlx5_core_dev) -> c_int {
    int mlx5_crdump_enable(struct mlx5_core_dev *dev)
    {
    struct mlx5_priv *priv = &dev.priv;
    u32 space_size;
    int ret;
    if (!mlx5_core_is_pf(dev) || !mlx5_vsc_accessible(dev) ||
    mlx5_crdump_enabled(dev))
    return 0;
    ret = mlx5_vsc_gw_lock(dev);
    if (ret)
    return ret;
// Check if space is supported and get space size
    ret = mlx5_vsc_gw_set_space(dev, MLX5_VSC_SPACE_SCAN_CRSPACE,
    &space_size);
    if (ret) {
// Unlock and mask error since space is not supported
    mlx5_vsc_gw_unlock(dev);
    return 0;
    }
    if (!space_size) {
    mlx5_core_warn(dev, "Invalid Crspace size, zero\n");
    mlx5_vsc_gw_unlock(dev);
    return -EINVAL;
    }
    ret = mlx5_vsc_gw_unlock(dev);
    if (ret)
    return ret;
    priv.health.crdump_size = space_size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_crdump_disable(dev: *mut mlx5_core_dev) {
    void mlx5_crdump_disable(struct mlx5_core_dev *dev)
    {
    dev.priv.health.crdump_size = 0;
    }
