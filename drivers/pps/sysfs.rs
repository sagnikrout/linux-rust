//! Automatically rewritten from C to Rust
//! Source: drivers/pps/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PPS sysfs support
//
// Copyright (C) 2007-2009   Rodolfo Giometti <giometti@linux.it>
//

//
// Attribute functions
//
    static ssize_t assert_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    if (!(pps.info.mode & PPS_CAPTUREASSERT))
    return 0;
    return sprintf(buf, "%lld.%09d#%d\n",
    (long long) pps.assert_tu.sec, pps.assert_tu.nsec,
    pps.assert_sequence);
    }
    static DEVICE_ATTR_RO(assert);
    static ssize_t clear_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    if (!(pps.info.mode & PPS_CAPTURECLEAR))
    return 0;
    return sprintf(buf, "%lld.%09d#%d\n",
    (long long) pps.clear_tu.sec, pps.clear_tu.nsec,
    pps.clear_sequence);
    }
    static DEVICE_ATTR_RO(clear);
    static ssize_t mode_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    return sprintf(buf, "%4x\n", pps.info.mode);
    }
    static DEVICE_ATTR_RO(mode);
    static ssize_t echo_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", !!pps.info.echo);
    }
    static DEVICE_ATTR_RO(echo);
    static ssize_t name_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    return sprintf(buf, "%s\n", pps.info.name);
    }
    static DEVICE_ATTR_RO(name);
    static ssize_t path_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_device *pps = dev_get_drvdata(dev);
    return sprintf(buf, "%s\n", pps.info.path);
    }
    static DEVICE_ATTR_RO(path);
    static struct attribute *pps_attrs[] = {
    &dev_attr_assert.attr,
    &dev_attr_clear.attr,
    &dev_attr_mode.attr,
    &dev_attr_echo.attr,
    &dev_attr_name.attr,
    &dev_attr_path.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pps_group = {
    .attrs = pps_attrs,
    };
    const struct attribute_group *pps_groups[] = {
    &pps_group,
    core::ptr::null_mut(),
    };
