//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/secvar-sysfs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019 IBM Corporation <nayna@linux.ibm.com>
//
// This code exposes secure variables to user via sysfs
//

pub const NAME_MAX_SIZE: c_int = 1024;
    static struct kobject *secvar_kobj;
    static struct kset *secvar_kset;
    static ssize_t format_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    char tmp[32];
    let mut len: isize = secvar_ops.format(tmp, sizeof(tmp));
    if (len > 0)
    return sysfs_emit(buf, "%s\n", tmp);
#[no_mangle]
pub unsafe extern "C" fn if(0: len <) -> else {
    else if (len < 0)
    pr_err("Error %zd reading format string\n", len);
    else
    pr_err("Got empty format string from backend\n");
    return -EIO;
    }
    static ssize_t size_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    u64 dsize;
    int rc;
    rc = secvar_ops.get(kobj.name, strlen(kobj.name) + 1, core::ptr::null_mut(), &dsize);
    if (rc) {
    if (rc != -ENOENT)
    pr_err("Error retrieving %s variable size %d\n", kobj.name, rc);
    return rc;
    }
    return sysfs_emit(buf, "%llu\n", dsize);
    }
    static ssize_t data_read(struct file *filep, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off,
    size_t count)
    {
    char *data;
    u64 dsize;
    int rc;
    rc = secvar_ops.get(kobj.name, strlen(kobj.name) + 1, core::ptr::null_mut(), &dsize);
    if (rc) {
    if (rc != -ENOENT)
    pr_err("Error getting %s variable size %d\n", kobj.name, rc);
    return rc;
    }
    pr_debug("dsize is %llu\n", dsize);
    data = kzalloc(dsize, GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    rc = secvar_ops.get(kobj.name, strlen(kobj.name) + 1, data, &dsize);
    if (rc) {
    pr_err("Error getting %s variable %d\n", kobj.name, rc);
    goto data_fail;
    }
    rc = memory_read_from_buffer(buf, count, &off, data, dsize);
    data_fail:
    kfree(data);
    return rc;
    }
    static ssize_t update_write(struct file *filep, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off,
    size_t count)
    {
    int rc;
    pr_debug("count is %ld\n", count);
    rc = secvar_ops.set(kobj.name, strlen(kobj.name) + 1, buf, count);
    if (rc) {
    pr_err("Error setting the %s variable %d\n", kobj.name, rc);
    return rc;
    }
    return count;
    }
    let mut format_attr: static struct kobj_attribute = __ATTR_RO(format);
    let mut size_attr: static struct kobj_attribute = __ATTR_RO(size);
    let mut __ro_after_init: static struct bin_attribute data_attr = __BIN_ATTR_RO(data, 0);
    let mut __ro_after_init: static struct bin_attribute update_attr = __BIN_ATTR_WO(update, 0);
    static const struct bin_attribute *const secvar_bin_attrs[] = {
    &data_attr,
    &update_attr,
    core::ptr::null_mut(),
    };
    static struct attribute *secvar_attrs[] = {
    &size_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group secvar_attr_group = {
    .attrs = secvar_attrs,
    .bin_attrs = secvar_bin_attrs,
    };
    __ATTRIBUTE_GROUPS(secvar_attr);
    static const struct kobj_type secvar_ktype = {
    .sysfs_ops	= &kobj_sysfs_ops,
    .default_groups = secvar_attr_groups,
    };
#[no_mangle]
unsafe extern "C" fn update_kobj_size() -> __init int {
    static __init int update_kobj_size(void)
    {
    u64 varsize;
    let mut rc: c_int = secvar_ops.max_size(&varsize);
    if (rc)
    return rc;
    data_attr.size = varsize;
    update_attr.size = varsize;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn add_var(name: *const c_char) -> __init int {
    static __init int add_var(const char *name)
    {
    struct kobject *kobj;
    int rc;
    kobj = kzalloc_obj(*kobj);
    if (!kobj)
    return -ENOMEM;
    kobject_init(kobj, &secvar_ktype);
    rc = kobject_add(kobj, &secvar_kset.kobj, "%s", name);
    if (rc) {
    pr_warn("kobject_add error %d for attribute: %s\n", rc,
    name);
    kobject_put(kobj);
    return rc;
    }
    kobject_uevent(kobj, KOBJ_ADD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn secvar_sysfs_load() -> __init int {
    static __init int secvar_sysfs_load(void)
    {
    let mut namesize: u64 = 0;
    char *name;
    int rc;
    name = kzalloc(NAME_MAX_SIZE, GFP_KERNEL);
    if (!name)
    return -ENOMEM;
    do {
    rc = secvar_ops.get_next(name, &namesize, NAME_MAX_SIZE);
    if (rc) {
    if (rc != -ENOENT)
    pr_err("error getting secvar from firmware %d\n", rc);
    else
    rc = 0;
    break;
    }
    rc = add_var(name);
    } while (!rc);
    kfree(name);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn secvar_sysfs_load_static() -> __init int {
    static __init int secvar_sysfs_load_static(void)
    {
    const char * const *name_ptr = secvar_ops.var_names;
    int rc;
    while (*name_ptr) {
    rc = add_var(*name_ptr);
    if (rc)
    return rc;
    name_ptr++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn secvar_sysfs_init() -> __init int {
    static __init int secvar_sysfs_init(void)
    {
    u64 max_size;
    int rc;
    if (!secvar_ops) {
    pr_warn("Failed to retrieve secvar operations\n");
    return -ENODEV;
    }
    secvar_kobj = kobject_create_and_add("secvar", firmware_kobj);
    if (!secvar_kobj) {
    pr_err("Failed to create firmware kobj\n");
    return -ENOMEM;
    }
    rc = sysfs_create_file(secvar_kobj, &format_attr.attr);
    if (rc) {
    pr_err("Failed to create format object\n");
    rc = -ENOMEM;
    goto err;
    }
    secvar_kset = kset_create_and_add("vars", core::ptr::null_mut(), secvar_kobj);
    if (!secvar_kset) {
    pr_err("sysfs kobject registration failed\n");
    rc = -ENOMEM;
    goto err;
    }
    rc = update_kobj_size();
    if (rc) {
    pr_err("Cannot read the size of the attribute\n");
    goto err;
    }
    rc = plpks_config_create_softlink(secvar_kobj);
    if (rc) {
    pr_err("Failed to create softlink to PLPKS config directory");
    goto err;
    }
    pr_info("/sys/firmware/secvar/config is now deprecated.\n");
    pr_info("Will be removed in future versions.\n");
    if (secvar_ops.get_next)
    rc = secvar_sysfs_load();
    else
    rc = secvar_sysfs_load_static();
    if (rc) {
    pr_err("Failed to create variable attributes\n");
    goto err;
    }
// Due to sysfs limitations, we will only ever get a write buffer of
// up to 1 page in size. Print a warning if this is potentially going
// to cause problems, so that the user is aware.
    secvar_ops.max_size(&max_size);
    if (max_size > PAGE_SIZE)
    pr_warn_ratelimited("PAGE_SIZE (%lu) is smaller than maximum object size (%llu), writes are limited to PAGE_SIZE\n",
    PAGE_SIZE, max_size);
    return 0;
    err:
    kobject_put(secvar_kobj);
    return rc;
    }
    late_initcall(secvar_sysfs_init);
