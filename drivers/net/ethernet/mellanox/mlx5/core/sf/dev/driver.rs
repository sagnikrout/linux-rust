//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/dev/driver.c
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
// Copyright (c) 2020 Mellanox Technologies Ltd

#[no_mangle]
unsafe extern "C" fn mlx5_core_peer_devlink_set(sf_dev: *mut mlx5_sf_dev, devlink: *mut devlink) -> c_int {
    static int mlx5_core_peer_devlink_set(struct mlx5_sf_dev *sf_dev, struct devlink *devlink)
    {
    struct mlx5_sf_peer_devlink_event_ctx event_ctx = {
    .fn_id = sf_dev.fn_id,
    .devlink = devlink,
    };
    int ret;
    ret = mlx5_blocking_notifier_call_chain(sf_dev.parent_mdev,
    MLX5_DRIVER_EVENT_SF_PEER_DEVLINK,
    &event_ctx);
    let mut ret: return = = NOTIFY_OK ? event_ctx.err : 0;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_dev_probe(adev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int mlx5_sf_dev_probe(struct auxiliary_device *adev, const struct auxiliary_device_id *id)
    {
    struct mlx5_sf_dev *sf_dev = container_of(adev, struct mlx5_sf_dev, adev);
    struct mlx5_core_dev *mdev;
    struct devlink *devlink;
    int err;
    devlink = mlx5_devlink_alloc(&adev.dev);
    if (!devlink)
    return -ENOMEM;
    mdev = devlink_priv(devlink);
    mdev.device = &adev.dev;
    mdev.pdev = sf_dev.parent_mdev.pdev;
    mdev.bar_addr = sf_dev.bar_base_addr;
    mdev.coredev_type = MLX5_COREDEV_SF;
    mdev.priv.parent_mdev = sf_dev.parent_mdev;
    mdev.priv.adev_idx = adev.id;
    sf_dev.mdev = mdev;
// Only local SFs do light probe
    if (MLX5_ESWITCH_MANAGER(sf_dev.parent_mdev))
    mlx5_dev_set_lightweight(mdev);
    err = mlx5_mdev_init(mdev, MLX5_SF_PROF);
    if (err) {
    mlx5_core_warn(mdev, "mlx5_mdev_init on err=%d\n", err);
    goto mdev_err;
    }
    mdev.iseg = ioremap(mdev.bar_addr, sizeof(*mdev.iseg));
    if (!mdev.iseg) {
    mlx5_core_warn(mdev, "remap error\n");
    err = -ENOMEM;
    goto remap_err;
    }
// Peer devlink logic expects to work on unregistered devlink instance.
    err = mlx5_core_peer_devlink_set(sf_dev, devlink);
    if (err) {
    mlx5_core_warn(mdev, "mlx5_core_peer_devlink_set err=%d\n", err);
    goto peer_devlink_set_err;
    }
    if (MLX5_ESWITCH_MANAGER(sf_dev.parent_mdev))
    err = mlx5_init_one_light(mdev);
    else
    err = mlx5_init_one(mdev);
    if (err) {
    mlx5_core_warn(mdev, "mlx5_init_one err=%d\n", err);
    goto init_one_err;
    }
    mlx5_vhca_debugfs_init(mdev);
    return 0;
    init_one_err:
    peer_devlink_set_err:
    iounmap(mdev.iseg);
    remap_err:
    mlx5_mdev_uninit(mdev);
    mdev_err:
    mlx5_devlink_free(devlink);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_dev_remove(adev: *mut auxiliary_device) {
    static void mlx5_sf_dev_remove(struct auxiliary_device *adev)
    {
    struct mlx5_sf_dev *sf_dev = container_of(adev, struct mlx5_sf_dev, adev);
    struct mlx5_core_dev *mdev = sf_dev.mdev;
    struct devlink *devlink;
    devlink = priv_to_devlink(mdev);
    set_bit(MLX5_BREAK_FW_WAIT, &mdev.intf_state);
    mlx5_drain_health_wq(mdev);
    if (mlx5_dev_is_lightweight(mdev))
    mlx5_uninit_one_light(mdev);
    else
    mlx5_uninit_one(mdev);
    iounmap(mdev.iseg);
    mlx5_mdev_uninit(mdev);
    mlx5_devlink_free(devlink);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_sf_dev_shutdown(adev: *mut auxiliary_device) {
    static void mlx5_sf_dev_shutdown(struct auxiliary_device *adev)
    {
    struct mlx5_sf_dev *sf_dev = container_of(adev, struct mlx5_sf_dev, adev);
    struct mlx5_core_dev *mdev = sf_dev.mdev;
    set_bit(MLX5_BREAK_FW_WAIT, &mdev.intf_state);
    mlx5_drain_health_wq(mdev);
    mlx5_unload_one(mdev, false);
    }
    static const struct auxiliary_device_id mlx5_sf_dev_id_table[] = {
    { .name = MLX5_ADEV_NAME "." MLX5_SF_DEV_ID_NAME, },
    { },
    };
    MODULE_DEVICE_TABLE(auxiliary, mlx5_sf_dev_id_table);
    static struct auxiliary_driver mlx5_sf_driver = {
    .name = MLX5_SF_DEV_ID_NAME,
    .probe = mlx5_sf_dev_probe,
    .remove = mlx5_sf_dev_remove,
    .shutdown = mlx5_sf_dev_shutdown,
    .id_table = mlx5_sf_dev_id_table,
    };
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_driver_register() -> c_int {
    int mlx5_sf_driver_register(void)
    {
    return auxiliary_driver_register(&mlx5_sf_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_sf_driver_unregister() {
    void mlx5_sf_driver_unregister(void)
    {
    auxiliary_driver_unregister(&mlx5_sf_driver);
    }
