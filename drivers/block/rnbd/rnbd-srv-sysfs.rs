//! Automatically rewritten from C to Rust
//! Source: drivers/block/rnbd/rnbd-srv-sysfs.c
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
// RDMA Network Block Driver
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

    static struct device *rnbd_dev;
    static const struct class rnbd_dev_class = {
    .name = "rnbd-server",
    };
    static struct kobject *rnbd_devs_kobj;
#[no_mangle]
unsafe extern "C" fn rnbd_srv_dev_release(kobj: *mut kobject) {
    static void rnbd_srv_dev_release(struct kobject *kobj)
    {
    struct rnbd_srv_dev *dev;
    dev = container_of(kobj, struct rnbd_srv_dev, dev_kobj);
    kfree(dev);
    }
    static const struct kobj_type dev_ktype = {
    .sysfs_ops = &kobj_sysfs_ops,
    .release = rnbd_srv_dev_release
    };
    int rnbd_srv_create_dev_sysfs(struct rnbd_srv_dev *dev,
    struct block_device *bdev)
    {
    struct kobject *bdev_kobj;
    int ret;
    ret = kobject_init_and_add(&dev.dev_kobj, &dev_ktype,
    rnbd_devs_kobj, "%pg", bdev);
    if (ret) {
    kobject_put(&dev.dev_kobj);
    return ret;
    }
    dev.dev_sessions_kobj = kobject_create_and_add("sessions",
    &dev.dev_kobj);
    if (!dev.dev_sessions_kobj) {
    ret = -ENOMEM;
    goto free_dev_kobj;
    }
    bdev_kobj = &disk_to_dev(bdev.bd_disk).kobj;
    ret = sysfs_create_link(&dev.dev_kobj, bdev_kobj, "block_dev");
    if (ret)
    goto put_sess_kobj;
    return 0;
    put_sess_kobj:
    kobject_put(dev.dev_sessions_kobj);
    free_dev_kobj:
    kobject_del(&dev.dev_kobj);
    kobject_put(&dev.dev_kobj);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rnbd_srv_destroy_dev_sysfs(dev: *mut rnbd_srv_dev) {
    void rnbd_srv_destroy_dev_sysfs(struct rnbd_srv_dev *dev)
    {
    sysfs_remove_link(&dev.dev_kobj, "block_dev");
    kobject_del(dev.dev_sessions_kobj);
    kobject_put(dev.dev_sessions_kobj);
    kobject_del(&dev.dev_kobj);
    kobject_put(&dev.dev_kobj);
    }
    static ssize_t read_only_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *page)
    {
    struct rnbd_srv_sess_dev *sess_dev;
    sess_dev = container_of(kobj, struct rnbd_srv_sess_dev, kobj);
    return sysfs_emit(page, "%d\n", sess_dev.readonly);
    }
    static struct kobj_attribute rnbd_srv_dev_session_ro_attr =
    __ATTR_RO(read_only);
    static ssize_t access_mode_show(struct kobject *kobj,
    struct kobj_attribute *attr,
    char *page)
    {
    struct rnbd_srv_sess_dev *sess_dev;
    sess_dev = container_of(kobj, struct rnbd_srv_sess_dev, kobj);
    return sysfs_emit(page, "%s\n",
    rnbd_access_modes[sess_dev.access_mode].str);
    }
    static struct kobj_attribute rnbd_srv_dev_session_access_mode_attr =
    __ATTR_RO(access_mode);
    static ssize_t mapping_path_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *page)
    {
    struct rnbd_srv_sess_dev *sess_dev;
    sess_dev = container_of(kobj, struct rnbd_srv_sess_dev, kobj);
    return sysfs_emit(page, "%s\n", sess_dev.pathname);
    }
    static struct kobj_attribute rnbd_srv_dev_session_mapping_path_attr =
    __ATTR_RO(mapping_path);
    static ssize_t rnbd_srv_dev_session_force_close_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *page)
    {
    return sysfs_emit(page, "Usage: echo 1 > %s\n",
    attr.attr.name);
    }
    static ssize_t rnbd_srv_dev_session_force_close_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct rnbd_srv_sess_dev *sess_dev;
    sess_dev = container_of(kobj, struct rnbd_srv_sess_dev, kobj);
    if (!sysfs_streq(buf, "1")) {
    rnbd_srv_err(sess_dev, "%s: invalid value: '%s'\n",
    attr.attr.name, buf);
    return -EINVAL;
    }
    rnbd_srv_info(sess_dev, "force close requested\n");
    rnbd_srv_sess_dev_force_close(sess_dev, attr);
    return count;
    }
    static struct kobj_attribute rnbd_srv_dev_session_force_close_attr =
    __ATTR(force_close, 0644,
    rnbd_srv_dev_session_force_close_show,
    rnbd_srv_dev_session_force_close_store);
    static struct attribute *rnbd_srv_default_dev_sessions_attrs[] = {
    &rnbd_srv_dev_session_access_mode_attr.attr,
    &rnbd_srv_dev_session_ro_attr.attr,
    &rnbd_srv_dev_session_mapping_path_attr.attr,
    &rnbd_srv_dev_session_force_close_attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group rnbd_srv_default_dev_session_attr_group = {
    .attrs = rnbd_srv_default_dev_sessions_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn rnbd_srv_destroy_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev) {
    void rnbd_srv_destroy_dev_session_sysfs(struct rnbd_srv_sess_dev *sess_dev)
    {
    sysfs_remove_group(&sess_dev.kobj,
    &rnbd_srv_default_dev_session_attr_group);
    kobject_del(&sess_dev.kobj);
    kobject_put(&sess_dev.kobj);
    }
#[no_mangle]
unsafe extern "C" fn rnbd_srv_sess_dev_release(kobj: *mut kobject) {
    static void rnbd_srv_sess_dev_release(struct kobject *kobj)
    {
    struct rnbd_srv_sess_dev *sess_dev;
    sess_dev = container_of(kobj, struct rnbd_srv_sess_dev, kobj);
    rnbd_destroy_sess_dev(sess_dev, sess_dev.keep_id);
    }
    static const struct kobj_type rnbd_srv_sess_dev_ktype = {
    .sysfs_ops	= &kobj_sysfs_ops,
    .release	= rnbd_srv_sess_dev_release,
    };
#[no_mangle]
pub unsafe extern "C" fn rnbd_srv_create_dev_session_sysfs(sess_dev: *mut rnbd_srv_sess_dev) -> c_int {
    int rnbd_srv_create_dev_session_sysfs(struct rnbd_srv_sess_dev *sess_dev)
    {
    int ret;
    ret = kobject_init_and_add(&sess_dev.kobj, &rnbd_srv_sess_dev_ktype,
    sess_dev.dev.dev_sessions_kobj, "%s",
    sess_dev.sess.sessname);
    if (ret) {
    kobject_put(&sess_dev.kobj);
    return ret;
    }
    ret = sysfs_create_group(&sess_dev.kobj,
    &rnbd_srv_default_dev_session_attr_group);
    if (ret) {
    kobject_del(&sess_dev.kobj);
    kobject_put(&sess_dev.kobj);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rnbd_srv_create_sysfs_files() -> c_int {
    int rnbd_srv_create_sysfs_files(void)
    {
    int err;
    err = class_register(&rnbd_dev_class);
    if (err)
    return err;
    rnbd_dev = device_create(&rnbd_dev_class, core::ptr::null_mut(),
    MKDEV(0, 0), core::ptr::null_mut(), "ctl");
    if (IS_ERR(rnbd_dev)) {
    err = PTR_ERR(rnbd_dev);
    goto cls_destroy;
    }
    rnbd_devs_kobj = kobject_create_and_add("devices", &rnbd_dev.kobj);
    if (!rnbd_devs_kobj) {
    err = -ENOMEM;
    goto dev_destroy;
    }
    return 0;
    dev_destroy:
    device_destroy(&rnbd_dev_class, MKDEV(0, 0));
    cls_destroy:
    class_unregister(&rnbd_dev_class);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn rnbd_srv_destroy_sysfs_files() {
    void rnbd_srv_destroy_sysfs_files(void)
    {
    kobject_del(rnbd_devs_kobj);
    kobject_put(rnbd_devs_kobj);
    device_destroy(&rnbd_dev_class, MKDEV(0, 0));
    class_unregister(&rnbd_dev_class);
    }
