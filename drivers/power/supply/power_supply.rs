//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/power/supply/power_supply.h
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
// Functions private to power supply class
//
// Copyright © 2007  Anton Vorontsov <cbou@mail.ru>
// Copyright © 2004  Szabolcs Gyurko
// Copyright © 2003  Ian Molton <spyro@f2s.com>
//
// Modified: 2004, Oct     Szabolcs Gyurko
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_ext_registration {
    pub list_head: list_head,
    pub ext: *const power_supply_ext,
    pub dev: *mut device,
    pub data: *mut c_void,
}

// Make sure that the macro is a single expression

extern "C" {
    pub fn power_supply_init_attrs() -> void __init;
}
extern "C" {
    pub fn power_supply_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int;
}

extern "C" {
    pub fn power_supply_update_leds(psy: *mut power_supply);
}
extern "C" {
    pub fn power_supply_create_triggers(psy: *mut power_supply) -> c_int;
}
extern "C" {
    pub fn power_supply_remove_triggers(psy: *mut power_supply);
}

extern "C" {
    pub fn power_supply_add_hwmon_sysfs(psy: *mut power_supply) -> c_int;
}
extern "C" {
    pub fn power_supply_remove_hwmon_sysfs(psy: *mut power_supply);
}

