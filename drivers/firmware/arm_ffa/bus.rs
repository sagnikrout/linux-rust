//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_ffa/bus.c
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
// Copyright (C) 2021 ARM Ltd.
//

    static DEFINE_IDA(ffa_bus_id);
#[no_mangle]
unsafe extern "C" fn ffa_device_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int ffa_device_match(struct device *dev, const struct device_driver *drv)
    {
    const struct ffa_device_id *id_table;
    struct ffa_device *ffa_dev;
    id_table = to_ffa_driver(drv).id_table;
    ffa_dev = to_ffa_dev(dev);
    if (!id_table)
    return 0;
    while (!uuid_is_null(&id_table.uuid)) {
//
// FF-A v1.0 doesn't provide discovery of UUIDs, just the
// partition IDs, so match it unconditionally here and handle
// it via the installed bus notifier during driver binding.
//
    if (uuid_is_null(&ffa_dev.uuid))
    return 1;
    if (uuid_equal(&ffa_dev.uuid, &id_table.uuid))
    return 1;
    id_table++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ffa_device_probe(dev: *mut device) -> c_int {
    static int ffa_device_probe(struct device *dev)
    {
    struct ffa_driver *ffa_drv = to_ffa_driver(dev.driver);
    struct ffa_device *ffa_dev = to_ffa_dev(dev);
// UUID can be still NULL with FF-A v1.0, so just skip probing them
    if (uuid_is_null(&ffa_dev.uuid))
    return -ENODEV;
    return ffa_drv.probe(ffa_dev);
    }
#[no_mangle]
unsafe extern "C" fn ffa_device_remove(dev: *mut device) {
    static void ffa_device_remove(struct device *dev)
    {
    struct ffa_driver *ffa_drv = to_ffa_driver(dev.driver);
    if (ffa_drv.remove)
    ffa_drv.remove(to_ffa_dev(dev));
    }
#[no_mangle]
unsafe extern "C" fn ffa_device_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int ffa_device_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct ffa_device *ffa_dev = to_ffa_dev(dev);
    return add_uevent_var(env, "MODALIAS=" FFA_UEVENT_MODALIAS_FMT,
    ffa_dev.vm_id, &ffa_dev.uuid);
    }
    static ssize_t modalias_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ffa_device *ffa_dev = to_ffa_dev(dev);
    return sysfs_emit(buf, FFA_UEVENT_MODALIAS_FMT, ffa_dev.vm_id,
    &ffa_dev.uuid);
    }
    static DEVICE_ATTR_RO(modalias);
    static ssize_t partition_id_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ffa_device *ffa_dev = to_ffa_dev(dev);
    return sprintf(buf, "0x%04x\n", ffa_dev.vm_id);
    }
    static DEVICE_ATTR_RO(partition_id);
    static ssize_t uuid_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ffa_device *ffa_dev = to_ffa_dev(dev);
    return sprintf(buf, "%pUb\n", &ffa_dev.uuid);
    }
    static DEVICE_ATTR_RO(uuid);
    static struct attribute *ffa_device_attributes_attrs[] = {
    &dev_attr_partition_id.attr,
    &dev_attr_uuid.attr,
    &dev_attr_modalias.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ffa_device_attributes);
    const struct bus_type ffa_bus_type = {
    .name		= "arm_ffa",
    .match		= ffa_device_match,
    .probe		= ffa_device_probe,
    .remove		= ffa_device_remove,
    .uevent		= ffa_device_uevent,
    .dev_groups	= ffa_device_attributes_groups,
    };
    EXPORT_SYMBOL_GPL(ffa_bus_type);
    int ffa_driver_register(struct ffa_driver *driver, struct module *owner,
    const char *mod_name)
    {
    int ret;
    if (!driver.probe || !driver.id_table)
    return -EINVAL;
    driver.driver.bus = &ffa_bus_type;
    driver.driver.name = driver.name;
    driver.driver.owner = owner;
    driver.driver.mod_name = mod_name;
    ret = driver_register(&driver.driver);
    if (!ret)
    pr_debug("registered new ffa driver %s\n", driver.name);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ffa_driver_register);
