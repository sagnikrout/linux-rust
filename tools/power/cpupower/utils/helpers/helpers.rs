//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/utils/helpers/helpers.h
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
// (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
//
// Miscellaneous helpers which do not fit or are worth
// to put into separate headers
//

// Internationalization

// Internationalization
// Global verbose (-d) stuff
//
// define DEBUG via global Makefile variable
// Debug output is sent to stderr, do:
// cpupower monitor 2>/tmp/debug
// to split debug output away from normal output
//

// Global verbose (-v) stuff
// cpuid and cpuinfo helpers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpupower_cpu_vendor {
    X86_VENDOR_AMD, X86_VENDOR_HYGON, X86_VENDOR_MAX};

pub const CPUPOWER_CAP_INV_TSC: c_uint = 0x00000001;
pub const CPUPOWER_CAP_APERF: c_uint = 0x00000002;
pub const CPUPOWER_CAP_AMD_CPB: c_uint = 0x00000004;
pub const CPUPOWER_CAP_PERF_BIAS: c_uint = 0x00000008;
pub const CPUPOWER_CAP_HAS_TURBO_RATIO: c_uint = 0x00000010;
pub const CPUPOWER_CAP_IS_SNB: c_uint = 0x00000020;
pub const CPUPOWER_CAP_INTEL_IDA: c_uint = 0x00000040;
pub const CPUPOWER_CAP_AMD_RDPRU: c_uint = 0x00000080;
pub const CPUPOWER_CAP_AMD_HW_PSTATE: c_uint = 0x00000100;
pub const CPUPOWER_CAP_AMD_PSTATEDEF: c_uint = 0x00000200;
pub const CPUPOWER_CAP_AMD_CPB_MSR: c_uint = 0x00000400;
pub const CPUPOWER_CAP_AMD_PSTATE: c_uint = 0x00000800;

pub const CPUPOWER_AMD_CPBDIS: c_uint = 0x02000000;

pub const MAX_HW_PSTATES: c_int = 10;

    struct cpupower_cpu_info {
    enum cpupower_cpu_vendor vendor;
    unsigned int family;
    unsigned int model;
    unsigned int stepping;
// CPU capabilities read out from cpuid
    unsigned long long caps;
}

// get_cpu_info
//
// Extract CPU vendor, family, model, stepping info from /proc/cpuinfo
//
// Returns 0 on success or a negative error code
// Only used on x86, below global's struct values are zero/unknown on
// other archs
//
extern "C" {
    pub fn get_cpu_info(cpu_info: *mut cpupower_cpu_info) -> c_int;
}
// cpuid and cpuinfo helpers
extern "C" {
    pub fn cpufreq_has_generic_boost_support(active: *mut bool) -> c_int;
}
extern "C" {
    pub fn cpupower_set_generic_turbo_boost(turbo_boost: c_int) -> c_int;
}
// X86 ONLY

// Read/Write msr
extern "C" {
    pub fn read_msr(cpu: c_int, idx: c_uint, val: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn write_msr(cpu: c_int, idx: c_uint, val: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn cpupower_intel_set_perf_bias(cpu: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn cpupower_intel_get_perf_bias(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn msr_intel_get_turbo_ratio(cpu: c_uint) -> c_ulonglong;
}
extern "C" {
    pub fn cpupower_set_epp(cpu: c_uint, epp: *mut c_char) -> c_int;
}
extern "C" {
    pub fn cpupower_set_amd_pstate_mode(mode: *mut c_char) -> c_int;
}
// Read/Write msr
// PCI stuff
extern "C" {
    pub fn amd_pci_get_num_boost_states(active: *mut c_int, states: *mut c_int) -> c_int;
}
// PCI stuff
// AMD HW pstate decoding
extern "C" {
    pub fn cpupower_set_intel_turbo_boost(turbo_boost: c_int) -> c_int;
}
// AMD P-State stuff
extern "C" {
    pub fn cpupower_amd_pstate_enabled() -> bool;
}
// AMD P-State stuff
//
// CPUID functions returning a single datum
//
extern "C" {
    pub fn cpuid_eax(op: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpuid_ebx(op: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpuid_ecx(op: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpuid_edx(op: c_uint) -> c_uint;
}
// cpuid and cpuinfo helpers
// X86 ONLY

// Read/Write msr
// cpuid and cpuinfo helpers

//
// CPU State related functions
//
extern "C" {
    pub fn get_cpustate();
}
extern "C" {
    pub fn print_online_cpus();
}
extern "C" {
    pub fn print_offline_cpus();
}
extern "C" {
    pub fn print_speed(speed: c_ulong, no_rounding: c_int);
}
extern "C" {
    pub fn cppc_show_perf_and_freq(cpu: c_uint, no_rounding: c_int);
}
