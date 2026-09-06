//! Automatically rewritten from C to Rust
//! Source: tools/lib/thermal/thermal.c
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


// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

#[no_mangle]
pub unsafe extern "C" fn for_each_thermal_threshold(th: *mut thermal_threshold, cb: cb_th_t, arg: *mut c_void) -> c_int {
    int for_each_thermal_threshold(struct thermal_threshold *th, cb_th_t cb, void *arg)
    {
    int i, ret = 0;
    if (!th)
    return 0;
    for (i = 0; th[i].temperature != INT_MAX; i++)
    ret |= cb(&th[i], arg);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn for_each_thermal_cdev(cdev: *mut thermal_cdev, cb: cb_tc_t, arg: *mut c_void) -> c_int {
    int for_each_thermal_cdev(struct thermal_cdev *cdev, cb_tc_t cb, void *arg)
    {
    int i, ret = 0;
    if (!cdev)
    return 0;
    for (i = 0; cdev[i].id != -1; i++)
    ret |= cb(&cdev[i], arg);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn for_each_thermal_trip(tt: *mut thermal_trip, cb: cb_tt_t, arg: *mut c_void) -> c_int {
    int for_each_thermal_trip(struct thermal_trip *tt, cb_tt_t cb, void *arg)
    {
    int i, ret = 0;
    if (!tt)
    return 0;
    for (i = 0; tt[i].id != -1; i++)
    ret |= cb(&tt[i], arg);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn for_each_thermal_zone(tz: *mut thermal_zone, cb: cb_tz_t, arg: *mut c_void) -> c_int {
    int for_each_thermal_zone(struct thermal_zone *tz, cb_tz_t cb, void *arg)
    {
    int i, ret = 0;
    if (!tz)
    return 0;
    for (i = 0; tz[i].id != -1; i++)
    ret |= cb(&tz[i], arg);
    return ret;
    }
    struct thermal_zone *thermal_zone_find_by_name(struct thermal_zone *tz,
    const char *name)
    {
    int i;
    if (!tz || !name)
    return core::ptr::null_mut();
    for (i = 0; tz[i].id != -1; i++) {
    if (!strcmp(tz[i].name, name))
    return &tz[i];
    }
    return core::ptr::null_mut();
    }
    struct thermal_zone *thermal_zone_find_by_id(struct thermal_zone *tz, int id)
    {
    int i;
    if (!tz || id < 0)
    return core::ptr::null_mut();
    for (i = 0; tz[i].id != -1; i++) {
    if (tz[i].id == id)
    return &tz[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __thermal_zone_discover(tz: *mut thermal_zone, th: *mut c_void) -> c_int {
    static int __thermal_zone_discover(struct thermal_zone *tz, void *th)
    {
    if (thermal_cmd_get_trip(th, tz) < 0)
    return -1;
    if (thermal_cmd_threshold_get(th, tz))
    return -1;
    if (thermal_cmd_get_governor(th, tz))
    return -1;
    return 0;
    }
    struct thermal_zone *thermal_zone_discover(struct thermal_handler *th)
    {
    struct thermal_zone *tz;
    if (thermal_cmd_get_tz(th, &tz) < 0)
    return core::ptr::null_mut();
    if (for_each_thermal_zone(tz, __thermal_zone_discover, th))
    return core::ptr::null_mut();
    return tz;
    }
#[no_mangle]
pub unsafe extern "C" fn thermal_exit(th: *mut thermal_handler) {
    void thermal_exit(struct thermal_handler *th)
    {
    thermal_cmd_exit(th);
    thermal_events_exit(th);
    thermal_sampling_exit(th);
    free(th);
    }
    struct thermal_handler *thermal_init(struct thermal_ops *ops)
    {
    struct thermal_handler *th;
    th = malloc(sizeof(*th));
    if (!th)
    return core::ptr::null_mut();
    th.ops = ops;
    if (thermal_events_init(th))
    goto out_free;
    if (thermal_sampling_init(th))
    goto out_free;
    if (thermal_cmd_init(th))
    goto out_free;
    return th;
    out_free:
    free(th);
    return core::ptr::null_mut();
    }
