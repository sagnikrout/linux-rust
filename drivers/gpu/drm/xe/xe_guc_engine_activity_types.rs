//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_engine_activity_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

//
// struct engine_activity - Engine specific activity data
//
// Contains engine specific activity data and snapshot of the
// structures from GuC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct engine_activity {
// @active: current activity
    pub active: u64,
// @last_cpu_ts: cpu timestamp in nsec of previous sample
    pub last_cpu_ts: u64,
// @quanta: total quanta used on HW
    pub quanta: u64,
// @quanta_ns: total quanta_ns used on HW
    pub quanta_ns: u64,
//
// @quanta_remainder_ns: remainder when the CPU time is scaled as
// per the quanta_ratio. This remainder is used in subsequent
// quanta calculations.
//
    pub quanta_remainder_ns: u64,
// @total: total engine activity
    pub total: u64,
// @running: true if engine is running some work
    pub running: bool,
// @metadata: snapshot of engine activity metadata
    pub metadata: guc_engine_activity_metadata,
// @activity: snapshot of engine activity counter
    pub activity: guc_engine_activity,
}

//
// struct engine_activity_group - Activity data for all engines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct engine_activity_group {
// @engine: engine specific activity data
    pub engine: [engine_activity; GUC_MAX_ENGINE_CLASSES][GUC_MAX_INSTANCES_PER_CLASS],
}

//
// struct engine_activity_buffer - engine activity buffers
//
// This contains the buffers allocated for metadata and activity data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct engine_activity_buffer {
// @activity_bo: object allocated to hold activity data
    pub activity_bo: *mut xe_bo,
// @metadata_bo: object allocated to hold activity metadata
    pub metadata_bo: *mut xe_bo,
}

//
// struct xe_guc_engine_activity - Data used by engine activity implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_engine_activity {
// @gpm_timestamp_shift: Right shift value for the gpm timestamp
    pub gpm_timestamp_shift: u32,
// @num_activity_group: number of activity groups
    pub num_activity_group: u32,
// @num_functions: number of functions
    pub num_functions: u32,
// @supported: indicates support for engine activity stats
    pub supported: bool,
//
// @eag: holds the device level engine activity data in native mode.
// In SRIOV mode, points to an array with entries which holds the engine
// activity data for PF and VF's
//
    pub eag: *mut engine_activity_group,
// @device_buffer: buffer object for global engine activity
    pub device_buffer: engine_activity_buffer,
// @function_buffer: buffer object for per-function engine activity
    pub function_buffer: engine_activity_buffer,
}
