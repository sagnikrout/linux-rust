//! Automatically rewritten from C to Rust
//! Source: sound/aoa/soundbus/sysfs.c
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

// FIX UP

    static ssize_t modalias_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct soundbus_dev *sdev = to_soundbus_device(dev);
    struct platform_device *of = &sdev.ofdev;
    if (*sdev.modalias)
    return sysfs_emit(buf, "%s\n", sdev.modalias);
    else
    return sysfs_emit(buf, "of:N%pOFn%c%s\n",
    of.dev.of_node, 'T',
    of_node_get_device_type(of.dev.of_node));
    }
    static DEVICE_ATTR_RO(modalias);
    static ssize_t name_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct soundbus_dev *sdev = to_soundbus_device(dev);
    struct platform_device *of = &sdev.ofdev;
    return sysfs_emit(buf, "%pOFn\n", of.dev.of_node);
    }
    static DEVICE_ATTR_RO(name);
    static ssize_t type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct soundbus_dev *sdev = to_soundbus_device(dev);
    struct platform_device *of = &sdev.ofdev;
    return sysfs_emit(buf, "%s\n", of_node_get_device_type(of.dev.of_node));
    }
    static DEVICE_ATTR_RO(type);
    struct attribute *soundbus_dev_attrs[] = {
    &dev_attr_name.attr,
    &dev_attr_type.attr,
    &dev_attr_modalias.attr,
    core::ptr::null_mut(),
    };
