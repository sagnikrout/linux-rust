//! Automatically rewritten from C to Rust
//! Source: drivers/base/auxiliary_sysfs.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2024, NVIDIA CORPORATION & AFFILIATES
//

pub const AUXILIARY_MAX_IRQ_NAME: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auxiliary_irq_info {
    pub sysfs_attr: device_attribute,
    pub name: [c_char; AUXILIARY_MAX_IRQ_NAME],
}

    static struct attribute *auxiliary_irq_attrs[] = {
    core::ptr::null_mut()
    };
    static const struct attribute_group auxiliary_irqs_group = {
    .name = "irqs",
    .attrs = auxiliary_irq_attrs,
    };
#[no_mangle]
unsafe extern "C" fn auxiliary_irq_dir_prepare(auxdev: *mut auxiliary_device) -> c_int {
    static int auxiliary_irq_dir_prepare(struct auxiliary_device *auxdev)
    {
    let mut ret: c_int = 0;
    guard(mutex)(&auxdev.sysfs.lock);
    if (auxdev.sysfs.irq_dir_exists)
    return 0;
    ret = devm_device_add_group(&auxdev.dev, &auxiliary_irqs_group);
    if (ret)
    return ret;
    auxdev.sysfs.irq_dir_exists = true;
    xa_init(&auxdev.sysfs.irqs);
    return 0;
    }
//
// auxiliary_device_sysfs_irq_add - add a sysfs entry for the given IRQ
// @auxdev: auxiliary bus device to add the sysfs entry.
// @irq: The associated interrupt number.
//
// This function should be called after auxiliary device have successfully
// received the irq.
// The driver is responsible to add a unique irq for the auxiliary device. The
// driver can invoke this function from multiple thread context safely for
// unique irqs of the auxiliary devices. The driver must not invoke this API
// multiple times if the irq is already added previously.
//
// Return: zero on success or an error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn auxiliary_device_sysfs_irq_add(auxdev: *mut auxiliary_device, irq: c_int) -> c_int {
    int auxiliary_device_sysfs_irq_add(struct auxiliary_device *auxdev, int irq)
    {
    struct auxiliary_irq_info *info __free(kfree) = core::ptr::null_mut();
    struct device *dev = &auxdev.dev;
    int ret;
    ret = auxiliary_irq_dir_prepare(auxdev);
    if (ret)
    return ret;
    info = kzalloc_obj(*info);
    if (!info)
    return -ENOMEM;
    sysfs_attr_init(&info.sysfs_attr.attr);
    snprintf(info.name, AUXILIARY_MAX_IRQ_NAME, "%d", irq);
    ret = xa_insert(&auxdev.sysfs.irqs, irq, info, GFP_KERNEL);
    if (ret)
    return ret;
    info.sysfs_attr.attr.name = info.name;
    ret = sysfs_add_file_to_group(&dev.kobj, &info.sysfs_attr.attr,
    auxiliary_irqs_group.name);
    if (ret)
    goto sysfs_add_err;
    xa_store(&auxdev.sysfs.irqs, irq, no_free_ptr(info), GFP_KERNEL);
    return 0;
    sysfs_add_err:
    xa_erase(&auxdev.sysfs.irqs, irq);
    return ret;
    }
    EXPORT_SYMBOL_GPL(auxiliary_device_sysfs_irq_add);
//
// auxiliary_device_sysfs_irq_remove - remove a sysfs entry for the given IRQ
// @auxdev: auxiliary bus device to add the sysfs entry.
// @irq: the IRQ to remove.
//
// This function should be called to remove an IRQ sysfs entry.
// The driver must invoke this API when IRQ is released by the device.
//
#[no_mangle]
pub unsafe extern "C" fn auxiliary_device_sysfs_irq_remove(auxdev: *mut auxiliary_device, irq: c_int) {
    void auxiliary_device_sysfs_irq_remove(struct auxiliary_device *auxdev, int irq)
    {
    struct auxiliary_irq_info *info __free(kfree) = xa_load(&auxdev.sysfs.irqs, irq);
    struct device *dev = &auxdev.dev;
    if (!info) {
    dev_err(&auxdev.dev, "IRQ %d doesn't exist\n", irq);
    return;
    }
    sysfs_remove_file_from_group(&dev.kobj, &info.sysfs_attr.attr,
    auxiliary_irqs_group.name);
    xa_erase(&auxdev.sysfs.irqs, irq);
    }
    EXPORT_SYMBOL_GPL(auxiliary_device_sysfs_irq_remove);
