//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/platform/vfio_amba.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 - Virtual Open Systems
// Author: Antonios Motakis <a.motakis@virtualopensystems.com>
//

// probing devices from the AMBA bus
    static struct resource *get_amba_resource(struct vfio_platform_device *vdev,
    int i)
    {
    struct amba_device *adev = (struct amba_device *) vdev.opaque;
    if (i == 0)
    return &adev.res;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn get_amba_irq(vdev: *mut vfio_platform_device, i: c_int) -> c_int {
    static int get_amba_irq(struct vfio_platform_device *vdev, int i)
    {
    struct amba_device *adev = (struct amba_device *) vdev.opaque;
    let mut ret: c_int = 0;
    if (i < AMBA_NR_IRQS)
    ret = adev.irq[i];
// zero is an unset IRQ for AMBA devices
    return ret ? ret : -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn vfio_amba_init_dev(core_vdev: *mut vfio_device) -> c_int {
    static int vfio_amba_init_dev(struct vfio_device *core_vdev)
    {
    struct vfio_platform_device *vdev =
    container_of(core_vdev, struct vfio_platform_device, vdev);
    struct amba_device *adev = to_amba_device(core_vdev.dev);
    int ret;
    vdev.name = kasprintf(GFP_KERNEL, "vfio-amba-%08x", adev.periphid);
    if (!vdev.name)
    return -ENOMEM;
    vdev.opaque = (void *) adev;
    vdev.flags = VFIO_DEVICE_FLAGS_AMBA;
    vdev.get_resource = get_amba_resource;
    vdev.get_irq = get_amba_irq;
    vdev.reset_required = false;
    ret = vfio_platform_init_common(vdev);
    if (ret)
    kfree(vdev.name);
    return ret;
    }
    static const struct vfio_device_ops vfio_amba_ops;
#[no_mangle]
unsafe extern "C" fn vfio_amba_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int vfio_amba_probe(struct amba_device *adev, const struct amba_id *id)
    {
    struct vfio_platform_device *vdev;
    int ret;
    dev_err_once(&adev.dev, "DEPRECATION: vfio-amba is deprecated and will be removed in a future kernel release\n");
    vdev = vfio_alloc_device(vfio_platform_device, vdev, &adev.dev,
    &vfio_amba_ops);
    if (IS_ERR(vdev))
    return PTR_ERR(vdev);
    ret = vfio_register_group_dev(&vdev.vdev);
    if (ret)
    goto out_put_vdev;
    pm_runtime_enable(&adev.dev);
    dev_set_drvdata(&adev.dev, vdev);
    return 0;
    out_put_vdev:
    vfio_put_device(&vdev.vdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vfio_amba_release_dev(core_vdev: *mut vfio_device) {
    static void vfio_amba_release_dev(struct vfio_device *core_vdev)
    {
    struct vfio_platform_device *vdev =
    container_of(core_vdev, struct vfio_platform_device, vdev);
    vfio_platform_release_common(vdev);
    kfree(vdev.name);
    }
#[no_mangle]
unsafe extern "C" fn vfio_amba_remove(adev: *mut amba_device) {
    static void vfio_amba_remove(struct amba_device *adev)
    {
    struct vfio_platform_device *vdev = dev_get_drvdata(&adev.dev);
    vfio_unregister_group_dev(&vdev.vdev);
    pm_runtime_disable(vdev.device);
    vfio_put_device(&vdev.vdev);
    }
    static const struct vfio_device_ops vfio_amba_ops = {
    .name		= "vfio-amba",
    .init		= vfio_amba_init_dev,
    .release	= vfio_amba_release_dev,
    .open_device	= vfio_platform_open_device,
    .close_device	= vfio_platform_close_device,
    .ioctl		= vfio_platform_ioctl,
    .get_region_info_caps = vfio_platform_ioctl_get_region_info,
    .read		= vfio_platform_read,
    .write		= vfio_platform_write,
    .mmap		= vfio_platform_mmap,
    .bind_iommufd	= vfio_iommufd_physical_bind,
    .unbind_iommufd	= vfio_iommufd_physical_unbind,
    .attach_ioas	= vfio_iommufd_physical_attach_ioas,
    .detach_ioas	= vfio_iommufd_physical_detach_ioas,
    };
    static const struct amba_id vfio_amba_ids[] = {
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, vfio_amba_ids);
    static struct amba_driver vfio_amba_driver = {
    .probe = vfio_amba_probe,
    .remove = vfio_amba_remove,
    .id_table = vfio_amba_ids,
    .drv = {
    .name = "vfio-amba",
    },
    .driver_managed_dma = true,
    };
    module_amba_driver(vfio_amba_driver);
    MODULE_VERSION(DRIVER_VERSION);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
