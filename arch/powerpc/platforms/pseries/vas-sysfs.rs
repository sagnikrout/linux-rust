//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/vas-sysfs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2022-23 IBM Corp.
//

    static struct kobject *pseries_vas_kobj;
    static struct kobject *gzip_caps_kobj;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_caps_entry {
    pub kobj: kobject,
    pub caps: *mut vas_cop_feat_caps,
}

//
// This function is used to get the notification from the drmgr when
// QoS credits are changed.
//
    static ssize_t update_total_credits_store(struct vas_cop_feat_caps *caps,
    const char *buf, size_t count)
    {
    int err;
    u16 creds;
    err = kstrtou16(buf, 0, &creds);
//
// The user space interface from the management console
// notifies OS with the new QoS credits and then the
// hypervisor. So OS has to use this new credits value
// and reconfigure VAS windows (close or reopen depends
// on the credits available) instead of depending on VAS
// QoS capabilities from the hypervisor.
//
    if (!err)
    err = vas_reconfig_capabilties(caps.win_type, creds);
    if (err)
    return -EINVAL;
    pr_info("Set QoS total credits %u\n", creds);
    return count;
    }

    static ssize_t _name##_show(struct vas_cop_feat_caps *caps, char *buf) 	\
    {									\
    return sysfs_emit(buf, "%d\n", atomic_read(&caps._name));	\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_sysfs_entry {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct vas_cop_feat_caps , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct vas_cop_feat_caps , char ,,
}

    sysfs_caps_entry_read(_name);		\
    static struct vas_sysfs_entry _name##_attribute = __ATTR(_name,	\
    0444, _name##_show, core::ptr::null_mut());
//
// Create sysfs interface:
// /sys/devices/virtual/misc/vas/vas0/gzip/default_capabilities
// This directory contains the following VAS GZIP capabilities
// for the default credit type.
// /sys/devices/virtual/misc/vas/vas0/gzip/default_capabilities/nr_total_credits
// Total number of default credits assigned to the LPAR which
// can be changed with DLPAR operation.
// /sys/devices/virtual/misc/vas/vas0/gzip/default_capabilities/nr_used_credits
// Number of credits used by the user space. One credit will
// be assigned for each window open.
//
// /sys/devices/virtual/misc/vas/vas0/gzip/qos_capabilities
// This directory contains the following VAS GZIP capabilities
// for the Quality of Service (QoS) credit type.
// /sys/devices/virtual/misc/vas/vas0/gzip/qos_capabilities/nr_total_credits
// Total number of QoS credits assigned to the LPAR. The user
// has to define this value using HMC interface. It can be
// changed dynamically by the user.
// /sys/devices/virtual/misc/vas/vas0/gzip/qos_capabilities/nr_used_credits
// Number of credits used by the user space.
// /sys/devices/virtual/misc/vas/vas0/gzip/qos_capabilities/update_total_credits
// Update total QoS credits dynamically
//
    VAS_ATTR_RO(nr_total_credits);
    VAS_ATTR_RO(nr_used_credits);
    static struct vas_sysfs_entry update_total_credits_attribute =
    __ATTR(update_total_credits, 0200, core::ptr::null_mut(), update_total_credits_store);
    static struct attribute *vas_def_capab_attrs[] = {
    &nr_total_credits_attribute.attr,
    &nr_used_credits_attribute.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(vas_def_capab);
    static struct attribute *vas_qos_capab_attrs[] = {
    &nr_total_credits_attribute.attr,
    &nr_used_credits_attribute.attr,
    &update_total_credits_attribute.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(vas_qos_capab);
    static ssize_t vas_type_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    struct vas_caps_entry *centry;
    struct vas_cop_feat_caps *caps;
    struct vas_sysfs_entry *entry;
    centry = to_caps_entry(kobj);
    caps = centry.caps;
    entry = container_of(attr, struct vas_sysfs_entry, attr);
    if (!entry.show)
    return -EIO;
    return entry.show(caps, buf);
    }
    static ssize_t vas_type_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t count)
    {
    struct vas_caps_entry *centry;
    struct vas_cop_feat_caps *caps;
    struct vas_sysfs_entry *entry;
    centry = to_caps_entry(kobj);
    caps = centry.caps;
    entry = container_of(attr, struct vas_sysfs_entry, attr);
    if (!entry.store)
    return -EIO;
    return entry.store(caps, buf, count);
    }
#[no_mangle]
unsafe extern "C" fn vas_type_release(kobj: *mut kobject) {
    static void vas_type_release(struct kobject *kobj)
    {
    struct vas_caps_entry *centry = to_caps_entry(kobj);
    kfree(centry);
    }
    static const struct sysfs_ops vas_sysfs_ops = {
    .show	=	vas_type_show,
    .store	=	vas_type_store,
    };
    static const struct kobj_type vas_def_attr_type = {
    .release	=	vas_type_release,
    .sysfs_ops      =       &vas_sysfs_ops,
    .default_groups	=	vas_def_capab_groups,
    };
    static const struct kobj_type vas_qos_attr_type = {
    .release	=	vas_type_release,
    .sysfs_ops	=	&vas_sysfs_ops,
    .default_groups	=	vas_qos_capab_groups,
    };
    static char *vas_caps_kobj_name(struct vas_caps_entry *centry,
    struct kobject **kobj)
    {
    struct vas_cop_feat_caps *caps = centry.caps;
    if (caps.descriptor == VAS_GZIP_QOS_CAPABILITIES) {
    kobject_init(&centry.kobj, &vas_qos_attr_type);
// kobj = gzip_caps_kobj;
    return "qos_capabilities";
    } else if (caps.descriptor == VAS_GZIP_DEFAULT_CAPABILITIES) {
    kobject_init(&centry.kobj, &vas_def_attr_type);
// kobj = gzip_caps_kobj;
    return "default_capabilities";
    } else
    return "Unknown";
    }
//
// Add feature specific capability dir entry.
// Ex: VDefGzip or VQosGzip
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_add_vas_caps(caps: *mut vas_cop_feat_caps) -> c_int {
    int sysfs_add_vas_caps(struct vas_cop_feat_caps *caps)
    {
    struct vas_caps_entry *centry;
    struct kobject *kobj = core::ptr::null_mut();
    let mut ret: c_int = 0;
    char *name;
    centry = kzalloc_obj(*centry);
    if (!centry)
    return -ENOMEM;
    centry.caps = caps;
    name  = vas_caps_kobj_name(centry, &kobj);
    if (kobj) {
    ret = kobject_add(&centry.kobj, kobj, "%s", name);
    if (ret) {
    pr_err("VAS: sysfs kobject add / event failed %d\n",
    ret);
    kobject_put(&centry.kobj);
    }
    }
    return ret;
    }
    static struct miscdevice vas_miscdev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "vas",
    };
//
// Add VAS and VasCaps (overall capabilities) dir entries.
//
#[no_mangle]
pub unsafe extern "C" fn sysfs_pseries_vas_init(vas_caps: *mut vas_all_caps) -> int __init {
    int __init sysfs_pseries_vas_init(struct vas_all_caps *vas_caps)
    {
    int ret;
    ret = misc_register(&vas_miscdev);
    if (ret < 0) {
    pr_err("%s: register vas misc device failed\n", __func__);
    return ret;
    }
//
// The hypervisor does not expose multiple VAS instances, but can
// see multiple VAS instances on PowerNV. So create 'vas0' directory
// on pseries.
//
    pseries_vas_kobj = kobject_create_and_add("vas0",
    &vas_miscdev.this_device.kobj);
    if (!pseries_vas_kobj) {
    misc_deregister(&vas_miscdev);
    pr_err("Failed to create VAS sysfs entry\n");
    return -ENOMEM;
    }
    if ((vas_caps.feat_type & VAS_GZIP_QOS_FEAT_BIT) ||
    (vas_caps.feat_type & VAS_GZIP_DEF_FEAT_BIT)) {
    gzip_caps_kobj = kobject_create_and_add("gzip",
    pseries_vas_kobj);
    if (!gzip_caps_kobj) {
    pr_err("Failed to create VAS GZIP capability entry\n");
    kobject_put(pseries_vas_kobj);
    misc_deregister(&vas_miscdev);
    return -ENOMEM;
    }
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn sysfs_add_vas_caps(caps: *mut vas_cop_feat_caps) -> c_int {
    int sysfs_add_vas_caps(struct vas_cop_feat_caps *caps)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sysfs_pseries_vas_init(vas_caps: *mut vas_all_caps) -> int __init {
    int __init sysfs_pseries_vas_init(struct vas_all_caps *vas_caps)
    {
    return 0;
    }
