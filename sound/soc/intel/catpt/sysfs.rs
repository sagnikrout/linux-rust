//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/catpt/sysfs.c
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

    static ssize_t fw_version_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct catpt_dev *cdev = dev_get_drvdata(dev);
    struct catpt_fw_version version;
    int ret;
    ret = pm_runtime_resume_and_get(cdev.dev);
    if (ret)
    return ret;
    ret = catpt_ipc_get_fw_version(cdev, &version);
    pm_runtime_put_autosuspend(cdev.dev);
    if (ret)
    return CATPT_IPC_RET(ret);
    return sysfs_emit(buf, "%d.%d.%d.%d\n", version.type, version.major,
    version.minor, version.build);
    }
    static DEVICE_ATTR_RO(fw_version);
    static ssize_t fw_info_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct catpt_dev *cdev = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%s\n", cdev.ipc.config.fw_info);
    }
    static DEVICE_ATTR_RO(fw_info);
    static struct attribute *catpt_attrs[] = {
    &dev_attr_fw_version.attr,
    &dev_attr_fw_info.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group catpt_attr_group = {
    .attrs = catpt_attrs,
    };
    const struct attribute_group *catpt_attr_groups[] = {
    &catpt_attr_group,
    core::ptr::null_mut()
    };
