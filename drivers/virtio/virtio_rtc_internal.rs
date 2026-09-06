//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virtio/virtio_rtc_internal.h
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
// virtio_rtc internal interfaces
//
// Copyright (C) 2022-2023 OpenSynergy GmbH
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

// driver core IFs
extern "C" {
    pub fn viortc_read(viortc: *mut viortc_dev, vio_clk_id: u16, reading: *mut u64) -> c_int;
}
// PTP IFs

// HW counter IFs
//
// viortc_hw_xtstamp_params() - get HW-specific xtstamp params
// @hw_counter: virtio_rtc HW counter type
// @cs_id: clocksource id corresponding to hw_counter
//
// Gets the HW-specific xtstamp params. Returns an error if the driver cannot
// support xtstamp.
//
// Context: Process context.
// Return: Zero on success, negative error code otherwise.
//
extern "C" {
    pub fn viortc_hw_xtstamp_params(hw_counter: *mut u8, cs_id: *mut clocksource_ids) -> c_int;
}
// RTC class IFs

extern "C" {
    pub fn viortc_class_alarm(viortc_class: *mut viortc_class, vio_clk_id: u16);
}
extern "C" {
    pub fn viortc_class_stop(viortc_class: *mut viortc_class);
}
extern "C" {
    pub fn viortc_class_register(viortc_class: *mut viortc_class) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