#[no_mangle]
pub unsafe extern "C" fn ffa_driver_unregister(driver: *mut ffa_driver) {
    void ffa_driver_unregister(struct ffa_driver *driver)
    {
    driver_unregister(&driver.driver);
    }
    EXPORT_SYMBOL_GPL(ffa_driver_unregister);
#[no_mangle]
unsafe extern "C" fn ffa_release_device(dev: *mut device) {
    static void ffa_release_device(struct device *dev)
    {
    struct ffa_device *ffa_dev = to_ffa_dev(dev);
    ida_free(&ffa_bus_id, ffa_dev.id);
    kfree(ffa_dev);
    }
#[no_mangle]
unsafe extern "C" fn __ffa_devices_unregister(dev: *mut device, data: *mut c_void) -> c_int {
    static int __ffa_devices_unregister(struct device *dev, void *data)
    {
    device_unregister(dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ffa_devices_unregister() {
    void ffa_devices_unregister(void)
    {
    bus_for_each_dev(&ffa_bus_type, core::ptr::null_mut(), core::ptr::null_mut(),
    __ffa_devices_unregister);
    }
    EXPORT_SYMBOL_GPL(ffa_devices_unregister);
#[no_mangle]
pub unsafe extern "C" fn ffa_device_is_valid(ffa_dev: *mut ffa_device) -> bool {
    bool ffa_device_is_valid(struct ffa_device *ffa_dev)
    {
    let mut valid: bool = false;
    struct device *dev = core::ptr::null_mut();
    struct ffa_device *tmp_dev;
    do {
    dev = bus_find_next_device(&ffa_bus_type, dev);
    tmp_dev = to_ffa_dev(dev);
    if (tmp_dev == ffa_dev) {
    valid = true;
    break;
    }
    put_device(dev);
    } while (dev);
    put_device(dev);
    return valid;
    }
    struct ffa_device *
    ffa_device_register(const struct ffa_partition_info *part_info,
    const struct ffa_ops *ops, struct device *parent)
    {
    int id, ret;
    struct device *dev;
    struct ffa_device *ffa_dev;
    if (!part_info)
    return core::ptr::null_mut();
    id = ida_alloc_min(&ffa_bus_id, 1, GFP_KERNEL);
    if (id < 0)
    return core::ptr::null_mut();
    ffa_dev = kzalloc_obj(*ffa_dev);
    if (!ffa_dev) {
    ida_free(&ffa_bus_id, id);
    return core::ptr::null_mut();
    }
    dev = &ffa_dev.dev;
    dev.parent = parent;
    dev.bus = &ffa_bus_type;
    dev.release = ffa_release_device;
    dev.dma_mask = &dev.coherent_dma_mask;
    dev_set_name(&ffa_dev.dev, "arm-ffa-%d", id);
    ffa_dev.id = id;
    ffa_dev.vm_id = part_info.id;
    ffa_dev.properties = part_info.properties;
    ffa_dev.ops = ops;
    uuid_copy(&ffa_dev.uuid, &part_info.uuid);
    ret = device_register(&ffa_dev.dev);
    if (ret) {
    dev_err(dev, "unable to register device %s err=%d\n",
    dev_name(dev), ret);
    put_device(dev);
    return core::ptr::null_mut();
    }
    return ffa_dev;
    }
    EXPORT_SYMBOL_GPL(ffa_device_register);
#[no_mangle]
pub unsafe extern "C" fn ffa_device_unregister(ffa_dev: *mut ffa_device) {
    void ffa_device_unregister(struct ffa_device *ffa_dev)
    {
    if (!ffa_dev)
    return;
    device_unregister(&ffa_dev.dev);
    }
    EXPORT_SYMBOL_GPL(ffa_device_unregister);
#[no_mangle]
unsafe extern "C" fn arm_ffa_bus_init() -> int __init {
    static int __init arm_ffa_bus_init(void)
    {
    return bus_register(&ffa_bus_type);
    }
    subsys_initcall(arm_ffa_bus_init);
#[no_mangle]
unsafe extern "C" fn arm_ffa_bus_exit() -> void __exit {
    static void __exit arm_ffa_bus_exit(void)
    {
    ffa_devices_unregister();
    bus_unregister(&ffa_bus_type);
    ida_destroy(&ffa_bus_id);
    }
    module_exit(arm_ffa_bus_exit);
    MODULE_ALIAS("ffa-core");
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("ARM FF-A bus");
    MODULE_LICENSE("GPL");
