//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/mgb4/mgb4_sysfs_pci.c
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
//
// Copyright (C) 2021-2022 Digiteq Automotive
// author: Martin Tuma <martin.tuma@digiteqautomotive.com>
//
// This module handles all the sysfs info/configuration that is related to the
// PCI card device.
//

    static ssize_t module_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mgb4_dev *mgbdev = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", mgbdev.module_version & 0x0F);
    }
    static ssize_t module_type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mgb4_dev *mgbdev = dev_get_drvdata(dev);
    return sprintf(buf, "%u\n", mgbdev.module_version >> 4);
    }
    static ssize_t fw_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mgb4_dev *mgbdev = dev_get_drvdata(dev);
    let mut config: u32 = mgb4_read_reg(&mgbdev.video, 0xC4);
    return sprintf(buf, "%u\n", config & 0xFFFF);
    }
    static ssize_t fw_type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mgb4_dev *mgbdev = dev_get_drvdata(dev);
    let mut config: u32 = mgb4_read_reg(&mgbdev.video, 0xC4);
    return sprintf(buf, "%u\n", config >> 24);
    }
    static ssize_t serial_number_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct mgb4_dev *mgbdev = dev_get_drvdata(dev);
    let mut sn: u32 = mgbdev.serial_number;
    return sprintf(buf, "%03d-%03d-%03d-%03d\n", sn >> 24, (sn >> 16) & 0xFF,
    (sn >> 8) & 0xFF, sn & 0xFF);
    }
    static DEVICE_ATTR_RO(module_version);
    static DEVICE_ATTR_RO(module_type);
    static DEVICE_ATTR_RO(fw_version);
    static DEVICE_ATTR_RO(fw_type);
    static DEVICE_ATTR_RO(serial_number);
    struct attribute *mgb4_pci_attrs[] = {
    &dev_attr_module_type.attr,
    &dev_attr_module_version.attr,
    &dev_attr_fw_type.attr,
    &dev_attr_fw_version.attr,
    &dev_attr_serial_number.attr,
    core::ptr::null_mut()
    };
