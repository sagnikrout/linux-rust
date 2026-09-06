//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/mdev/mdev_sysfs.c
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
// File attributes for Mediated devices
//
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
// Author: Neo Jia <cjia@nvidia.com>
// Kirti Wankhede <kwankhede@nvidia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_type_attribute {
    pub attr: attribute,
    ssize_t (*show)(struct mdev_type *mtype,
    pub buf): *mut *mut mdev_type_attribute attr, char,
    ssize_t (*store)(struct mdev_type *mtype,
    struct mdev_type_attribute *attr, const char *buf,
    pub count): usize,
}

    struct mdev_type_attribute mdev_type_attr_##_name = __ATTR_RO(_name)

    struct mdev_type_attribute mdev_type_attr_##_name = __ATTR_WO(_name)
    static ssize_t mdev_type_attr_show(struct kobject *kobj,
    struct attribute *__attr, char *buf)
    {
    struct mdev_type_attribute *attr = to_mdev_type_attr(__attr);
    struct mdev_type *type = to_mdev_type(kobj);
    let mut ret: isize = -EIO;
    if (attr.show)
    ret = attr.show(type, attr, buf);
    return ret;
    }
    static ssize_t mdev_type_attr_store(struct kobject *kobj,
    struct attribute *__attr,
    const char *buf, size_t count)
    {
    struct mdev_type_attribute *attr = to_mdev_type_attr(__attr);
    struct mdev_type *type = to_mdev_type(kobj);
    let mut ret: isize = -EIO;
    if (attr.store)
    ret = attr.store(type, attr, buf, count);
    return ret;
    }
    static const struct sysfs_ops mdev_type_sysfs_ops = {
    .show = mdev_type_attr_show,
    .store = mdev_type_attr_store,
    };
    static ssize_t create_store(struct mdev_type *mtype,
    struct mdev_type_attribute *attr, const char *buf,
    size_t count)
    {
    char *str;
    guid_t uuid;
    int ret;
    if ((count < UUID_STRING_LEN) || (count > UUID_STRING_LEN + 1))
    return -EINVAL;
    str = kstrndup(buf, count, GFP_KERNEL);
    if (!str)
    return -ENOMEM;
    ret = guid_parse(str, &uuid);
    kfree(str);
    if (ret)
    return ret;
    ret = mdev_device_create(mtype, &uuid);
    if (ret)
    return ret;
    return count;
    }
    static MDEV_TYPE_ATTR_WO(create);
    static ssize_t device_api_show(struct mdev_type *mtype,
    struct mdev_type_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%s\n", mtype.parent.mdev_driver.device_api);
    }
    static MDEV_TYPE_ATTR_RO(device_api);
    static ssize_t name_show(struct mdev_type *mtype,
    struct mdev_type_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    mtype.pretty_name ? mtype.pretty_name : mtype.sysfs_name);
    }
    static MDEV_TYPE_ATTR_RO(name);
    static ssize_t available_instances_show(struct mdev_type *mtype,
    struct mdev_type_attribute *attr,
    char *buf)
    {
    struct mdev_driver *drv = mtype.parent.mdev_driver;
    if (drv.get_available)
    return sysfs_emit(buf, "%u\n", drv.get_available(mtype));
    return sysfs_emit(buf, "%u\n",
    atomic_read(&mtype.parent.available_instances));
    }
    static MDEV_TYPE_ATTR_RO(available_instances);
    static ssize_t description_show(struct mdev_type *mtype,
    struct mdev_type_attribute *attr,
    char *buf)
    {
    return mtype.parent.mdev_driver.show_description(mtype, buf);
    }
    static MDEV_TYPE_ATTR_RO(description);
    static struct attribute *mdev_types_core_attrs[] = {
    &mdev_type_attr_create.attr,
    &mdev_type_attr_device_api.attr,
    &mdev_type_attr_name.attr,
    &mdev_type_attr_available_instances.attr,
    &mdev_type_attr_description.attr,
    core::ptr::null_mut(),
    };
    static umode_t mdev_types_core_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    if (attr == &mdev_type_attr_description.attr &&
    !to_mdev_type(kobj).parent.mdev_driver.show_description)
    return 0;
    return attr.mode;
    }
    static struct attribute_group mdev_type_core_group = {
    .attrs = mdev_types_core_attrs,
    .is_visible = mdev_types_core_is_visible,
    };
    static const struct attribute_group *mdev_type_groups[] = {
    &mdev_type_core_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn mdev_type_release(kobj: *mut kobject) {
    static void mdev_type_release(struct kobject *kobj)
    {
    struct mdev_type *type = to_mdev_type(kobj);
    pr_debug("Releasing group %s\n", kobj.name);
// Pairs with the get in mdev_type_add()
    put_device(type.parent.dev);
    }
    static const struct kobj_type mdev_type_ktype = {
    .sysfs_ops	= &mdev_type_sysfs_ops,
    .release	= mdev_type_release,
    .default_groups	= mdev_type_groups,
    };
#[no_mangle]
unsafe extern "C" fn mdev_type_add(parent: *mut mdev_parent, type: *mut mdev_type) -> c_int {
    static int mdev_type_add(struct mdev_parent *parent, struct mdev_type *type)
    {
    int ret;
    type.kobj.kset = parent.mdev_types_kset;
    type.parent = parent;
// Pairs with the put in mdev_type_release()
    get_device(parent.dev);
    ret = kobject_init_and_add(&type.kobj, &mdev_type_ktype, core::ptr::null_mut(),
    "%s-%s", dev_driver_string(parent.dev),
    type.sysfs_name);
    if (ret) {
    kobject_put(&type.kobj);
    return ret;
    }
    type.devices_kobj = kobject_create_and_add("devices", &type.kobj);
    if (!type.devices_kobj) {
    ret = -ENOMEM;
    goto attr_devices_failed;
    }
    return 0;
    attr_devices_failed:
    kobject_del(&type.kobj);
    kobject_put(&type.kobj);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mdev_type_remove(type: *mut mdev_type) {
    static void mdev_type_remove(struct mdev_type *type)
    {
    kobject_put(type.devices_kobj);
    kobject_del(&type.kobj);
    kobject_put(&type.kobj);
    }
// mdev sysfs functions
#[no_mangle]
pub unsafe extern "C" fn parent_remove_sysfs_files(parent: *mut mdev_parent) {
    void parent_remove_sysfs_files(struct mdev_parent *parent)
    {
    int i;
    for (i = 0; i < parent.nr_types; i++)
    mdev_type_remove(parent.types[i]);
    kset_unregister(parent.mdev_types_kset);
    }
#[no_mangle]
pub unsafe extern "C" fn parent_create_sysfs_files(parent: *mut mdev_parent) -> c_int {
    int parent_create_sysfs_files(struct mdev_parent *parent)
    {
    int ret, i;
    parent.mdev_types_kset = kset_create_and_add("mdev_supported_types",
    core::ptr::null_mut(), &parent.dev.kobj);
    if (!parent.mdev_types_kset)
    return -ENOMEM;
    for (i = 0; i < parent.nr_types; i++) {
    ret = mdev_type_add(parent, parent.types[i]);
    if (ret)
    goto out_err;
    }
    return 0;
    out_err:
    while (--i >= 0)
    mdev_type_remove(parent.types[i]);
    kset_unregister(parent.mdev_types_kset);
    return ret;
    }
    static ssize_t remove_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct mdev_device *mdev = to_mdev_device(dev);
    unsigned long val;
    if (kstrtoul(buf, 0, &val) < 0)
    return -EINVAL;
    if (val && device_remove_file_self(dev, attr)) {
    int ret;
    ret = mdev_device_remove(mdev);
    if (ret)
    return ret;
    }
    return count;
    }
    static DEVICE_ATTR_WO(remove);
    static struct attribute *mdev_device_attrs[] = {
    &dev_attr_remove.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group mdev_device_group = {
    .attrs = mdev_device_attrs,
    };
    const struct attribute_group *mdev_device_groups[] = {
    &mdev_device_group,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn mdev_create_sysfs_files(mdev: *mut mdev_device) -> c_int {
    int mdev_create_sysfs_files(struct mdev_device *mdev)
    {
    struct mdev_type *type = mdev.type;
    struct kobject *kobj = &mdev.dev.kobj;
    int ret;
    ret = sysfs_create_link(type.devices_kobj, kobj, dev_name(&mdev.dev));
    if (ret)
    return ret;
    ret = sysfs_create_link(kobj, &type.kobj, "mdev_type");
    if (ret)
    goto type_link_failed;
    return ret;
    type_link_failed:
    sysfs_remove_link(mdev.type.devices_kobj, dev_name(&mdev.dev));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mdev_remove_sysfs_files(mdev: *mut mdev_device) {
    void mdev_remove_sysfs_files(struct mdev_device *mdev)
    {
    struct kobject *kobj = &mdev.dev.kobj;
    sysfs_remove_link(kobj, "mdev_type");
    sysfs_remove_link(mdev.type.devices_kobj, dev_name(&mdev.dev));
    }
