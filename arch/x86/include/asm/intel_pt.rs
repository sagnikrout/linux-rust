//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel_pt.h
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
pub const PT_CPUID_LEAVES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pt_capabilities {
    PT_CAP_max_subleaf = 0,
    PT_CAP_cr3_filtering,
    PT_CAP_psb_cyc,
    PT_CAP_ip_filtering,
    PT_CAP_mtc,
    PT_CAP_ptwrite,
    PT_CAP_power_event_trace,
    PT_CAP_event_trace,
    PT_CAP_tnt_disable,
    PT_CAP_topa_output,
    PT_CAP_topa_multiple_entries,
    PT_CAP_single_range_output,
    PT_CAP_output_subsys,
    PT_CAP_payloads_lip,
    PT_CAP_num_address_ranges,
    PT_CAP_mtc_periods,
    PT_CAP_cycle_thresholds,
    PT_CAP_psb_periods,
}

extern "C" {
    pub fn cpu_emergency_stop_pt();
}
extern "C" {
    pub fn intel_pt_validate_hw_cap(cap: pt_capabilities) -> u32;
}
extern "C" {
    pub fn intel_pt_validate_cap(caps: *mut u32, cap: pt_capabilities) -> u32;
}
extern "C" {
    pub fn is_intel_pt_event(event: *mut perf_event) -> c_int;
}

