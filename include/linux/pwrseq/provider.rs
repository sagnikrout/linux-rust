//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pwrseq/provider.h
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
// Copyright (C) 2024 Linaro Ltd.
//
extern "C" {
    pub fn int(: *mut *mut pwrseq_power_state_func)(struct pwrseq_device) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut pwrseq_match_func)(struct pwrseq_device, : *mut device) -> typedef;
}
pub const PWRSEQ_NO_MATCH: c_int = 0;
pub const PWRSEQ_MATCH_OK: c_int = 1;
//
// struct pwrseq_unit_data - Configuration of a single power sequencing
// unit.
// @name: Name of the unit.
// @deps: Units that must be enabled before this one and disabled after it
// in the order they come in this array. Must be NULL-terminated.
// @enable: Callback running the part of the power-on sequence provided by
// this unit.
// @disable: Callback running the part of the power-off sequence provided
// by this unit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_unit_data {
    pub name: *const c_char,
    pub deps: *const pwrseq_unit_data,
    pub enable: pwrseq_power_state_func,
    pub disable: pwrseq_power_state_func,
}

//
// struct pwrseq_target_data - Configuration of a power sequencing target.
// @name: Name of the target.
// @unit: Final unit that this target must reach in order to be considered
// enabled.
// @post_enable: Callback run after the target unit has been enabled, *after
// the state lock has been released. It's useful for implementing
// boot-up delays without blocking other users from powering up
// using the same power sequencer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_target_data {
    pub name: *const c_char,
    pub unit: *const pwrseq_unit_data,
    pub post_enable: pwrseq_power_state_func,
}

//
// struct pwrseq_config - Configuration used for registering a new provider.
// @parent: Parent device for the sequencer. Must be set.
// @owner: Module providing this device.
// @drvdata: Private driver data.
// @match: Provider callback used to match the consumer device to the sequencer.
// @targets: Array of targets for this power sequencer. Must be NULL-terminated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_config {
    pub parent: *mut device,
    pub owner: *mut module,
    pub drvdata: *mut c_void,
    pub match: pwrseq_match_func,
    pub targets: *const pwrseq_target_data,
}

extern "C" {
    pub fn pwrseq_device_unregister(pwrseq: *mut pwrseq_device);
}
