//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amd/x3d_vcache.c
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
// AMD 3D V-Cache Performance Optimizer Driver
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
// Perry Yuan <perry.yuan@amd.com>
// Mario Limonciello <mario.limonciello@amd.com>
//

    static char *x3d_mode = "frequency";
    module_param(x3d_mode, charp, 0);
    MODULE_PARM_DESC(x3d_mode, "Initial 3D-VCache mode; 'frequency' (default) or 'cache'");
pub const DSM_REVISION_ID: c_int = 0;
pub const DSM_SET_X3D_MODE: c_int = 1;
    static guid_t x3d_guid = GUID_INIT(0xdff8e55f, 0xbcfd, 0x46fb, 0xba, 0x0a,
    0xef, 0xd0, 0x45, 0x0f, 0x34, 0xee);
    enum amd_x3d_mode_type {
    MODE_INDEX_FREQ,
    MODE_INDEX_CACHE,
    };
    static const char * const amd_x3d_mode_strings[] = {
    [MODE_INDEX_FREQ] = "frequency",
    [MODE_INDEX_CACHE] = "cache",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_x3d_dev {
    pub dev: *mut device,
    pub ahandle: acpi_handle,
// To protect x3d mode setting
    pub lock: mutex,
    pub curr_mode: enum amd_x3d_mode_type,
}

#[no_mangle]
unsafe extern "C" fn amd_x3d_get_mode(data: *mut amd_x3d_dev) -> c_int {
    static int amd_x3d_get_mode(struct amd_x3d_dev *data)
    {
    guard(mutex)(&data.lock);
    return data.curr_mode;
    }
#[no_mangle]
unsafe extern "C" fn amd_x3d_mode_switch(data: *mut amd_x3d_dev, new_state: c_int) -> c_int {
    static int amd_x3d_mode_switch(struct amd_x3d_dev *data, int new_state)
    {
    union acpi_object *out, argv;
    guard(mutex)(&data.lock);
    argv.type = ACPI_TYPE_INTEGER;
    argv.integer.value = new_state;
    out = acpi_evaluate_dsm(data.ahandle, &x3d_guid, DSM_REVISION_ID,
    DSM_SET_X3D_MODE, &argv);
    if (!out) {
    dev_err(data.dev, "failed to evaluate _DSM\n");
    return -EINVAL;
    }
    data.curr_mode = new_state;
    kfree(out);
    return 0;
    }
    static ssize_t amd_x3d_mode_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct amd_x3d_dev *data = dev_get_drvdata(dev);
    int ret;
    ret = sysfs_match_string(amd_x3d_mode_strings, buf);
    if (ret < 0)
    return ret;
    ret = amd_x3d_mode_switch(data, ret);
    if (ret < 0)
    return ret;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn amd_x3d_mode_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t amd_x3d_mode_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct amd_x3d_dev *data = dev_get_drvdata(dev);
    let mut mode: c_int = amd_x3d_get_mode(data);
    return sysfs_emit(buf, "%s\n", amd_x3d_mode_strings[mode]);
    }
    static DEVICE_ATTR_RW(amd_x3d_mode);
    static struct attribute *amd_x3d_attrs[] = {
    &dev_attr_amd_x3d_mode.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(amd_x3d);
#[no_mangle]
unsafe extern "C" fn amd_x3d_resume_handler(dev: *mut device) -> c_int {
    static int amd_x3d_resume_handler(struct device *dev)
    {
    struct amd_x3d_dev *data = dev_get_drvdata(dev);
    let mut ret: c_int = amd_x3d_get_mode(data);
    return amd_x3d_mode_switch(data, ret);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(amd_x3d_pm, core::ptr::null_mut(), amd_x3d_resume_handler);
    static const struct acpi_device_id amd_x3d_acpi_ids[] = {
    {"AMDI0101"},
    { },
    };
    MODULE_DEVICE_TABLE(acpi, amd_x3d_acpi_ids);
#[no_mangle]
unsafe extern "C" fn amd_x3d_probe(pdev: *mut platform_device) -> c_int {
    static int amd_x3d_probe(struct platform_device *pdev)
    {
    struct amd_x3d_dev *data;
    acpi_handle handle;
    int ret;
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return -ENODEV;
    if (!acpi_check_dsm(handle, &x3d_guid, DSM_REVISION_ID, BIT(DSM_SET_X3D_MODE)))
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = &pdev.dev;
    ret = devm_mutex_init(data.dev, &data.lock);
    if (ret)
    return ret;
    data.ahandle = handle;
    platform_set_drvdata(pdev, data);
    ret = match_string(amd_x3d_mode_strings, ARRAY_SIZE(amd_x3d_mode_strings), x3d_mode);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, -EINVAL, "invalid mode %s\n", x3d_mode);
    return amd_x3d_mode_switch(data, ret);
    }
    static struct platform_driver amd_3d_vcache_driver = {
    .driver = {
    .name = "amd_x3d_vcache",
    .dev_groups = amd_x3d_groups,
    .acpi_match_table = amd_x3d_acpi_ids,
    .pm = pm_sleep_ptr(&amd_x3d_pm),
    },
    .probe = amd_x3d_probe,
    };
    module_platform_driver(amd_3d_vcache_driver);
    MODULE_DESCRIPTION("AMD 3D V-Cache Performance Optimizer Driver");
    MODULE_LICENSE("GPL");
