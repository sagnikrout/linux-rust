//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/hyperv_timer.h
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
// Definitions for the clocksource provided by the Hyper-V
// hypervisor to guest VMs, as described in the Hyper-V Top
// Level Functional Spec (TLFS).
//
// Copyright (C) 2019, Microsoft, Inc.
//
// Author:  Michael Kelley <mikelley@microsoft.com>
//

pub const HV_MAX_MAX_DELTA_TICKS: c_uint = 0xffffffff;
pub const HV_MIN_DELTA_TICKS: c_int = 1;

// Routines called by the VMbus driver
extern "C" {
    pub fn hv_stimer_alloc(have_percpu_irqs: bool) -> c_int;
}
extern "C" {
    pub fn hv_stimer_cleanup(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn hv_stimer_global_cleanup();
}
extern "C" {
    pub fn hv_init_clocksource();
}
extern "C" {
    pub fn hv_remap_tsc_clocksource();
}
extern "C" {
    pub fn hv_get_tsc_pfn() -> c_ulong;
}
extern "C" {
    pub fn hv_adj_sched_clock_offset(offset: u64);
}
//
// The protocol for reading Hyper-V TSC page is specified in Hypervisor
// Top-Level Functional Specification ver. 3.0 and above. To get the
// reference time we must do the following:
// - READ ReferenceTscSequence
// A special '0' value indicates the time source is unreliable and we
// need to use something else. The currently published specification
// versions (up to 4.0b) contain a mistake and wrongly claim '-1'
// instead of '0' as the special value, see commit c35b82ef0294.
// - ReferenceTime =
// ((RDTSC() * ReferenceTscScale) >> 64) + ReferenceTscOffset
// - READ ReferenceTscSequence again. In case its value has changed
// since our first reading we need to discard ReferenceTime and repeat
// the whole sequence as the hypervisor was updating the page in
// between.
//
// Make sure we read sequence before we read other values from
// TSC page.
//
// cur_tsc = hv_get_raw_timer();
//
// Make sure we read sequence after we read all other values
// from TSC page.
//
// time = mul_u64_u64_shr(*cur_tsc, scale, 64) + offset;

