//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/video.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_video_brightness_flags {
    pub /: *mut *mut u8 _BCL_no_ac_battery_levels:1; / no AC/Battery levels in _BCL,
    pub /: *mut *mut u8 _BCL_reversed:1; / _BCL package is in a reversed order,
    pub /: *mut *mut u8 _BQC_use_index:1; / _BQC returns an index value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_video_device_brightness {
    pub curr: c_int,
    pub count: c_int,
    pub levels: *mut c_int,
    pub flags: acpi_video_brightness_flags,
}

pub const ACPI_VIDEO_DISPLAY_CRT: c_int = 1;
pub const ACPI_VIDEO_DISPLAY_TV: c_int = 2;
pub const ACPI_VIDEO_DISPLAY_DVI: c_int = 3;
pub const ACPI_VIDEO_DISPLAY_LCD: c_int = 4;
pub const ACPI_VIDEO_DISPLAY_LEGACY_MONITOR: c_uint = 0x0100;
pub const ACPI_VIDEO_DISPLAY_LEGACY_PANEL: c_uint = 0x0110;
pub const ACPI_VIDEO_DISPLAY_LEGACY_TV: c_uint = 0x0200;
pub const ACPI_VIDEO_NOTIFY_SWITCH: c_uint = 0x80;
pub const ACPI_VIDEO_NOTIFY_PROBE: c_uint = 0x81;
pub const ACPI_VIDEO_NOTIFY_CYCLE: c_uint = 0x82;
pub const ACPI_VIDEO_NOTIFY_NEXT_OUTPUT: c_uint = 0x83;
pub const ACPI_VIDEO_NOTIFY_PREV_OUTPUT: c_uint = 0x84;
pub const ACPI_VIDEO_NOTIFY_CYCLE_BRIGHTNESS: c_uint = 0x85;
pub const ACPI_VIDEO_NOTIFY_INC_BRIGHTNESS: c_uint = 0x86;
pub const ACPI_VIDEO_NOTIFY_DEC_BRIGHTNESS: c_uint = 0x87;
pub const ACPI_VIDEO_NOTIFY_ZERO_BRIGHTNESS: c_uint = 0x88;
pub const ACPI_VIDEO_NOTIFY_DISPLAY_OFF: c_uint = 0x89;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_backlight_type {
    acpi_backlight_undef = -1,
    acpi_backlight_none = 0,
    acpi_backlight_video,
    acpi_backlight_vendor,
    acpi_backlight_native,
    acpi_backlight_nvidia_wmi_ec,
    acpi_backlight_apple_gmux,
    acpi_backlight_dell_uart,
}

extern "C" {
    pub fn acpi_video_register() -> c_int;
}
extern "C" {
    pub fn acpi_video_unregister();
}
extern "C" {
    pub fn acpi_video_register_backlight();
}
//
// Note: The value returned by acpi_video_handles_brightness_key_presses()
// may change over time and should not be cached.
//
extern "C" {
    pub fn acpi_video_handles_brightness_key_presses() -> bool;
}
extern "C" {
    pub fn __acpi_video_get_backlight_type(_arg: false, _arg: NULL) -> return;
}
//
// This function MUST only be called by GPU drivers to check if the driver
// should register a backlight class device. This function not only checks
// if a GPU native backlight device should be registered it *also* tells
// the ACPI video-detect code that native GPU backlight control is available.
// Therefor calling this from any place other then the GPU driver is wrong!
// To check if GPU native backlight control is used in other places instead use:
// if (acpi_video_get_backlight_type() == acpi_backlight_native) { ... }
//

