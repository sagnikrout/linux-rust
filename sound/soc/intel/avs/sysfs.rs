//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/sysfs.c
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
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn fw_version_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t fw_version_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct avs_dev *adev = to_avs_dev(dev);
    struct avs_fw_version *fw_version = &adev.fw_cfg.fw_version;
    return sysfs_emit(buf, "%d.%d.%d.%d\n", fw_version.major, fw_version.minor,
    fw_version.hotfix, fw_version.build);
    }
    static DEVICE_ATTR_RO(fw_version);
    static struct attribute *avs_fw_attrs[] = {
    &dev_attr_fw_version.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group avs_attr_group = {
    .name = "avs",
    .attrs = avs_fw_attrs,
    };
    const struct attribute_group *avs_attr_groups[] = {
    &avs_attr_group,
    core::ptr::null_mut()
    };
