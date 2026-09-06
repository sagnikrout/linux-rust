//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel_telemetry.h
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
// Intel SOC Telemetry Driver Header File
// Copyright (C) 2015, Intel Corporation.
// All Rights Reserved.
//
pub const TELEM_MAX_EVENTS_SRAM: c_int = 28;
pub const TELEM_MAX_OS_ALLOCATED_EVENTS: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum telemetry_unit {
    TELEM_PSS = 0,
    TELEM_IOSS,
    TELEM_UNIT_NONE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_evtlog {
    pub telem_evtid: u32,
    pub telem_evtlog: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_evtconfig {
// Array of Event-IDs to Enable
    pub evtmap: *mut u32,
// Number of Events (<29) in evtmap
    pub num_evts: u8,
// Sampling period
    pub period: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_evtmap {
    pub name: *const c_char,
    pub evt_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_unit_config {
    pub telem_evts: *mut telemetry_evtmap,
    pub regmap: *mut void __iomem,
    pub ssram_evts_used: u8,
    pub curr_period: u8,
    pub max_period: u8,
    pub min_period: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_plt_config {
    pub pss_config: telemetry_unit_config,
    pub ioss_config: telemetry_unit_config,
    pub telem_trace_lock: mutex,
    pub telem_lock: mutex,
    pub pmc: *mut intel_pmc_dev,
    pub scu: *mut intel_scu_ipc_dev,
    pub telem_in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_core_ops {
    pub verbosity): *mut u32,
    pub verbosity): u32,
    pub log_all_evts): int len, int,
    pub log_all_evts): int len, int,
}

extern "C" {
    pub fn telemetry_clear_pltdata() -> c_int;
}
