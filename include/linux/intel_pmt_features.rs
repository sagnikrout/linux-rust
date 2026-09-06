//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel_pmt_features.h
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

// Common masks

// Per Core Performance Telemetry (PCPT) specific masks

// Per Core Environmental Telemetry (PCET) specific masks

// Per RMID Performance Telemetry specific masks

// Accelerator Telemetry specific masks

// Uncore Telemetry specific masks

// Crash Log specific masks

// PeTe Log specific masks

// TPMI control specific masks

// Tracing specific masks

// Per RMID Energy Telemetry specific masks

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmt_feature_id {
    FEATURE_INVALID			= 0x0,
    FEATURE_PER_CORE_PERF_TELEM	= 0x1,
    FEATURE_PER_CORE_ENV_TELEM	= 0x2,
    FEATURE_PER_RMID_PERF_TELEM	= 0x3,
    FEATURE_ACCEL_TELEM		= 0x4,
    FEATURE_UNCORE_TELEM		= 0x5,
    FEATURE_CRASH_LOG		= 0x6,
    FEATURE_PETE_LOG		= 0x7,
    FEATURE_TPMI_CTRL		= 0x8,
    FEATURE_RESERVED		= 0x9,
    FEATURE_TRACING			= 0xA,
    FEATURE_PER_RMID_ENERGY_TELEM	= 0xB,
    FEATURE_MAX			= 0xB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum feature_layout {
    LAYOUT_RMID,
    LAYOUT_WATCHER,
    LAYOUT_COMMAND,
    LAYOUT_CAPS_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_cap {
    pub mask: u32,
    pub name: *const c_char,
}
