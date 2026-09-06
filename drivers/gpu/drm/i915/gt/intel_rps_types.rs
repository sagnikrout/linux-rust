//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_rps_types.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ips {
    pub last_count1: u64,
    pub last_time1: c_ulong,
    pub chipset_power: c_ulong,
    pub last_count2: u64,
    pub last_time2: u64,
    pub gfx_power: c_ulong,
    pub corr: u8,
    pub m: int c,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rps_ei {
    pub ktime: ktime_t,
    pub render_c0: u32,
    pub media_c0: u32,
}

//
// struct intel_rps_freq_caps - rps freq capabilities
// @rp0_freq: non-overclocked max frequency
// @rp1_freq: "less than" RP0 power/frequency
// @min_freq: aka RPn, minimum frequency
//
// Freq caps exposed by HW, values are in "hw units" and intel_gpu_freq()
// should be used to convert to MHz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rps_freq_caps {
    pub rp0_freq: u8,
    pub rp1_freq: u8,
    pub min_freq: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rps {
    pub /: *mut *mut mutex lock; / protects enabling and the worker,
//
// work, interrupts_enabled and pm_iir are protected by
// gt->irq_lock
//
    pub timer: timer_list,
    pub work: work_struct,
    pub flags: c_ulong,
    pub pm_timestamp: ktime_t,
    pub pm_interval: u32,
    pub pm_iir: u32,
// PM interrupt bits that should never be masked
    pub pm_intrmsk_mbz: u32,
    pub pm_events: u32,
// Frequencies are stored in potentially platform dependent multiples.
// In other words, *_freq needs to be multiplied by X to be interesting.
// Soft limits are those which are used for the dynamic reclocking done
// by the driver (raise frequencies under heavy loads, and lower for
// lighter loads). Hard limits are those imposed by the hardware.
//
// A distinction is made for overclocking, which is never enabled by
// default, and is considered to be above the hard limit if it's
// possible at all.
//
    pub /: *mut *mut u8 cur_freq; / Current frequency (cached, may not == HW),
    pub /: *mut *mut u8 last_freq; / Last SWREQ frequency,
    pub /: *mut *mut u8 min_freq_softlimit; / Minimum frequency permitted by the driver,
    pub /: *mut *mut u8 max_freq_softlimit; / Max frequency permitted by the driver,
    pub /: *mut *mut u8 max_freq; / Maximum frequency, RP0 if not overclocking,
    pub /: *mut *mut u8 min_freq; / AKA RPn. Minimum frequency,
    pub /: *mut *mut u8 boost_freq; / Frequency to request when wait boosting,
    pub /: *mut *mut u8 idle_freq; / Frequency to request when we are idle,
    pub /: *mut *mut u8 efficient_freq; / AKA RPe. Pre-determined balanced frequency,
    pub /: *mut *mut u8 rp1_freq; / "less than" RP0 power/frequency,
    pub /: *mut *mut u8 rp0_freq; / Non-overclocked max frequency.,
    pub /: *mut *mut u16 gpll_ref_freq; / vlv/chv GPLL reference frequency,
    pub last_adj: c_int,
    pub mutex: mutex,
    pub mode: { LOW_POWER, BETWEEN, HIGH_POWER },
    pub interactive: c_uint,
    pub /: *mut *mut u8 up_threshold; / Current %busy required to uplock,
    pub /: *mut *mut u8 down_threshold; / Current %busy required to downclock,
    pub power: },
    pub num_waiters: core::sync::atomic::AtomicI32,
    pub boosts: c_uint,
// manual wa residency calculations
    pub ei: intel_rps_ei,
    pub ips: intel_ips,
}
