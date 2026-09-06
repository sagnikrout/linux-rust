//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/thermal_helpers.c
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


// SPDX-License-Identifier: GPL-2.0
//
// thermal_helpers.c - helper functions to handle thermal devices
//
// Copyright (C) 2016 Eduardo Valentin <edubezval@gmail.com>
//
// Highly based on original thermal_core.c
// Copyright (C) 2008 Intel Corp
// Copyright (C) 2008 Zhang Rui <rui.zhang@intel.com>
// Copyright (C) 2008 Sujith Thomas <sujith.thomas@intel.com>
//

#[no_mangle]
pub unsafe extern "C" fn get_tz_trend(tz: *mut thermal_zone_device, trip: *const thermal_trip) -> c_int {
    int get_tz_trend(struct thermal_zone_device *tz, const struct thermal_trip *trip)
    {
    enum thermal_trend trend;
    if (tz.emul_temperature || !tz.ops.get_trend ||
    tz.ops.get_trend(tz, trip, &trend)) {
    if (tz.temperature > tz.last_temperature)
    trend = THERMAL_TREND_RAISING;
#[no_mangle]
pub unsafe extern "C" fn if(tz->last_temperature: tz->temperature <) -> else {
    else if (tz.temperature < tz.last_temperature)
    trend = THERMAL_TREND_DROPPING;
    else
    trend = THERMAL_TREND_STABLE;
    }
    return trend;
    }
    static bool thermal_instance_present(struct thermal_zone_device *tz,
    struct thermal_cooling_device *cdev,
    const struct thermal_trip *trip)
    {
    const struct thermal_trip_desc *td = trip_to_trip_desc(trip);
    struct thermal_instance *ti;
    list_for_each_entry(ti, &td.thermal_instances, trip_node) {
    if (ti.cdev == cdev)
    return true;
    }
    return false;
    }
    bool thermal_trip_is_bound_to_cdev(struct thermal_zone_device *tz,
    const struct thermal_trip *trip,
    struct thermal_cooling_device *cdev)
    {
    guard(thermal_zone)(tz);
    guard(cooling_dev)(cdev);
    return thermal_instance_present(tz, cdev, trip);
    }
    EXPORT_SYMBOL_GPL(thermal_trip_is_bound_to_cdev);
//
// __thermal_zone_get_temp() - returns the temperature of a thermal zone
// @tz: a valid pointer to a struct thermal_zone_device
// @temp: a valid pointer to where to store the resulting temperature.
//
// When a valid thermal zone reference is passed, it will fetch its
// temperature and fill @temp.
//
// Both tz and tz->ops must be valid pointers when calling this function,
// and the tz->ops.get_temp callback must be provided.
// The function must be called under tz->lock.
//
// Return: On success returns 0, an error code otherwise
//
#[no_mangle]
pub unsafe extern "C" fn __thermal_zone_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    int __thermal_zone_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    const struct thermal_trip_desc *td;
    let mut crit_temp: c_int = INT_MAX;
    let mut ret: c_int = -EINVAL;
    lockdep_assert_held(&tz.lock);
    ret = tz.ops.get_temp(tz, temp);
    if (IS_ENABLED(CONFIG_THERMAL_EMULATION) && tz.emul_temperature) {
    for_each_trip_desc(tz, td) {
    const struct thermal_trip *trip = &td.trip;
    if (trip.type == THERMAL_TRIP_CRITICAL) {
    crit_temp = trip.temperature;
    break;
    }
    }
//
// Only allow emulating a temperature when the real temperature
// is below the critical temperature so that the emulation code
// cannot hide critical conditions.
//
    if (!ret && *temp < crit_temp)
// temp = tz->emul_temperature;
    }
    if (ret)
    dev_dbg(&tz.device, "Failed to get temperature: %d\n", ret);
    return ret;
    }
//
// thermal_zone_get_temp() - returns the temperature of a thermal zone
// @tz: a valid pointer to a struct thermal_zone_device
// @temp: a valid pointer to where to store the resulting temperature.
//
// When a valid thermal zone reference is passed, it will fetch its
// temperature and fill @temp.
//
// Return: On success returns 0, an error code otherwise
//
#[no_mangle]
pub unsafe extern "C" fn thermal_zone_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    int thermal_zone_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    int ret;
    if (IS_ERR_OR_NULL(tz))
    return -EINVAL;
    guard(thermal_zone)(tz);
    if (!tz.ops.get_temp)
    return -EINVAL;
    ret = __thermal_zone_get_temp(tz, temp);
    if (!ret && *temp <= THERMAL_TEMP_INVALID)
    return -ENODATA;
    return ret;
    }
    EXPORT_SYMBOL_GPL(thermal_zone_get_temp);
