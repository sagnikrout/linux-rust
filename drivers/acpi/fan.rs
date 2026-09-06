//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/fan.h
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
// ACPI fan device IDs are shared between the fan driver and the device power
// management code.
//
// Add new device IDs before the generic ACPI fan one.
//

pub const ACPI_FPS_NAME_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fan_fps {
    pub control: u64,
    pub trip_point: u64,
    pub speed: u64,
    pub noise_level: u64,
    pub power: u64,
    pub name: [c_char; ACPI_FPS_NAME_LEN],
    pub dev_attr: device_attribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fan_fif {
    pub revision: u8,
    pub fine_grain_ctrl: u8,
    pub step_size: u8,
    pub low_speed_notification: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fan_fst {
    pub revision: u64,
    pub control: u64,
    pub speed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fan {
    pub handle: acpi_handle,
    pub acpi4: bool,
    pub has_fst: bool,
    pub fif: acpi_fan_fif,
    pub fps: *mut acpi_fan_fps,
    pub fps_count: c_int,
// A value of 0 means that trippoint-related functions are not supported
    pub fan_trip_granularity: u32,

    pub hdev: *mut device,

    pub cdev: *mut thermal_cooling_device,
    pub fst_speed: device_attribute,
    pub fine_grain_control: device_attribute,
}

//
// acpi_fan_speed_valid - Check if fan speed value is valid
// @speed: Speed value returned by the ACPI firmware
//
// Check if the fan speed value returned by the ACPI firmware is valid. This function is
// necessary as ACPI firmware implementations can return 0xFFFFFFFF to signal that the
// ACPI fan does not support speed reporting. Additionally, some buggy ACPI firmware
// implementations return a value larger than the 32-bit integer value defined by
// the ACPI specification when using placeholder values. Such invalid values are also
// detected by this function.
//
// Returns: True if the fan speed value is valid, false otherwise.
//
// acpi_fan_power_valid - Check if fan power value is valid
// @power: Power value returned by the ACPI firmware
//
// Check if the fan power value returned by the ACPI firmware is valid.
// See acpi_fan_speed_valid() for details.
//
// Returns: True if the fan power value is valid, false otherwise.
//
extern "C" {
    pub fn acpi_fan_get_fst(handle: acpi_handle, fst: *mut acpi_fan_fst) -> c_int;
}
extern "C" {
    pub fn acpi_fan_create_attributes(device: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_fan_delete_attributes(device: *mut acpi_device);
}

extern "C" {
    pub fn devm_acpi_fan_create_hwmon(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_fan_notify_hwmon(dev: *mut device);
}

