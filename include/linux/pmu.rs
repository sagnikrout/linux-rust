//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pmu.h
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
// Definitions for talking to the PMU.  The PMU is a microcontroller
// which controls battery charging and system power on PowerBook 3400
// and 2400 models as well as the RTC and various other things.
//
// Copyright (C) 1998 Paul Mackerras.
//

extern "C" {
    pub fn find_via_pmu() -> int __init;
}
extern "C" {
    pub fn pmu_queue_request(req: *mut adb_request) -> c_int;
}
extern "C" {
    pub fn pmu_poll();
}
extern "C" {
    pub fn pmu_wait_complete(req: *mut adb_request);
}
// For use before switching interrupts off for a long time;
// warning: not stackable
//

extern "C" {
    pub fn pmu_suspend();
}
extern "C" {
    pub fn pmu_resume();
}

extern "C" {
    pub fn pmu_enable_irled(on: c_int);
}
extern "C" {
    pub fn pmu_get_time() -> time64_t;
}
extern "C" {
    pub fn pmu_set_rtc_time(tm: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn pmu_restart();
}
extern "C" {
    pub fn pmu_shutdown();
}
extern "C" {
    pub fn pmu_unlock();
}
extern "C" {
    pub fn pmu_present() -> c_int;
}
extern "C" {
    pub fn pmu_get_model() -> c_int;
}
extern "C" {
    pub fn pmu_backlight_set_sleep(sleep: c_int);
}
pub const PMU_MAX_BATTERIES: c_int = 2;
// values for pmu_power_flags
pub const PMU_PWR_AC_PRESENT: c_uint = 0x00000001;
// values for pmu_battery_info.flags
pub const PMU_BATT_PRESENT: c_uint = 0x00000001;
pub const PMU_BATT_CHARGING: c_uint = 0x00000002;
pub const PMU_BATT_TYPE_MASK: c_uint = 0x000000f0;
pub const PMU_BATT_TYPE_SMART: c_uint = 0x00000010 /* Smart battery */;
pub const PMU_BATT_TYPE_HOOPER: c_uint = 0x00000020 /* 3400/3500 */;
pub const PMU_BATT_TYPE_COMET: c_uint = 0x00000030 /* 2400 */;
// Backlight
extern "C" {
    pub fn pmu_backlight_init();
}
// some code needs to know if the PMU was suspended for hibernation

// if power management is not configured it can't be suspended
pub const pmu_sys_suspended: c_int = 0;

