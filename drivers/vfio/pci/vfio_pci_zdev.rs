//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/pci/vfio_pci_zdev.c
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
// VFIO ZPCI devices support
//
// Copyright (C) IBM Corp. 2020.  All rights reserved.
// Author(s): Pierre Morel <pmorel@linux.ibm.com>
// Matthew Rosato <mjrosato@linux.ibm.com>
//

//
// Add the Base PCI Function information to the device info region.
//
#[no_mangle]
unsafe extern "C" fn zpci_base_cap(zdev: *mut zpci_dev, caps: *mut vfio_info_cap) -> c_int {
    static int zpci_base_cap(struct zpci_dev *zdev, struct vfio_info_cap *caps)
    {
    struct vfio_device_info_cap_zpci_base cap = {
    .header.id = VFIO_DEVICE_INFO_CAP_ZPCI_BASE,
    .header.version = 3,
    .start_dma = zdev.start_dma,
    .end_dma = zdev.end_dma,
    .pchid = zdev.pchid,
    .vfn = zdev.vfn,
    .fmb_length = zdev.fmb_length,
    .pft = zdev.pft,
    .gid = zdev.pfgid,
    .fh = zdev.fh,
    .ccdf_err_length = sizeof(struct zpci_ccdf_err)
    };
    return vfio_info_add_capability(caps, &cap.header, sizeof(cap));
    }
//
// Add the Base PCI Function Group information to the device info region.
//
#[no_mangle]
unsafe extern "C" fn zpci_group_cap(zdev: *mut zpci_dev, caps: *mut vfio_info_cap) -> c_int {
    static int zpci_group_cap(struct zpci_dev *zdev, struct vfio_info_cap *caps)
    {
    struct vfio_device_info_cap_zpci_group cap = {
    .header.id = VFIO_DEVICE_INFO_CAP_ZPCI_GROUP,
    .header.version = 2,
    .dasm = zdev.dma_mask,
    .msi_addr = zdev.msi_addr,
    .flags = VFIO_DEVICE_INFO_ZPCI_FLAG_REFRESH,
    .mui = zdev.fmb_update,
    .noi = zdev.max_msi,
    .maxstbl = ZPCI_MAX_WRITE_SIZE,
    .version = zdev.version,
    .reserved = 0,
    .imaxstbl = zdev.maxstbl
    };
    return vfio_info_add_capability(caps, &cap.header, sizeof(cap));
    }
//
// Add the device utility string to the device info region.
//
#[no_mangle]
unsafe extern "C" fn zpci_util_cap(zdev: *mut zpci_dev, caps: *mut vfio_info_cap) -> c_int {
    static int zpci_util_cap(struct zpci_dev *zdev, struct vfio_info_cap *caps)
    {
    struct vfio_device_info_cap_zpci_util *cap;
    let mut cap_size: c_int = sizeof(*cap) + CLP_UTIL_STR_LEN;
    int ret;
    cap = kmalloc(cap_size, GFP_KERNEL);
    if (!cap)
    return -ENOMEM;
    cap.header.id = VFIO_DEVICE_INFO_CAP_ZPCI_UTIL;
    cap.header.version = 1;
    cap.size = CLP_UTIL_STR_LEN;
    memcpy(cap.util_str, zdev.util_str, cap.size);
    ret = vfio_info_add_capability(caps, &cap.header, cap_size);
    kfree(cap);
    return ret;
    }
//
// Add the function path string to the device info region.
//
#[no_mangle]
unsafe extern "C" fn zpci_pfip_cap(zdev: *mut zpci_dev, caps: *mut vfio_info_cap) -> c_int {
    static int zpci_pfip_cap(struct zpci_dev *zdev, struct vfio_info_cap *caps)
    {
    struct vfio_device_info_cap_zpci_pfip *cap;
    let mut cap_size: c_int = sizeof(*cap) + CLP_PFIP_NR_SEGMENTS;
    int ret;
    cap = kmalloc(cap_size, GFP_KERNEL);
    if (!cap)
    return -ENOMEM;
    cap.header.id = VFIO_DEVICE_INFO_CAP_ZPCI_PFIP;
    cap.header.version = 1;
    cap.size = CLP_PFIP_NR_SEGMENTS;
    memcpy(cap.pfip, zdev.pfip, cap.size);
    ret = vfio_info_add_capability(caps, &cap.header, cap_size);
    kfree(cap);
    return ret;
    }
//
// Add all supported capabilities to the VFIO_DEVICE_GET_INFO capability chain.
//
    int vfio_pci_info_zdev_add_caps(struct vfio_pci_core_device *vdev,
    struct vfio_info_cap *caps)
    {
    struct zpci_dev *zdev = to_zpci(vdev.pdev);
    int ret;
    if (!zdev)
    return -ENODEV;
    ret = zpci_base_cap(zdev, caps);
    if (ret)
    return ret;
    ret = zpci_group_cap(zdev, caps);
    if (ret)
    return ret;
    if (zdev.util_str_avail) {
    ret = zpci_util_cap(zdev, caps);
    if (ret)
    return ret;
    }
    ret = zpci_pfip_cap(zdev, caps);
    return ret;
    }
    int vfio_pci_zdev_feature_err(struct vfio_device *device, u32 flags,
    void __user *arg, size_t argsz)
    {
    struct vfio_device_feature_zpci_err err;
    struct vfio_pci_core_device *vdev;
    let mut ccdf: zpci_ccdf_err = {};
    struct zpci_dev *zdev;
    int ret;
    vdev = container_of(device, struct vfio_pci_core_device, vdev);
    zdev = to_zpci(vdev.pdev);
    if (!zdev)
    return -ENODEV;
    ret = vfio_check_feature(flags, argsz, VFIO_DEVICE_FEATURE_GET,
    sizeof(err));
    if (ret != 1)
    return ret;
    if (copy_from_user(&err, arg, sizeof(err)))
    return -EFAULT;
    if (!err.data)
    return -EINVAL;
    ret = zpci_get_pending_error(zdev, &ccdf);
    if (ret)
    return ret;
    if (copy_to_user(u64_to_user_ptr(err.data), &ccdf, sizeof(ccdf))) {
    dev_warn_ratelimited(device.dev,
    "Failed to handle PCI error event for PCI function 0x%x",
    zdev.fid);
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_pci_zdev_open_device(vdev: *mut vfio_pci_core_device) -> c_int {
    int vfio_pci_zdev_open_device(struct vfio_pci_core_device *vdev)
    {
    struct zpci_dev *zdev = to_zpci(vdev.pdev);
    int ret;
    if (!zdev)
    return -ENODEV;
    zpci_start_mediated_recovery(zdev);
    if (!vdev.vdev.kvm)
    return 0;
    ret = -ENOENT;
    if (zpci_kvm_hook.kvm_register)
    ret = zpci_kvm_hook.kvm_register(zdev, vdev.vdev.kvm);
    if (ret)
    zpci_stop_mediated_recovery(zdev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vfio_pci_zdev_close_device(vdev: *mut vfio_pci_core_device) {
    void vfio_pci_zdev_close_device(struct vfio_pci_core_device *vdev)
    {
    struct zpci_dev *zdev = to_zpci(vdev.pdev);
    if (!zdev)
    return;
    zpci_stop_mediated_recovery(zdev);
    if (!vdev.vdev.kvm)
    return;
    if (zpci_kvm_hook.kvm_unregister)
    zpci_kvm_hook.kvm_unregister(zdev);
    }
