//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-sensor.c
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
// PowerNV sensor code
//
// Copyright (C) 2013 IBM
//

//
// This will return sensor information to driver based on the requested sensor
// handle. A handle is an opaque id for the powernv, read by the driver from the
// device tree..
//
#[no_mangle]
pub unsafe extern "C" fn opal_get_sensor_data(sensor_hndl: u32, sensor_data: *mut u32) -> c_int {
    int opal_get_sensor_data(u32 sensor_hndl, u32 *sensor_data)
    {
    int ret, token;
    struct opal_msg msg;
    __be32 data;
    token = opal_async_get_token_interruptible();
    if (token < 0)
    return token;
    ret = opal_sensor_read(sensor_hndl, token, &data);
    switch (ret) {
    case OPAL_ASYNC_COMPLETION:
    ret = opal_async_wait_response(token, &msg);
    if (ret) {
    pr_err("%s: Failed to wait for the async response, %d\n",
    __func__, ret);
    goto out;
    }
    ret = opal_error_code(opal_get_async_rc(msg));
// sensor_data = be32_to_cpu(data);
    break;
    case OPAL_SUCCESS:
    ret = 0;
// sensor_data = be32_to_cpu(data);
    break;
    case OPAL_WRONG_STATE:
    ret = -EIO;
    break;
    default:
    ret = opal_error_code(ret);
    break;
    }
    out:
    opal_async_release_token(token);
    return ret;
    }
    EXPORT_SYMBOL_GPL(opal_get_sensor_data);
#[no_mangle]
pub unsafe extern "C" fn opal_get_sensor_data_u64(sensor_hndl: u32, sensor_data: *mut u64) -> c_int {
    int opal_get_sensor_data_u64(u32 sensor_hndl, u64 *sensor_data)
    {
    int ret, token;
    struct opal_msg msg;
    __be64 data;
    if (!opal_check_token(OPAL_SENSOR_READ_U64)) {
    u32 sdata;
    ret = opal_get_sensor_data(sensor_hndl, &sdata);
    if (!ret)
// sensor_data = sdata;
    return ret;
    }
    token = opal_async_get_token_interruptible();
    if (token < 0)
    return token;
    ret = opal_sensor_read_u64(sensor_hndl, token, &data);
    switch (ret) {
    case OPAL_ASYNC_COMPLETION:
    ret = opal_async_wait_response(token, &msg);
    if (ret) {
    pr_err("%s: Failed to wait for the async response, %d\n",
    __func__, ret);
    goto out_token;
    }
    ret = opal_error_code(opal_get_async_rc(msg));
// sensor_data = be64_to_cpu(data);
    break;
    case OPAL_SUCCESS:
    ret = 0;
// sensor_data = be64_to_cpu(data);
    break;
    case OPAL_WRONG_STATE:
    ret = -EIO;
    break;
    default:
    ret = opal_error_code(ret);
    break;
    }
    out_token:
    opal_async_release_token(token);
    return ret;
    }
    EXPORT_SYMBOL_GPL(opal_get_sensor_data_u64);
#[no_mangle]
pub unsafe extern "C" fn opal_sensor_init() -> int __init {
    int __init opal_sensor_init(void)
    {
    struct platform_device *pdev;
    struct device_node *sensor;
    sensor = of_find_node_by_path("/ibm,opal/sensors");
    if (!sensor) {
    pr_err("Opal node 'sensors' not found\n");
    return -ENODEV;
    }
    pdev = of_platform_device_create(sensor, "opal-sensor", core::ptr::null_mut());
    of_node_put(sensor);
    return PTR_ERR_OR_ZERO(pdev);
    }
