//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acpi_amd_wbrf.h
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
// Wifi Band Exclusion Interface (AMD ACPI Implementation)
// Copyright (C) 2023 Advanced Micro Devices
//

// The maximum number of frequency band ranges
pub const MAX_NUM_OF_WBRF_RANGES: c_int = 11;
// Record actions
pub const WBRF_RECORD_ADD: c_uint = 0x0;
pub const WBRF_RECORD_REMOVE: c_uint = 0x1;
//
// struct freq_band_range - Wifi frequency band range definition
// @start: start frequency point (in Hz)
// @end: end frequency point (in Hz)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_band_range {
    pub start: u64,
    pub end: u64,
}

//
// struct wbrf_ranges_in_out - wbrf ranges info
// @num_of_ranges: total number of band ranges in this struct
// @band_list: array of Wifi band ranges
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wbrf_ranges_in_out {
    pub num_of_ranges: u64,
    pub band_list: [freq_band_range; MAX_NUM_OF_WBRF_RANGES],
}

//
// enum wbrf_notifier_actions - wbrf notifier actions index
// @WBRF_CHANGED: there was some frequency band updates. The consumers
// should retrieve the latest active frequency bands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wbrf_notifier_actions {
    WBRF_CHANGED,
}

extern "C" {
    pub fn acpi_amd_wbrf_supported_producer(dev: *mut device) -> bool;
}
extern "C" {
    pub fn acpi_amd_wbrf_add_remove(dev: *mut device, action: u8, in: *mut wbrf_ranges_in_out) -> c_int;
}
extern "C" {
    pub fn acpi_amd_wbrf_supported_consumer(dev: *mut device) -> bool;
}
extern "C" {
    pub fn amd_wbrf_retrieve_freq_band(dev: *mut device, out: *mut wbrf_ranges_in_out) -> c_int;
}
extern "C" {
    pub fn amd_wbrf_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn amd_wbrf_unregister_notifier(nb: *mut notifier_block) -> c_int;
}

