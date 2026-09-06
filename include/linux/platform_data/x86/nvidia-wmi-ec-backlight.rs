//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/nvidia-wmi-ec-backlight.h
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
// Copyright (c) 2020, NVIDIA CORPORATION.  All rights reserved.
//

//
// enum wmi_brightness_method - WMI method IDs
// @WMI_BRIGHTNESS_METHOD_LEVEL:  Get/Set EC brightness level status
// @WMI_BRIGHTNESS_METHOD_SOURCE: Get/Set EC Brightness Source
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_brightness_method {
    WMI_BRIGHTNESS_METHOD_LEVEL = 1,
    WMI_BRIGHTNESS_METHOD_SOURCE = 2,
    WMI_BRIGHTNESS_METHOD_MAX
}

//
// enum wmi_brightness_mode - Operation mode for WMI-wrapped method
// @WMI_BRIGHTNESS_MODE_GET:            Get the current brightness level/source.
// @WMI_BRIGHTNESS_MODE_SET:            Set the brightness level.
// @WMI_BRIGHTNESS_MODE_GET_MAX_LEVEL:  Get the maximum brightness level. This
// is only valid when the WMI method is
// %WMI_BRIGHTNESS_METHOD_LEVEL.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_brightness_mode {
    WMI_BRIGHTNESS_MODE_GET = 0,
    WMI_BRIGHTNESS_MODE_SET = 1,
    WMI_BRIGHTNESS_MODE_GET_MAX_LEVEL = 2,
    WMI_BRIGHTNESS_MODE_MAX
}

//
// enum wmi_brightness_source - Backlight brightness control source selection
// @WMI_BRIGHTNESS_SOURCE_GPU: Backlight brightness is controlled by the GPU.
// @WMI_BRIGHTNESS_SOURCE_EC:  Backlight brightness is controlled by the
// system's Embedded Controller (EC).
// @WMI_BRIGHTNESS_SOURCE_AUX: Backlight brightness is controlled over the
// DisplayPort AUX channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_brightness_source {
    WMI_BRIGHTNESS_SOURCE_GPU = 1,
    WMI_BRIGHTNESS_SOURCE_EC = 2,
    WMI_BRIGHTNESS_SOURCE_AUX = 3,
    WMI_BRIGHTNESS_SOURCE_MAX
}

//
// struct wmi_brightness_args - arguments for the WMI-wrapped ACPI method
// @mode:    Pass in an &enum wmi_brightness_mode value to select between
// getting or setting a value.
// @val:     In parameter for value to set when using %WMI_BRIGHTNESS_MODE_SET
// mode. Not used in conjunction with %WMI_BRIGHTNESS_MODE_GET or
// %WMI_BRIGHTNESS_MODE_GET_MAX_LEVEL mode.
// @ret:     Out parameter returning retrieved value when operating in
// %WMI_BRIGHTNESS_MODE_GET or %WMI_BRIGHTNESS_MODE_GET_MAX_LEVEL
// mode. Not used in %WMI_BRIGHTNESS_MODE_SET mode.
// @ignored: Padding; not used. The ACPI method expects a 24 byte params struct.
//
// This is the parameters structure for the WmiBrightnessNotify ACPI method as
// wrapped by WMI. The value passed in to @val or returned by @ret will be a
// brightness value when the WMI method ID is %WMI_BRIGHTNESS_METHOD_LEVEL, or
// an &enum wmi_brightness_source value with %WMI_BRIGHTNESS_METHOD_SOURCE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_brightness_args {
    pub mode: u32,
    pub val: u32,
    pub ret: u32,
    pub ignored: [u32; 3],
}
