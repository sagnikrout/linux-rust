//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/amdxdna_sysfs.c
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
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//

#[no_mangle]
unsafe extern "C" fn vbnv_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t vbnv_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct amdxdna_dev *xdna = dev_get_drvdata(dev);
    if (!xdna.vbnv)
    return sprintf(buf, "\n");
    return sprintf(buf, "%s\n", xdna.vbnv);
    }
    static DEVICE_ATTR_RO(vbnv);
#[no_mangle]
unsafe extern "C" fn device_type_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t device_type_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct amdxdna_dev *xdna = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", xdna.dev_info.device_type);
    }
    static DEVICE_ATTR_RO(device_type);
#[no_mangle]
unsafe extern "C" fn fw_version_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t fw_version_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct amdxdna_dev *xdna = dev_get_drvdata(dev);
    return sprintf(buf, "%d.%d.%d.%d\n", xdna.fw_ver.major,
    xdna.fw_ver.minor, xdna.fw_ver.sub,
    xdna.fw_ver.build);
    }
    static DEVICE_ATTR_RO(fw_version);
    static struct attribute *amdxdna_attrs[] = {
    &dev_attr_device_type.attr,
    &dev_attr_vbnv.attr,
    &dev_attr_fw_version.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group amdxdna_attr_group = {
    .attrs = amdxdna_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn amdxdna_sysfs_init(xdna: *mut amdxdna_dev) -> c_int {
    int amdxdna_sysfs_init(struct amdxdna_dev *xdna)
    {
    int ret;
    ret = sysfs_create_group(&xdna.ddev.dev.kobj, &amdxdna_attr_group);
    if (ret)
    XDNA_ERR(xdna, "Create attr group failed");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn amdxdna_sysfs_fini(xdna: *mut amdxdna_dev) {
    void amdxdna_sysfs_fini(struct amdxdna_dev *xdna)
    {
    sysfs_remove_group(&xdna.ddev.dev.kobj, &amdxdna_attr_group);
    }
