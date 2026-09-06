//! Automatically rewritten from C to Rust
//! Source: mm/cma_sysfs.c
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
//
// CMA SysFS Interface
//
// Copyright (c) 2021 Minchan Kim <minchan@kernel.org>
//

    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)
#[no_mangle]
pub unsafe extern "C" fn cma_sysfs_account_success_pages(cma: *mut cma, nr_pages: c_ulong) {
    void cma_sysfs_account_success_pages(struct cma *cma, unsigned long nr_pages)
    {
    atomic64_add(nr_pages, &cma.nr_pages_succeeded);
    }
#[no_mangle]
pub unsafe extern "C" fn cma_sysfs_account_fail_pages(cma: *mut cma, nr_pages: c_ulong) {
    void cma_sysfs_account_fail_pages(struct cma *cma, unsigned long nr_pages)
    {
    atomic64_add(nr_pages, &cma.nr_pages_failed);
    }
#[no_mangle]
pub unsafe extern "C" fn cma_sysfs_account_release_pages(cma: *mut cma, nr_pages: c_ulong) {
    void cma_sysfs_account_release_pages(struct cma *cma, unsigned long nr_pages)
    {
    atomic64_add(nr_pages, &cma.nr_pages_released);
    }
    static inline struct cma *cma_from_kobj(struct kobject *kobj)
    {
    return container_of(kobj, struct cma_kobject, kobj).cma;
    }
    static ssize_t alloc_pages_success_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct cma *cma = cma_from_kobj(kobj);
    return sysfs_emit(buf, "%llu\n",
    atomic64_read(&cma.nr_pages_succeeded));
    }
    CMA_ATTR_RO(alloc_pages_success);
    static ssize_t alloc_pages_fail_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct cma *cma = cma_from_kobj(kobj);
    return sysfs_emit(buf, "%llu\n", atomic64_read(&cma.nr_pages_failed));
    }
    CMA_ATTR_RO(alloc_pages_fail);
    static ssize_t release_pages_success_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct cma *cma = cma_from_kobj(kobj);
    return sysfs_emit(buf, "%llu\n", atomic64_read(&cma.nr_pages_released));
    }
    CMA_ATTR_RO(release_pages_success);
    static ssize_t total_pages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct cma *cma = cma_from_kobj(kobj);
    return sysfs_emit(buf, "%lu\n", cma.count);
    }
    CMA_ATTR_RO(total_pages);
    static ssize_t available_pages_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct cma *cma = cma_from_kobj(kobj);
    return sysfs_emit(buf, "%lu\n", cma.available_count);
    }
    CMA_ATTR_RO(available_pages);
#[no_mangle]
unsafe extern "C" fn cma_kobj_release(kobj: *mut kobject) {
    static void cma_kobj_release(struct kobject *kobj)
    {
    struct cma *cma = cma_from_kobj(kobj);
    struct cma_kobject *cma_kobj = cma.cma_kobj;
    kfree(cma_kobj);
    cma.cma_kobj = core::ptr::null_mut();
    }
    static struct attribute *cma_attrs[] = {
    &alloc_pages_success_attr.attr,
    &alloc_pages_fail_attr.attr,
    &release_pages_success_attr.attr,
    &total_pages_attr.attr,
    &available_pages_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(cma);
    static const struct kobj_type cma_ktype = {
    .release = cma_kobj_release,
    .sysfs_ops = &kobj_sysfs_ops,
    .default_groups = cma_groups,
    };
#[no_mangle]
unsafe extern "C" fn cma_sysfs_init() -> int __init {
    static int __init cma_sysfs_init(void)
    {
    struct kobject *cma_kobj_root;
    struct cma_kobject *cma_kobj;
    struct cma *cma;
    int i, err;
    cma_kobj_root = kobject_create_and_add("cma", mm_kobj);
    if (!cma_kobj_root)
    return -ENOMEM;
    for (i = 0; i < cma_area_count; i++) {
    cma = &cma_areas[i];
    if (!test_bit(CMA_ACTIVATED, &cma.flags))
    continue;
    cma_kobj = kzalloc_obj(*cma_kobj);
    if (!cma_kobj) {
    err = -ENOMEM;
    goto out;
    }
    cma.cma_kobj = cma_kobj;
    cma_kobj.cma = cma;
    err = kobject_init_and_add(&cma_kobj.kobj, &cma_ktype,
    cma_kobj_root, "%s", cma.name);
    if (err) {
    kobject_put(&cma_kobj.kobj);
    goto out;
    }
    }
    return 0;
    out:
    while (--i >= 0) {
    cma = &cma_areas[i];
    if (cma.cma_kobj)
    kobject_put(&cma.cma_kobj.kobj);
    }
    kobject_put(cma_kobj_root);
    return err;
    }
    subsys_initcall(cma_sysfs_init);
