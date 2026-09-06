//! Automatically rewritten from C to Rust
//! Source: drivers/peci/sysfs.c
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
// Copyright (c) 2021 Intel Corporation

#[no_mangle]
unsafe extern "C" fn rescan_controller(dev: *mut device, data: *mut c_void) -> c_int {
    static int rescan_controller(struct device *dev, void *data)
    {
    if (dev.type != &peci_controller_type)
    return 0;
    return peci_controller_scan_devices(to_peci_controller(dev));
    }
#[no_mangle]
unsafe extern "C" fn rescan_store(bus: *const bus_type, buf: *const c_char, count: usize) -> isize {
    static ssize_t rescan_store(const struct bus_type *bus, const char *buf, size_t count)
    {
    bool res;
    int ret;
    ret = kstrtobool(buf, &res);
    if (ret)
    return ret;
    if (!res)
    return count;
    ret = bus_for_each_dev(&peci_bus_type, core::ptr::null_mut(), core::ptr::null_mut(), rescan_controller);
    if (ret)
    return ret;
    return count;
    }
    static BUS_ATTR_WO(rescan);
    static struct attribute *peci_bus_attrs[] = {
    &bus_attr_rescan.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group peci_bus_group = {
    .attrs = peci_bus_attrs,
    };
    const struct attribute_group *peci_bus_groups[] = {
    &peci_bus_group,
    core::ptr::null_mut()
    };
    static ssize_t remove_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct peci_device *device = to_peci_device(dev);
    bool res;
    int ret;
    ret = kstrtobool(buf, &res);
    if (ret)
    return ret;
    if (res && device_remove_file_self(dev, attr))
    peci_device_destroy(device);
    return count;
    }
    static DEVICE_ATTR_IGNORE_LOCKDEP(remove, 0200, core::ptr::null_mut(), remove_store);
    static struct attribute *peci_device_attrs[] = {
    &dev_attr_remove.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group peci_device_group = {
    .attrs = peci_device_attrs,
    };
    const struct attribute_group *peci_device_groups[] = {
    &peci_device_group,
    core::ptr::null_mut()
    };
