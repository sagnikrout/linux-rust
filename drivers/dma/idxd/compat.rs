//! Automatically rewritten from C to Rust
//! Source: drivers/dma/idxd/compat.c
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

    extern void device_driver_detach(struct device *dev);

    struct driver_attribute driver_attr_##_name =		\
    __ATTR_IGNORE_LOCKDEP(_name, _mode, _show, _store)
#[no_mangle]
unsafe extern "C" fn unbind_store(drv: *mut device_driver, buf: *const c_char, count: usize) -> isize {
    static ssize_t unbind_store(struct device_driver *drv, const char *buf, size_t count)
    {
    const struct bus_type *bus = drv.bus;
    struct device *dev;
    let mut rc: c_int = -ENODEV;
    dev = bus_find_device_by_name(bus, core::ptr::null_mut(), buf);
    if (!dev)
    return -ENODEV;
    if (dev.driver) {
    device_driver_detach(dev);
    rc = count;
    }
    put_device(dev);
    return rc;
    }
    static DRIVER_ATTR_IGNORE_LOCKDEP(unbind, 0200, core::ptr::null_mut(), unbind_store);
#[no_mangle]
unsafe extern "C" fn bind_store(drv: *mut device_driver, buf: *const c_char, count: usize) -> isize {
    static ssize_t bind_store(struct device_driver *drv, const char *buf, size_t count)
    {
    const struct bus_type *bus = drv.bus;
    struct device *dev;
    struct device_driver *alt_drv = core::ptr::null_mut();
    let mut rc: c_int = -ENODEV;
    struct idxd_dev *idxd_dev;
    dev = bus_find_device_by_name(bus, core::ptr::null_mut(), buf);
    if (!dev)
    return -ENODEV;
    if (dev.driver || drv != &dsa_drv.drv)
    goto err_put_dev;
    idxd_dev = confdev_to_idxd_dev(dev);
    if (is_idxd_dev(idxd_dev)) {
    alt_drv = driver_find("idxd", bus);
    } else if (is_idxd_wq_dev(idxd_dev)) {
    struct idxd_wq *wq = confdev_to_wq(dev);
    if (is_idxd_wq_kernel(wq))
    alt_drv = driver_find("dmaengine", bus);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_idxd_wq_user(wq)) -> else {
    else if (is_idxd_wq_user(wq))
    alt_drv = driver_find("user", bus);
    }
    if (!alt_drv)
    goto err_put_dev;
    rc = device_driver_attach(alt_drv, dev);
    if (rc < 0)
    goto err_put_dev;
    put_device(dev);
    return count;
    err_put_dev:
    put_device(dev);
    return rc;
    }
    static DRIVER_ATTR_IGNORE_LOCKDEP(bind, 0200, core::ptr::null_mut(), bind_store);
    static struct attribute *dsa_drv_compat_attrs[] = {
    &driver_attr_bind.attr,
    &driver_attr_unbind.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group dsa_drv_compat_attr_group = {
    .attrs = dsa_drv_compat_attrs,
    };
    static const struct attribute_group *dsa_drv_compat_groups[] = {
    &dsa_drv_compat_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn idxd_dsa_drv_probe(idxd_dev: *mut idxd_dev) -> c_int {
    static int idxd_dsa_drv_probe(struct idxd_dev *idxd_dev)
    {
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn idxd_dsa_drv_remove(idxd_dev: *mut idxd_dev) {
    static void idxd_dsa_drv_remove(struct idxd_dev *idxd_dev)
    {
    }
    static enum idxd_dev_type dev_types[] = {
    IDXD_DEV_NONE,
    };
    struct idxd_device_driver dsa_drv = {
    .name = "dsa",
    .probe = idxd_dsa_drv_probe,
    .remove = idxd_dsa_drv_remove,
    .type = dev_types,
    .drv = {
    .suppress_bind_attrs = true,
    .groups = dsa_drv_compat_groups,
    },
    };
    module_idxd_driver(dsa_drv);
    MODULE_IMPORT_NS("IDXD");
