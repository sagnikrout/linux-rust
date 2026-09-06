//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/power8-events-list.h
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
// Performance counter support for POWER8 processors.
//
// Copyright 2014 Sukadev Bhattiprolu, IBM Corporation.
//
// Power8 event codes.
//
// All L1 D cache load references counted at finish, gated by reject
// Load Missed L1
// Store Missed L1
// L1 cache data prefetches
// Instruction fetches from L1
// Demand iCache Miss
// Instruction Demand sectors wriittent into IL1
// Instruction prefetch written into IL1
// The data cache was reloaded from local core's L3 due to a demand load
// Demand LD - L3 Miss (not L2 hit and not L3 hit)
// All successful D-side store dispatches for this thread
// All successful D-side store dispatches for this thread that were L2 Miss
// Total HW L3 prefetches(Load+store)
// Data PTEG reload
// ITLB Reloaded
// Run_Instructions
// Alternate event code for PM_RUN_INST_CMPL
// Run_cycles
// Alternate event code for Run_cycles
// Marked store completed
// Alternate event code for Marked store completed
// Marked two path branch
// Alternate event code for PM_BR_MRK_2PATH
// L3 castouts in Mepf state
// Alternate event code for PM_L3_CO_MEPF
// Data cache was reloaded from a location other than L2 due to a marked load
// Alternate event code for PM_MRK_DATA_FROM_L2MISS
// Alternate event code for  PM_CMPLU_STALL
// Two path branch
// Alternate event code for PM_BR_2PATH
// # PPC Dispatched
// Alternate event code for PM_INST_DISP
// Marked filter Match
// Alternate event code for PM_MRK_FILT_MATCH
// Alternate event code for PM_LD_MISS_L1
//
// Memory Access Event -- mem_access
// Primary PMU event used here is PM_MRK_INST_CMPL, along with
// Random Load/Store Facility Sampling (RIS) in Random sampling mode (MMCRA[SM]).
//
