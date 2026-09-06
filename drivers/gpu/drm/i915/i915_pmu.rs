//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_pmu.h
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


//
// SPDX-License-Identifier: MIT
//
// Copyright © 2017-2018 Intel Corporation
//

//
// Non-engine events that we need to track enabled-disabled transition and
// current state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_pmu_tracked_events {
    __I915_PMU_ACTUAL_FREQUENCY_ENABLED = 0,
    __I915_PMU_REQUESTED_FREQUENCY_ENABLED,
    __I915_PMU_RC6_RESIDENCY_ENABLED,
    __I915_PMU_TRACKED_EVENT_COUNT, /* count marker */
}

//
// Slots used from the sampling timer (non-engine events) with some extras for
// convenience.
//
pub const I915_PMU_MAX_GT: c_int = 2;
//
// How many different events we track in the global PMU mask.
//
// It is also used to know to needed number of event reference counters.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_pmu_sample {
    pub cur: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_pmu {
//
// @base: PMU base.
//
    pub base: pmu,
//
// @registered: PMU is registered and not in the unregistering process.
//
    pub registered: bool,
//
// @name: Name as registered with perf core.
//
    pub name: *const c_char,
//
// @lock: Lock protecting enable mask and ref count handling.
//
    pub lock: spinlock_t,
//
// @unparked: GT unparked mask.
//
    pub unparked: c_uint,
//
// @timer: Timer for internal i915 PMU sampling.
//
    pub timer: hrtimer,
//
// @enable: Bitmask of specific enabled events.
//
// For some events we need to track their state and do some internal
// house keeping.
//
// Each engine event sampler type and event listed in enum
// i915_pmu_tracked_events gets a bit in this field.
//
// Low bits are engine samplers and other events continue from there.
//
    pub enable: u32,
//
// @timer_last:
//
// Timestamp of the previous timer invocation.
//
    pub timer_last: ktime_t,
//
// @enable_count: Reference counts for the enabled events.
//
// Array indices are mapped in the same way as bits in the @enable field
// and they are used to control sampling on/off when multiple clients
// are using the PMU API.
//
    pub enable_count: [c_uint; I915_PMU_MASK_BITS],
//
// @timer_enabled: Should the internal sampling timer be running.
//
    pub timer_enabled: bool,
//
// @sample: Current and previous (raw) counters for sampling events.
//
// These counters are updated from the i915 PMU sampling timer.
//
// Only global counters are held here, while the per-engine ones are in
// struct intel_engine_cs.
//
    pub sample: [i915_pmu_sample; I915_PMU_MAX_GT][__I915_NUM_PMU_SAMPLERS],
//
// @sleep_last: Last time GT parked for RC6 estimation.
//
    pub sleep_last: [ktime_t; I915_PMU_MAX_GT],
//
// @irq_count: Number of interrupts
//
// Intentionally unsigned long to avoid atomics or heuristics on 32bit.
// 4e9 interrupts are a lot and postprocessing can really deal with an
// occasional wraparound easily. It's 32bit after all.
//
    pub irq_count: c_ulong,
//
// @events_attr_group: Device events attribute group.
//
    pub events_attr_group: attribute_group,
//
// @i915_attr: Memory block holding device attributes.
//
    pub i915_attr: *mut c_void,
//
// @pmu_attr: Memory block holding device attributes.
//
    pub pmu_attr: *mut c_void,
}

extern "C" {
    pub fn i915_pmu_register(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_pmu_unregister(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_pmu_gt_parked(gt: *mut intel_gt);
}
extern "C" {
    pub fn i915_pmu_gt_unparked(gt: *mut intel_gt);
}

