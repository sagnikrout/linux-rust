//! Automatically rewritten from C to Rust
//! Source: drivers/counter/counter-core.c
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
// Generic Counter interface
// Copyright (C) 2020 William Breathitt Gray
//

// Provides a unique ID for each counter device
    static DEFINE_IDA(counter_ida);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counter_device_allochelper {
    pub counter: counter_device,
//
// This ensures private data behaves like if it were kmalloced
// separately. Also ensures the minimum alignment for safe DMA
// operations (which may or may not mean cache alignment).
//
    pub __aligned(ARCH_DMA_MINALIGN): unsigned long privdata[],
}

#[no_mangle]
unsafe extern "C" fn counter_device_release(dev: *mut device) {
    static void counter_device_release(struct device *dev)
    {
    struct counter_device *const counter =
    container_of(dev, struct counter_device, dev);
    counter_chrdev_remove(counter);
    ida_free(&counter_ida, dev.id);
    kfree(container_of(counter, struct counter_device_allochelper, counter));
    }
    static const struct device_type counter_device_type = {
    .name = "counter_device",
    .release = counter_device_release,
    };
    static const struct bus_type counter_bus_type = {
    .name = "counter",
    .dev_name = "counter",
    };
    static dev_t counter_devt;
//
// counter_priv - access counter device private data
// @counter: counter device
//
// Get the counter device private data
//
    void *counter_priv(const struct counter_device *const counter)
    {
    struct counter_device_allochelper *ch =
    container_of(counter, struct counter_device_allochelper, counter);
    return &ch.privdata;
    }
    EXPORT_SYMBOL_NS_GPL(counter_priv, "COUNTER");
//
// counter_alloc - allocate a counter_device
// @sizeof_priv: size of the driver private data
//
// This is part one of counter registration. The structure is allocated
// dynamically to ensure the right lifetime for the embedded struct device.
//
// If this succeeds, call counter_put() to get rid of the counter_device again.
//
    struct counter_device *counter_alloc(size_t sizeof_priv)
    {
    struct counter_device_allochelper *ch;
    struct counter_device *counter;
    struct device *dev;
    int err;
    ch = kzalloc(sizeof(*ch) + sizeof_priv, GFP_KERNEL);
    if (!ch)
    return core::ptr::null_mut();
    counter = &ch.counter;
    dev = &counter.dev;
// Acquire unique ID
    err = ida_alloc(&counter_ida, GFP_KERNEL);
    if (err < 0)
    goto err_ida_alloc;
    dev.id = err;
    mutex_init(&counter.ops_exist_lock);
    dev.type = &counter_device_type;
    dev.bus = &counter_bus_type;
    dev.devt = MKDEV(MAJOR(counter_devt), dev.id);
    err = counter_chrdev_add(counter);
    if (err < 0)
    goto err_chrdev_add;
    device_initialize(dev);
    err = dev_set_name(dev, COUNTER_NAME "%d", dev.id);
    if (err)
    goto err_dev_set_name;
    return counter;
    err_dev_set_name:
    put_device(dev);
    return core::ptr::null_mut();
    err_chrdev_add:
    ida_free(&counter_ida, dev.id);
    err_ida_alloc:
    kfree(ch);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_NS_GPL(counter_alloc, "COUNTER");
#[no_mangle]
pub unsafe extern "C" fn counter_put(counter: *mut counter_device) {
    void counter_put(struct counter_device *counter)
    {
    put_device(&counter.dev);
    }
    EXPORT_SYMBOL_NS_GPL(counter_put, "COUNTER");
//
// counter_add - complete registration of a counter
// @counter: the counter to add
//
// This is part two of counter registration.
//
// If this succeeds, call counter_unregister() to get rid of the counter_device again.
//
#[no_mangle]
pub unsafe extern "C" fn counter_add(counter: *mut counter_device) -> c_int {
    int counter_add(struct counter_device *counter)
    {
    int err;
    struct device *dev = &counter.dev;
    if (counter.parent) {
    dev.parent = counter.parent;
    dev.of_node = counter.parent.of_node;
    }
    err = counter_sysfs_add(counter);
    if (err < 0)
    return err;
// implies device_add(dev)
    return cdev_device_add(&counter.chrdev, dev);
    }
    EXPORT_SYMBOL_NS_GPL(counter_add, "COUNTER");
//
// counter_unregister - unregister Counter from the system
// @counter:	pointer to Counter to unregister
//
// The Counter is unregistered from the system.
//
#[no_mangle]
pub unsafe extern "C" fn counter_unregister(counter: *const *const counter_device) {
    void counter_unregister(struct counter_device *const counter)
    {
    if (!counter)
    return;
    cdev_device_del(&counter.chrdev, &counter.dev);
    mutex_lock(&counter.ops_exist_lock);
    counter.ops = core::ptr::null_mut();
    wake_up(&counter.events_wait);
    mutex_unlock(&counter.ops_exist_lock);
    }
    EXPORT_SYMBOL_NS_GPL(counter_unregister, "COUNTER");
#[no_mangle]
unsafe extern "C" fn devm_counter_release(counter: *mut c_void) {
    static void devm_counter_release(void *counter)
    {
    counter_unregister(counter);
    }
#[no_mangle]
unsafe extern "C" fn devm_counter_put(counter: *mut c_void) {
    static void devm_counter_put(void *counter)
    {
    counter_put(counter);
    }
//
// devm_counter_alloc - allocate a counter_device
// @dev: the device to register the release callback for
// @sizeof_priv: size of the driver private data
//
// This is the device managed version of counter_add(). It registers a cleanup
// callback to care for calling counter_put().
//
    struct counter_device *devm_counter_alloc(struct device *dev, size_t sizeof_priv)
    {
    struct counter_device *counter;
    int err;
    counter = counter_alloc(sizeof_priv);
    if (!counter)
    return core::ptr::null_mut();
    err = devm_add_action_or_reset(dev, devm_counter_put, counter);
    if (err < 0)
    return core::ptr::null_mut();
    return counter;
    }
    EXPORT_SYMBOL_NS_GPL(devm_counter_alloc, "COUNTER");
//
// devm_counter_add - complete registration of a counter
// @dev: the device to register the release callback for
// @counter: the counter to add
//
// This is the device managed version of counter_add(). It registers a cleanup
// callback to care for calling counter_unregister().
//
    int devm_counter_add(struct device *dev,
    struct counter_device *const counter)
    {
    int err;
    err = counter_add(counter);
    if (err < 0)
    return err;
    return devm_add_action_or_reset(dev, devm_counter_release, counter);
    }
    EXPORT_SYMBOL_NS_GPL(devm_counter_add, "COUNTER");
pub const COUNTER_DEV_MAX: c_int = 256;
#[no_mangle]
unsafe extern "C" fn counter_init() -> int __init {
    static int __init counter_init(void)
    {
    int err;
    err = bus_register(&counter_bus_type);
    if (err < 0)
    return err;
    err = alloc_chrdev_region(&counter_devt, 0, COUNTER_DEV_MAX,
    COUNTER_NAME);
    if (err < 0)
    goto err_unregister_bus;
    return 0;
    err_unregister_bus:
    bus_unregister(&counter_bus_type);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn counter_exit() -> void __exit {
    static void __exit counter_exit(void)
    {
    unregister_chrdev_region(counter_devt, COUNTER_DEV_MAX);
    bus_unregister(&counter_bus_type);
    }
    subsys_initcall(counter_init);
    module_exit(counter_exit);
    MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
    MODULE_DESCRIPTION("Generic Counter interface");
    MODULE_LICENSE("GPL v2");
