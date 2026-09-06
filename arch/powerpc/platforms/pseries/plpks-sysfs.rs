//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/plpks-sysfs.c
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
// Copyright (C) 2025 IBM Corporation, Srish Srinivasan <ssrish@linux.ibm.com>
//
// This code exposes PLPKS config to user via sysfs
//

// config attributes for sysfs

    static ssize_t name##_show(struct kobject *kobj,	\
    struct kobj_attribute *attr,	\
    char *buf)			\
    {							\
    return sysfs_emit(buf, fmt, func());		\
    }							\
    static struct kobj_attribute attr_##name = __ATTR_RO(name)
    PLPKS_CONFIG_ATTR(version, "%u\n", plpks_get_version);
    PLPKS_CONFIG_ATTR(max_object_size, "%u\n", plpks_get_maxobjectsize);
    PLPKS_CONFIG_ATTR(total_size, "%u\n", plpks_get_totalsize);
    PLPKS_CONFIG_ATTR(used_space, "%u\n", plpks_get_usedspace);
    PLPKS_CONFIG_ATTR(supported_policies, "%08x\n", plpks_get_supportedpolicies);
    PLPKS_CONFIG_ATTR(signed_update_algorithms, "%016llx\n",
    plpks_get_signedupdatealgorithms);
    PLPKS_CONFIG_ATTR(wrapping_features, "%016llx\n", plpks_get_wrappingfeatures);
    static const struct attribute *config_attrs[] = {
    &attr_version.attr,
    &attr_max_object_size.attr,
    &attr_total_size.attr,
    &attr_used_space.attr,
    &attr_supported_policies.attr,
    &attr_signed_update_algorithms.attr,
    &attr_wrapping_features.attr,
    core::ptr::null_mut(),
    };
    static struct kobject *plpks_kobj, *plpks_config_kobj;
#[no_mangle]
pub unsafe extern "C" fn plpks_config_create_softlink(from: *mut kobject) -> c_int {
    int plpks_config_create_softlink(struct kobject *from)
    {
    if (!plpks_config_kobj)
    return -EINVAL;
    return sysfs_create_link(from, plpks_config_kobj, "config");
    }
#[no_mangle]
unsafe extern "C" fn plpks_sysfs_config(kobj: *mut kobject) -> __init int {
    static __init int plpks_sysfs_config(struct kobject *kobj)
    {
    struct attribute_group config_group = {
    .name = core::ptr::null_mut(),
    .attrs = (struct attribute **)config_attrs,
    };
    return sysfs_create_group(kobj, &config_group);
    }
#[no_mangle]
unsafe extern "C" fn plpks_sysfs_init() -> __init int {
    static __init int plpks_sysfs_init(void)
    {
    int rc;
    if (!plpks_is_available())
    return -ENODEV;
    plpks_kobj = kobject_create_and_add("plpks", firmware_kobj);
    if (!plpks_kobj) {
    pr_err("Failed to create plpks kobj\n");
    return -ENOMEM;
    }
    plpks_config_kobj = kobject_create_and_add("config", plpks_kobj);
    if (!plpks_config_kobj) {
    pr_err("Failed to create plpks config kobj\n");
    kobject_put(plpks_kobj);
    return -ENOMEM;
    }
    rc = plpks_sysfs_config(plpks_config_kobj);
    if (rc) {
    pr_err("Failed to create attribute group for plpks config\n");
    kobject_put(plpks_config_kobj);
    kobject_put(plpks_kobj);
    return rc;
    }
    return 0;
    }
    machine_subsys_initcall(pseries, plpks_sysfs_init);
