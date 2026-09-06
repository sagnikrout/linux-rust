//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_pf_migration_types.h
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
// struct xe_sriov_pf_migration - Xe device level VF migration data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_pf_migration {
// @disabled: indicates whether VF migration feature is disabled
    pub disabled: bool,
}

//
// struct xe_sriov_migration_state - Per VF device-level migration related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_migration_state {
// @wq: waitqueue used to avoid busy-waiting for snapshot production/consumption
    pub wq: wait_queue_head_t,
// @lock: Mutex protecting the migration data
    pub lock: mutex,
// @pending: currently processed data packet of VF resource
    pub pending: *mut xe_sriov_packet,
// @trailer: data packet used to indicate the end of stream
    pub trailer: *mut xe_sriov_packet,
// @descriptor: data packet containing the metadata describing the device
    pub descriptor: *mut xe_sriov_packet,
}
