//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/coresight-pmu.h
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
// Copyright(C) 2015 Linaro Limited. All rights reserved.
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

//
// The legacy Trace ID system based on fixed calculation from the cpu
// number. This has been replaced by drivers using a dynamic allocation
// system - but need to retain the legacy algorithm for backward comparibility
// in certain situations:-
// a) new perf running on older systems that generate the legacy mapping
// b) older tools that may not update at the same time as the kernel.
//

//
// Interpretation of the PERF_RECORD_AUX_OUTPUT_HW_ID payload.
// Used to associate a CPU with the CoreSight Trace ID.
// [07:00] - Trace ID - uses 8 bits to make value easy to read in file.
// [39:08] - Sink ID - as reported in /sys/bus/event_source/devices/cs_etm/sinks
// Added in minor version 1.
// [55:40] - Unused (SBZ)
// [59:56] - Minor Version - previously existing fields are compatible with
// all minor versions.
// [63:60] - Major Version - previously existing fields mean different things
// in new major versions.
//

pub const CS_AUX_HW_ID_MAJOR_VERSION: c_int = 0;
pub const CS_AUX_HW_ID_MINOR_VERSION: c_int = 1;
