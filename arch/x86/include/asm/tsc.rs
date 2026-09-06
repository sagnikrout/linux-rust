//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/tsc.h
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
// x86 TSC related functions
//

//
// rdtsc() - returns the current TSC without ordering constraints
//
// rdtsc() returns the result of RDTSC as a 64-bit integer.  The
// only ordering constraint it supplies is the ordering implied by
// "asm volatile": it will put the RDTSC in the place you expect.  The
// CPU can and will speculatively execute that RDTSC, though, so the
// results can be non-monotonic if compared on different CPUs.
//
extern "C" {
    pub fn volatile(EAX_EDX_RET(val: "rdtsc" :, _arg: low, _arg: high)) -> asm;
}
extern "C" {
    pub fn EAX_EDX_VAL(_arg: val, _arg: low, _arg: high) -> return;
}
//
// rdtsc_ordered() - read the current TSC in program order
//
// rdtsc_ordered() returns the result of RDTSC as a 64-bit integer.
// It is ordered like a load to a global in-memory counter.  It should
// be impossible to observe non-monotonic rdtsc_unordered() behavior
// across multiple CPUs as long as the TSC is synced.
//
// The RDTSC instruction is not ordered relative to memory
// access.  The Intel SDM and the AMD APM are both vague on this
// point, but empirically an RDTSC instruction can be
// speculatively executed before prior loads.  An RDTSC
// immediately after an appropriate barrier appears to be
// ordered as a normal load, that is, it provides the same
// ordering guarantees as reading from a global memory location
// that some other imaginary CPU is updating continuously with a
// time stamp.
//
// Thus, use the preferred barrier on the respective CPU, aiming for
// RDTSCP as the default.
//
// RDTSCP clobbers ECX with MSR_TSC_AUX.
extern "C" {
    pub fn EAX_EDX_VAL(_arg: val, _arg: low, _arg: high) -> return;
}
//
// Standard way to access the cycle counter.
//
pub type cycles_t = c_ulonglong;
extern "C" {
    pub fn disable_TSC();
}
extern "C" {
    pub fn rdtsc() -> return;
}

extern "C" {
    pub fn tsc_early_init();
}
extern "C" {
    pub fn tsc_init();
}
extern "C" {
    pub fn mark_tsc_unstable(reason: *mut c_char);
}
extern "C" {
    pub fn unsynchronized_tsc() -> c_int;
}
extern "C" {
    pub fn check_tsc_unstable() -> c_int;
}
extern "C" {
    pub fn mark_tsc_async_resets(reason: *mut c_char);
}
extern "C" {
    pub fn native_calibrate_cpu_early() -> c_ulong;
}
extern "C" {
    pub fn native_calibrate_tsc() -> c_ulong;
}
extern "C" {
    pub fn native_sched_clock_from_tsc(tsc: u64) -> c_ulonglong;
}
//
// Boot-time check whether the TSCs are synchronized across
// all CPUs/cores:
//
extern "C" {
    pub fn tsc_store_and_check_tsc_adjust(bootcpu: bool) -> bool;
}
extern "C" {
    pub fn tsc_verify_tsc_adjust(resume: bool);
}
extern "C" {
    pub fn check_tsc_sync_target();
}
extern "C" {
    pub fn notsc_setup(: *mut c_char) -> c_int;
}
extern "C" {
    pub fn tsc_save_sched_clock_state();
}
extern "C" {
    pub fn tsc_restore_sched_clock_state();
}
extern "C" {
    pub fn cpu_khz_from_msr() -> c_ulong;
}
