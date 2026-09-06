//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/battery.h
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

pub const ACPI_BATTERY_NOTIFY_STATUS: c_uint = 0x80;
pub const ACPI_BATTERY_NOTIFY_INFO: c_uint = 0x81;
pub const ACPI_BATTERY_NOTIFY_THRESHOLD: c_uint = 0x82;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_battery_hook {
    pub name: *const c_char,
    pub hook): *mut *mut *mut int (add_battery)(struct power_supply battery, struct acpi_battery_hook,
    pub hook): *mut *mut *mut int (remove_battery)(struct power_supply battery, struct acpi_battery_hook,
    pub list: list_head,
}

extern "C" {
    pub fn battery_hook_register(hook: *mut acpi_battery_hook);
}
extern "C" {
    pub fn battery_hook_unregister(hook: *mut acpi_battery_hook);
}
extern "C" {
    pub fn devm_battery_hook_register(dev: *mut device, hook: *mut acpi_battery_hook) -> c_int;
}
