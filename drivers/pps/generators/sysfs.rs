//! Automatically rewritten from C to Rust
//! Source: drivers/pps/generators/sysfs.c
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
// PPS generators sysfs support
//
// Copyright (C) 2024 Rodolfo Giometti <giometti@enneenne.com>
//

//
// Attribute functions
//
    static ssize_t system_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_gen_device *pps_gen = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", pps_gen.info.use_system_clock);
    }
    static DEVICE_ATTR_RO(system);
    static ssize_t time_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pps_gen_device *pps_gen = dev_get_drvdata(dev);
    struct timespec64 time;
    int ret;
    ret = pps_gen.info.get_time(pps_gen, &time);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%llu %09lu\n", time.tv_sec, time.tv_nsec);
    }
    static DEVICE_ATTR_RO(time);
    static ssize_t enable_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct pps_gen_device *pps_gen = dev_get_drvdata(dev);
    bool status;
    int ret;
    ret = kstrtobool(buf, &status);
    if (ret)
    return ret;
    ret = pps_gen.info.enable(pps_gen, status);
    if (ret)
    return ret;
    pps_gen.enabled = status;
    return count;
    }
    static DEVICE_ATTR_WO(enable);
    static struct attribute *pps_gen_attrs[] = {
    &dev_attr_enable.attr,
    &dev_attr_time.attr,
    &dev_attr_system.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pps_gen_group = {
    .attrs = pps_gen_attrs,
    };
    const struct attribute_group *pps_gen_groups[] = {
    &pps_gen_group,
    core::ptr::null_mut(),
    };
