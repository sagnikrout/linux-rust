//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwmon/peci/common.h
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
// Copyright (c) 2021 Intel Corporation

//
// struct peci_sensor_state - PECI state information
// @valid: flag to indicate the sensor value is valid
// @last_updated: time of the last update in jiffies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_sensor_state {
    pub valid: bool,
    pub last_updated: c_ulong,
}

//
// struct peci_sensor_data - PECI sensor information
// @value: sensor value in milli units
// @state: sensor update state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_sensor_data {
    pub value: i32,
    pub state: peci_sensor_state,
}

//
// peci_sensor_need_update() - check whether sensor update is needed or not
// @state: pointer to sensor state struct
//
// Return: true if update is needed, false if not.
//
// peci_sensor_mark_updated() - mark the sensor is updated
// @state: pointer to sensor state struct
//
