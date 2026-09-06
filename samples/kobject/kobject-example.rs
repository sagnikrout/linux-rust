//! Automatically rewritten from C to Rust
//! Source: samples/kobject/kobject-example.c
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
// Sample kobject implementation
//
// Copyright (C) 2004-2007 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2007 Novell Inc.
//

//
// This module shows how to create a simple subdirectory in sysfs called
// /sys/kernel/kobject_example  In that directory, 3 files are created:
// "foo", "baz", and "bar".  If an integer is written to these files, it can be
// later read out of it.
//
    static int foo;
    static int baz;
    static int bar;
//
// The "foo" file where a static variable is read from and written to.
//
    static ssize_t foo_show(struct kobject *kobj, const struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", foo);
    }
    static ssize_t foo_store(struct kobject *kobj, const struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    ret = kstrtoint(buf, 10, &foo);
    if (ret < 0)
    return ret;
    return count;
    }
// Sysfs attributes cannot be world-writable.
    static const struct kobj_attribute foo_attribute =
    __KOBJ_ATTR(foo, 0664, foo_show, foo_store);
//
// More complex function where we determine which variable is being accessed by
// looking at the attribute for the "baz" and "bar" files.
//
    static ssize_t b_show(struct kobject *kobj, const struct kobj_attribute *attr,
    char *buf)
    {
    int var;
    if (strcmp(attr.attr.name, "baz") == 0)
    var = baz;
    else
    var = bar;
    return sysfs_emit(buf, "%d\n", var);
    }
    static ssize_t b_store(struct kobject *kobj, const struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int var, ret;
    ret = kstrtoint(buf, 10, &var);
    if (ret < 0)
    return ret;
    if (strcmp(attr.attr.name, "baz") == 0)
    baz = var;
    else
    bar = var;
    return count;
    }
    static const struct kobj_attribute baz_attribute =
    __KOBJ_ATTR(baz, 0664, b_show, b_store);
    static const struct kobj_attribute bar_attribute =
    __KOBJ_ATTR(bar, 0664, b_show, b_store);
//
// Create a group of attributes so that we can create and destroy them all
// at once.
//
    static const struct attribute *const attrs[] = {
    &foo_attribute.attr,
    &baz_attribute.attr,
    &bar_attribute.attr,
    core::ptr::null_mut(),	/* need to core::ptr::null_mut() terminate the list of attributes */
    };
//
// An unnamed attribute group will put all of the attributes directly in
// the kobject directory.  If we specify a name, a subdirectory will be
// created for the attributes with the directory being the name of the
// attribute group.
//
    static const struct attribute_group attr_group = {
    .attrs_const = attrs,
    };
    static struct kobject *example_kobj;
#[no_mangle]
unsafe extern "C" fn example_init() -> int __init {
    static int __init example_init(void)
    {
    int retval;
//
// Create a simple kobject with the name of "kobject_example",
// located under /sys/kernel
//
// As this is a simple directory, no uevent will be sent to
// userspace.  That is why this function should not be used for
// any type of dynamic kobjects, where the name and number are
// not known ahead of time.
//
    example_kobj = kobject_create_and_add("kobject_example", kernel_kobj);
    if (!example_kobj)
    return -ENOMEM;
// Create the files associated with this kobject
    retval = sysfs_create_group(example_kobj, &attr_group);
    if (retval)
    kobject_put(example_kobj);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn example_exit() -> void __exit {
    static void __exit example_exit(void)
    {
    kobject_put(example_kobj);
    }
    module_init(example_init);
    module_exit(example_exit);
    MODULE_DESCRIPTION("Sample kobject implementation");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Greg Kroah-Hartman <greg@kroah.com>");
