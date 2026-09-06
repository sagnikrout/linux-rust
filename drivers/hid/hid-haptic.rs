//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-haptic.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HID Haptic support for Linux
//
// Copyright (c) 2021 Angela Czubak <acz@semihalf.com>
//

pub const HID_HAPTIC_ORDINAL_WAVEFORMNONE: c_int = 1;
pub const HID_HAPTIC_ORDINAL_WAVEFORMSTOP: c_int = 2;
pub const HID_HAPTIC_MODE_DEVICE: c_int = 0;
pub const HID_HAPTIC_MODE_HOST: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_haptic_effect {
    pub report_buf: *mut u8,
    pub input_dev: *mut input_dev,
    pub work: work_struct,
    pub control: list_head,
    pub control_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_haptic_effect_node {
    pub node: list_head,
    pub file: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_haptic_device {
    pub input_dev: *mut input_dev,
    pub hdev: *mut hid_device,
    pub auto_trigger_report: *mut hid_report,
    pub auto_trigger_mutex: mutex,
    pub wq: *mut workqueue_struct,
    pub manual_trigger_report: *mut hid_report,
    pub manual_trigger_mutex: mutex,
    pub manual_trigger_report_len: usize,
    pub pressed_state: c_int,
    pub pressure_sum: i32,
    pub force_logical_minimum: i32,
    pub force_physical_minimum: i32,
    pub force_resolution: i32,
    pub mode: u32,
    pub default_auto_trigger: u32,
    pub vendor_page: u32,
    pub vendor_id: u32,
    pub max_waveform_id: u32,
    pub max_duration_id: u32,
    pub hid_usage_map: *mut u16,
    pub duration_map: *mut u32,
    pub press_ordinal: u16,
    pub release_ordinal: u16,
    pub effect: *mut hid_haptic_effect,
    pub stop_effect: hid_haptic_effect,
}

// usage);
extern "C" {
    pub fn hid_haptic_handle_press_release(haptic: *mut hid_haptic_device);
}
extern "C" {
    pub fn hid_haptic_pressure_reset(haptic: *mut hid_haptic_device);
}

// usage)
