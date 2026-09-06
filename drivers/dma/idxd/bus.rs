//! Automatically rewritten from C to Rust
//! Source: drivers/dma/idxd/bus.c
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
// Copyright(c) 2021 Intel Corporation. All rights rsvd.

    int __idxd_driver_register(struct idxd_device_driver *idxd_drv, struct module *owner,
    const char *mod_name)
    {
    struct device_driver *drv = &idxd_drv.drv;
    if (!idxd_drv.type) {
    pr_debug("driver type not set (%ps)\n", __builtin_return_address(0));
    return -EINVAL;
    }
    drv.name = idxd_drv.name;
    drv.bus = &dsa_bus_type;
    drv.owner = owner;
    drv.mod_name = mod_name;
    return driver_register(drv);
    }
    EXPORT_SYMBOL_GPL(__idxd_driver_register);
#[no_mangle]
pub unsafe extern "C" fn idxd_driver_unregister(idxd_drv: *mut idxd_device_driver) {
    void idxd_driver_unregister(struct idxd_device_driver *idxd_drv)
    {
    driver_unregister(&idxd_drv.drv);
    }
    EXPORT_SYMBOL_GPL(idxd_driver_unregister);
    static int idxd_config_bus_match(struct device *dev,
    const struct device_driver *drv)
    {
    const struct idxd_device_driver *idxd_drv =
    container_of_const(drv, struct idxd_device_driver, drv);
    struct idxd_dev *idxd_dev = confdev_to_idxd_dev(dev);
    let mut i: c_int = 0;
    while (idxd_drv.type[i] != IDXD_DEV_NONE) {
    if (idxd_dev.type == idxd_drv.type[i])
    return 1;
    i++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idxd_config_bus_probe(dev: *mut device) -> c_int {
    static int idxd_config_bus_probe(struct device *dev)
    {
    struct idxd_device_driver *idxd_drv =
    container_of(dev.driver, struct idxd_device_driver, drv);
    struct idxd_dev *idxd_dev = confdev_to_idxd_dev(dev);
    return idxd_drv.probe(idxd_dev);
    }
#[no_mangle]
unsafe extern "C" fn idxd_config_bus_remove(dev: *mut device) {
    static void idxd_config_bus_remove(struct device *dev)
    {
    struct idxd_device_driver *idxd_drv =
    container_of(dev.driver, struct idxd_device_driver, drv);
    struct idxd_dev *idxd_dev = confdev_to_idxd_dev(dev);
    idxd_drv.remove(idxd_dev);
    }
#[no_mangle]
unsafe extern "C" fn idxd_bus_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int idxd_bus_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    return add_uevent_var(env, "MODALIAS=" IDXD_DEVICES_MODALIAS_FMT, 0);
    }
    const struct bus_type dsa_bus_type = {
    .name = "dsa",
    .match = idxd_config_bus_match,
    .probe = idxd_config_bus_probe,
    .remove = idxd_config_bus_remove,
    .uevent = idxd_bus_uevent,
    };
    EXPORT_SYMBOL_GPL(dsa_bus_type);
#[no_mangle]
unsafe extern "C" fn dsa_bus_init() -> int __init {
    static int __init dsa_bus_init(void)
    {
    return bus_register(&dsa_bus_type);
    }
    module_init(dsa_bus_init);
#[no_mangle]
unsafe extern "C" fn dsa_bus_exit() -> void __exit {
    static void __exit dsa_bus_exit(void)
    {
    bus_unregister(&dsa_bus_type);
    }
    module_exit(dsa_bus_exit);
    MODULE_DESCRIPTION("IDXD driver dsa_bus_type driver");
    MODULE_LICENSE("GPL v2");
