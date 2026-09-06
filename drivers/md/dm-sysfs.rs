//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-sysfs.c
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
// Copyright (C) 2008 Red Hat, Inc. All rights reserved.
//
// This file is released under the GPL.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_sysfs_attr {
    pub attr: attribute,
    pub p): *mut *mut *mut ssize_t (show)(struct mapped_device md, char,
    pub count): *const *const *const *const ssize_t (store)(struct mapped_device md, char p, size_t,
}

    struct dm_sysfs_attr dm_attr_##_name = \
    __ATTR(_name, 0444, dm_attr_##_name##_show, core::ptr::null_mut())
    static ssize_t dm_attr_show(struct kobject *kobj, struct attribute *attr,
    char *page)
    {
    struct dm_sysfs_attr *dm_attr;
    struct mapped_device *md;
    ssize_t ret;
    dm_attr = container_of(attr, struct dm_sysfs_attr, attr);
    if (!dm_attr.show)
    return -EIO;
    md = dm_get_from_kobject(kobj);
    if (!md)
    return -EINVAL;
    ret = dm_attr.show(md, page);
    dm_put(md);
    return ret;
    }

    struct dm_sysfs_attr dm_attr_##_name = \
    __ATTR(_name, 0644, dm_attr_##_name##_show, dm_attr_##_name##_store)
    static ssize_t dm_attr_store(struct kobject *kobj, struct attribute *attr,
    const char *page, size_t count)
    {
    struct dm_sysfs_attr *dm_attr;
    struct mapped_device *md;
    ssize_t ret;
    dm_attr = container_of(attr, struct dm_sysfs_attr, attr);
    if (!dm_attr.store)
    return -EIO;
    md = dm_get_from_kobject(kobj);
    if (!md)
    return -EINVAL;
    ret = dm_attr.store(md, page, count);
    dm_put(md);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm_attr_name_show(md: *mut mapped_device, buf: *mut c_char) -> isize {
    static ssize_t dm_attr_name_show(struct mapped_device *md, char *buf)
    {
    if (dm_copy_name_and_uuid(md, buf, core::ptr::null_mut()))
    return -EIO;
    strcat(buf, "\n");
    return strlen(buf);
    }
#[no_mangle]
unsafe extern "C" fn dm_attr_uuid_show(md: *mut mapped_device, buf: *mut c_char) -> isize {
    static ssize_t dm_attr_uuid_show(struct mapped_device *md, char *buf)
    {
    if (dm_copy_name_and_uuid(md, core::ptr::null_mut(), buf))
    return -EIO;
    strcat(buf, "\n");
    return strlen(buf);
    }
#[no_mangle]
unsafe extern "C" fn dm_attr_suspended_show(md: *mut mapped_device, buf: *mut c_char) -> isize {
    static ssize_t dm_attr_suspended_show(struct mapped_device *md, char *buf)
    {
    return sysfs_emit(buf, "%d\n", dm_suspended_md(md));
    }
#[no_mangle]
unsafe extern "C" fn dm_attr_use_blk_mq_show(md: *mut mapped_device, buf: *mut c_char) -> isize {
    static ssize_t dm_attr_use_blk_mq_show(struct mapped_device *md, char *buf)
    {
// Purely for userspace compatibility
    return sysfs_emit(buf, "%d\n", true);
    }
    static DM_ATTR_RO(name);
    static DM_ATTR_RO(uuid);
    static DM_ATTR_RO(suspended);
    static DM_ATTR_RO(use_blk_mq);
    static DM_ATTR_RW(rq_based_seq_io_merge_deadline);
    static struct attribute *dm_attrs[] = {
    &dm_attr_name.attr,
    &dm_attr_uuid.attr,
    &dm_attr_suspended.attr,
    &dm_attr_use_blk_mq.attr,
    &dm_attr_rq_based_seq_io_merge_deadline.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(dm);
    static const struct sysfs_ops dm_sysfs_ops = {
    .show	= dm_attr_show,
    .store	= dm_attr_store,
    };
    static const struct kobj_type dm_ktype = {
    .sysfs_ops	= &dm_sysfs_ops,
    .default_groups	= dm_groups,
    .release	= dm_kobject_release,
    };
//
// Initialize kobj
// because nobody using md yet, no need to call explicit dm_get/put
//
#[no_mangle]
pub unsafe extern "C" fn dm_sysfs_init(md: *mut mapped_device) -> c_int {
    int dm_sysfs_init(struct mapped_device *md)
    {
    return kobject_init_and_add(dm_kobject(md), &dm_ktype,
    &disk_to_dev(dm_disk(md)).kobj,
    "%s", "dm");
    }
//
// Remove kobj, called after all references removed
//
#[no_mangle]
pub unsafe extern "C" fn dm_sysfs_exit(md: *mut mapped_device) {
    void dm_sysfs_exit(struct mapped_device *md)
    {
    struct kobject *kobj = dm_kobject(md);
    kobject_put(kobj);
    wait_for_completion(dm_get_completion_from_kobject(kobj));
    }
