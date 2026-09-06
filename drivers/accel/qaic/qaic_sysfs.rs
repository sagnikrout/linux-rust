//! Automatically rewritten from C to Rust
//! Source: drivers/accel/qaic/qaic_sysfs.c
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
// Copyright (c) 2020-2025, The Linux Foundation. All rights reserved.

pub const NAME_LEN: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_attribute {
    pub dev_attr: device_attribute,
    pub dbc_id: u32,
    pub name: [c_char; NAME_LEN],
}

#[no_mangle]
unsafe extern "C" fn dbc_state_show(dev: *mut device, a: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t dbc_state_show(struct device *dev, struct device_attribute *a, char *buf)
    {
    struct dbc_attribute *dbc_attr = container_of(a, struct dbc_attribute, dev_attr);
    struct drm_minor *minor = dev_get_drvdata(dev);
    struct qaic_device *qdev;
    qdev = to_qaic_device(minor.dev);
    return sysfs_emit(buf, "%d\n", qdev.dbc[dbc_attr.dbc_id].state);
    }
#[no_mangle]
pub unsafe extern "C" fn set_dbc_state(qdev: *mut qaic_device, dbc_id: u32, state: c_uint) {
    void set_dbc_state(struct qaic_device *qdev, u32 dbc_id, unsigned int state)
    {
    struct device *kdev = to_accel_kdev(qdev.qddev);
    char *envp[3] = {};
    char state_str[16];
    char id_str[12];
    envp[0] = id_str;
    envp[1] = state_str;
    if (state >= DBC_STATE_MAX)
    return;
    if (dbc_id >= qdev.num_dbc)
    return;
    if (state == qdev.dbc[dbc_id].state)
    return;
    scnprintf(id_str, ARRAY_SIZE(id_str), "DBC_ID=%d", dbc_id);
    scnprintf(state_str, ARRAY_SIZE(state_str), "DBC_STATE=%d", state);
    qdev.dbc[dbc_id].state = state;
    kobject_uevent_env(&kdev.kobj, KOBJ_CHANGE, envp);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_sysfs_init(qddev: *mut qaic_drm_device) -> c_int {
    int qaic_sysfs_init(struct qaic_drm_device *qddev)
    {
    struct device *kdev = to_accel_kdev(qddev);
    struct drm_device *drm = to_drm(qddev);
    let mut num_dbc: u32 = qddev.qdev.num_dbc;
    struct dbc_attribute *dbc_attrs;
    int i, ret;
    dbc_attrs = drmm_kcalloc(drm, num_dbc, sizeof(*dbc_attrs), GFP_KERNEL);
    if (!dbc_attrs)
    return -ENOMEM;
    for (i = 0; i < num_dbc; ++i) {
    struct dbc_attribute *dbc_attr = &dbc_attrs[i];
    sysfs_attr_init(&dbc_attr.dev_attr.attr);
    dbc_attr.dbc_id = i;
    scnprintf(dbc_attr.name, NAME_LEN, "dbc%d_state", i);
    dbc_attr.dev_attr.attr.name = dbc_attr.name;
    dbc_attr.dev_attr.attr.mode = 0444;
    dbc_attr.dev_attr.show = dbc_state_show;
    ret = sysfs_create_file(&kdev.kobj, &dbc_attr.dev_attr.attr);
    if (ret) {
    int j;
    for (j = 0; j < i; ++j) {
    dbc_attr = &dbc_attrs[j];
    sysfs_remove_file(&kdev.kobj, &dbc_attr.dev_attr.attr);
    }
    drmm_kfree(drm, dbc_attrs);
    return ret;
    }
    }
    qddev.sysfs_attrs = dbc_attrs;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_sysfs_remove(qddev: *mut qaic_drm_device) {
    void qaic_sysfs_remove(struct qaic_drm_device *qddev)
    {
    struct dbc_attribute *dbc_attrs = qddev.sysfs_attrs;
    struct device *kdev = to_accel_kdev(qddev);
    let mut num_dbc: u32 = qddev.qdev.num_dbc;
    int i;
    if (!dbc_attrs)
    return;
    qddev.sysfs_attrs = core::ptr::null_mut();
    for (i = 0; i < num_dbc; ++i)
    sysfs_remove_file(&kdev.kobj, &dbc_attrs[i].dev_attr.attr);
    drmm_kfree(to_drm(qddev), dbc_attrs);
    }
