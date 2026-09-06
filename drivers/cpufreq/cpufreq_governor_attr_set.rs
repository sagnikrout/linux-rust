//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/cpufreq_governor_attr_set.c
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
// Abstract code for CPUFreq governor tunable sysfs attributes.
//
// Copyright (C) 2016, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

    static inline struct governor_attr *to_gov_attr(struct attribute *attr)
    {
    return container_of(attr, struct governor_attr, attr);
    }
    static ssize_t governor_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    struct governor_attr *gattr = to_gov_attr(attr);
    return gattr.show(to_gov_attr_set(kobj), buf);
    }
    static ssize_t governor_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t count)
    {
    struct gov_attr_set *attr_set = to_gov_attr_set(kobj);
    struct governor_attr *gattr = to_gov_attr(attr);
    int ret;
    mutex_lock(&attr_set.update_lock);
    ret = attr_set.usage_count ? gattr.store(attr_set, buf, count) : -EBUSY;
    mutex_unlock(&attr_set.update_lock);
    return ret;
    }
    const struct sysfs_ops governor_sysfs_ops = {
    .show	= governor_show,
    .store	= governor_store,
    };
    EXPORT_SYMBOL_GPL(governor_sysfs_ops);
#[no_mangle]
pub unsafe extern "C" fn gov_attr_set_init(attr_set: *mut gov_attr_set, list_node: *mut list_head) {
    void gov_attr_set_init(struct gov_attr_set *attr_set, struct list_head *list_node)
    {
    INIT_LIST_HEAD(&attr_set.policy_list);
    mutex_init(&attr_set.update_lock);
    attr_set.usage_count = 1;
    list_add(list_node, &attr_set.policy_list);
    }
    EXPORT_SYMBOL_GPL(gov_attr_set_init);
#[no_mangle]
pub unsafe extern "C" fn gov_attr_set_get(attr_set: *mut gov_attr_set, list_node: *mut list_head) {
    void gov_attr_set_get(struct gov_attr_set *attr_set, struct list_head *list_node)
    {
    mutex_lock(&attr_set.update_lock);
    attr_set.usage_count++;
    list_add(list_node, &attr_set.policy_list);
    mutex_unlock(&attr_set.update_lock);
    }
    EXPORT_SYMBOL_GPL(gov_attr_set_get);
#[no_mangle]
pub unsafe extern "C" fn gov_attr_set_put(attr_set: *mut gov_attr_set, list_node: *mut list_head) -> c_uint {
    unsigned int gov_attr_set_put(struct gov_attr_set *attr_set, struct list_head *list_node)
    {
    unsigned int count;
    mutex_lock(&attr_set.update_lock);
    list_del(list_node);
    count = --attr_set.usage_count;
    mutex_unlock(&attr_set.update_lock);
    if (count)
    return count;
    mutex_destroy(&attr_set.update_lock);
    kobject_put(&attr_set.kobj);
    return 0;
    }
    EXPORT_SYMBOL_GPL(gov_attr_set_put);
