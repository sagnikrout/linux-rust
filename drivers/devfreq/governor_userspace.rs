//! Automatically rewritten from C to Rust
//! Source: drivers/devfreq/governor_userspace.c
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
// linux/drivers/devfreq/governor_userspace.c
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct userspace_data {
    pub user_frequency: c_ulong,
    pub valid: bool,
}

#[no_mangle]
unsafe extern "C" fn devfreq_userspace_func(df: *mut devfreq, freq: *mut c_ulong) -> c_int {
    static int devfreq_userspace_func(struct devfreq *df, unsigned long *freq)
    {
    struct userspace_data *data = df.governor_data;
    if (data.valid)
// freq = data->user_frequency;
    else
// freq = df->previous_freq; /* No user freq specified yet
    return 0;
    }
    static ssize_t set_freq_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct devfreq *devfreq = to_devfreq(dev);
    struct userspace_data *data;
    unsigned long wanted;
    let mut err: c_int = 0;
    err = kstrtoul(buf, 0, &wanted);
    if (err)
    return err;
    mutex_lock(&devfreq.lock);
    data = devfreq.governor_data;
    data.user_frequency = wanted;
    data.valid = true;
    err = update_devfreq(devfreq);
    if (err == 0)
    err = count;
    mutex_unlock(&devfreq.lock);
    return err;
    }
    static ssize_t set_freq_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct devfreq *devfreq = to_devfreq(dev);
    struct userspace_data *data;
    let mut err: c_int = 0;
    mutex_lock(&devfreq.lock);
    data = devfreq.governor_data;
    if (data.valid)
    err = sprintf(buf, "%lu\n", data.user_frequency);
    else
    err = sprintf(buf, "undefined\n");
    mutex_unlock(&devfreq.lock);
    return err;
    }
    static DEVICE_ATTR_RW(set_freq);
    static struct attribute *dev_entries[] = {
    &dev_attr_set_freq.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group dev_attr_group = {
    .name	= DEVFREQ_GOV_USERSPACE,
    .attrs	= dev_entries,
    };
#[no_mangle]
unsafe extern "C" fn userspace_init(devfreq: *mut devfreq) -> c_int {
    static int userspace_init(struct devfreq *devfreq)
    {
    let mut err: c_int = 0;
    struct userspace_data *data = kzalloc_obj(struct userspace_data);
    if (!data) {
    err = -ENOMEM;
    goto out;
    }
    data.valid = false;
    devfreq.governor_data = data;
    err = sysfs_create_group(&devfreq.dev.kobj, &dev_attr_group);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn userspace_exit(devfreq: *mut devfreq) {
    static void userspace_exit(struct devfreq *devfreq)
    {
//
// Remove the sysfs entry, unless this is being called after
// device_del(), which should have done this already via kobject_del().
//
    if (devfreq.dev.kobj.sd)
    sysfs_remove_group(&devfreq.dev.kobj, &dev_attr_group);
    kfree(devfreq.governor_data);
    devfreq.governor_data = core::ptr::null_mut();
    }
    static int devfreq_userspace_handler(struct devfreq *devfreq,
    unsigned int event, void *data)
    {
    let mut ret: c_int = 0;
    switch (event) {
    case DEVFREQ_GOV_START:
    ret = userspace_init(devfreq);
    break;
    case DEVFREQ_GOV_STOP:
    userspace_exit(devfreq);
    break;
    default:
    break;
    }
    return ret;
    }
    static struct devfreq_governor devfreq_userspace = {
    .name = DEVFREQ_GOV_USERSPACE,
    .get_target_freq = devfreq_userspace_func,
    .event_handler = devfreq_userspace_handler,
    };
#[no_mangle]
unsafe extern "C" fn devfreq_userspace_init() -> int __init {
    static int __init devfreq_userspace_init(void)
    {
    return devfreq_add_governor(&devfreq_userspace);
    }
    subsys_initcall(devfreq_userspace_init);
#[no_mangle]
unsafe extern "C" fn devfreq_userspace_exit() -> void __exit {
    static void __exit devfreq_userspace_exit(void)
    {
    int ret;
    ret = devfreq_remove_governor(&devfreq_userspace);
    if (ret)
    pr_err("%s: failed remove governor %d\n", __func__, ret);
    return;
    }
    module_exit(devfreq_userspace_exit);
    MODULE_DESCRIPTION("DEVFREQ Userspace governor");
    MODULE_LICENSE("GPL");
