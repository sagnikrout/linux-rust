//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/qcom/qcom_qseecom.c
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
// Driver for Qualcomm Secure Execution Environment (SEE) interface (QSEECOM).
// Responsible for setting up and managing QSEECOM client devices.
//
// Copyright (C) 2023 Maximilian Luz <luzmaximilian@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qseecom_app_desc {
    pub app_name: *const c_char,
    pub dev_name: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn qseecom_client_release(dev: *mut device) {
    static void qseecom_client_release(struct device *dev)
    {
    struct qseecom_client *client;
    client = container_of(dev, struct qseecom_client, aux_dev.dev);
    kfree(client);
    }
#[no_mangle]
unsafe extern "C" fn qseecom_client_remove(data: *mut c_void) {
    static void qseecom_client_remove(void *data)
    {
    struct qseecom_client *client = data;
    auxiliary_device_delete(&client.aux_dev);
    auxiliary_device_uninit(&client.aux_dev);
    }
    static int qseecom_client_register(struct platform_device *qseecom_dev,
    const struct qseecom_app_desc *desc)
    {
    struct qseecom_client *client;
    u32 app_id;
    int ret;
// Try to find the app ID, skip device if not found
    ret = qcom_scm_qseecom_app_get_id(desc.app_name, &app_id);
    if (ret)
    let mut ret: return = = -ENOENT ? 0 : ret;
    dev_info(&qseecom_dev.dev, "setting up client for %s\n", desc.app_name);
// Allocate and set-up the client device
    client = kzalloc_obj(*client);
    if (!client)
    return -ENOMEM;
    client.aux_dev.name = desc.dev_name;
    client.aux_dev.dev.parent = &qseecom_dev.dev;
    client.aux_dev.dev.release = qseecom_client_release;
    client.app_id = app_id;
    ret = auxiliary_device_init(&client.aux_dev);
    if (ret) {
    kfree(client);
    return ret;
    }
    ret = auxiliary_device_add(&client.aux_dev);
    if (ret) {
    auxiliary_device_uninit(&client.aux_dev);
    return ret;
    }
    ret = devm_add_action_or_reset(&qseecom_dev.dev, qseecom_client_remove, client);
    if (ret)
    return ret;
    return 0;
    }
//
// List of supported applications. One client device will be created per entry,
// assuming the app has already been loaded (usually by firmware bootloaders)
// and its ID can be queried successfully.
//
    static const struct qseecom_app_desc qcom_qseecom_apps[] = {
    { "qcom.tz.uefisecapp", "uefisecapp" },
    };
#[no_mangle]
unsafe extern "C" fn qcom_qseecom_probe(qseecom_dev: *mut platform_device) -> c_int {
    static int qcom_qseecom_probe(struct platform_device *qseecom_dev)
    {
    int ret;
    int i;
// Set up client devices for each base application
    for (i = 0; i < ARRAY_SIZE(qcom_qseecom_apps); i++) {
    ret = qseecom_client_register(qseecom_dev, &qcom_qseecom_apps[i]);
    if (ret)
    return ret;
    }
    return 0;
    }
    static struct platform_driver qcom_qseecom_driver = {
    .driver = {
    .name	= "qcom_qseecom",
    },
    .probe = qcom_qseecom_probe,
    };
#[no_mangle]
unsafe extern "C" fn qcom_qseecom_init() -> int __init {
    static int __init qcom_qseecom_init(void)
    {
    return platform_driver_register(&qcom_qseecom_driver);
    }
    subsys_initcall(qcom_qseecom_init);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Driver for the Qualcomm SEE (QSEECOM) interface");
    MODULE_LICENSE("GPL");
