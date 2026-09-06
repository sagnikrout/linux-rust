//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/resctrl_types.h
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
// Copyright (C) 2025 Arm Ltd.
// Based on arch/x86/kernel/cpu/resctrl/internal.h
//

pub const MBM_OVERFLOW_INTERVAL: c_int = 1000;
// Reads to Local DRAM Memory

// Reads to Remote DRAM Memory

// Non-Temporal Writes to Local Memory

// Non-Temporal Writes to Remote Memory

// Reads to Local Memory the system identifies as "Slow Memory"

// Reads to Remote Memory the system identifies as "Slow Memory"

// Dirty Victims to All Types of Memory

// Max event bits supported

// Number of memory transactions that an MBM event can be configured with
pub const NUM_MBM_TRANSACTIONS: c_int = 7;
// Event IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_event_id {
// Must match value of first event below
    QOS_FIRST_EVENT			= 0x01,

//
// These values match those used to program IA32_QM_EVTSEL before
// reading IA32_QM_CTR on RDT systems.
//
    QOS_L3_OCCUP_EVENT_ID		= 0x01,
    QOS_L3_MBM_TOTAL_EVENT_ID	= 0x02,
    QOS_L3_MBM_LOCAL_EVENT_ID	= 0x03,

// Intel Telemetry Events
    PMT_EVENT_ENERGY,
    PMT_EVENT_ACTIVITY,
    PMT_EVENT_STALLS_LLC_HIT,
    PMT_EVENT_C1_RES,
    PMT_EVENT_UNHALTED_CORE_CYCLES,
    PMT_EVENT_STALLS_LLC_MISS,
    PMT_EVENT_AUTO_C6_RES,
    PMT_EVENT_UNHALTED_REF_CYCLES,
    PMT_EVENT_UOPS_RETIRED,

// Must be the last
    QOS_NUM_EVENTS,
}

