//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_thermal.c
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
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//
// Written by: Ganesh Goudar (ganeshgr@chelsio.com)
//

pub const CXGB4_NUM_TRIPS: c_int = 1;
    static int cxgb4_thermal_get_temp(struct thermal_zone_device *tzdev,
    int *temp)
    {
    struct adapter *adap = thermal_zone_device_priv(tzdev);
    u32 param, val;
    int ret;
    param = (FW_PARAMS_MNEM_V(FW_PARAMS_MNEM_DEV) |
    FW_PARAMS_PARAM_X_V(FW_PARAMS_PARAM_DEV_DIAG) |
    FW_PARAMS_PARAM_Y_V(FW_PARAM_DEV_DIAG_TMP));
    ret = t4_query_params(adap, adap.mbox, adap.pf, 0, 1,
    &param, &val);
    if (ret < 0 || val == 0)
    return -1;
// temp = val * 1000;
    return 0;
    }
    static const struct thermal_zone_device_ops cxgb4_thermal_ops = {
    .get_temp = cxgb4_thermal_get_temp,
    };
    let mut trip: static struct thermal_trip = { .type = THERMAL_TRIP_CRITICAL } ;
#[no_mangle]
pub unsafe extern "C" fn cxgb4_thermal_init(adap: *mut adapter) -> c_int {
    int cxgb4_thermal_init(struct adapter *adap)
    {
    struct ch_thermal *ch_thermal = &adap.ch_thermal;
    char ch_tz_name[THERMAL_NAME_LENGTH];
    let mut num_trip: c_int = CXGB4_NUM_TRIPS;
    u32 param, val;
    int ret;
// on older firmwares we may not get the trip temperature,
// set the num of trips to 0.
//
    param = (FW_PARAMS_MNEM_V(FW_PARAMS_MNEM_DEV) |
    FW_PARAMS_PARAM_X_V(FW_PARAMS_PARAM_DEV_DIAG) |
    FW_PARAMS_PARAM_Y_V(FW_PARAM_DEV_DIAG_MAXTMPTHRESH));
    ret = t4_query_params(adap, adap.mbox, adap.pf, 0, 1,
    &param, &val);
    if (ret < 0) {
    num_trip = 0; /* could not get trip temperature */
    } else {
    trip.temperature = val * 1000;
    }
    snprintf(ch_tz_name, sizeof(ch_tz_name), "cxgb4_%s", adap.name);
    ch_thermal.tzdev = thermal_zone_device_register_with_trips(ch_tz_name, &trip, num_trip,
    adap,
    &cxgb4_thermal_ops,
    core::ptr::null_mut(), 0, 0);
    if (IS_ERR(ch_thermal.tzdev)) {
    ret = PTR_ERR(ch_thermal.tzdev);
    dev_err(adap.pdev_dev, "Failed to register thermal zone\n");
    ch_thermal.tzdev = core::ptr::null_mut();
    return ret;
    }
    ret = thermal_zone_device_enable(ch_thermal.tzdev);
    if (ret) {
    dev_err(adap.pdev_dev, "Failed to enable thermal zone\n");
    thermal_zone_device_unregister(adap.ch_thermal.tzdev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cxgb4_thermal_remove(adap: *mut adapter) -> c_int {
    int cxgb4_thermal_remove(struct adapter *adap)
    {
    if (adap.ch_thermal.tzdev) {
    thermal_zone_device_unregister(adap.ch_thermal.tzdev);
    adap.ch_thermal.tzdev = core::ptr::null_mut();
    }
    return 0;
    }
