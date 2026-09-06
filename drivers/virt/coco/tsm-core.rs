//! Automatically rewritten from C to Rust
//! Source: drivers/virt/coco/tsm-core.c
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
// Copyright(c) 2024-2025 Intel Corporation. All rights reserved.

    static void tsm_release(struct device *);
    static const struct class tsm_class = {
    .name		= "tsm",
    .dev_release	= tsm_release
    };
    static DEFINE_IDA(tsm_ida);
#[no_mangle]
unsafe extern "C" fn match_id(dev: *mut device, data: *const c_void) -> c_int {
    static int match_id(struct device *dev, const void *data)
    {
    struct tsm_dev *tsm_dev = container_of(dev, struct tsm_dev, dev);
    let mut id: c_int = *(const int *)data;
    return tsm_dev.id == id;
    }
    struct tsm_dev *find_tsm_dev(int id)
    {
    struct device *dev = class_find_device(&tsm_class, core::ptr::null_mut(), &id, match_id);
    if (!dev)
    return core::ptr::null_mut();
    return container_of(dev, struct tsm_dev, dev);
    }
    static struct tsm_dev *alloc_tsm_dev(struct device *parent)
    {
    struct device *dev;
    int id;
    struct tsm_dev *tsm_dev __free(kfree) =
    kzalloc_obj(*tsm_dev);
    if (!tsm_dev)
    return ERR_PTR(-ENOMEM);
    id = ida_alloc(&tsm_ida, GFP_KERNEL);
    if (id < 0)
    return ERR_PTR(id);
    tsm_dev.id = id;
    dev = &tsm_dev.dev;
    dev.parent = parent;
    dev.class = &tsm_class;
    device_initialize(dev);
    return no_free_ptr(tsm_dev);
    }
    static struct tsm_dev *tsm_register_pci_or_reset(struct tsm_dev *tsm_dev,
    struct pci_tsm_ops *pci_ops)
    {
    int rc;
    if (!pci_ops)
    return tsm_dev;
    tsm_dev.pci_ops = pci_ops;
    rc = pci_tsm_register(tsm_dev);
    if (rc) {
    dev_err(tsm_dev.dev.parent,
    "PCI/TSM registration failure: %d\n", rc);
    device_unregister(&tsm_dev.dev);
    return ERR_PTR(rc);
    }
// Notify TSM userspace that PCI/TSM operations are now possible
    kobject_uevent(&tsm_dev.dev.kobj, KOBJ_CHANGE);
    return tsm_dev;
    }
    struct tsm_dev *tsm_register(struct device *parent, struct pci_tsm_ops *pci_ops)
    {
    struct tsm_dev *tsm_dev __free(put_tsm_dev) = alloc_tsm_dev(parent);
    struct device *dev;
    int rc;
    if (IS_ERR(tsm_dev))
    return tsm_dev;
    dev = &tsm_dev.dev;
    rc = dev_set_name(dev, "tsm%d", tsm_dev.id);
    if (rc)
    return ERR_PTR(rc);
    rc = device_add(dev);
    if (rc)
    return ERR_PTR(rc);
    return tsm_register_pci_or_reset(no_free_ptr(tsm_dev), pci_ops);
    }
    EXPORT_SYMBOL_GPL(tsm_register);
#[no_mangle]
pub unsafe extern "C" fn tsm_unregister(tsm_dev: *mut tsm_dev) {
    void tsm_unregister(struct tsm_dev *tsm_dev)
    {
    if (tsm_dev.pci_ops)
    pci_tsm_unregister(tsm_dev);
    device_unregister(&tsm_dev.dev);
    }
    EXPORT_SYMBOL_GPL(tsm_unregister);
#[no_mangle]
unsafe extern "C" fn tsm_release(dev: *mut device) {
    static void tsm_release(struct device *dev)
    {
    struct tsm_dev *tsm_dev = container_of(dev, typeof(*tsm_dev), dev);
    ida_free(&tsm_ida, tsm_dev.id);
    kfree(tsm_dev);
    }
#[no_mangle]
unsafe extern "C" fn tsm_init() -> int __init {
    static int __init tsm_init(void)
    {
    return class_register(&tsm_class);
    }
    module_init(tsm_init)
#[no_mangle]
unsafe extern "C" fn tsm_exit() -> void __exit {
    static void __exit tsm_exit(void)
    {
    class_unregister(&tsm_class);
    }
    module_exit(tsm_exit)
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TEE Security Manager Class Device");
