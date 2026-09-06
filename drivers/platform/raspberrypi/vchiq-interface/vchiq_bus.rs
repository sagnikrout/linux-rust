//! Automatically rewritten from C to Rust
//! Source: drivers/platform/raspberrypi/vchiq-interface/vchiq_bus.c
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
// vchiq_device.c - VCHIQ generic device and bus-type
//
// Copyright (c) 2023 Ideas On Board Oy
//

#[no_mangle]
unsafe extern "C" fn vchiq_bus_type_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int vchiq_bus_type_match(struct device *dev, const struct device_driver *drv)
    {
    if (dev.bus == &vchiq_bus_type &&
    strcmp(dev_name(dev), drv.name) == 0)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vchiq_bus_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int vchiq_bus_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct vchiq_device *device = container_of_const(dev, struct vchiq_device, dev);
    return add_uevent_var(env, "MODALIAS=vchiq:%s", dev_name(&device.dev));
    }
#[no_mangle]
unsafe extern "C" fn vchiq_bus_probe(dev: *mut device) -> c_int {
    static int vchiq_bus_probe(struct device *dev)
    {
    struct vchiq_device *device = to_vchiq_device(dev);
    struct vchiq_driver *driver = to_vchiq_driver(dev.driver);
    return driver.probe(device);
    }
#[no_mangle]
unsafe extern "C" fn vchiq_bus_remove(dev: *mut device) {
    static void vchiq_bus_remove(struct device *dev)
    {
    struct vchiq_device *device = to_vchiq_device(dev);
    struct vchiq_driver *driver = to_vchiq_driver(dev.driver);
    if (driver.remove)
    driver.remove(device);
    }
    const struct bus_type vchiq_bus_type = {
    .name   = "vchiq-bus",
    .match  = vchiq_bus_type_match,
    .uevent = vchiq_bus_uevent,
    .probe  = vchiq_bus_probe,
    .remove = vchiq_bus_remove,
    };
#[no_mangle]
unsafe extern "C" fn vchiq_device_release(dev: *mut device) {
    static void vchiq_device_release(struct device *dev)
    {
    struct vchiq_device *device = to_vchiq_device(dev);
    kfree(device);
    }
    struct vchiq_device *
    vchiq_device_register(struct device *parent, const char *name)
    {
    struct vchiq_device *device;
    int ret;
    device = kzalloc_obj(*device);
    if (!device)
    return core::ptr::null_mut();
    device.dev.init_name = name;
    device.dev.parent = parent;
    device.dev.bus = &vchiq_bus_type;
    device.dev.dma_mask = &device.dev.coherent_dma_mask;
    device.dev.release = vchiq_device_release;
    device.drv_mgmt = dev_get_drvdata(parent);
    of_dma_configure(&device.dev, parent.of_node, true);
    ret = device_register(&device.dev);
    if (ret) {
    dev_err(parent, "Cannot register %s: %d\n", name, ret);
    put_device(&device.dev);
    return core::ptr::null_mut();
    }
    return device;
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_device_unregister(vchiq_dev: *mut vchiq_device) {
    void vchiq_device_unregister(struct vchiq_device *vchiq_dev)
    {
    device_unregister(&vchiq_dev.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn vchiq_driver_register(vchiq_drv: *mut vchiq_driver) -> c_int {
    int vchiq_driver_register(struct vchiq_driver *vchiq_drv)
    {
    vchiq_drv.driver.bus = &vchiq_bus_type;
    return driver_register(&vchiq_drv.driver);
    }
    EXPORT_SYMBOL_GPL(vchiq_driver_register);
#[no_mangle]
pub unsafe extern "C" fn vchiq_driver_unregister(vchiq_drv: *mut vchiq_driver) {
    void vchiq_driver_unregister(struct vchiq_driver *vchiq_drv)
    {
    driver_unregister(&vchiq_drv.driver);
    }
    EXPORT_SYMBOL_GPL(vchiq_driver_unregister);
