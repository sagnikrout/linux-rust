//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/gov_fair_share.c
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
// fair_share.c - A simple weight based Thermal governor
//
// Copyright (C) 2012 Intel Corp
// Copyright (C) 2012 Durgadoss R <durgadoss.r@intel.com>
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

#[no_mangle]
unsafe extern "C" fn get_trip_level(tz: *mut thermal_zone_device) -> c_int {
    static int get_trip_level(struct thermal_zone_device *tz)
    {
    const struct thermal_trip_desc *level_td = core::ptr::null_mut();
    const struct thermal_trip_desc *td;
    let mut trip_level: c_int = -1;
    for_each_trip_desc(tz, td) {
    if (td.threshold > tz.temperature)
    continue;
    trip_level++;
    if (!level_td || td.threshold > level_td.threshold)
    level_td = td;
    }
// Bail out if the temperature is not greater than any trips.
    if (trip_level < 0)
    return 0;
    trace_thermal_zone_trip(tz, thermal_zone_trip_id(tz, &level_td.trip),
    level_td.trip.type);
    return trip_level;
    }
//
// fair_share_throttle - throttles devices associated with the given zone
// @tz: thermal_zone_device
// @td: trip point descriptor
// @trip_level: number of trips crossed by the zone temperature
//
// Throttling Logic: This uses three parameters to calculate the new
// throttle state of the cooling devices associated with the given zone.
//
// Parameters used for Throttling:
// P1. max_state: Maximum throttle state exposed by the cooling device.
// P2. weight[i]/total_weight:
// How 'effective' the 'i'th device is, in cooling the given zone.
// P3. trip_level/max_no_of_trips:
// This describes the extent to which the devices should be throttled.
// We do not want to throttle too much when we trip a lower temperature,
// whereas the throttling is at full swing if we trip critical levels.
// new_state of cooling device = P3 * P2 * P1
//
    static void fair_share_throttle(struct thermal_zone_device *tz,
    const struct thermal_trip_desc *td,
    int trip_level)
    {
    struct thermal_instance *instance;
    let mut total_weight: c_int = 0;
    let mut nr_instances: c_int = 0;
    list_for_each_entry(instance, &td.thermal_instances, trip_node) {
    total_weight += instance.weight;
    nr_instances++;
    }
    list_for_each_entry(instance, &td.thermal_instances, trip_node) {
    struct thermal_cooling_device *cdev = instance.cdev;
    u64 dividend;
    u32 divisor;
    dividend = trip_level;
    dividend *= cdev.max_state;
    divisor = tz.num_trips;
    if (total_weight) {
    dividend *= instance.weight;
    divisor *= total_weight;
    } else {
    divisor *= nr_instances;
    }
    instance.target = div_u64(dividend, divisor);
    thermal_cdev_update_nocheck(cdev);
    }
    }
#[no_mangle]
unsafe extern "C" fn fair_share_manage(tz: *mut thermal_zone_device) {
    static void fair_share_manage(struct thermal_zone_device *tz)
    {
    let mut trip_level: c_int = get_trip_level(tz);
    const struct thermal_trip_desc *td;
    lockdep_assert_held(&tz.lock);
    for_each_trip_desc(tz, td) {
    const struct thermal_trip *trip = &td.trip;
    if (trip.temperature == THERMAL_TEMP_INVALID ||
    trip.type == THERMAL_TRIP_CRITICAL ||
    trip.type == THERMAL_TRIP_HOT)
    continue;
    fair_share_throttle(tz, td, trip_level);
    }
    }
    static struct thermal_governor thermal_gov_fair_share = {
    .name	= "fair_share",
    .manage	= fair_share_manage,
    };
    THERMAL_GOVERNOR_DECLARE(thermal_gov_fair_share);