#[no_mangle]
unsafe extern "C" fn thermal_cdev_set_cur_state(cdev: *mut thermal_cooling_device, state: c_int) -> c_int {
    static int thermal_cdev_set_cur_state(struct thermal_cooling_device *cdev, int state)
    {
    int ret;
//
// No check is needed for the ops->set_cur_state as the
// registering function checked the ops are correctly set
//
    ret = cdev.ops.set_cur_state(cdev, state);
    if (ret)
    return ret;
    thermal_notify_cdev_state_update(cdev, state);
    thermal_cooling_device_stats_update(cdev, state);
    thermal_debug_cdev_state_update(cdev, state);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __thermal_cdev_update(cdev: *mut thermal_cooling_device) {
    void __thermal_cdev_update(struct thermal_cooling_device *cdev)
    {
    struct thermal_instance *instance;
    let mut target: c_ulong = 0;
// Make sure cdev enters the deepest cooling state
    list_for_each_entry(instance, &cdev.thermal_instances, cdev_node) {
    if (instance.target == THERMAL_NO_TARGET)
    continue;
    if (instance.target > target)
    target = instance.target;
    }
    thermal_cdev_set_cur_state(cdev, target);
    trace_cdev_update(cdev, target);
    dev_dbg(&cdev.device, "set to state %lu\n", target);
    }
//
// thermal_cdev_update - update cooling device state if needed
// @cdev:	pointer to struct thermal_cooling_device
//
// Update the cooling device state if there is a need.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_cdev_update(cdev: *mut thermal_cooling_device) {
    void thermal_cdev_update(struct thermal_cooling_device *cdev)
    {
    guard(cooling_dev)(cdev);
    if (!cdev.updated) {
    __thermal_cdev_update(cdev);
    cdev.updated = true;
    }
    }
//
// thermal_cdev_update_nocheck() - Unconditionally update cooling device state
// @cdev: Target cooling device.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_cdev_update_nocheck(cdev: *mut thermal_cooling_device) {
    void thermal_cdev_update_nocheck(struct thermal_cooling_device *cdev)
    {
    guard(cooling_dev)(cdev);
    __thermal_cdev_update(cdev);
    }
//
// thermal_zone_get_slope - return the slope attribute of the thermal zone
// @tz: thermal zone device with the slope attribute
//
// Return: If the thermal zone device has a slope attribute, return it, else
// return 1.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_zone_get_slope(tz: *mut thermal_zone_device) -> c_int {
    int thermal_zone_get_slope(struct thermal_zone_device *tz)
    {
    if (tz && tz.tzp)
    return tz.tzp.slope;
    return 1;
    }
    EXPORT_SYMBOL_GPL(thermal_zone_get_slope);
//
// thermal_zone_get_offset - return the offset attribute of the thermal zone
// @tz: thermal zone device with the offset attribute
//
// Return: If the thermal zone device has a offset attribute, return it, else
// return 0.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_zone_get_offset(tz: *mut thermal_zone_device) -> c_int {
    int thermal_zone_get_offset(struct thermal_zone_device *tz)
    {
    if (tz && tz.tzp)
    return tz.tzp.offset;
    return 0;
    }
    EXPORT_SYMBOL_GPL(thermal_zone_get_offset);
