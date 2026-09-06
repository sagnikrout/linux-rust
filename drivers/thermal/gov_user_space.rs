//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/gov_user_space.c
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
// user_space.c - A simple user space Thermal events notifier
//
// Copyright (C) 2012 Intel Corp
// Copyright (C) 2012 Durgadoss R <durgadoss.r@intel.com>
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

#[no_mangle]
unsafe extern "C" fn user_space_bind(tz: *mut thermal_zone_device) -> c_int {
    static int user_space_bind(struct thermal_zone_device *tz)
    {
    pr_info_once("Consider using thermal netlink events interface\n");
    return 0;
    }
//
// user_space_trip_crossed - Notify user space about trip crossing events
// @tz: thermal_zone_device
// @trip: trip point
// @upward: whether or not the trip has been crossed on the way up
//
// This function notifies the user space through UEvents.
//
    static void user_space_trip_crossed(struct thermal_zone_device *tz,
    const struct thermal_trip *trip,
    bool upward)
    {
    char *thermal_prop[5];
    int i;
    lockdep_assert_held(&tz.lock);
    thermal_prop[0] = kasprintf(GFP_KERNEL, "NAME=%s", tz.type);
    thermal_prop[1] = kasprintf(GFP_KERNEL, "TEMP=%d", tz.temperature);
    thermal_prop[2] = kasprintf(GFP_KERNEL, "TRIP=%d",
    thermal_zone_trip_id(tz, trip));
    thermal_prop[3] = kasprintf(GFP_KERNEL, "EVENT=%d", tz.notify_event);
    thermal_prop[4] = core::ptr::null_mut();
    kobject_uevent_env(&tz.device.kobj, KOBJ_CHANGE, thermal_prop);
    for (i = 0; i < 4; ++i)
    kfree(thermal_prop[i]);
    }
    static struct thermal_governor thermal_gov_user_space = {
    .name		= "user_space",
    .trip_crossed	= user_space_trip_crossed,
    .bind_to_tz	= user_space_bind,
    };
    THERMAL_GOVERNOR_DECLARE(thermal_gov_user_space);
