//! Automatically rewritten from C to Rust
//! Source: lib/kunit/device.c
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
// KUnit-managed device implementation
//
// Implementation of struct kunit_device helpers for fake devices whose
// lifecycle is managed by KUnit.
//
// Copyright (C) 2023, Google LLC.
// Author: David Gow <davidgow@google.com>
//

// Wrappers for use with kunit_add_action()
    KUNIT_DEFINE_ACTION_WRAPPER(device_unregister_wrapper, device_unregister, struct device *);
    KUNIT_DEFINE_ACTION_WRAPPER(driver_unregister_wrapper, driver_unregister, struct device_driver *);
// The root device for the KUnit bus, parent of all kunit_devices.
    static struct device *kunit_bus_device;
// A device owned by a KUnit test.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_device {
    pub dev: device,
// The KUnit test which owns this device.
    pub owner: *mut kunit,
// If the driver is managed by KUnit and unique to this device.
    pub driver: *const device_driver,
}

    static const struct bus_type kunit_bus_type = {
    .name		= "kunit",
    };
// Register the 'kunit_bus' used for fake devices.
#[no_mangle]
pub unsafe extern "C" fn kunit_bus_init() -> c_int {
    int kunit_bus_init(void)
    {
    int error;
    kunit_bus_device = root_device_register("kunit");
    if (IS_ERR(kunit_bus_device))
    return PTR_ERR(kunit_bus_device);
    error = bus_register(&kunit_bus_type);
    if (error)
    root_device_unregister(kunit_bus_device);
    return error;
    }
// Unregister the 'kunit_bus' in case the KUnit module is unloaded.
#[no_mangle]
pub unsafe extern "C" fn kunit_bus_shutdown() {
    void kunit_bus_shutdown(void)
    {
// Make sure the bus exists before we unregister it.
    if (IS_ERR_OR_NULL(kunit_bus_device))
    return;
    bus_unregister(&kunit_bus_type);
    root_device_unregister(kunit_bus_device);
    kunit_bus_device = core::ptr::null_mut();
    }
// Release a 'fake' KUnit device.
#[no_mangle]
unsafe extern "C" fn kunit_device_release(d: *mut device) {
    static void kunit_device_release(struct device *d)
    {
    kfree(to_kunit_device(d));
    }
//
// Create and register a KUnit-managed struct device_driver on the kunit_bus.
// Returns an error pointer on failure.
//
    struct device_driver *kunit_driver_create(struct kunit *test, const char *name)
    {
    struct device_driver *driver;
    let mut err: c_int = -ENOMEM;
    driver = kunit_kzalloc(test, sizeof(*driver), GFP_KERNEL);
    if (!driver)
    return ERR_PTR(err);
    driver.name = kunit_kstrdup_const(test, name, GFP_KERNEL);
    driver.bus = &kunit_bus_type;
    driver.owner = THIS_MODULE;
    err = driver_register(driver);
    if (err) {
    kunit_kfree(test, driver);
    return ERR_PTR(err);
    }
    kunit_add_action(test, driver_unregister_wrapper, driver);
    return driver;
    }
    EXPORT_SYMBOL_GPL(kunit_driver_create);
// Helper which creates a kunit_device, attaches it to the kunit_bus
    static struct kunit_device *kunit_device_register_internal(struct kunit *test,
    const char *name)
    {
    struct kunit_device *kunit_dev;
    let mut err: c_int = -ENOMEM;
    kunit_dev = kzalloc_obj(*kunit_dev);
    if (!kunit_dev)
    return ERR_PTR(err);
    kunit_dev.owner = test;
    err = dev_set_name(&kunit_dev.dev, "%s.%s", test.name, name);
    if (err) {
    kfree(kunit_dev);
    return ERR_PTR(err);
    }
    kunit_dev.dev.release = kunit_device_release;
    kunit_dev.dev.bus = &kunit_bus_type;
    kunit_dev.dev.parent = kunit_bus_device;
    err = device_register(&kunit_dev.dev);
    if (err) {
    put_device(&kunit_dev.dev);
    return ERR_PTR(err);
    }
    kunit_dev.dev.dma_mask = &kunit_dev.dev.coherent_dma_mask;
    kunit_dev.dev.coherent_dma_mask = DMA_BIT_MASK(32);
    kunit_add_action(test, device_unregister_wrapper, &kunit_dev.dev);
    return kunit_dev;
    }
//
// Create and register a new KUnit-managed device, using the user-supplied device_driver.
// On failure, returns an error pointer.
//
    struct device *kunit_device_register_with_driver(struct kunit *test,
    const char *name,
    const struct device_driver *drv)
    {
    struct kunit_device *kunit_dev = kunit_device_register_internal(test, name);
    if (IS_ERR_OR_NULL(kunit_dev))
    return ERR_CAST(kunit_dev);
    return &kunit_dev.dev;
    }
    EXPORT_SYMBOL_GPL(kunit_device_register_with_driver);
//
// Create and register a new KUnit-managed device, including a matching device_driver.
// On failure, returns an error pointer.
//
    struct device *kunit_device_register(struct kunit *test, const char *name)
    {
    struct device_driver *drv;
    struct kunit_device *dev;
    drv = kunit_driver_create(test, name);
    if (IS_ERR(drv))
    return ERR_CAST(drv);
    dev = kunit_device_register_internal(test, name);
    if (IS_ERR(dev)) {
    kunit_release_action(test, driver_unregister_wrapper, (void *)drv);
    return ERR_CAST(dev);
    }
// Request the driver be freed.
    dev.driver = drv;
    return &dev.dev;
    }
    EXPORT_SYMBOL_GPL(kunit_device_register);
// Unregisters a KUnit-managed device early (including the driver, if automatically created).
#[no_mangle]
pub unsafe extern "C" fn kunit_device_unregister(test: *mut kunit, dev: *mut device) {
    void kunit_device_unregister(struct kunit *test, struct device *dev)
    {
    const struct device_driver *driver = to_kunit_device(dev).driver;
    kunit_release_action(test, device_unregister_wrapper, dev);
    if (driver) {
    const char *driver_name = driver.name;
    kunit_release_action(test, driver_unregister_wrapper, (void *)driver);
    kunit_kfree_const(test, driver_name);
    }
    }
    EXPORT_SYMBOL_GPL(kunit_device_unregister);
