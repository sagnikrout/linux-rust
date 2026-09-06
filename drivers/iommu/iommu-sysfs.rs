//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/iommu-sysfs.c
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
// IOMMU sysfs class support
//
// Copyright (C) 2014 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//

//
// We provide a common class "devices" group which initially has no attributes.
// As devices are added to the IOMMU, we'll add links to the group.
//
    static struct attribute *devices_attr[] = {
    core::ptr::null_mut(),
    };
    static const struct attribute_group devices_attr_group = {
    .name = "devices",
    .attrs = devices_attr,
    };
    static const struct attribute_group *dev_groups[] = {
    &devices_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn release_device(dev: *mut device) {
    static void release_device(struct device *dev)
    {
    kfree(dev);
    }
    static const struct class iommu_class = {
    .name = "iommu",
    .dev_release = release_device,
    .dev_groups = dev_groups,
    };
#[no_mangle]
unsafe extern "C" fn iommu_dev_init() -> int __init {
    static int __init iommu_dev_init(void)
    {
    return class_register(&iommu_class);
    }
    postcore_initcall(iommu_dev_init);
//
// Init the struct device for the IOMMU. IOMMU specific attributes can
// be provided as an attribute group, allowing a unique namespace per
// IOMMU type.
//
    int iommu_device_sysfs_add(struct iommu_device *iommu,
    struct device *parent,
    const struct attribute_group **groups,
    const char *fmt, ...)
    {
    va_list vargs;
    int ret;
    iommu.dev = kzalloc_obj(*iommu.dev);
    if (!iommu.dev)
    return -ENOMEM;
    device_initialize(iommu.dev);
    iommu.dev.class = &iommu_class;
    iommu.dev.parent = parent;
    iommu.dev.groups = groups;
    va_start(vargs, fmt);
    ret = kobject_set_name_vargs(&iommu.dev.kobj, fmt, vargs);
    va_end(vargs);
    if (ret)
    goto error;
    ret = device_add(iommu.dev);
    if (ret)
    goto error;
    dev_set_drvdata(iommu.dev, iommu);
    return 0;
    error:
    put_device(iommu.dev);
    return ret;
    }
    EXPORT_SYMBOL_GPL(iommu_device_sysfs_add);
#[no_mangle]
pub unsafe extern "C" fn iommu_device_sysfs_remove(iommu: *mut iommu_device) {
    void iommu_device_sysfs_remove(struct iommu_device *iommu)
    {
    dev_set_drvdata(iommu.dev, core::ptr::null_mut());
    device_unregister(iommu.dev);
    iommu.dev = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(iommu_device_sysfs_remove);
//
// IOMMU drivers can indicate a device is managed by a given IOMMU using
// this interface.  A link to the device will be created in the "devices"
// directory of the IOMMU device in sysfs and an "iommu" link will be
// created under the linked device, pointing back at the IOMMU device.
//
#[no_mangle]
pub unsafe extern "C" fn iommu_device_link(iommu: *mut iommu_device, link: *mut device) -> c_int {
    int iommu_device_link(struct iommu_device *iommu, struct device *link)
    {
    int ret;
    ret = sysfs_add_link_to_group(&iommu.dev.kobj, "devices",
    &link.kobj, dev_name(link));
    if (ret)
    return ret;
    ret = sysfs_create_link_nowarn(&link.kobj, &iommu.dev.kobj, "iommu");
    if (ret)
    sysfs_remove_link_from_group(&iommu.dev.kobj, "devices",
    dev_name(link));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn iommu_device_unlink(iommu: *mut iommu_device, link: *mut device) {
    void iommu_device_unlink(struct iommu_device *iommu, struct device *link)
    {
    sysfs_remove_link(&link.kobj, "iommu");
    sysfs_remove_link_from_group(&iommu.dev.kobj, "devices", dev_name(link));
    }
