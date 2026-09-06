//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/power12-events-list.h
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
// Performance counter support for POWER12 processors.
//
// Copyright 2026 Athira Rajeev, IBM Corporation.
//
// Power12 event codes.
//
// All L1 D cache load references counted at finish, gated by reject
// Load Missed L1
// Store Missed L1
// L1 cache data prefetches
// Demand iCache Miss
// Instruction fetches from L1
// Instruction Demand sectors writtent into IL1
// Instruction prefetch written into IL1
// The data cache was reloaded from local core's L3 due to a demand load
// Demand LD - L3 Miss (not L2 hit and not L3 hit)
// All successful D-side store dispatches for this thread
// All successful D-side store dispatches for this thread that were L2 Miss
// Total HW L3 prefetches(Load+store)
// Data PTEG reload
// ITLB Reloaded
//
// Memory Access Events
//
// Primary PMU event used here is PM_MRK_INST_CMPL (0x401e0)
// To enable capturing of memory profiling, these MMCRA bits
// needs to be programmed and corresponding raw event format
// encoding.
//
// MMCRA bits encoding needed are
// SM (Sampling Mode)
// EM (Eligibility for Random Sampling)
// TECE (Threshold Event Counter Event)
// TS (Threshold Start Event)
// TE (Threshold End Event)
//
// Corresponding Raw Encoding bits:
// sample [EM,SM]
// thresh_sel (TECE)
// thresh start (TS)
// thresh end (TE)
//
