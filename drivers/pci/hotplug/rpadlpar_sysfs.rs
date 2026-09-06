//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/rpadlpar_sysfs.c
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
// Interface for Dynamic Logical Partitioning of I/O Slots on
// RPA-compliant PPC64 platform.
//
// John Rose <johnrose@austin.ibm.com>
// October 2003
//
// Copyright (C) 2003 IBM.
//

// Those two have no quotes because they are passed to __ATTR() which
// stringifies the argument (yuck !)
//

    static ssize_t add_slot_store(struct kobject *kobj, struct kobj_attribute *attr,
    const char *buf, size_t nbytes)
    {
    char drc_name[MAX_DRC_NAME_LEN];
    char *end;
    int rc;
    if (nbytes >= MAX_DRC_NAME_LEN)
    return 0;
    strscpy(drc_name, buf, nbytes + 1);
    end = strchr(drc_name, '\n');
    if (end)
// end = '\0';
    rc = dlpar_add_slot(drc_name);
    if (rc)
    return rc;
    return nbytes;
    }
    static ssize_t add_slot_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "0\n");
    }
    static ssize_t remove_slot_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t nbytes)
    {
    char drc_name[MAX_DRC_NAME_LEN];
    int rc;
    char *end;
    if (nbytes >= MAX_DRC_NAME_LEN)
    return 0;
    strscpy(drc_name, buf, nbytes + 1);
    end = strchr(drc_name, '\n');
    if (end)
// end = '\0';
    rc = dlpar_remove_slot(drc_name);
    if (rc)
    return rc;
    return nbytes;
    }
    static ssize_t remove_slot_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "0\n");
    }
    static struct kobj_attribute add_slot_attr =
    __ATTR(ADD_SLOT_ATTR_NAME, 0644, add_slot_show, add_slot_store);
    static struct kobj_attribute remove_slot_attr =
    __ATTR(REMOVE_SLOT_ATTR_NAME, 0644, remove_slot_show, remove_slot_store);
    static struct attribute *default_attrs[] = {
    &add_slot_attr.attr,
    &remove_slot_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group dlpar_attr_group = {
    .attrs = default_attrs,
    };
    static struct kobject *dlpar_kobj;
#[no_mangle]
pub unsafe extern "C" fn dlpar_sysfs_init() -> c_int {
    int dlpar_sysfs_init(void)
    {
    int error;
    dlpar_kobj = kobject_create_and_add(DLPAR_KOBJ_NAME,
    &pci_slots_kset.kobj);
    if (!dlpar_kobj)
    return -EINVAL;
    error = sysfs_create_group(dlpar_kobj, &dlpar_attr_group);
    if (error)
    kobject_put(dlpar_kobj);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn dlpar_sysfs_exit() {
    void dlpar_sysfs_exit(void)
    {
    sysfs_remove_group(dlpar_kobj, &dlpar_attr_group);
    kobject_put(dlpar_kobj);
    }
