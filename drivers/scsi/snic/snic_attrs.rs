//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/snic/snic_attrs.c
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

    static ssize_t
    snic_show_sym_name(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct snic *snic = shost_priv(class_to_shost(dev));
    return sysfs_emit(buf, "%s\n", snic.name);
    }
    static ssize_t
    snic_show_state(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct snic *snic = shost_priv(class_to_shost(dev));
    return sysfs_emit(buf, "%s\n", snic_state_str[snic_get_state(snic)]);
    }
    static ssize_t
    snic_show_drv_version(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n", SNIC_DRV_VERSION);
    }
    static ssize_t
    snic_show_link_state(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct snic *snic = shost_priv(class_to_shost(dev));
    if (snic.config.xpt_type == SNIC_DAS)
    snic.link_status = svnic_dev_link_status(snic.vdev);
    return sysfs_emit(buf, "%s\n",
    (snic.link_status) ? "Link Up" : "Link Down");
    }
    static DEVICE_ATTR(snic_sym_name, S_IRUGO, snic_show_sym_name, core::ptr::null_mut());
    static DEVICE_ATTR(snic_state, S_IRUGO, snic_show_state, core::ptr::null_mut());
    static DEVICE_ATTR(drv_version, S_IRUGO, snic_show_drv_version, core::ptr::null_mut());
    static DEVICE_ATTR(link_state, S_IRUGO, snic_show_link_state, core::ptr::null_mut());
    static struct attribute *snic_host_attrs[] = {
    &dev_attr_snic_sym_name.attr,
    &dev_attr_snic_state.attr,
    &dev_attr_drv_version.attr,
    &dev_attr_link_state.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group snic_host_attr_group = {
    .attrs = snic_host_attrs
    };
    const struct attribute_group *snic_host_groups[] = {
    &snic_host_attr_group,
    core::ptr::null_mut()
    };
