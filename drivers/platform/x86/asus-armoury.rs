//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/asus-armoury.h
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
// Definitions for kernel modules using asus-armoury driver
//
// Copyright (c) 2024 Luke Jones <luke@ljones.dev>
//

//
// armoury_attr_uint_store() - Send an uint to WMI method if within min/max.
// @kobj: Pointer to the driver object.
// @attr: Pointer to the attribute calling this function.
// @buf: The buffer to read from, this is parsed to `uint` type.
// @count: Required by sysfs attribute macros, pass in from the callee attr.
// @min: Minimum accepted value. Below this returns -EINVAL.
// @max: Maximum accepted value. Above this returns -EINVAL.
// @store_value: Pointer to where the parsed value should be stored.
// @wmi_dev: The WMI function ID to use.
//
// This function is intended to be generic so it can be called from any "_store"
// attribute which works only with integers.
//
// Integers to be sent to the WMI method is inclusive range checked and
// an error returned if out of range.
//
// If the value is valid and WMI is success then the sysfs attribute is notified
// and if asus_bios_requires_reboot() is true then reboot attribute
// is also notified.
//
// Returns: Either count, or an error.
//
// armoury_attr_uint_show() - Receive an uint from a WMI method.
// @kobj: Pointer to the driver object.
// @attr: Pointer to the attribute calling this function.
// @buf: The buffer to write to, as an `uint` type.
// @wmi_dev: The WMI function ID to use.
//
// This function is intended to be generic so it can be called from any "_show"
// attribute which works only with integers.
//
// Returns: Either count, or an error.
//

// Create functions and attributes for use in other macros or on their own
// Shows a formatted static variable

// Boolean style enumeration, base macro. Requires adding show/store

//
// Requires <name>_current_value_show(), <name>_current_value_show()
//

//
// Requires <name>_current_value_show(), <name>_current_value_show()
// and <name>_possible_values_show()
//

//
// ROG PPT attributes need a little different in setup as they
// require rog_tunables members.
//

// Default is always the maximum value unless *_def is specified
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_limits {
    pub ppt_pl1_spl_min: u8,
    pub ppt_pl1_spl_def: u8,
    pub ppt_pl1_spl_max: u8,
    pub ppt_pl2_sppt_min: u8,
    pub ppt_pl2_sppt_def: u8,
    pub ppt_pl2_sppt_max: u8,
    pub ppt_pl3_fppt_min: u8,
    pub ppt_pl3_fppt_def: u8,
    pub ppt_pl3_fppt_max: u8,
    pub ppt_apu_sppt_min: u8,
    pub ppt_apu_sppt_def: u8,
    pub ppt_apu_sppt_max: u8,
    pub ppt_platform_sppt_min: u8,
    pub ppt_platform_sppt_def: u8,
    pub ppt_platform_sppt_max: u8,
// Nvidia GPU specific, default is always max
    pub macro: u8 nv_dynamic_boost_def; // unused. exists for,
    pub nv_dynamic_boost_min: u8,
    pub nv_dynamic_boost_max: u8,
    pub macro: u8 nv_temp_target_def; // unused. exists for,
    pub nv_temp_target_min: u8,
    pub nv_temp_target_max: u8,
    pub macro: u8 nv_tgp_def; // unused. exists for,
    pub nv_tgp_min: u8,
    pub nv_tgp_max: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_data {
    pub ac_data: *const power_limits,
    pub dc_data: *const power_limits,
    pub requires_fan_curve: bool,
}

//
// For each available attribute there must be a min and a max.
// _def is not required and will be assumed to be default == max if missing.
//
// This model is full AMD. No Nvidia dGPU.
// Yes this laptop is very limited
// Advantage Edition Laptop, no PL1 or PL2 limits
