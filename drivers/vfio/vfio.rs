//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/vfio.h
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
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_file {
    pub device: *mut vfio_device,
    pub group: *mut vfio_group,
    pub access_granted: u8,
    pub /: *mut *mut u32 devid; / only valid when iommufd is valid,
    pub /: *mut *mut spinlock_t kvm_ref_lock; / protect kvm field,
    pub kvm: *mut kvm,
    pub /: *mut *mut *mut iommufd_ctx iommufd; / protected by vfio_device_set::lock,
}

extern "C" {
    pub fn vfio_device_put_registration(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_try_get_registration(device: *mut vfio_device) -> bool;
}
extern "C" {
    pub fn vfio_df_open(df: *mut vfio_device_file) -> c_int;
}
extern "C" {
    pub fn vfio_df_close(df: *mut vfio_device_file);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfio_group_type {
//
// Physical device with IOMMU backing.
//
    VFIO_IOMMU,

//
// Virtual device without IOMMU backing. The VFIO core fakes up an
// iommu_group as the iommu_group sysfs interface is part of the
// userspace ABI.  The user of these devices must not be able to
// directly trigger unmediated DMA.
//
    VFIO_EMULATED_IOMMU,

//
// Physical device without IOMMU backing. The VFIO core fakes up an
// iommu_group as the iommu_group sysfs interface is part of the
// userspace ABI.  Users can trigger unmediated DMA by the device,
// usage is highly dangerous, requires an explicit opt-in and will
// taint the kernel.
//
    VFIO_NO_IOMMU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_group {
    pub dev: device,
    pub cdev: cdev,
//
// When drivers is non-zero a driver is attached to the struct device
// that provided the iommu_group and thus the iommu_group is a valid
// pointer. When drivers is 0 the driver is being detached. Once users
// reaches 0 then the iommu_group is invalid.
//
    pub drivers: refcount_t,
    pub container_users: c_uint,
    pub iommu_group: *mut iommu_group,
    pub container: *mut vfio_container,
    pub device_list: list_head,
    pub device_lock: mutex,
    pub vfio_next: list_head,

    pub container_next: list_head,

    pub type: vfio_group_type,
    pub group_lock: mutex,
    pub kvm: *mut kvm,
    pub opened_file: *mut file,
    pub iommufd: *mut iommufd_ctx,
    pub kvm_ref_lock: spinlock_t,
    pub cdev_device_open_cnt: c_uint,
}

extern "C" {
    pub fn vfio_device_block_group(device: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_device_unblock_group(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_remove_group(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_group_register(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_group_unregister(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_group_use_iommu(device: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_device_group_unuse_iommu(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_df_group_close(df: *mut vfio_device_file);
}
extern "C" {
    pub fn vfio_group_enforced_coherent(group: *mut vfio_group) -> bool;
}
extern "C" {
    pub fn vfio_group_set_kvm(group: *mut vfio_group, kvm: *mut kvm);
}
extern "C" {
    pub fn vfio_device_has_container(device: *mut vfio_device) -> bool;
}
extern "C" {
    pub fn vfio_group_init() -> int __init;
}
extern "C" {
    pub fn vfio_group_cleanup();
}

//
// struct vfio_iommu_driver_ops - VFIO IOMMU driver callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_iommu_driver_ops {
    pub name: *mut c_char,
    pub owner: *mut module,
    pub arg): *mut *mut *mut void (open)(unsigned long,
    pub iommu_data): *mut *mut void (release)(void,
    pub arg): c_ulong,
    pub vfio_group_type): enum,
    pub group): *mut iommu_group,
    pub pages): *mut page,
    pub npage): dma_addr_t user_iova, int,
    pub vdev): *mut vfio_device,
    pub vdev): *mut vfio_device,
    pub write): *mut *mut void data, size_t count, bool,
    pub group): *mut iommu_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_iommu_driver {
    pub ops: *const vfio_iommu_driver_ops,
    pub vfio_next: list_head,
}

extern "C" {
    pub fn vfio_register_iommu_driver(ops: *const vfio_iommu_driver_ops) -> c_int;
}
extern "C" {
    pub fn vfio_unregister_iommu_driver(ops: *const vfio_iommu_driver_ops);
}
extern "C" {
    pub fn vfio_group_use_container(group: *mut vfio_group) -> c_int;
}
extern "C" {
    pub fn vfio_group_unuse_container(group: *mut vfio_group);
}
extern "C" {
    pub fn vfio_group_detach_container(group: *mut vfio_group);
}
extern "C" {
    pub fn vfio_device_container_register(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_container_unregister(device: *mut vfio_device);
}
extern "C" {
    pub fn vfio_container_init() -> int __init;
}
extern "C" {
    pub fn vfio_container_cleanup();
}

extern "C" {
    pub fn vfio_df_iommufd_bind(df: *mut vfio_device_file) -> c_int;
}
extern "C" {
    pub fn vfio_df_iommufd_unbind(df: *mut vfio_device_file);
}

extern "C" {
    pub fn vfio_init_device_cdev(device: *mut vfio_device);
}
extern "C" {
    pub fn cdev_device_add(_arg: &device->cdev, _arg: &device->device) -> return;
}
extern "C" {
    pub fn vfio_device_fops_cdev_open(inode: *mut inode, filep: *mut file) -> c_int;
}
extern "C" {
    pub fn vfio_df_unbind_iommufd(df: *mut vfio_device_file);
}
extern "C" {
    pub fn vfio_cdev_init() -> c_int;
}
extern "C" {
    pub fn vfio_cdev_cleanup();
}

extern "C" {
    pub fn device_add(_arg: &device->device) -> return;
}

extern "C" {
    pub fn vfio_virqfd_init() -> int __init;
}
extern "C" {
    pub fn vfio_virqfd_exit();
}

extern "C" {
    pub fn vfio_device_get_kvm_safe(device: *mut vfio_device, kvm: *mut kvm);
}
extern "C" {
    pub fn vfio_device_put_kvm(device: *mut vfio_device);
}

extern "C" {
    pub fn vfio_debugfs_create_root();
}
extern "C" {
    pub fn vfio_debugfs_remove_root();
}
extern "C" {
    pub fn vfio_device_debugfs_init(vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_device_debugfs_exit(vdev: *mut vfio_device);
}

