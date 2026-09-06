//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_issues.h
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
// (C) COPYRIGHT 2014-2018 ARM Limited. All rights reserved.
// Copyright 2019 Linaro, Ltd., Rob Herring <robh@kernel.org>

//
// This is not a complete list of issues, but only the ones the driver needs
// to care about.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_hw_issue {
// Need way to guarantee that all previously-translated memory accesses
// are committed
    HW_ISSUE_6367,

// On job complete with non-done the cache is not flushed
    HW_ISSUE_6787,

// Write of PRFCNT_CONFIG_MODE_MANUAL to PRFCNT_CONFIG causes a
// instrumentation dump if PRFCNT_TILER_EN is enabled
    HW_ISSUE_8186,

// TIB: Reports faults from a vtile which has not yet been allocated
    HW_ISSUE_8245,

// uTLB deadlock could occur when writing to an invalid page at the
// same time as access to a valid page in the same uTLB cache line ( ==
// 4 PTEs == 16K block of mapping)
    HW_ISSUE_8316,

// HT: TERMINATE for RUN command ignored if previous LOAD_DESCRIPTOR is
// still executing
    HW_ISSUE_8394,

// CSE: Sends a TERMINATED response for a task that should not be
// terminated
    HW_ISSUE_8401,

// Repeatedly Soft-stopping a job chain consisting of (Vertex Shader,
// Cache Flush, Tiler) jobs causes DATA_INVALID_FAULT on tiler job.
    HW_ISSUE_8408,

// Disable the Pause Buffer in the LS pipe.
    HW_ISSUE_8443,

// Change in RMUs in use causes problems related with the core's SDC
    HW_ISSUE_8987,

// Compute endpoint has a 4-deep queue of tasks, meaning a soft stop
// won't complete until all 4 tasks have completed
    HW_ISSUE_9435,

// HT: Tiler returns TERMINATED for non-terminated command
    HW_ISSUE_9510,

// Occasionally the GPU will issue multiple page faults for the same
// address before the MMU page table has been read by the GPU
    HW_ISSUE_9630,

// RA DCD load request to SDC returns invalid load ignore causing
// colour buffer mismatch
    HW_ISSUE_10327,

// MMU TLB invalidation hazards
    HW_ISSUE_10649,

// Missing cache flush in multi core-group configuration
    HW_ISSUE_10676,

// Chicken bit on T72X for a hardware workaround in compiler
    HW_ISSUE_10797,

// Soft-stopping fragment jobs might fail with TILE_RANGE_FAULT
    HW_ISSUE_10817,

// Intermittent missing interrupt on job completion
    HW_ISSUE_10883,

// Soft-stopping fragment jobs might fail with TILE_RANGE_ERROR
// (similar to issue 10817) and can use #10817 workaround
    HW_ISSUE_10959,

// Soft-stopped fragment shader job can restart with out-of-bound
// restart index
    HW_ISSUE_10969,

// Race condition can cause tile list corruption
    HW_ISSUE_11020,

// Write buffer can cause tile list corruption
    HW_ISSUE_11024,

// Pause buffer can cause a fragment job hang
    HW_ISSUE_11035,

// Dynamic Core Scaling not supported due to errata
    HW_ISSUE_11056,

// Clear encoder state for a hard stopped fragment job which is AFBC
// encoded by soft resetting the GPU. Only for T76X r0p0, r0p1 and
// r0p1_50rel0
    HW_ISSUE_T76X_3542,

// Keep tiler module clock on to prevent GPU stall
    HW_ISSUE_T76X_3953,

// Must ensure L2 is not transitioning when we reset. Workaround with a
// busy wait until L2 completes transition; ensure there is a maximum
// loop count as she may never complete her transition. (On chips
// without this errata, it's totally okay if L2 transitions.)
    HW_ISSUE_TMIX_8463,

// Don't set SC_LS_ATTR_CHECK_DISABLE/SC_LS_ALLOW_ATTR_TYPES
    GPUCORE_1619,

// When a hard-stop follows close after a soft-stop, the completion
// code for the terminated job may be incorrectly set to STOPPED
    HW_ISSUE_TMIX_8438,

// "Protected mode" is buggy on Mali-G31 some Bifrost chips, so the
// kernel must fiddle with L2 caches to prevent data leakage
    HW_ISSUE_TGOX_R1_1234,

// Must set SC_VAR_ALGORITHM
    HW_ISSUE_TTRX_2968_TTRX_3162,

// Bus fault from occlusion query write may cause future fragment jobs
// to hang
    HW_ISSUE_TTRX_3076,

// Must issue a dummy job before starting real work to prevent hangs
    HW_ISSUE_TTRX_3485,

    HW_ISSUE_END
}

pub const hw_issues_g31: c_int = 0;

pub const hw_issues_g51: c_int = 0;
pub const hw_issues_g52: c_int = 0;

pub const hw_issues_g72: c_int = 0;
pub const hw_issues_g76: c_int = 0;

extern "C" {
    pub fn test_bit(_arg: issue, _arg: pfdev->features.hw_issues) -> return;
}
